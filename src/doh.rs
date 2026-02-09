use crate::dns_utils::{base64url_encode, build_dns_query, parse_dns_response};
use crate::types::{Config, DoHMethod, DoHResult, DoHStatus, SHUTDOWN_REQUESTED};
use reqwest::{Client, StatusCode};

use std::net::{TcpStream, ToSocketAddrs};
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone)]
pub struct DoHTester {
    client: Client,
    config: Arc<Config>,
    verify_tls: bool,
}

impl DoHTester {
    pub fn new(config: Arc<Config>) -> Result<Self, anyhow::Error> {
        let timeout = Duration::from_secs(config.timeout as u64);

        let client = Client::builder()
            .timeout(timeout)
            .danger_accept_invalid_certs(false)
            .build()?;

        Ok(Self {
            client,
            config,
            verify_tls: true,
        })
    }

    pub fn with_insecure(config: Arc<Config>) -> Result<Self, anyhow::Error> {
        let timeout = Duration::from_secs(config.timeout as u64);

        let client = Client::builder()
            .timeout(timeout)
            .danger_accept_invalid_certs(true)
            .build()?;

        Ok(Self {
            client,
            config,
            verify_tls: false,
        })
    }

    pub async fn test_endpoint(
        &self,
        doh_url: &str,
        domain: &str,
        attempts: u32,
        min_success: u32,
    ) -> DoHResult {
        if SHUTDOWN_REQUESTED.load(std::sync::atomic::Ordering::Relaxed) {
            let (host, port) = parse_host_port(doh_url);
            return DoHResult::interrupted(doh_url.to_string(), host, port, attempts);
        }

        let (host, port) = parse_host_port(doh_url);
        let mut result = DoHResult::new(doh_url.to_string(), host.clone(), port, attempts);

        let tcp_result = test_tcp_connectivity(&host, port, self.config.timeout);
        result.tcp_ok = tcp_result.success;
        result.doh_server_ip = tcp_result.ip;

        if !tcp_result.success {
            result.notes.push(format!(
                "tcp_connect_failed:{}",
                tcp_result.error.unwrap_or_default()
            ));
            return classify_result(result, None);
        }

        let tls_result = test_tls_handshake(&host, port, self.config.timeout, self.verify_tls);
        result.tls_ok = tls_result.success;
        result.tls_info = tls_result.info;

        if !tls_result.success {
            result.notes.push(format!(
                "tls_handshake_failed:{}",
                tls_result.error.unwrap_or_default()
            ));
            return classify_result(result, None);
        }

        let mut success_count = 0;
        let mut last_ips = Vec::new();
        let mut last_method = None;
        let mut last_latency = None;

        for i in 0..attempts {
            if SHUTDOWN_REQUESTED.load(std::sync::atomic::Ordering::Relaxed) {
                result.notes.push(format!("interrupted_at_attempt{}", i + 1));
                break;
            }

            match self.try_query(doh_url, domain).await {
                Ok(Some((ips, latency, method))) => {
                    success_count += 1;
                    last_ips = ips;
                    last_method = Some(method);
                    last_latency = Some(latency);
                }
                Ok(None) => {
                    result.notes.push(format!("attempt{}:no_response", i + 1));
                }
                Err(e) => {
                    result.notes.push(format!("attempt{}:{}", i + 1, e));
                }
            }
        }

        result.successes = success_count;
        result.target_ips = last_ips.join(",");
        result.method = last_method;
        result.latency_ms = last_latency;

        if SHUTDOWN_REQUESTED.load(std::sync::atomic::Ordering::Relaxed) {
            result.status = DoHStatus::Interrupted;
        } else if success_count >= min_success {
            result.status = DoHStatus::Working;
        } else if success_count > 0 {
            result.status = DoHStatus::Flaky;
            result.notes.push("tls_ok_but_no_doh_answers".to_string());
        } else {
            result.notes.push("tls_ok_but_no_doh_answers".to_string());
        }

        result
    }

    async fn try_query(
        &self,
        doh_url: &str,
        domain: &str,
    ) -> Result<Option<(Vec<String>, f64, DoHMethod)>, String> {
        let query_wire = build_dns_query(domain);

        if let Some(result) = self.try_get_wire(doh_url, &query_wire).await? {
            return Ok(Some(result));
        }

        if let Some(result) = self.try_post_wire(doh_url, &query_wire).await? {
            return Ok(Some(result));
        }

        if let Some(result) = self.try_get_json(doh_url, domain).await? {
            return Ok(Some(result));
        }

        Ok(None)
    }

    async fn try_get_wire(
        &self,
        doh_url: &str,
        query_wire: &[u8],
    ) -> Result<Option<(Vec<String>, f64, DoHMethod)>, String> {
        let encoded = base64url_encode(query_wire);
        let url = format!("{}?dns={}", doh_url, encoded);

        let start = std::time::Instant::now();
        let response = self
            .client
            .get(&url)
            .header("accept", "application/dns-message")
            .header("user-agent", "doh_tester/1.0")
            .send()
            .await
            .map_err(|e| format!("GET-wire-exc:{}", e))?;

        let latency = start.elapsed().as_secs_f64() * 1000.0;

        if response.status() == StatusCode::OK {
            let content_type = response
                .headers()
                .get("content-type")
                .and_then(|v| v.to_str().ok())
                .unwrap_or_default();

            if content_type.contains("dns-message") {
                let bytes = response
                    .bytes()
                    .await
                    .map_err(|e| format!("GET-wire-parse-fail:{}", e))?;

                if let Some(ips) = parse_dns_response(&bytes) {
                    if !ips.is_empty() {
                        return Ok(Some((ips, latency, DoHMethod::GetWire)));
                    }
                } else {
                    return Err("GET-wire-parse-fail".to_string());
                }
            } else {
                return Err(format!("GET-wire-bad-content-type:{}", content_type));
            }
        } else {
            return Err(format!("GET-wire-{}", response.status()));
        }

        Ok(None)
    }

    async fn try_post_wire(
        &self,
        doh_url: &str,
        query_wire: &[u8],
    ) -> Result<Option<(Vec<String>, f64, DoHMethod)>, String> {
        let start = std::time::Instant::now();
        let response = self
            .client
            .post(doh_url)
            .header("accept", "application/dns-message")
            .header("content-type", "application/dns-message")
            .header("user-agent", "doh_tester/1.0")
            .body(query_wire.to_vec())
            .send()
            .await
            .map_err(|e| format!("POST-wire-exc:{}", e))?;

        let latency = start.elapsed().as_secs_f64() * 1000.0;

        if response.status() == StatusCode::OK {
            let content_type = response
                .headers()
                .get("content-type")
                .and_then(|v| v.to_str().ok())
                .unwrap_or_default();

            if content_type.contains("dns-message") {
                let bytes = response
                    .bytes()
                    .await
                    .map_err(|e| format!("POST-wire-parse-fail:{}", e))?;

                if let Some(ips) = parse_dns_response(&bytes) {
                    if !ips.is_empty() {
                        return Ok(Some((ips, latency, DoHMethod::PostWire)));
                    }
                } else {
                    return Err("POST-wire-parse-fail".to_string());
                }
            } else {
                return Err(format!("POST-wire-bad-content-type:{}", content_type));
            }
        } else {
            return Err(format!("POST-wire-{}", response.status()));
        }

        Ok(None)
    }

    async fn try_get_json(
        &self,
        doh_url: &str,
        domain: &str,
    ) -> Result<Option<(Vec<String>, f64, DoHMethod)>, String> {
        let start = std::time::Instant::now();
        let response = self
            .client
            .get(doh_url)
            .query(&[("name", domain), ("type", "A")])
            .header("accept", "application/dns-json")
            .header("user-agent", "doh_tester/1.0")
            .send()
            .await
            .map_err(|e| format!("GET-json-exc:{}", e))?;

        let latency = start.elapsed().as_secs_f64() * 1000.0;

        if response.status() == StatusCode::OK {
            let content_type = response
                .headers()
                .get("content-type")
                .and_then(|v| v.to_str().ok())
                .unwrap_or_default();

            if content_type.contains("json") {
                let json: serde_json::Value = response
                    .json()
                    .await
                    .map_err(|e| format!("GET-json-parse-fail:{}", e))?;

                let mut ips = Vec::new();
                if let Some(answers) = json.get("Answer").and_then(|a| a.as_array()) {
                    for ans in answers {
                        if ans.get("type").and_then(|t| t.as_u64()) == Some(1) {
                            if let Some(data) = ans.get("data").and_then(|d| d.as_str()) {
                                ips.push(data.to_string());
                            }
                        }
                    }
                }

                if !ips.is_empty() {
                    return Ok(Some((ips, latency, DoHMethod::GetJson)));
                }
            } else {
                return Err(format!("GET-json-bad-content-type:{}", content_type));
            }
        } else {
            return Err(format!("GET-json-{}", response.status()));
        }

        Ok(None)
    }
}

struct TcpResult {
    success: bool,
    ip: Option<String>,
    error: Option<String>,
}

fn test_tcp_connectivity(host: &str, port: u16, timeout: f64) -> TcpResult {
    let timeout = Duration::from_secs_f64(timeout);

    if let Ok(addrs) = (host, port).to_socket_addrs() {
        for addr in addrs {
            match TcpStream::connect_timeout(&addr, timeout) {
                Ok(stream) => {
                    // Connection succeeded — that's all we need to verify
                    drop(stream);
                    return TcpResult {
                        success: true,
                        ip: Some(addr.ip().to_string()),
                        error: None,
                    };
                }
                Err(_) => continue,
            }
        }
    }

    TcpResult {
        success: false,
        ip: None,
        error: Some("TCP connection failed".to_string()),
    }
}

struct TlsResult {
    success: bool,
    info: Option<String>,
    error: Option<String>,
}

fn test_tls_handshake(host: &str, port: u16, timeout: f64, verify: bool) -> TlsResult {
    let timeout_dur = Duration::from_secs_f64(timeout);

    if let Ok(addrs) = (host, port).to_socket_addrs() {
        for addr in addrs {
            if let Ok(stream) = TcpStream::connect_timeout(&addr, timeout_dur) {
                let _ = stream.set_read_timeout(Some(timeout_dur));
                let _ = stream.set_write_timeout(Some(timeout_dur));

                let connector = if verify {
                    match native_tls::TlsConnector::new() {
                        Ok(c) => c,
                        Err(e) => {
                            return TlsResult {
                                success: false,
                                info: None,
                                error: Some(format!("TLS connector build error: {}", e)),
                            };
                        }
                    }
                } else {
                    match native_tls::TlsConnector::builder()
                        .danger_accept_invalid_hostnames(true)
                        .danger_accept_invalid_certs(true)
                        .build()
                    {
                        Ok(c) => c,
                        Err(e) => {
                            return TlsResult {
                                success: false,
                                info: None,
                                error: Some(format!("TLS connector build error: {}", e)),
                            };
                        }
                    }
                };

                match connector.connect(host, stream) {
                    Ok(tls_stream) => {
                        let peer_cert = tls_stream.peer_certificate();
                        let info = match &peer_cert {
                            Ok(Some(_cert)) => "TLS handshake OK, certificate present".to_string(),
                            Ok(None) => "TLS handshake OK, no peer certificate".to_string(),
                            Err(e) => format!("TLS handshake OK, cert error: {}", e),
                        };

                        return TlsResult {
                            success: true,
                            info: Some(info),
                            error: None,
                        };
                    }
                    Err(e) => {
                        return TlsResult {
                            success: false,
                            info: None,
                            error: Some(format!("TLS handshake failed: {}", e)),
                        };
                    }
                }
            }
        }
    }

    TlsResult {
        success: false,
        info: None,
        error: Some("TLS handshake failed: no addresses resolved".to_string()),
    }
}

fn parse_host_port(url: &str) -> (String, u16) {
    let stripped = url.strip_prefix("https://").unwrap_or(url);
    let stripped = stripped.strip_prefix("http://").unwrap_or(stripped);

    let host_port_part = if let Some(slash_pos) = stripped.find('/') {
        &stripped[..slash_pos]
    } else {
        stripped
    };

    let (host, port) = if let Some(col_pos) = host_port_part.rfind(':') {
        let potential_port = &host_port_part[col_pos + 1..];
        if let Ok(p) = potential_port.parse::<u16>() {
            (host_port_part[..col_pos].to_string(), p)
        } else {
            (host_port_part.to_string(), 443u16)
        }
    } else {
        (host_port_part.to_string(), 443u16)
    };

    if host.is_empty() {
        ("unknown".to_string(), port)
    } else {
        (host, port)
    }
}

fn classify_result(mut result: DoHResult, _blocked_reason: Option<&str>) -> DoHResult {
    if result.successes >= 2 {
        result.status = DoHStatus::Working;
    } else if result.successes > 0 {
        result.status = DoHStatus::Flaky;
    } else {
        result.status = DoHStatus::Blocked;
    }
    result
}

pub fn load_doh_list(path: &str) -> Result<Vec<String>, anyhow::Error> {
    let content = std::fs::read_to_string(path)?;
    let mut urls = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if !trimmed.is_empty() && !trimmed.starts_with('#') {
            urls.push(trimmed.to_string());
        }
    }

    Ok(urls)
}
