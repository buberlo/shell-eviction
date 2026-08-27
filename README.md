# Shell Eviction

> Your terminal history files as a landlord, fining directories for failed commands and wasted cd loops.

A CLI scans shell history to build dwell time and failure reports for every directory, then issues eviction notices when a folder is too noisy or too empty. It can also suggest cleanup commands without running them.

## Features
- Import zsh, bash, or fish history and compute directory dwell time plus failed command counts.
- Issue eviction notices for directories with too many failed commands or suspicious cd loops.
- Generate a weekly tenant report listing fines, most-used aliases, and cleanup candidates.
- Dry-run output emits suggested rm, rename, or archive commands with a safety score.

## Stack
- Rust
- clap
- serde

## Getting started
```
cargo run -- --history ~/.zsh_history --format zsh --report weekly --dry-run
```

---
*Farmed 🚜 by [Appshaker](https://github.com/buberlo) — shaken into existence.*
