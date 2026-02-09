use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::sync::atomic::AtomicBool;

pub static SHUTDOWN_REQUESTED: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DoHStatus {
    #[serde(rename = "WORKING")]
    Working,
    #[serde(rename = "FLAKY")]
    Flaky,
    #[serde(rename = "BLOCKED")]
    Blocked,
    #[serde(rename = "INTERRUPTED")]
    Interrupted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DoHMethod {
    #[serde(rename = "GET-wire")]
    GetWire,
    #[serde(rename = "POST-wire")]
    PostWire,
    #[serde(rename = "GET-json")]
    GetJson,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoHResult {
    pub status: DoHStatus,
    pub url: String,
    pub host: String,
    pub port: u16,
    pub tcp_ok: bool,
    pub tls_ok: bool,
    pub tls_info: Option<String>,
    pub doh_server_ip: Option<String>,
    pub successes: u32,
    pub attempts: u32,
    pub target_ips: String,
    pub method: Option<DoHMethod>,
    pub latency_ms: Option<f64>,
    pub notes: Vec<String>,
}

impl DoHResult {
    pub fn new(url: String, host: String, port: u16, attempts: u32) -> Self {
        Self {
            status: DoHStatus::Blocked,
            url,
            host,
            port,
            tcp_ok: false,
            tls_ok: false,
            tls_info: None,
            doh_server_ip: None,
            successes: 0,
            attempts,
            target_ips: String::new(),
            method: None,
            latency_ms: None,
            notes: Vec::new(),
        }
    }

    pub fn interrupted(url: String, host: String, port: u16, attempts: u32) -> Self {
        let mut result = Self::new(url, host, port, attempts);
        result.status = DoHStatus::Interrupted;
        result.notes.push("interrupted_before_testing".to_string());
        result
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub doh_file: String,
    pub output_file: String,
    pub timeout: f64,
    pub workers: usize,
    pub attempts: u32,
    pub min_success: u32,
    pub remove_working_from_doh_file: bool,
    pub working_only: bool,
    #[serde(default)]
    pub json_output: JsonOutput,
    pub show_headers: bool,
    pub show_status: bool,
    pub show_doh_url: bool,
    pub show_host: bool,
    pub show_doh_ip: bool,
    pub show_target_ip: bool,
    pub show_ping: bool,
}

/// Represents the JSON output configuration.
///
/// In `config.json` this field can be:
///   - `false`          → Disabled (no JSON output)
///   - `true`           → Auto (timestamped JSON file)
///   - `"some_file.json"` → Write JSON to the given filename
#[derive(Debug, Clone, Default)]
pub enum JsonOutput {
    #[default]
    Disabled,
    Auto,
    Filename(String),
}

impl Serialize for JsonOutput {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            JsonOutput::Disabled => serializer.serialize_bool(false),
            JsonOutput::Auto => serializer.serialize_bool(true),
            JsonOutput::Filename(name) => serializer.serialize_str(name),
        }
    }
}

impl<'de> Deserialize<'de> for JsonOutput {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{self, Visitor};

        struct JsonOutputVisitor;

        impl<'de> Visitor<'de> for JsonOutputVisitor {
            type Value = JsonOutput;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a boolean or a string")
            }

            fn visit_bool<E: de::Error>(self, value: bool) -> Result<JsonOutput, E> {
                if value {
                    Ok(JsonOutput::Auto)
                } else {
                    Ok(JsonOutput::Disabled)
                }
            }

            fn visit_str<E: de::Error>(self, value: &str) -> Result<JsonOutput, E> {
                if value.is_empty() {
                    Ok(JsonOutput::Disabled)
                } else {
                    Ok(JsonOutput::Filename(value.to_string()))
                }
            }

            fn visit_string<E: de::Error>(self, value: String) -> Result<JsonOutput, E> {
                if value.is_empty() {
                    Ok(JsonOutput::Disabled)
                } else {
                    Ok(JsonOutput::Filename(value))
                }
            }
        }

        deserializer.deserialize_any(JsonOutputVisitor)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            doh_file: "doh.txt".to_string(),
            output_file: String::new(),
            timeout: 8.0,
            workers: 20,
            attempts: 3,
            min_success: 2,
            remove_working_from_doh_file: false,
            working_only: false,
            json_output: JsonOutput::Disabled,
            show_headers: true,
            show_status: true,
            show_doh_url: true,
            show_host: true,
            show_doh_ip: true,
            show_target_ip: false,
            show_ping: true,
        }
    }
}

impl Config {
    pub fn load_from_file(path: &str) -> Result<Self, anyhow::Error> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), anyhow::Error> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}
