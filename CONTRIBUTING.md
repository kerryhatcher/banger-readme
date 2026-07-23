# Contributing to rmb

Thanks for your interest in contributing! This document covers how to set up your development environment, make changes, and submit pull requests.

## Development Setup

### Prerequisites

- **Rust** — Install via [rustup](https://rustup.rs/). The minimum supported version is tracked in `Cargo.toml` (`edition = "2021"` requires Rust 1.56+).
- **Git** — For cloning and version control.
- **OpenSSL development headers** — Required by the `git2` crate. On Ubuntu/Debian: `sudo apt install libssl-dev pkg-config`. On macOS: `brew install openssl`.

### Build

```bash
git clone https://github.com/kwhatcher/banger-readme
cd banger-readme
cargo build
```

The binary will be at `target/debug/rmb`.

### Run Tests

```bash
cargo test
cargo clippy -- -D warnings
cargo fmt -- --check
```

### Run the Tool Locally

```bash
# Score a README
cargo run -- score .

# Score a remote README
cargo run -- score https://raw.githubusercontent.com/user/repo/main/README.md --no-hygiene

# Install a plugin
cargo run -- install https://github.com/user/my-plugin
```

## Project Structure

```
src/
├── main.rs          # CLI entry point (install, list, remove, score commands)
├── config.rs        # Path detection for Pi and Claude Code directories
├── git.rs           # Git clone, repo slug extraction, HEAD SHA
├── install.rs       # Installation logic for both targets
├── plugin.rs        # Plugin detection (SKILL.md / .claude-plugin.json)
└── score/           # README scoring engine
    ├── mod.rs       # Module root
    ├── engine.rs    # Orchestrator: fetch → parse → analyze → score
    ├── rules.rs     # Regex patterns, weights, detection heuristics
    ├── content.rs   # 14-section content completeness analyzer
    ├── visuals.rs   # Visual design quality checks
    ├── hygiene.rs   # Project hygiene checks (companion files)
    ├── antipatterns.rs  # Anti-pattern detection
    ├── funnel.rs    # Cognitive funneling order analyzer
    └── report.rs    # Terminal + JSON output formatting
```

## Making Changes

1. **Branch naming:** Use descriptive branch names like `feat/score-command` or `fix/logo-detection`.
2. **Commit messages:** Follow [Conventional Commits](https://www.conventionalcommits.org/): `feat:`, `fix:`, `docs:`, `chore:`, `refactor:`.
3. **Code style:** Run `cargo fmt` before committing. The project uses default rustfmt settings.
4. **Linting:** Run `cargo clippy -- -D warnings`. All warnings must be resolved.
5. **Tests:** Add tests for new functionality. Run `cargo test` to verify everything passes.

## Submitting Changes

1. Fork the repository
2. Create a feature branch
3. Make your changes with clear commit messages
4. Push to your fork
5. Open a pull request against the `main` branch

In your PR description, explain:
- What the change does
- Why it's needed
- How you tested it

A maintainer will review your PR. CI must pass before merging.

## Finding Something to Work On

- Check the [open issues](https://github.com/kwhatcher/banger-readme/issues) for bugs and feature requests
- Look for issues labeled `good first issue` for beginner-friendly tasks
- The [scoring engine design doc](docs/scoring-engine-design.md) has planned enhancements

## Code of Conduct

This project adheres to the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## Questions?

Open a [GitHub Discussion](https://github.com/kwhatcher/banger-readme/discussions) or file an issue.
