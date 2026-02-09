# Security Policy

## Supported Versions

| Version | Supported          |
|---------|--------------------|
| 1.x.x   | ✅ Active support  |
| < 1.0   | ❌ Not supported   |

## Reporting a Vulnerability

We take the security of DoH Tester seriously. If you discover a security vulnerability, please report it responsibly.

### 🔒 Private Disclosure

**Please do NOT open a public GitHub issue for security vulnerabilities.**

Instead, report vulnerabilities through one of these channels:

1. **GitHub Private Advisory** — [Open a security advisory](https://github.com/SkipTutorial/doh_tester/security/advisories/new) (preferred)
2. **Email** — Send details to the maintainers listed in `Cargo.toml`

### What to Include

Please provide as much of the following as possible:

- A clear description of the vulnerability
- Steps to reproduce the issue
- The affected version(s)
- The potential impact (e.g. data leak, denial of service, code execution)
- A suggested fix or patch, if you have one

### What to Expect

| Timeframe | Action |
|---|---|
| **48 hours** | We will acknowledge receipt of your report |
| **7 days** | We will provide an initial assessment and severity rating |
| **30 days** | We aim to release a fix for confirmed vulnerabilities |

After the fix is released, we will publicly credit you (unless you prefer to remain anonymous).

## Scope

The following are considered in scope:

- **DNS packet parsing** — Buffer overflows, out-of-bounds reads, or malformed packet handling in `dns_utils.rs`
- **TLS verification bypass** — Scenarios where TLS verification is expected but can be circumvented without `--insecure`
- **Credential / data leakage** — Config file handling, output file permissions, or logging that unintentionally exposes sensitive data
- **Dependency vulnerabilities** — Known CVEs in direct dependencies (`reqwest`, `native-tls`, `tokio`, etc.)
- **Command injection** — Any path where user-supplied input (domain, file paths, URLs) could lead to unintended command execution

The following are **out of scope**:

- DoH endpoints themselves being malicious or returning incorrect data (this is the expected use case of the tool)
- Denial-of-service via extremely large `doh.txt` files (resource usage scales linearly by design)
- Findings that require physical access to the machine running the tool

## Security Best Practices for Users

- **Keep your binary up to date** — Always use the latest release to benefit from security patches.
- **Protect your config file** — `config.json` may contain file paths relevant to your environment. Restrict file permissions where appropriate.
- **Use `--insecure` with caution** — This flag disables TLS certificate verification and should only be used for testing private/self-signed endpoints in trusted environments.
- **Review your DoH list** — Only include endpoints you trust in `doh.txt`. The tool will make network requests to every listed URL.
- **Audit dependencies** — Run `cargo audit` periodically to check for known vulnerabilities in the dependency tree.

## Dependency Auditing

We recommend running the following regularly:

```bash
# Install cargo-audit
cargo install cargo-audit

# Check for known vulnerabilities
cargo audit
```

## Acknowledgements

We gratefully recognise the contributions of security researchers who help keep this project safe. Acknowledged reporters will be listed here after fixes are released (with permission).

<!-- 
| Reporter | Vulnerability | Fixed In |
|---|---|---|
| — | — | — |
-->