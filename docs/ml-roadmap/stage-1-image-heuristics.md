# Stage 1 — Image Heuristics

**Status:** Planned · **Effort:** Small · **Gated:** No (default)

## What

Add computer-vision-free image quality checks using the `image` crate already
in the dependency tree. No new dependencies.

## Dependencies

None. The `image` crate is already available transitively through the existing
dependency graph.

## New Checks in Visual Design (+4 pts)

### Logo Dimensions (2 pts)

Open the first image found in the README (the logo/banner) and check:

| Check | Threshold | Points |
|-------|-----------|--------|
| Logo width ≥ 128px | HiDPI minimum | 1 pt |
| Logo height ≥ 128px | HiDPI minimum | 1 pt |
| Logo is square-ish (0.5 ≤ ratio ≤ 2.0) | Not stretched | implicit |
| Logo file size ≤ 200 KB | Fast page load | warning only |

If no logo image is found, this check is skipped (not penalized — the existing
"Has logo image" check already covers that).

### Banner Dimensions (1 pt)

If a header/banner image is detected (wide image, >2:1 aspect ratio):

| Check | Threshold | Points |
|-------|-----------|--------|
| Banner width ≥ 1200px | Fills GitHub README container | 1 pt |
| Banner aspect ratio ≥ 2:1 | Looks like a banner, not a square | implicit |

### Image Format Optimization (1 pt)

For each image in the README:

| Issue | Action |
|-------|--------|
| BMP or TIFF format | −0.5 pt, suggest PNG/WebP |
| PNG > 1 MB | Warning, suggest optimization |
| JPEG quality artifacts detectable | Warning, suggest PNG for screenshots |

### Image Count vs. README Length (bonus context)

Not scored, but surfaced as a suggestion: "Your README is 800 lines with 0
images — consider adding a screenshot or demo GIF."

## Implementation Notes

- Use `image::open()` on local image paths extracted from the README.
- Skip remote URLs (HTTP images) — only check local files in the repo.
- Cache image metadata to avoid re-opening the same file.
- All checks are best-effort. If an image fails to decode, skip it with a
  warning rather than crashing.
- Add an `ImageHeuristicsResult` struct or fold checks into the existing
  `VisualResult`.

## Files to Create/Modify

| File | Action |
|------|--------|
| `src/score/image_heuristics.rs` | New module — `ImageHeuristicsResult`, `analyze()` |
| `src/score/visuals.rs` | Wire image heuristic checks into visual scoring |
| `src/score/engine.rs` | Pass repo directory path for local file resolution |
| `src/score/report.rs` | Add `image_heuristics` field to `ScoredReport` |
| `src/score/mod.rs` | Register `image_heuristics` module |
