<div align="center">

# 🔒 DoH Tester

**High-performance DNS-over-HTTPS endpoint tester written in Rust**

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build](https://img.shields.io/github/actions/workflow/status/SkipTutorial/doh_tester/build.yml?branch=main&logo=github)](https://github.com/SkipTutorial/doh_tester/actions)
[![Release](https://img.shields.io/github/v/release/SkipTutorial/doh_tester?logo=github)](https://github.com/SkipTutorial/doh_tester/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/SkipTutorial/doh_tester/total?logo=github)](https://github.com/SkipTutorial/doh_tester/releases)

🇺🇸 [English](README.md) · 🇷🇺 [Русский](README-RU.md) · 🇨🇳 [中文](README-ZH.md) · 🇮🇷 [فارسی](README-FA.md)

Validate hundreds of DoH endpoints in seconds — test TCP connectivity, TLS handshakes, and DNS resolution across multiple protocols with a single command.

<img src="terminal.PNG" alt="DoH Tester terminal output" width="700">

</div>

---

## 📖 Table of Contents

- [Features](#-features)
- [Quick Start](#-quick-start)
- [Installation](#-installation)
- [Usage](#-usage)
- [Configuration](#-configuration)
- [DoH File Format](#-doh-file-format)
- [Output Formats](#-output-formats)
- [Architecture](#-architecture)
- [Performance](#-performance)
- [Building from Source](#-building-from-source)
- [Contributing](#-contributing)
- [License](#-license)

---

## ✨ Features

| Feature | Description |
|---|---|
| 🔌 **TCP Connectivity** | Tests raw TCP connections to each endpoint (IPv4 & IPv6) |
| 🔐 **TLS Verification** | Full TLS handshake with optional `--insecure` bypass |
| 📡 **Multi-Protocol DoH** | GET wire (RFC 8484), POST wire (RFC 8484), GET JSON API |
| ⚡ **Parallel Workers** | Configurable concurrency via semaphore-based task pool |
| 🏷️ **Smart Classification** | Endpoints classified as **WORKING**, **FLAKY**, **BLOCKED**, or **INTERRUPTED** |
| ⏱️ **Latency Measurement** | Per-endpoint response time in milliseconds |
| 📊 **Flexible Output** | Tabular text, clean URL lists, and structured JSON |
| 🛑 **Graceful Shutdown** | Ctrl+C stops new work and preserves partial results |
| 📝 **JSON Config** | Persistent settings via `config.json` — auto-created on first run |
| 🔄 **Live Progress** | Real-time `[N/total] ✓/✗` status for every endpoint |

---

## 🚀 Quick Start

```bash
# Download the latest release (or build from source — see below)
# Then run against the bundled endpoint list:
doh_tester example.com
```

That's it. The tool loads `config.json` and `doh.txt` from the current directory, tests every endpoint concurrently, writes a timestamped results file, and prints a summary.

---

## 📦 Installation

### Pre-built Binaries

Download the latest binary for your platform from [**Releases**](https://github.com/SkipTutorial/doh_tester/releases/latest):

| Platform | Asset |
|---|---|
| Windows x64 | `doh_tester-x86_64-pc-windows-gnu.exe` |
| Linux x64 | `doh_tester-x86_64-unknown-linux-gnu` |
| macOS x64 | `doh_tester-x86_64-apple-darwin` |
| macOS ARM | `doh_tester-aarch64-apple-darwin` |

### From Source

```bash
git clone https://github.com/SkipTutorial/doh_tester.git
cd doh_tester
cargo build --release
# Binary: target/release/doh_tester(.exe)
```

> See [Building from Source](#-building-from-source) for detailed platform-specific instructions.

### Cargo Install

```bash
cargo install --git https://github.com/SkipTutorial/doh_tester.git
```

---

## 🛠️ Usage

```
doh_tester [OPTIONS] <DOMAIN>
```

### Arguments

| Argument | Description |
|---|---|
| `<DOMAIN>` | Domain name to resolve (required) — e.g. `example.com` |

### Options

| Flag | Long | Description | Default |
|---|---|---|---|
| `-c` | `--config` | Path to configuration file | `config.json` |
| `-f` | `--doh-file` | Path to DoH endpoint list | from config |
| `-t` | `--timeout` | Per-operation timeout (seconds) | `8.0` |
| `-w` | `--workers` | Number of parallel workers | `20` |
| `-a` | `--attempts` | DNS query attempts per endpoint | `3` |
| `-m` | `--min-success` | Minimum successes to mark WORKING | `2` |
| | `--insecure` | Skip TLS certificate verification | `false` |
| `-o` | `--output` | Output file path | timestamped |
| `-W` | `--working-only` | Only include WORKING results | `false` |
| | `--clean-output` | Output only working URLs (one per line) | `false` |
| | `--json-output` | Write JSON output (optional path) | — |
| `-h` | `--help` | Print help | |
| `-V` | `--version` | Print version | |

### Examples

```bash
# Basic test with defaults
doh_tester example.com

# Fast scan: low timeout, single attempt, many workers
doh_tester google.com -t 3 -a 1 -m 1 -w 50

# Export only working endpoints as a clean list
doh_tester example.com --clean-output -o working_servers.txt

# Full JSON report
doh_tester example.com --json-output results.json

# Working-only results with custom DoH list
doh_tester example.com -W -f my_endpoints.txt

# Test a private DoH server with a self-signed certificate
doh_tester internal.corp --insecure -f private_doh.txt

# Override config file location
doh_tester example.com -c /etc/doh_tester/config.json
```

---

## ⚙️ Configuration

On first run, if `config.json` doesn't exist, the tool creates one with sensible defaults. All CLI flags override their corresponding config values.

```jsonc
{
  "doh_file": "doh.txt",           // Default DoH endpoint list
  "output_file": "",                // "" = auto-timestamped filename
  "timeout": 8.0,                   // Seconds per operation
  "workers": 20,                    // Parallel tasks
  "attempts": 3,                    // DNS queries per endpoint
  "min_success": 2,                 // Threshold for WORKING status
  "remove_working_from_doh_file": false,
  "working_only": false,            // Filter results to WORKING only
  "json_output": false,             // false | true (auto) | "filename.json"
  "show_headers": true,             // Table column headers
  "show_status": true,              // WORKING / BLOCKED / etc.
  "show_doh_url": true,             // Endpoint URL column
  "show_host": true,                // Hostname column
  "show_doh_ip": true,              // Resolved DoH server IP
  "show_target_ip": false,          // Resolved target domain IP
  "show_ping": true                 // Latency column
}
```

### `json_output` Values

| Value | Behaviour |
|---|---|
| `false` | No JSON output |
| `true` | Write JSON to an auto-timestamped file |
| `"report.json"` | Write JSON to the specified path |

---

## 📄 DoH File Format

`doh.txt` contains one HTTPS URL per line. Blank lines and lines starting with `#` are ignored.

```text
# ── Public Resolvers ────────────────────────
https://cloudflare-dns.com/dns-query
https://dns.google/dns-query
https://dns.quad9.net/dns-query
https://doh.opendns.com/dns-query

# ── Privacy-focused ─────────────────────────
https://dns.mullvad.net/dns-query
https://doh.applied-privacy.net/query
https://dns.adguard-dns.com/dns-query

# ── Regional / Self-hosted ──────────────────
https://doh.internal.example.com/dns-query
```

> 💡 The bundled `doh.txt` ships with ~490 public endpoints sourced from community lists.

---

## 📊 Output Formats

### Text Table (default)

Written to the output file and displayed via `--show-*` config flags:

```
# Generated: 2026-02-09 12:30:00
STATUS   URL                                      HOST                      DOH_IP             PING_MS
---------------------------------------------------------------------------------------------------------
Working  https://cloudflare-dns.com/dns-query      cloudflare-dns.com        104.16.249.249     45.2
Working  https://dns.google/dns-query              dns.google                8.8.8.8            32.1
Blocked  https://blocked.example/dns-query         blocked.example           -                  -
Flaky    https://unreliable.example/dns-query      unreliable.example        192.0.2.1          120.5
```

### Clean Output (`--clean-output`)

One working URL per line — ideal for piping into other tools:

```
https://cloudflare-dns.com/dns-query
https://dns.google/dns-query
https://dns.quad9.net/dns-query
```

### JSON (`--json-output`)

Machine-readable structured data for every tested endpoint:

```json
[
  {
    "status": "WORKING",
    "url": "https://cloudflare-dns.com/dns-query",
    "host": "cloudflare-dns.com",
    "port": 443,
    "tcp_ok": true,
    "tls_ok": true,
    "tls_info": "TLS handshake OK, certificate present",
    "doh_server_ip": "104.16.249.249",
    "successes": 3,
    "attempts": 3,
    "target_ips": "93.184.216.34",
    "method": "GET-wire",
    "latency_ms": 45.2,
    "notes": []
  }
]
```

### Live Progress

While running, the tool prints real-time status to `stdout`:

```
Loaded config from config.json
Testing 492 DoH endpoints for domain example.com (verify_tls=true)
Settings: timeout=8s, workers=20, attempts=3, min_success=2
[1/492] ✓ https://cloudflare-dns.com/dns-query (45ms)
[2/492] ✓ https://dns.google/dns-query (32ms)
[3/492] ✗ https://blocked.example/dns-query (-ms)
...
Done. Results saved to 2026-02-09T12-30-00.txt. WORKING=385, FLAKY=12, BLOCKED=95 [full details]
```

---

## 🏗️ Architecture

```
src/
├── main.rs        CLI parsing, task orchestration, shutdown handling
├── types.rs       Config, DoHResult, DoHStatus, enums, serde logic
├── doh.rs         DoHTester — TCP/TLS probes, GET/POST wire & JSON queries
├── dns_utils.rs   Raw DNS packet builder/parser, base64url encoding
└── output.rs      Text table writer, JSON serialiser, summary printer
```

### Testing Pipeline per Endpoint

```
┌──────────┐     ┌───────────┐     ┌──────────────────┐     ┌──────────────┐
│ TCP Test │────▶│ TLS Test  │────▶│ DoH Query (×N)   │────▶│ Classify     │
│ connect  │     │ handshake │     │ GET wire → POST  │     │ WORKING /    │
│ timeout  │     │ cert info │     │ wire → GET JSON   │     │ FLAKY /      │
└──────────┘     └───────────┘     └──────────────────┘     │ BLOCKED      │
     │ fail           │ fail              │                  └──────────────┘
     ▼                ▼                   ▼
  BLOCKED          BLOCKED         count successes
```

1. **TCP connect** — Verifies the host:port is reachable within the timeout.
2. **TLS handshake** — Validates the certificate chain (unless `--insecure`).
3. **DoH queries** — Tries up to three methods in order: GET wire-format, POST wire-format, GET JSON API. Repeats for `--attempts` rounds.
4. **Classification** — `successes >= min_success` → WORKING, `> 0` → FLAKY, else → BLOCKED.

### Concurrency Model

All endpoints are spawned as tokio tasks behind a counting semaphore sized to `--workers`, preventing resource exhaustion while maximising throughput. A background task watches for `Ctrl+C`, sets a global `AtomicBool` flag, and the worker loop drains gracefully.

---

## ⚡ Performance

| Metric | Python Original | Rust Fork |
|---|---|---|
| 492 endpoints, 3 attempts | ~8 min | **~35 sec** |
| Memory usage | ~80 MB | **~12 MB** |
| Binary size (release) | ~15 MB (PyInstaller) | **~4 MB** |
| Dependencies at runtime | Python 3.8+ | **None (static)** |

> Benchmarked on Windows 11, 20 workers, 8 s timeout, gigabit connection.

---

## 🔨 Building from Source

### Prerequisites

| Platform | Requirements |
|---|---|
| **All** | [Rust 1.70+](https://rustup.rs/) |
| **Windows (GNU)** | MinGW-w64 — `scoop install mingw` or [manual install](https://www.mingw-w64.org/) |
| **Windows (MSVC)** | [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/) with C++ workload + Windows SDK |
| **Linux** | `build-essential`, `libssl-dev`, `pkg-config` |
| **macOS** | Xcode Command Line Tools (`xcode-select --install`) |

### Build Commands

```bash
# Debug build
cargo build

# Optimised release build (recommended)
cargo build --release

# Run tests
cargo test

# Run clippy lints
cargo clippy

# Windows GNU — if dlltool is not on PATH:
export PATH="$HOME/scoop/apps/mingw/current/bin:$PATH"
cargo build --release
```

### Cross-Compilation

```bash
# Linux → Windows
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu

# macOS → Linux (requires cross-linker)
cargo install cross
cross build --release --target x86_64-unknown-linux-gnu
```

---

## 🏷️ Status Classifications

| Status | Symbol | Meaning |
|---|---|---|
| **WORKING** | ✓ | TCP ✓, TLS ✓, and ≥ `min_success` DNS queries returned valid IPs |
| **FLAKY** | ~ | TCP ✓, TLS ✓, but fewer than `min_success` queries succeeded |
| **BLOCKED** | ✗ | TCP failed, TLS failed, or zero successful DNS queries |
| **INTERRUPTED** | ! | Testing was aborted before this endpoint could complete |

---

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

1. Fork the repository
2. Create a feature branch (`git checkout -b feat/amazing-feature`)
3. Make your changes and add tests
4. Run `cargo test && cargo clippy`
5. Commit (`git commit -m 'feat: add amazing feature'`)
6. Push and open a Pull Request

---

## 📜 License

This project is licensed under the [MIT License](LICENSE).

---

## 🔗 Related

- 🐍 **Original Python version** — [BLACKGAMER1221/doh_tester](https://github.com/BLACKGAMER1221/doh_tester)
- 📄 **RFC 8484** — [DNS Queries over HTTPS (DoH)](https://datatracker.ietf.org/doc/html/rfc8484)
- 📄 **RFC 8310** — [Usage Profiles for DNS over TLS and DNS over DTLS](https://datatracker.ietf.org/doc/html/rfc8310)
- 📄 **Google DNS JSON API** — [developers.google.com/speed/public-dns/docs/doh/json](https://developers.google.com/speed/public-dns/docs/doh/json)

---

<div align="center">

**If this tool is useful to you, consider giving it a ⭐**

</div>