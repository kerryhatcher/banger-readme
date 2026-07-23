<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/header.svg">
    <img src="assets/header.svg" alt="rmb — README Banger" width="100%">
  </picture>
</p>

<p align="center">
  <a href="https://github.com/kerryhatcher/banger-readme/actions"><img src="https://github.com/kerryhatcher/banger-readme/actions/workflows/ci.yml/badge.svg" alt="CI"></a>
  <a href="https://crates.io/crates/rmb"><img src="https://img.shields.io/crates/v/rmb.svg?label=version" alt="Version"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License"></a>
</p>

**rmb** is a CLI for building and rating great READMEs. It **scores** any README against 50+ research-backed criteria — locally, no AI, no API calls — and **bootstraps the rmb skill** into your coding agents ([Pi](https://github.com/earendil-works/pi) and [Claude Code](https://docs.anthropic.com/en/docs/claude-code)) so they can author README files (logo, header, and all 14 sections) that actually score well.

## 📋 Table of Contents

- [Features](#-features)
- [Quick Start](#-quick-start)
- [Why rmb?](#-why-rmb)
- [README Scoring](#-readme-scoring)
- [Agent Setup](#-agent-setup)
- [Usage](#-usage)
- [Documentation](#-documentation)
- [Installation](#-installation)
- [Contributing](#-contributing)
- [License](#-license)
- [Acknowledgements](#-acknowledgements)

## 🎯 Why rmb?

Most open source READMEs are mediocre. Research shows that a great README can **5× your star conversion rate** — but most projects never invest the time. `rmb` attacks that from both ends:

1. **Measure what you have** — There's no objective way to know if your README is good. `rmb score` gives a deterministic, research-backed score with specific, ranked recommendations — no LLM, no API calls, results in milliseconds.
2. **Build a better one** — `rmb install` bootstraps the rmb skill into your coding agent, so it can author READMEs (logo, header, all 14 sections) against the same research the scorer checks.

## ✨ Features

- **📊 README scoring** — Scores any README against 50+ criteria across 5 categories — no LLM required
- **🎯 Actionable feedback** — Every failed check comes with a specific, ranked recommendation
- **🤖 Agent bootstrap** — One command drops the rmb README skill into Pi and Claude Code
- **🔒 Fully local** — Scoring is deterministic and offline; no AI, no API keys, no telemetry
- **⚙️ CI-friendly** — `--json` output and `--check --threshold` for failing builds on low scores

## 🚀 Quick Start

```bash
# Install the CLI
cargo install banger-readme

# Score a README
rmb score .

# Bootstrap the rmb skill into your agents (Pi + Claude Code)
rmb install
```

## 📊 README Scoring

`rmb score` evaluates READMEs against 50+ deterministic criteria — no AI, no API calls, instant results.

```
╔══════════════════════════════════════════════════╗
║         README Score: 85/100  [A]               ║
║         "Excellent — well above average"         ║
║         Top 5% of open source READMEs            ║
╚══════════════════════════════════════════════════╝

┌─ Content Completeness ────── 32/35 ─────┐
│ ✅ Logo/Banner    ✅ Badges    ✅ One-liner  │
│ ✅ Demo           ✅ Features  ✅ Quick Start│
│ ✅ TOC            ✅ Why       ✅ Install    │
│ ✅ Usage          ✅ API Ref   ✅ Contrib    │
│ ✅ License        ✅ Acknowledgements       │
└──────────────────────────────────────────┘
```

**Five scoring categories:**

| Category | Weight | What It Checks |
|----------|--------|----------------|
| Content Completeness | 35 pts | 14 essential sections from logo to acknowledgements |
| Visual Design | 25 pts | Logo, badges, emojis, images, collapsible sections, dark/light mode |
| Project Hygiene | 20 pts | LICENSE, CONTRIBUTING.md, CODE_OF_CONDUCT.md, SECURITY.md, CI, tests |
| Cognitive Funneling | 15 pts | Section ordering: broad→specific, one-liner before install, license at bottom |
| Anti-Patterns | −15 pts | Deductions for placeholders, duplicate titles, badge abuse, wall of text |

## 🤖 Agent Setup

`rmb install` bootstraps the **rmb README skill** into your coding agents. Once installed, your agent can author READMEs — generating logos and headers, structuring all 14 sections, and following the same research `rmb score` checks against.

```bash
# Bootstrap into both Pi and Claude Code (default)
rmb install

# Only Pi
rmb install --pi-only

# Only Claude Code
rmb install --claude-only
```

It fetches the bundled `banger-readme` plugin from GitHub and installs it to:

| Target | Location |
|---|---|
| Pi coding harness | `~/.pi/agent/skills/banger-readme/` |
| Claude Code | `~/.claude/plugins/cache/` (skills symlinked into `~/.claude/skills/`) |

Re-running `rmb install` replaces the existing installation, so it doubles as an update command.

## 📖 Usage

<details>
<summary><b>🔍 Scoring a README</b></summary>

```bash
# Score the README in the current directory
rmb score .

# Score a remote README
rmb score https://raw.githubusercontent.com/user/repo/main/README.md --no-hygiene

# Machine-readable JSON output
rmb score . --json

# CI mode: fail if score below threshold
rmb score . --check --threshold 70
```

</details>

<details>
<summary><b>🤖 Bootstrapping into agents</b></summary>

```bash
# Install the rmb skill into both Pi and Claude Code
rmb install

# Only one target
rmb install --pi-only
rmb install --claude-only
```

</details>

## 📖 Documentation

| Document | Description |
|----------|-------------|
| [Scoring Engine Design](docs/scoring-engine-design.md) | Full architecture and detection rules for `rmb score` |
| [README Best Practices Research](docs/readme-best-practices-research.md) | Index of all research on what makes a great README |
| [Why READMEs Matter](docs/why-readmes-matter.md) | The data, psychology, and philosophy |
| [Anatomy of a README](docs/anatomy-of-a-readme.md) | The 14-section structure in detail |
| [Project Hygiene](docs/project-hygiene.md) | License, contributing, CoC, security, tests |
| [Aesthetics & Visual Design](docs/aesthetics-and-visual-design.md) | Formatting, visual elements, linkification |
| [Emerging Patterns](docs/emerging-patterns.md) | Trends for 2024–2026 |
| [Anti-Patterns](docs/anti-patterns.md) | What repels users and common mistakes |
| [Exemplary READMEs](docs/exemplary-readmes.md) | Curated examples by category |
| [Examples Audit](docs/examples-audit.md) | How 31 production READMEs rank against criteria |

## 🛠️ Installation

### From Source

```bash
git clone https://github.com/kerryhatcher/banger-readme
cd banger-readme
cargo build --release
```

The binary will be at `target/release/rmb`.

### Via Cargo

```bash
cargo install banger-readme
```

### Running Tests

```bash
cargo test
cargo clippy -- -D warnings
cargo fmt -- --check
```

## 💬 Getting Help

- **Questions?** Open a [GitHub Discussion](https://github.com/kerryhatcher/banger-readme/discussions)
- **Bug?** File an [issue](https://github.com/kerryhatcher/banger-readme/issues)
- **Security issue?** See [SECURITY.md](SECURITY.md)

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup, project structure, code standards, and the pull request process.

<a href="https://github.com/kerryhatcher/banger-readme/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=kerryhatcher/banger-readme" />
</a>

## 📄 License

MIT — see [LICENSE](LICENSE) for details.

## 🙏 Acknowledgements

Built with research from:

- [Awesome README](https://github.com/matiassingers/awesome-readme) — 21.2k stars, curated list of 100+ exemplary READMEs
- [Art of README](https://github.com/hackergrrl/art-of-readme) — Foundational essay on README psychology
- [Readme Driven Development](https://tom.preston-werner.com/2010/08/23/readme-driven-development.html) — Tom Preston-Werner
- [Daytona's 4,000-star launch guide](https://www.daytona.io/dotfiles/how-to-write-4000-stars-github-readme-for-your-project)
- [Elegant READMEs](https://www.yegor256.com/2019/04/23/elegant-readme.html) — Yegor Bugayenko
