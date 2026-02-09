use crate::types::{Config, DoHResult};
use std::fs::File;
use std::io::Write;

pub fn write_results(results: &[DoHResult], config: &Config, output_path: &str, working_only: bool) {
    let filtered_results: Vec<&DoHResult> = if working_only {
        results.iter().filter(|r| r.status == crate::types::DoHStatus::Working).collect()
    } else {
        results.iter().collect()
    };

    write_text_output(&filtered_results, config, output_path, working_only);

    if !working_only || config.working_only {
        match &config.json_output {
            crate::types::JsonOutput::Disabled => {}
            crate::types::JsonOutput::Auto => {
                let json_path = format!("{}.json", chrono::Local::now().format("%Y-%m-%dT%H-%M-%S"));
                write_json_output(&filtered_results, &json_path);
            }
            crate::types::JsonOutput::Filename(filename) => {
                write_json_output(&filtered_results, filename);
            }
        }
    }
}

fn write_text_output(results: &[&DoHResult], config: &Config, output_path: &str, _working_only: bool) {
    let mut file = match File::create(output_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to create output file: {}", e);
            return;
        }
    };

    let mut lines = Vec::new();

    if config.show_headers {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        lines.push(format!("# Generated: {}", timestamp));

        let mut headers = Vec::new();
        let mut widths = Vec::new();

        if config.show_status {
            headers.push("STATUS");
            widths.push(8);
        }
        if config.show_doh_url {
            headers.push("URL");
            widths.push(40);
        }
        if config.show_host {
            headers.push("HOST");
            widths.push(25);
        }
        if config.show_doh_ip {
            headers.push("DOH_IP");
            widths.push(18);
        }
        if config.show_target_ip {
            headers.push("TARGET_IP");
            widths.push(18);
        }
        if config.show_ping {
            headers.push("PING_MS");
            widths.push(10);
        }

        let header_line = headers
            .iter()
            .zip(widths.iter())
            .map(|(h, w)| format!("{:<width$}", h, width = *w))
            .collect::<Vec<_>>()
            .join(" ");
        lines.push(header_line.clone());

        let separator = "-".repeat(header_line.len());
        lines.push(separator);
    }

    for result in results {
        let mut row = Vec::new();

        if config.show_status {
            row.push(format!("{:<8}", format!("{:?}", result.status)));
        }
        if config.show_doh_url {
            let url = if result.url.len() > 38 {
                &result.url[..38]
            } else {
                &result.url
            };
            row.push(format!("{:<40}", url));
        }
        if config.show_host {
            row.push(format!("{:<25}", result.host));
        }
        if config.show_doh_ip {
            let ip = result.doh_server_ip.as_deref().unwrap_or("-");
            row.push(format!("{:<18}", ip));
        }
        if config.show_target_ip {
            let ip = if result.target_ips.is_empty() { "-" } else { &result.target_ips };
            row.push(format!("{:<18}", ip));
        }
        if config.show_ping {
            let latency = result.latency_ms.map(|l| format!("{:.1}", l)).unwrap_or_else(|| "-".to_string());
            row.push(format!("{:<10}", latency));
        }

        lines.push(row.join(" "));
    }

    for line in &lines {
        if let Err(e) = writeln!(file, "{}", line) {
            eprintln!("Failed to write to file: {}", e);
        }
    }

    println!("Results written to {}", output_path);
}

fn write_json_output(results: &[&DoHResult], output_path: &str) {
    let json_results: Vec<serde_json::Value> = results
        .iter()
        .map(|r| {
            serde_json::json!({
                "status": format!("{:?}", r.status),
                "url": r.url,
                "host": r.host,
                "port": r.port,
                "tcp_ok": r.tcp_ok,
                "tls_ok": r.tls_ok,
                "tls_info": r.tls_info,
                "doh_server_ip": r.doh_server_ip,
                "successes": r.successes,
                "attempts": r.attempts,
                "target_ips": r.target_ips,
                "method": r.method.as_ref().map(|m| format!("{:?}", m)),
                "latency_ms": r.latency_ms,
                "notes": r.notes.join(";")
            })
        })
        .collect();

    let json_output = serde_json::to_string_pretty(&json_results).unwrap_or_default();

    if let Err(e) = std::fs::write(output_path, &json_output) {
        eprintln!("Failed to write JSON output: {}", e);
    } else {
        println!("JSON results written to {}", output_path);
    }
}

pub fn print_summary(results: &[DoHResult], output_path: &str, config: &Config) {
    let working = results.iter().filter(|r| r.status == crate::types::DoHStatus::Working).count();
    let flaky = results.iter().filter(|r| r.status == crate::types::DoHStatus::Flaky).count();
    let blocked = results.iter().filter(|r| r.status == crate::types::DoHStatus::Blocked).count();
    let interrupted = results.iter().filter(|r| r.status == crate::types::DoHStatus::Interrupted).count();

    let mode = if config.working_only {
        "working-only"
    } else {
        "full details"
    };

    println!(
        "Done. Results saved to {}. WORKING={}, FLAKY={}, BLOCKED={}{} [{}]",
        output_path,
        working,
        flaky,
        blocked,
        if interrupted > 0 { format!(", INTERRUPTED={}", interrupted) } else { String::new() },
        mode
    );
}

pub fn clean_output(results: &[DoHResult], output_path: &str) {
    let mut file = match File::create(output_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to create output file: {}", e);
            return;
        }
    };

    for result in results {
        if result.status == crate::types::DoHStatus::Working {
            if let Err(e) = writeln!(file, "{}", result.url) {
                eprintln!("Failed to write to file: {}", e);
            }
        }
    }

    println!("Clean URLs written to {}", output_path);
}
