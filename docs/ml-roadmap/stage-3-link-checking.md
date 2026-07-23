# Stage 3 — Broken Link Detection

**Status:** Planned · **Effort:** Medium · **Gated:** `--links` flag / `links` feature

## What

Use `lychee-lib` as a statically-linked library to check every URL in the
README for broken links, redirects, and timeouts. Gated behind `--links`
because it pulls in `tokio` and an async HTTP client.

## Dependencies

```toml
[dependencies]
lychee-lib = "0.18"        # Link checking library
tokio = { version = "1", features = ["rt"] }  # Async runtime
```

`lychee-lib` is pure Rust (98.8% of the lychee codebase). It is the same
library the `lychee` CLI uses internally. No shelling out — everything
compiles into `rmb`.

## New Checks in Project Hygiene (+5 pts)

### Broken Link Detection (3 pts)

Extract all HTTP(S) URLs from the README and check each one:

| Result | Points |
|--------|--------|
| All links return 2xx | 3 pts |
| 1 broken link | 1 pt |
| 2+ broken links | 0 pts |

Broken = 404, 410, 5xx, DNS failure, timeout, or SSL error.

### Redirect Chain Detection (1 pt)

| Result | Points |
|--------|--------|
| No redirects | 1 pt |
| 1–2 redirects (e.g., HTTP→HTTPS) | 1 pt (acceptable) |
| 3+ redirects on any link | 0 pts, warning |

### Anchor Fragment Validation (1 pt)

For links with `#fragment` anchors pointing to the same file or other local
markdown files, verify the anchor target exists:

| Result | Points |
|--------|--------|
| All anchors resolve | 1 pt |
| 1+ broken anchor | 0 pts |

This catches the common pattern of renaming a heading and forgetting to update
the table of contents links.

## Implementation Notes

- **Async bridge.** `lychee-lib` is async. Since `rmb` is currently synchronous
  (`reqwest::blocking`), you have two options:
  - **Option A:** Add `tokio` and wrap the call in `tokio::runtime::Runtime::block_on()`.
    Simple, adds one runtime dependency.
  - **Option B:** Use `reqwest`'s blocking client to implement a simpler link
    checker directly. Avoids `tokio` but loses `lychee-lib`'s features
    (retry logic, rate limiting, fragment checking, cache).
  - **Recommendation:** Option A. `lychee-lib` brings too much value to
    reimplement.
- **Timeout.** Set a 10-second per-link timeout to avoid hanging on slow servers.
- **Concurrency.** Limit to 10 concurrent checks to avoid rate limiting.
- **Caching.** Cache results for the session — don't re-check the same URL twice.
- **GitHub links.** If a `GITHUB_TOKEN` env var is present, pass it to avoid
  rate limiting on `github.com` URLs.
- **Skip patterns.** Skip `mailto:` links, `localhost` URLs, and links matching
  user-configured exclude patterns.
- Add a `LinkCheckResult` struct.

## Files to Create/Modify

| File | Action |
|------|--------|
| `Cargo.toml` | Add `lychee-lib`, `tokio` behind `links` feature |
| `src/score/link_check.rs` | New module — `LinkCheckResult`, `analyze()` |
| `src/score/engine.rs` | Conditionally wire when `--links` is set |
| `src/score/report.rs` | Add `link_check` field to `ScoredReport` |
| `src/score/mod.rs` | Register `link_check` module |
| `src/main.rs` | Add `--links` CLI flag |
