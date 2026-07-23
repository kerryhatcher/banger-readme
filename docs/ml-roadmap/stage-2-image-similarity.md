# Stage 2 — Image Similarity

**Status:** Planned · **Effort:** Small · **Gated:** `--deep` flag / `deep` feature

## What

Use structural similarity (SSIM) metrics to compare dark/light mode image
variants and detect placeholder images. Gated behind `--deep` because it
adds a new dependency and involves pixel-level image processing.

## Dependencies

```toml
[dependencies]
image-similarity = "0.1"   # SSIM, MS-SSIM, PSNR, MSE, NCC, GMSD
```

Pure Rust. Pulls in `image` (already available). ~50 KB.

## New Checks

### Dark/Light Mode Distinctiveness (2 pts)

When a README declares both `header-dark.svg` and `header.svg` (or uses
`<picture>` with `prefers-color-scheme` media queries), verify the two
images are actually different:

| SSIM Score | Interpretation | Points |
|------------|---------------|--------|
| < 0.85 | Clearly different — good | 2 pts |
| 0.85–0.95 | Subtly different — acceptable | 1 pt |
| > 0.95 | Near-identical — pointless | 0 pts, warning |

This catches the common mistake of copying the same image to both filenames
without actually adapting colors for dark mode.

### Placeholder Detection (−1 pt penalty)

Compare each image in the README against known placeholder patterns:

- Solid-color images (all pixels within 5% intensity range)
- Generic placeholder services (e.g., `via.placeholder.com`, `placehold.co`)
- Images that are pure white or pure black

If an image matches a placeholder pattern, deduct 1 pt from anti-patterns
and suggest replacing it with a real screenshot.

### Screenshot Quality (1 pt)

For images identified as screenshots (by filename heuristics like `screenshot`,
`demo`, `screen`):

| Check | Threshold | Points |
|-------|-----------|--------|
| Resolution ≥ 800×600 | Readable on GitHub | 1 pt |
| Not blurry (Laplacian variance check) | Text should be legible | implicit |

## Implementation Notes

- SSIM comparison requires both images to be the same dimensions. Resize the
  smaller one to match if needed.
- SVG files must be rasterized before comparison. Use the `image` crate's
  limited SVG support or skip SVG comparisons with a note.
- The `--deep` flag enables this entire module. Without it, these checks are
  silently skipped.
- Add an `ImageSimilarityResult` struct.

## Files to Create/Modify

| File | Action |
|------|--------|
| `Cargo.toml` | Add `image-similarity` behind `deep` feature |
| `src/score/image_similarity.rs` | New module — `ImageSimilarityResult`, `analyze()` |
| `src/score/engine.rs` | Conditionally wire when `--deep` is set |
| `src/score/report.rs` | Add `image_similarity` field to `ScoredReport` |
| `src/score/mod.rs` | Register `image_similarity` module |
| `src/main.rs` | Add `--deep` CLI flag |
