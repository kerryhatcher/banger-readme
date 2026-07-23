# ML-Enhanced Scoring Roadmap

Non-LLM machine learning, computer vision, and NLP enhancements for the `rmb`
README scoring engine. Each stage builds on the last and can ship independently.

## Stage Overview

| Stage | Name | New Deps | Gated? | Effort |
|-------|------|----------|--------|--------|
| [0](./stage-0-text-quality.md) | Text Quality Analysis | `writing-analysis`, `textstat` | No (default) | Small |
| [1](./stage-1-image-heuristics.md) | Image Heuristics | None (uses existing `image`) | No (default) | Small |
| [2](./stage-2-image-similarity.md) | Image Similarity | `image-similarity` | `--deep` | Small |
| [3](./stage-3-link-checking.md) | Broken Link Detection | `lychee-lib`, `tokio` | `--links` | Medium |
| [4](./stage-4-multi-language.md) | Multi-Language Readability | `readsight` | `--multi-lang` | Medium |

## Feature Flags

```toml
[features]
default = ["fast"]
fast = ["writing-analysis"]
deep = ["fast", "image-similarity"]
links = ["deep", "lychee-lib"]
multi-lang = ["deep", "readsight"]
full = ["links", "multi-lang"]
```

## CLI Interface

```
rmb score .                  # Stages 0–1 (fast path, always available)
rmb score . --deep           # Stages 0–2 (+ image similarity)
rmb score . --links          # Stages 0–3 (+ broken link checking)
rmb score . --multi-lang     # Stages 0–2,4 (+ i18n readability)
rmb score . --full           # All stages
```

## Scoring Impact Summary

| Category | Current Max | After All Stages | New Checks |
|----------|-------------|------------------|------------|
| Content Completeness | 35 pts | 35 pts | — |
| Visual Design | 25 pts | 31 pts | +6 (image dimensions, format, dark/light distinctiveness) |
| Project Hygiene | 20 pts | 25 pts | +5 (broken links, link freshness) |
| Cognitive Funneling | 15 pts | 15 pts | — |
| Anti-Patterns | −15 pts | −21 pts | −6 (clichés, passive voice, filler words, readability) |
| **Text Quality** (new) | — | 10 pts | Readability grade, sentiment, sentence variety |
| **Total** | 80 pts | 100 pts | |

## Design Principles

- **Static linking only.** No shelling out to external binaries. Everything compiles into `rmb`.
- **Dual-readability consensus.** `writing-analysis` and `textstat` cross-validate each other.
  When they agree, high confidence. When they disagree, flag for manual review.
- **Progressive disclosure.** Heavier features sit behind compile-time features and runtime
  flags. The default `cargo install` stays lean.
- **Pure Rust.** No C/C++ dependencies, no system library requirements, no Python.
