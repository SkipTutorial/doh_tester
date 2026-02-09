use clap::Parser;
use std::path::Path;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use tokio::task::JoinSet;

mod dns_utils;
mod doh;
mod output;
mod types;

use types::{Config, DoHResult, DoHStatus, SHUTDOWN_REQUESTED};
use doh::{load_doh_list, DoHTester};
use output::{clean_output, print_summary, write_results};

#[derive(Parser, Debug)]
#[command(name = "doh_tester")]
#[command(author = "DoH Tester Contributors")]
#[command(version = "1.0.0")]
#[command(about = "High-performance DNS-over-HTTPS endpoint tester", long_about = None)]
struct Args {
    #[arg(help = "Domain to resolve (e.g. example.com)")]
    domain: String,

    #[arg(short, long, default_value = "config.json")]
    #[arg(help = "Path to config.json")]
    config: String,

    #[arg(short = 'f', long)]
    #[arg(help = "Path to doh.txt (default from config)")]
    doh_file: Option<String>,

    #[arg(short, long)]
    #[arg(help = "Per-operation timeout in seconds (default from config)")]
    timeout: Option<f64>,

    #[arg(short, long)]
    #[arg(help = "Parallel worker threads (default from config)")]
    workers: Option<usize>,

    #[arg(short, long)]
    #[arg(help = "DNS query attempts per DoH (default from config)")]
    attempts: Option<u32>,

    #[arg(short = 'm', long)]
    #[arg(help = "Min successful replies to mark WORKING (default from config)")]
    min_success: Option<u32>,

    #[arg(long)]
    #[arg(help = "Skip TLS certificate verification")]
    insecure: bool,

    #[arg(short, long)]
    #[arg(help = "Output file (default: timestamped)")]
    output: Option<String>,

    #[arg(short = 'W', long)]
    #[arg(help = "Show only WORKING results")]
    working_only: bool,

    #[arg(long)]
    #[arg(help = "Output only working DoH URLs (one per line)")]
    clean_output: bool,

    #[arg(long)]
    #[arg(help = "Write JSON output (auto-timestamp or specify path)")]
    json_output: Option<String>,
}

fn load_or_create_config(config_path: &str) -> Config {
    if Path::new(config_path).exists() {
        if let Ok(config) = Config::load_from_file(config_path) {
            println!("Loaded config from {}", config_path);
            return config;
        }
        println!("Warning: Invalid config file, using defaults");
    }

    let default_config = Config::default();
    if let Err(e) = default_config.save_to_file(config_path) {
        println!("Note: Could not create default config file: {}", e);
    } else {
        println!("Created default config file: {}", config_path);
    }

    default_config
}

async fn run_tests(args: &Args, config: &Config) -> Vec<DoHResult> {
    let doh_file = args.doh_file.as_deref().unwrap_or(&config.doh_file);

    if !Path::new(doh_file).exists() {
        eprintln!("doh file not found: {}", doh_file);
        std::process::exit(2);
    }

    let doh_list = match load_doh_list(doh_file) {
        Ok(list) => list,
        Err(e) => {
            eprintln!("Failed to load doh file: {}", e);
            std::process::exit(1);
        }
    };

    if doh_list.is_empty() {
        eprintln!("No DoH endpoints found in {}", doh_file);
        std::process::exit(1);
    }

    println!(
        "Testing {} DoH endpoints for domain {} (verify_tls={})",
        doh_list.len(),
        args.domain,
        !args.insecure
    );

    let timeout = args.timeout.unwrap_or(config.timeout);
    let workers = args.workers.unwrap_or(config.workers);
    let attempts = args.attempts.unwrap_or(config.attempts);
    let min_success = args.min_success.unwrap_or(config.min_success);

    println!(
        "Settings: timeout={}s, workers={}, attempts={}, min_success={}",
        timeout, workers, attempts, min_success
    );

    let config_arc = Arc::new(config.clone());

    let doh_tester = if args.insecure {
        match DoHTester::with_insecure(Arc::clone(&config_arc)) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Failed to create DoH tester: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        match DoHTester::new(Arc::clone(&config_arc)) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Failed to create DoH tester: {}", e);
                std::process::exit(1);
            }
        }
    };

    // Use a semaphore to limit concurrency to the configured worker count
    let semaphore = Arc::new(tokio::sync::Semaphore::new(workers));
    let mut join_set = JoinSet::new();

    for doh_url in &doh_list {
        let doh_url = doh_url.clone();
        let domain = args.domain.clone();
        let tester = doh_tester.clone();
        let sem = Arc::clone(&semaphore);

        join_set.spawn(async move {
            let _permit = sem.acquire().await;
            tester
                .test_endpoint(&doh_url, &domain, attempts, min_success)
                .await
        });
    }

    let mut results = Vec::new();
    let total = doh_list.len();
    let mut completed = 0;

    while let Some(result) = join_set.join_next().await {
        match result {
            Ok(r) => {
                completed += 1;
                let status_symbol = match r.status {
                    DoHStatus::Working => "✓",
                    DoHStatus::Flaky => "~",
                    DoHStatus::Blocked => "✗",
                    DoHStatus::Interrupted => "!",
                };
                println!(
                    "[{}/{}] {} {} ({}ms)",
                    completed,
                    total,
                    status_symbol,
                    r.url,
                    r.latency_ms
                        .map(|l| format!("{:.0}", l))
                        .unwrap_or_else(|| "-".to_string())
                );
                results.push(r);

                if SHUTDOWN_REQUESTED.load(Ordering::Relaxed) {
                    println!("Shutdown requested, aborting remaining tasks...");
                    join_set.shutdown().await;
                    break;
                }
            }
            Err(e) => {
                completed += 1;
                eprintln!("[{}/{}] Task error: {:?}", completed, total, e);
            }
        }
    }

    results
}

/// Set up a background task that listens for Ctrl+C and sets the shutdown flag.
fn spawn_shutdown_listener() {
    tokio::spawn(async {
        if let Ok(()) = tokio::signal::ctrl_c().await {
            println!("\nShutdown requested (Ctrl+C)...");
            SHUTDOWN_REQUESTED.store(true, Ordering::Relaxed);
        }
    });
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let config = load_or_create_config(&args.config);

    // Determine output path: CLI flag > config file > auto-timestamp
    let output_path = if let Some(ref o) = args.output {
        o.clone()
    } else if !config.output_file.is_empty() {
        config.output_file.clone()
    } else {
        chrono::Local::now()
            .format("%Y-%m-%dT%H-%M-%S.txt")
            .to_string()
    };

    // Spawn a background Ctrl+C listener instead of using tokio::select!
    spawn_shutdown_listener();

    let results = run_tests(&args, &config).await;

    if results.is_empty() {
        println!("No results collected.");
        return;
    }

    if args.clean_output {
        clean_output(&results, &output_path);
    } else {
        let working_only = args.working_only || config.working_only;
        write_results(&results, &config, &output_path, working_only);
    }

    // Handle --json_output flag
    if let Some(ref json_path) = args.json_output {
        let path = if json_path.is_empty() {
            chrono::Local::now()
                .format("%Y-%m-%dT%H-%M-%S.json")
                .to_string()
        } else {
            json_path.clone()
        };
        let json_data = serde_json::to_string_pretty(&results).unwrap_or_default();
        if let Err(e) = std::fs::write(&path, &json_data) {
            eprintln!("Failed to write JSON output: {}", e);
        } else {
            println!("JSON results written to {}", path);
        }
    }

    print_summary(&results, &output_path, &config);
}
