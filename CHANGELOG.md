# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-02-09

### Added

- **Full Rust rewrite** of the original Python DoH tester for dramatically improved performance and reliability.
- **TCP connectivity testing** — verifies host:port reachability with configurable timeout.
- **TLS handshake verification** — validates certificate chains using `native-tls`, with optional `--insecure` bypass for self-signed certificates.
- **Multi-protocol DoH queries** — automatically tries three methods in order:
  - `GET` wire format (RFC 8484, base64url-encoded `?dns=` parameter)
  - `POST` wire format (RFC 8484, binary body)
  - `GET` JSON API (Google/Cloudflare-compatible `?name=&type=` parameters)
- **Smart endpoint classification** — each endpoint is labelled as `WORKING`, `FLAKY`, `BLOCKED`, or `INTERRUPTED` based on configurable success thresholds.
- **Parallel testing** with semaphore-bounded `tokio` task pool (`--workers` flag).
- **Live progress output** — real-time `[N/total] ✓/✗ url (latency)` lines printed to stdout as each endpoint completes.
- **Graceful Ctrl+C shutdown** — background signal listener sets an `AtomicBool` flag; in-progress tasks drain and partial results are preserved.
- **Flexible output formats:**
  - Tabular text file with configurable columns (`show_status`, `show_doh_url`, `show_host`, `show_doh_ip`, `show_target_ip`, `show_ping`).
  - Clean URL list (`--clean-output`) — one working URL per line, ideal for piping.
  - Structured JSON (`--json-output`) — full result data for every tested endpoint.
- **JSON configuration file** (`config.json`) — auto-created with sensible defaults on first run; all settings overridable via CLI flags.
- **Custom DNS packet builder and parser** — zero-dependency wire-format construction and A-record extraction without external DNS libraries.
- **Base64url encoder** — RFC 4648 §5 encoding without padding for DoH GET wire queries.
- **Per-endpoint latency measurement** in milliseconds.
- **Bundled `doh.txt`** with ~490 public DoH endpoints sourced from community lists.
- **Unit tests** for DNS packet building, response parsing, base64url encoding, and name compression handling.
- **GitHub Actions CI** — automated build, test, and release workflows for Windows, Linux, and macOS.
- **Comprehensive documentation** — README, CONTRIBUTING, CODE_OF_CONDUCT, SECURITY policy, issue and PR templates.

### Changed

- Replaced Python `requests` + `dnspython` stack with `reqwest` (async HTTP) + hand-rolled DNS wire format for a fully self-contained binary.
- Replaced Python `concurrent.futures.ThreadPoolExecutor` with `tokio` async runtime and `JoinSet` for true async I/O.
- Replaced `signal.signal(SIGINT, ...)` handler with a `tokio::signal::ctrl_c()` background task for cross-platform shutdown.
- Replaced PyInstaller packaging with native Cargo release builds (LTO, single codegen unit) producing a ~4 MB static binary.

### Performance

- **~10–15× faster** than the Python original on the same endpoint list and settings.
- **~6× lower memory** usage (≈12 MB vs ≈80 MB).
- **~4× smaller binary** (≈4 MB vs ≈15 MB PyInstaller bundle).

---

_For the original Python version's history, see [BLACKGAMER1221/doh_tester](https://github.com/BLACKGAMER1221/doh_tester)._

[1.0.0]: https://github.com/SkipTutorial/doh_tester/releases/tag/v1.0.0