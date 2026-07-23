# README Scoring Engine — Design Document

> A deterministic, rule-based analysis engine for the `rmb` CLI that scores READMEs against research-backed criteria without using any LLM.

---

## Overview

The `rmb score` command will analyze a README file (local path or URL) and produce:

1. **A numeric score (0–100)** across weighted categories
2. **A breakdown by category** with per-criterion pass/fail
3. **Actionable, specific recommendations** ordered by impact
4. **A comparison against exemplar benchmarks**

All analysis is done with deterministic heuristics — regex patterns, Markdown structure parsing, file system checks, and basic HTTP validation. No AI, no API calls beyond fetching the README.

---

## Architecture

```
src/
├── main.rs              # CLI: add "score" subcommand
├── score/
│   ├── mod.rs           # Module root, orchestrator
│   ├── engine.rs        # Top-level scoring engine
│   ├── content.rs       # Content completeness checks (14 sections)
│   ├── visuals.rs       # Visual design quality checks
│   ├── hygiene.rs       # Project hygiene checks (companion files)
│   ├── antipatterns.rs  # Anti-pattern detection
│   ├── funnel.rs        # Cognitive funneling order check
│   ├── report.rs        # Report generation & formatting
│   └── rules.rs         # Shared rule definitions & weights
```

### Data Flow

```
README source (file/URL)
        │
        ▼
┌──────────────────┐
│  Fetch & Parse   │  ← Fetch URL content or read local file
│  (Markdown AST)  │  ← Parse into sections, headers, code blocks, links, images
└──────┬───────────┘
       │
       ▼
┌──────────────────┐
│  Run Analyzers   │  ← Five parallel analysis passes
│  ┌─────────────┐ │
│  │ Content     │ │  ← Section presence, completeness
│  │ Visuals     │ │  ← Logo, badges, emojis, images, formatting
│  │ Hygiene     │ │  ← Companion files, license, CI
│  │ Antipatterns│ │  ← Placeholders, broken links, badge abuse
│  │ Funnel      │ │  ← Section ordering, broad→specific flow
│  └─────────────┘ │
└──────┬───────────┘
       │
       ▼
┌──────────────────┐
│  Compute Score   │  ← Weighted aggregation, penalty application
└──────┬───────────┘
       │
       ▼
┌──────────────────┐
│  Generate Report │  ← Terminal output: score, breakdown, recommendations
└──────────────────┘
```

---

## Scoring Categories & Weights

Total: 100 points across 5 categories.

| # | Category | Weight | Rationale |
|---|----------|--------|-----------|
| 1 | **Content Completeness** | 35 pts | The 14-section structure is the backbone of a great README |
| 2 | **Visual Design** | 25 pts | Visual elements directly impact conversion (Daytona research) |
| 3 | **Project Hygiene** | 20 pts | Trust signals determine whether users adopt or bounce |
| 4 | **Cognitive Funneling** | 15 pts | Ordering determines whether readers reach key sections |
| 5 | **Anti-Pattern Penalties** | −15 pts | Deductions for common mistakes (applied as negatives) |

---

## Category 1: Content Completeness (35 points)

Each of the 14 essential sections is worth points. Detection is based on heading text matching, content heuristics, and structural patterns.

### Detection Rules

| # | Section | Points | Detection Heuristic |
|---|---------|--------|---------------------|
| 1 | **Logo/Banner** | 3 | `<img>` tag within first 5 lines, or markdown image `![]()` with height/width attr, or SVG reference |
| 2 | **Badges** | 3 | ≥2 shields.io or similar badge URLs in a contiguous block near the top |
| 3 | **One-liner** | 3 | First paragraph after badges, 50–200 chars, no bullets, no newlines |
| 4 | **Demo GIF/Screenshot** | 4 | Image reference with `.gif` extension OR `![screenshot]`/`![demo]` alt text within first 100 lines |
| 5 | **Feature Highlights** | 3 | Heading matching `feature` + bullet list with ≥3 items |
| 6 | **Quick Start** | 3 | Heading matching `quick.?start|getting.?started` + code block within 10 lines |
| 7 | **Table of Contents** | 2 | Section with ≥5 markdown links in `[text](#anchor)` format, or `<details>` TOC |
| 8 | **The "Why"** | 2 | Heading matching `why|motivation|background|philosophy|problem` |
| 9 | **Installation** | 3 | Heading matching `install|setup|build` + code block or platform-specific subsections |
| 10 | **Usage/Examples** | 3 | Heading matching `usage|example|how.?to` + code block |
| 11 | **API Reference** | 2 | Heading matching `api|reference|documentation` OR link to external docs site |
| 12 | **Contributing** | 2 | Heading matching `contribut` OR link to `CONTRIBUTING.md` |
| 13 | **License** | 1 | Heading matching `license` OR `LICENSE` file detected in repo |
| 14 | **Acknowledgements** | 1 | Heading matching `acknowledge|credit|thanks|contributor` |

### Bonus Points (within Content)

| Bonus | Points | Detection |
|-------|--------|-----------|
| Multi-language README | +2 | ≥2 `README-*.md` files or language switcher links |
| Live demo link | +1 | URL with `demo` or `playground` in link text |
| Benchmark/performance data | +1 | Table with timing/speed/benchmark data |

**Max content score: 35 + 4 bonus = 39 (capped at 35)**

---

## Category 2: Visual Design (25 points)

### Detection Rules

| # | Criterion | Points | Detection Heuristic |
|---|-----------|--------|---------------------|
| 1 | **Has logo image** | 3 | `<img>` with `height` attr ≤100px or `width` ≤200px near top |
| 2 | **Badge organization** | 3 | Badges grouped in ≤5 per line, separated by blank lines |
| 3 | **Header hierarchy** | 2 | No `####` (h4) headers detected |
| 4 | **Emoji in headers** | 2 | ≥3 section headers contain emoji characters |
| 5 | **Image/GIF count** | 3 | ≥2 images/GIFs in body (beyond logo) |
| 6 | **Collapsible sections** | 2 | ≥1 `<details>` tag used |
| 7 | **Code syntax highlighting** | 2 | Code blocks use language identifiers (e.g., ```rust, ```bash) |
| 8 | **Link density** | 2 | ≥1 link per 200 characters of body text (aggressive linkification) |
| 9 | **Dark/light mode support** | 2 | `#gh-dark-mode-only` or `#gh-light-mode-only` in image URLs |
| 10 | **Table usage** | 2 | ≥1 markdown table used for structured data |
| 11 | **Line length discipline** | 2 | ≥80% of non-code, non-link lines ≤80 characters |

**Max visual score: 25**

---

## Category 3: Project Hygiene (20 points)

These checks look at the repository context (files in the same directory as the README).

### Detection Rules

| # | Criterion | Points | Detection |
|---|-----------|--------|-----------|
| 1 | **LICENSE file** | 3 | `LICENSE`, `LICENSE.md`, `LICENSE.txt`, `LICENCE` exists |
| 2 | **CONTRIBUTING.md** | 3 | `CONTRIBUTING.md` or `CONTRIBUTING.rst` exists |
| 3 | **CODE_OF_CONDUCT.md** | 2 | `CODE_OF_CONDUCT.md` or `CODE-OF-CONDUCT.md` exists |
| 4 | **SECURITY.md** | 2 | `SECURITY.md` exists |
| 5 | **CHANGELOG.md** | 1 | `CHANGELOG.md` or `CHANGES.md` exists |
| 6 | **CI badge present** | 3 | Badge URL contains `github.com/.../actions` or `travis-ci` or `circleci` or `ci` |
| 7 | **Test mention** | 2 | README mentions `test` in context of running tests |
| 8 | **Version badge** | 2 | Badge showing version (npm, crates.io, PyPI, GitHub release) |
| 9 | **Support channels** | 2 | README mentions Discord/Slack/Matrix/Discussions/IRC |

**Max hygiene score: 20**

---

## Category 4: Cognitive Funneling (15 points)

Checks whether content follows the broad→specific ordering principle.

### Detection Rules

| # | Criterion | Points | Detection |
|---|-----------|--------|-----------|
| 1 | **One-liner before installation** | 3 | Description paragraph appears before install heading |
| 2 | **Demo before API docs** | 3 | Image/GIF appears before API/reference section |
| 3 | **Features before contributing** | 2 | Features section appears before contributing section |
| 4 | **Quick start before detailed install** | 3 | Quick start section appears before full installation section |
| 5 | **Install before usage** | 2 | Installation section appears before usage section |
| 6 | **License at bottom** | 2 | License section is in the last 20% of the document |

**Max funnel score: 15**

---

## Category 5: Anti-Pattern Penalties (−15 points)

Deductions for common mistakes. These are applied as negative points.

| # | Anti-Pattern | Penalty | Detection |
|---|-------------|---------|-----------|
| 1 | **"Coming soon" / "TODO" placeholders** | −3 | Regex: `coming.?soon\|TODO\|TBD\|work in progress\|under construction` |
| 2 | **Duplicate title (H1 = repo name)** | −1 | First H1 matches repo/directory name |
| 3 | **No headers at all** | −5 | Zero markdown headers detected (wall of text) |
| 4 | **Excessive length** | −2 | >2000 lines (kitchen sink README) |
| 5 | **Badge abuse** | −2 | >15 badge URLs detected |
| 6 | **No code blocks** | −2 | Zero fenced code blocks in README |
| 7 | **Missing alt text on images** | −1 | Images without alt text: `![]()` or `<img>` without `alt` |
| 8 | **All-caps headers** | −1 | >50% of headers are ALL CAPS (shouting) |

**Max penalty: −15 (floor at 0 total score)**

---

## Scoring Formula

```
RAW_SCORE = content_score + visual_score + hygiene_score + funnel_score
PENALTY   = sum of anti-pattern deductions
FINAL     = max(0, RAW_SCORE - PENALTY)
GRADE     = letter grade based on FINAL
```

### Grade Scale

| Score | Grade | Label |
|-------|-------|-------|
| 90–100 | A+ | Exceptional — ranks with the best in Awesome README |
| 80–89 | A | Excellent — well above average |
| 70–79 | B | Good — solid, with room for improvement |
| 60–69 | C | Adequate — covers basics, misses opportunities |
| 40–59 | D | Underdeveloped — significant gaps |
| 0–39 | F | Poor — needs fundamental work |

---

## Report Output Format

The `rmb score` command produces a terminal report:

```
╔══════════════════════════════════════════╗
║         README Score: 72/100  [B]        ║
║         "Good — solid, with room         ║
║          for improvement"                ║
╚══════════════════════════════════════════╝

┌─ Content Completeness ────── 24/35 ─────┐
│ ✅ Logo/Banner                           │
│ ✅ Badges                                │
│ ✅ One-liner                             │
│ ❌ Demo GIF/Screenshot     ← +4 pts     │
│ ✅ Feature Highlights                     │
│ ✅ Quick Start                            │
│ ❌ Table of Contents        ← +2 pts     │
│ ✅ The "Why"                              │
│ ✅ Installation                           │
│ ✅ Usage/Examples                         │
│ ✅ API Reference                          │
│ ✅ Contributing                           │
│ ✅ License                                │
│ ❌ Acknowledgements         ← +1 pt      │
└──────────────────────────────────────────┘

┌─ Visual Design ─────────────── 18/25 ────┐
│ ✅ Has logo image                         │
│ ✅ Badge organization                     │
│ ✅ Header hierarchy                       │
│ ❌ Emoji in headers         ← +2 pts     │
│ ❌ Image/GIF count          ← +3 pts     │
│ ❌ Collapsible sections     ← +2 pts     │
│ ✅ Code syntax highlighting               │
│ ✅ Link density                           │
│ ❌ Dark/light mode          ← +2 pts     │
│ ✅ Table usage                            │
│ ✅ Line length discipline                 │
└──────────────────────────────────────────┘

┌─ Project Hygiene ───────────── 18/20 ────┐
│ ✅ LICENSE file                           │
│ ✅ CONTRIBUTING.md                        │
│ ✅ CODE_OF_CONDUCT.md                     │
│ ❌ SECURITY.md              ← +2 pts     │
│ ✅ CI badge                               │
│ ✅ Test mention                           │
│ ✅ Version badge                          │
│ ✅ Support channels                       │
└──────────────────────────────────────────┘

┌─ Cognitive Funneling ───────── 12/15 ────┐
│ ✅ One-liner before install               │
│ ✅ Demo before API docs                   │
│ ✅ Features before contributing           │
│ ❌ Quick start before install ← +3 pts   │
│ ✅ Install before usage                   │
│ ✅ License at bottom                      │
└──────────────────────────────────────────┘

┌─ Anti-Patterns ──────────────── −0 ─────┐
│ (none detected)                           │
└──────────────────────────────────────────┘

────────────────────────────────────────────
🎯 Top 3 Recommendations (by impact):
  1. Add a demo GIF — +4 pts, highest ROI
  2. Add a table of contents — +2 pts
  3. Add emoji to section headers — +2 pts
────────────────────────────────────────────
```

### Machine-Readable Output

`rmb score --json` produces:

```json
{
  "score": 72,
  "grade": "B",
  "label": "Good — solid, with room for improvement",
  "categories": {
    "content": { "score": 24, "max": 35, "checks": { ... } },
    "visuals": { "score": 18, "max": 25, "checks": { ... } },
    "hygiene": { "score": 18, "max": 20, "checks": { ... } },
    "funnel": { "score": 12, "max": 15, "checks": { ... } },
    "antipatterns": { "penalty": 0, "max_penalty": 15, "findings": [] }
  },
  "recommendations": [
    { "criterion": "Demo GIF/Screenshot", "points": 4, "impact": "high",
      "message": "Add an animated GIF showing your project in action" },
    { "criterion": "Table of Contents", "points": 2, "impact": "medium",
      "message": "Add a TOC for READMEs longer than 2 scrolls" }
  ],
  "benchmark": {
    "percentile": 65,
    "comparison": "Better than 65% of analyzed READMEs"
  }
}
```

---

## CLI Interface

```
rmb score <TARGET> [OPTIONS]

Arguments:
  <TARGET>  Path to README.md, URL, or local directory

Options:
  --json         Output machine-readable JSON
  --check        Exit with non-zero code if score < threshold
  --threshold N  Minimum acceptable score (default: 70)
  --verbose      Show detailed check reasoning
  --no-hygiene   Skip project hygiene checks (for remote URLs)
```

### Target Resolution

1. If TARGET is a URL → fetch it, save to temp, analyze
2. If TARGET is a file → analyze directly
3. If TARGET is a directory → look for `README.md` (or `README`, `readme.md`, etc.)
4. If TARGET is a GitHub URL without raw → convert to raw URL

---

## Implementation Plan

### Phase 1: Core Engine (src/score/)

**Files to create:**

| File | Purpose | Lines (est.) |
|------|---------|-------------|
| `src/score/mod.rs` | Module root, public API | ~30 |
| `src/score/engine.rs` | Orchestrator: fetch, parse, run analyzers, compute score | ~80 |
| `src/score/rules.rs` | Rule definitions, weights, detection patterns (regex constants) | ~120 |
| `src/score/content.rs` | Content completeness analyzer (14 sections) | ~200 |
| `src/score/visuals.rs` | Visual design analyzer | ~150 |
| `src/score/hygiene.rs` | Project hygiene analyzer (file system checks) | ~100 |
| `src/score/antipatterns.rs` | Anti-pattern detector | ~120 |
| `src/score/funnel.rs` | Cognitive funneling order analyzer | ~100 |
| `src/score/report.rs` | Report formatting (terminal + JSON) | ~150 |

**Total: ~1,050 lines of new Rust code**

### Phase 2: CLI Integration

- Add `Score` variant to `Commands` enum in `main.rs`
- Wire up target resolution (URL fetch, file read, directory scan)
- Add `--json`, `--check`, `--threshold`, `--verbose`, `--no-hygiene` flags

### Phase 3: New Dependencies

Add to `Cargo.toml`:
```toml
# For HTTP fetching of remote READMEs
reqwest = { version = "0.12", features = ["blocking"] }
# For markdown parsing (extract headers, links, images, code blocks)
pulldown-cmark = "0.11"
```

### Phase 4: Testing

- Unit tests for each analyzer with fixture READMEs
- Integration test: score known READMEs and verify expected ranges
- Snapshot tests for report output

---

## Key Design Decisions

### 1. No LLM, No AI

Every check is a deterministic rule. This means:
- **Zero API costs** — runs entirely locally
- **Instant results** — no network latency beyond initial fetch
- **Reproducible scores** — same README always gets the same score
- **Explainable** — every point gained or lost has a specific reason

### 2. Markdown-Aware Parsing

Using `pulldown-cmark` to parse Markdown into an AST rather than raw regex on text. This gives us:
- Accurate header level detection
- Proper code block vs inline code distinction
- Link and image extraction with alt text
- Table detection

### 3. Conservative Detection

Heuristics err on the side of false negatives over false positives. A section that "might" be a feature list but doesn't match the pattern won't get points — this keeps scores meaningful and recommendations actionable.

### 4. Repository Context

When scoring a local directory, the engine checks for companion files (LICENSE, CONTRIBUTING.md, etc.) in the same directory. When scoring a remote URL, hygiene checks are skipped unless the user provides a `--repo` flag pointing to the repository root.

### 5. Benchmark Data

Scores become more useful with context. The engine will ship with benchmark scores from well-known projects (computed at build time) so users can see how their README compares to exemplars like meilisearch, dioxus, and httpie.

---

## Example: Scoring the Examples.md Projects

Predicted scores based on the manual audit:

| Project | Content | Visuals | Hygiene | Funnel | Penalties | **Total** | Grade |
|---------|---------|---------|---------|--------|-----------|-----------|-------|
| meilisearch | 32 | 22 | 18 | 13 | 0 | **85** | A |
| dioxus | 32 | 23 | 17 | 12 | 0 | **84** | A |
| jj | 26 | 14 | 18 | 13 | 0 | **71** | B |
| helix | 20 | 12 | 16 | 10 | −1 | **57** | D |
| yt-dlp | 28 | 15 | 16 | 11 | −2 | **68** | C |
| rustdesk | 22 | 13 | 15 | 9 | −1 | **58** | D |
| ruff | 18 | 10 | 19 | 10 | 0 | **57** | D |

These align with the manual tier assessments: meilisearch/dioxus in Tier 1 (Excellent), jj in Tier 2 (Good), and the rest in Tier 3 (Functional).

---

## Future Enhancements (Post-MVP)

- **Link checker:** Actually validate URLs (async, cached)
- **Readability scores:** Flesch-Kincaid or similar on description text
- **Diff mode:** Score two versions of a README and show improvement
- **Watch mode:** Re-score on file changes
- **CI integration:** GitHub Action that scores READMEs in PRs
- **Historical tracking:** Store scores over time to show README improvement
- **Custom rule sets:** Let users define their own scoring criteria via config file
