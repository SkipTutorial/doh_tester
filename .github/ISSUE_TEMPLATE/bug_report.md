---
name: 🐛 Bug Report
about: Report a bug or unexpected behaviour
title: "[Bug]: "
labels: bug
assignees: ""
---

## Describe the Bug

A clear and concise description of what the bug is.

## Steps to Reproduce

1. Run `doh_tester ...` with these arguments: `...`
2. Using this config: (paste relevant `config.json` fields)
3. See error / unexpected output

## Expected Behaviour

What you expected to happen.

## Actual Behaviour

What actually happened. Include terminal output, error messages, or screenshots.

<details>
<summary>Terminal Output</summary>

```
Paste the full terminal output here
```

</details>

## Environment

- **OS:** (e.g. Windows 11 23H2, Ubuntu 24.04, macOS 15.3)
- **DoH Tester version:** (`doh_tester --version`)
- **Rust version:** (`rustc --version`, if built from source)
- **Toolchain:** (e.g. `stable-x86_64-pc-windows-gnu`, `stable-x86_64-unknown-linux-gnu`)
- **Installation method:** (pre-built binary / `cargo install` / built from source)

## Configuration

<details>
<summary>config.json</summary>

```json
Paste your config.json here (remove any sensitive paths if needed)
```

</details>

## DoH Endpoint(s) Involved

If the bug is specific to certain endpoints, list them here:

```
https://example.doh.server/dns-query
```

## Additional Context

Add any other context about the problem here — network environment, proxy settings, firewall rules, etc.