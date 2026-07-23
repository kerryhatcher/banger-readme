# Changelog

All notable changes to this project are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres
to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- **Scoped `rmb` to its purpose: building and rating READMEs.** `rmb install` is now
  bootstrap-only — it installs the bundled rmb README skill into Pi and Claude Code.
- Reworded CLI help, crate description, and all docs to reflect the README-first purpose.
- New score-dial header and logo; README header now renders full width.

### Added
- GitHub Actions CI workflow (`fmt`, `clippy`, `build`, `test`).

### Removed
- `rmb list` and `rmb remove` — generic plugin-management commands outside the tool's scope.
- Arbitrary git-URL / local-path installs, plus `install --branch` and `install --force`.

### Fixed
- Clippy `-D warnings` violations in the scoring engine.

## [0.2.2] - 2026-07-22

### Changed
- `rmb install` with no arguments defaults to self-installing the bundled plugin.

## [0.2.1] - 2026-07-22

### Added
- `rmb install` (no URL) self-install command for the bundled plugin.

## [0.2.0] - 2026-07-22

### Changed
- Refactored the skill into Open Plugin format; added support for installing from a local directory.
- Renamed the crate to `banger-readme` for crates.io; the binary remains `rmb`.

## [0.1.0] - 2026-07-22

### Added
- `rmb score` — score READMEs against 50+ research-backed criteria.
- Five-category scoring engine: content, visuals, hygiene, funneling, anti-patterns.
- JSON output mode and `--check` / `--threshold` flags for CI gating.
- HTML-aware parsing for READMEs that use HTML formatting.
- Implicit section detection (code blocks as quick starts, docs links as API refs).
- Benchmark percentiles for score context and a partial-credit system for close matches.

[Unreleased]: https://github.com/kerryhatcher/banger-readme/compare/v0.2.2...HEAD
[0.2.2]: https://github.com/kerryhatcher/banger-readme/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/kerryhatcher/banger-readme/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/kerryhatcher/banger-readme/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/kerryhatcher/banger-readme/releases/tag/v0.1.0
