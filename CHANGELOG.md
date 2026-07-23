# Changelog

## 0.1.0 (Unreleased)

### Added
- `rmb install` — Install plugins from git repos to Pi and Claude Code
- `rmb list` — List installed plugins across both systems
- `rmb remove` — Remove plugins from either or both targets
- `rmb score` — Score READMEs against 50+ research-backed criteria
- Auto-detection of plugin type (Pi skill, Claude Code plugin, or both)
- Five-category scoring engine: content, visuals, hygiene, funneling, anti-patterns
- JSON output mode for CI integration
- `--check` and `--threshold` flags for CI gating
- HTML-aware parsing for READMEs that use HTML formatting
- Implicit section detection (code blocks as quick starts, docs links as API refs)
- Benchmark percentiles for score context
- Partial credit system for close-but-not-exact matches
