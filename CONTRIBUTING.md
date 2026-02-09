# Contributing to DoH Tester

First off, thank you for considering contributing to DoH Tester! Every contribution helps make this tool better for everyone who cares about DNS privacy and security.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Style Guide](#style-guide)
- [Commit Messages](#commit-messages)
- [Pull Request Process](#pull-request-process)
- [Reporting Bugs](#reporting-bugs)
- [Suggesting Features](#suggesting-features)
- [Adding DoH Endpoints](#adding-doh-endpoints)

---

## Code of Conduct

This project adheres to the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behaviour to the repository maintainers.

---

## How Can I Contribute?

There are many ways to contribute, even if you don't write code:

- 🐛 **Report bugs** — Found something broken? [Open an issue](#reporting-bugs).
- 💡 **Suggest features** — Have an idea? [Start a discussion](#suggesting-features).
- 📄 **Improve docs** — Typos, unclear explanations, missing examples — all welcome.
- 🌍 **Translate** — Help translate the README into other languages.
- 🔗 **Add DoH endpoints** — Know a public DoH server not in our list? [Add it](#adding-doh-endpoints).
- 🧪 **Write tests** — More test coverage is always appreciated.
- 🔧 **Fix issues** — Browse [open issues](https://github.com/SkipTutorial/doh_tester/issues) and pick one.

---

## Getting Started

### Prerequisites

- [Rust 1.70+](https://rustup.rs/) (stable toolchain)
- **Windows (GNU target):** MinGW-w64 (`scoop install mingw`)
- **Windows (MSVC target):** Visual Studio Build Tools with C++ workload and Windows SDK
- **Linux:** `build-essential`, `libssl-dev`, `pkg-config`
- **macOS:** Xcode Command Line Tools (`xcode-select --install`)

### Fork and Clone

```bash
# Fork the repo on GitHub, then:
git clone https://github.com/<your-username>/doh_tester.git
cd doh_tester
```

### Verify Everything Builds

```bash
cargo build
cargo test
cargo clippy
```

If all three commands pass with zero errors and zero warnings, you're ready to go.

---

## Development Workflow

1. **Create a branch** from `main`:

   ```bash
   git checkout -b feat/my-new-feature
   # or
   git checkout -b fix/issue-42
   ```

2. **Make your changes.** Keep commits small and focused.

3. **Add or update tests** for any changed behaviour.

4. **Run the full check suite:**

   ```bash
   cargo fmt --check        # Formatting
   cargo clippy              # Lints
   cargo test                # Unit tests
   cargo build --release     # Make sure release profile compiles
   ```

5. **Push and open a Pull Request** against `main`.

---

## Style Guide

### Rust Code

- **Format with `rustfmt`** — Run `cargo fmt` before committing. The project uses the default `rustfmt` configuration.
- **No warnings** — Code must compile with zero warnings under `cargo clippy`. If a warning is genuinely a false positive, suppress it with an inline `#[allow(...)]` and a comment explaining why.
- **Error handling** — Prefer `anyhow::Result` for application-level errors. Use `Option` / `Result` return types instead of panicking.
- **Documentation** — Add `///` doc comments to all public items. Use `//` comments for internal implementation notes.
- **Naming** — Follow Rust conventions: `snake_case` for functions and variables, `PascalCase` for types, `SCREAMING_SNAKE_CASE` for constants.

### Project Structure

```
src/
├── main.rs        # CLI definition, orchestration, entry point
├── types.rs       # Shared types: Config, DoHResult, DoHStatus, enums
├── doh.rs         # DoHTester: TCP/TLS probes, DoH query methods
├── dns_utils.rs   # Low-level DNS packet building/parsing, base64url
└── output.rs      # Text/JSON output formatting, summary printing
```

When adding new functionality, consider which module it belongs in:

| Change | Where |
|---|---|
| New CLI flag | `main.rs` (Args struct) |
| New config field | `types.rs` (Config struct + Default impl) |
| New DoH query method | `doh.rs` (DoHTester impl) |
| DNS packet changes | `dns_utils.rs` |
| New output format | `output.rs` |

### Tests

- Place unit tests in a `#[cfg(test)] mod tests` block at the bottom of the relevant source file.
- Name tests descriptively: `test_parse_dns_response_no_answers` is better than `test_parse_2`.
- For DNS parsing, include the raw bytes as a hex array with comments describing the packet structure.

---

## Commit Messages

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <short summary>

<optional body>

<optional footer>
```

### Types

| Type | Purpose |
|---|---|
| `feat` | New feature |
| `fix` | Bug fix |
| `docs` | Documentation only |
| `style` | Formatting, missing semicolons, etc. (no logic change) |
| `refactor` | Code restructuring without behaviour change |
| `perf` | Performance improvement |
| `test` | Adding or updating tests |
| `chore` | Build process, CI, dependencies |

### Examples

```
feat(doh): add DNS-over-HTTPS/3 support

fix(dns_utils): skip question section before parsing answers

docs(readme): add cross-compilation instructions

chore(ci): add macOS ARM build target
```

---

## Pull Request Process

1. **Fill out the PR template** — Describe what your change does and why.
2. **Link related issues** — Use `Closes #42` or `Fixes #42` in the PR description.
3. **Keep the diff small** — Large PRs are harder to review. If a change is big, consider splitting it into multiple PRs.
4. **Ensure CI passes** — All checks (build, test, clippy, fmt) must be green.
5. **Respond to review feedback** — Maintainers may request changes. Push additional commits to the same branch; don't force-push during review.
6. **Squash if asked** — We may ask you to squash commits for a cleaner history before merging.

### What We Look For in Reviews

- Does the code do what the PR claims?
- Are there tests for the new behaviour?
- Is the code clear and idiomatic Rust?
- Are error paths handled gracefully?
- Does it maintain backward compatibility with `config.json`?

---

## Reporting Bugs

When opening a bug report, please include:

1. **What you expected** to happen.
2. **What actually happened** (include error messages and terminal output).
3. **Steps to reproduce** the problem.
4. **Environment details:**
   - OS and version (e.g. Windows 11 23H2, Ubuntu 24.04)
   - Rust version (`rustc --version`)
   - DoH Tester version (`doh_tester --version`)
   - Relevant `config.json` settings
5. **The DoH endpoint(s)** involved, if applicable.

Use the [Bug Report issue template](https://github.com/SkipTutorial/doh_tester/issues/new?template=bug_report.md) if one is available.

---

## Suggesting Features

Feature suggestions are welcome! When proposing a feature:

1. **Check existing issues** first to avoid duplicates.
2. **Describe the problem** the feature solves — "As a user, I want to ... so that ...".
3. **Propose a solution** if you have one in mind, but stay open to alternatives.
4. **Consider scope** — Does it fit the project's goal of testing DoH endpoints?

Use the [Feature Request issue template](https://github.com/SkipTutorial/doh_tester/issues/new?template=feature_request.md) if one is available.

---

## Adding DoH Endpoints

The `doh.txt` file contains the community-maintained list of public DoH servers. To add new endpoints:

1. Verify the endpoint is **publicly accessible** (not behind authentication).
2. Verify it speaks **standard DoH** (RFC 8484 wire format or Google-style JSON API).
3. Test it manually with the tool: `doh_tester example.com -f <(echo "https://your.endpoint/dns-query")`.
4. Add the URL to `doh.txt` in **alphabetical order** within the appropriate section.
5. Open a PR with the title `chore(doh.txt): add <provider-name> endpoint`.

**Do not add:**

- Endpoints that require API keys or authentication.
- Endpoints known to be experimental or frequently offline.
- Duplicate URLs (check for existing entries first).

---

## Questions?

If you're unsure about anything, don't hesitate to [open a discussion](https://github.com/SkipTutorial/doh_tester/discussions) or comment on an existing issue. We're happy to help you get started.

Thank you for helping make DoH Tester better! 🎉