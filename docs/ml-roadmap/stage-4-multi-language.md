# Stage 4 — Multi-Language Readability

**Status:** Planned · **Effort:** Medium · **Gated:** `--multi-lang` flag / `multi-lang` feature

## What

Extend text quality scoring to non-English READMEs using `readsight`, which
provides 17 readability formulas across 86 languages. Gated because it embeds
~2 MB of hyphenation pattern files.

## Dependencies

```toml
[dependencies]
readsight = "1.0"          # 86 languages, 17 formulas, TeX hyphenation
```

Pure Rust. Embeds language data and hyphenation patterns at compile time via
`include_dir!`. No filesystem or network access at runtime. ~2 MB of embedded
data.

## New Checks

### Multi-Language README Detection

Scan the repository for non-English README variants:

- `README-zh.md`, `README-ja.md`, `README-ko.md` (explicit language codes)
- `README.md` in non-English languages (detected via `whatlang` or character set analysis)
- `README_zh.md`, `README_ja.md` (underscore variants)

### Per-Language Readability (3 pts per additional language)

For each non-English README found, run the language-appropriate readability
formulas:

| Language | Available Formulas |
|----------|-------------------|
| Chinese (zh) | 5 universal (Gunning Fog, SMOG, Coleman-Liau, ARI, LIX) |
| Japanese (ja) | 5 universal |
| Korean (ko) | 5 universal |
| Russian (ru) | Flesch Reading Ease, Flesch-Kincaid + 5 universal |
| German (de) | Flesch Reading Ease, Flesch-Kincaid, Wiener Sachtextformel + 5 universal |
| Spanish (es) | Flesch Reading Ease, Flesch-Kincaid, Fernández-Huerta, Szigriszt-Pazos + 5 universal |
| French (fr) | Flesch Reading Ease, Flesch-Kincaid + 5 universal |
| Italian (it) | Flesch Reading Ease, Flesch-Kincaid, Gulpease + 5 universal |
| Arabic (ar) | OSMAN + 5 universal |
| ...and 76 more | At minimum, 5 universal formulas |

Score each non-English README on the same readability criteria as Stage 0,
using language-appropriate grade level thresholds.

### Translation Completeness (2 pts bonus)

Compare the section structure of the primary README against each translation:

| Check | Points |
|-------|--------|
| All sections present in translation | 2 pts |
| Missing 1–2 sections | 1 pt |
| Missing 3+ sections | 0 pts, warning |

This catches stale translations that haven't been updated alongside the primary
README.

### Language Detection

Use character-set heuristics to auto-detect the language of each README:

| Script | Likely Languages |
|--------|-----------------|
| CJK Unified Ideographs | zh, ja, ko |
| Cyrillic | ru, uk, be, bg, sr, mk |
| Arabic | ar, fa, ur |
| Devanagari | hi, mr, ne |
| Latin (with diacritics) | de, fr, es, it, pt, etc. |

`readsight` requires an explicit language code. Auto-detection narrows it down
to a script family, then `readsight` can try the most common language for that
script.

## Implementation Notes

- `readsight` is the heaviest dependency in the roadmap (~2 MB embedded data).
  It must be behind both a compile-time feature flag and a runtime `--multi-lang`
  flag.
- Language detection is best-effort. If unsure, skip and note in the report.
- The primary README (English) continues to use Stage 0's `writing-analysis` +
  `textstat` dual-readability. `readsight` is only used for non-English variants.
- Add a `MultiLangResult` struct.
- The `--multi-lang` flag implies `--deep` (image similarity checks also run).

## Files to Create/Modify

| File | Action |
|------|--------|
| `Cargo.toml` | Add `readsight` behind `multi-lang` feature |
| `src/score/multi_lang.rs` | New module — `MultiLangResult`, `analyze()` |
| `src/score/engine.rs` | Conditionally wire when `--multi-lang` is set |
| `src/score/report.rs` | Add `multi_lang` field to `ScoredReport` |
| `src/score/mod.rs` | Register `multi_lang` module |
| `src/main.rs` | Add `--multi-lang` CLI flag |
