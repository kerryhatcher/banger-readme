# Stage 0 — Text Quality Analysis

**Status:** Planned · **Effort:** Small · **Gated:** No (default feature `fast`)

## What

Add rule-based NLP analysis to score the *quality of writing* in README copy.
Two crates provide cross-validated readability scores plus style checks.

## Dependencies

```toml
[dependencies]
writing-analysis = "0.1"   # Readability, clichés, passive voice, sentiment, sentence variety
textstat = "0.1"           # Readability only — zero-dependency second opinion
```

Both are pure Rust. `textstat` is `no_std` and ~20 KB. `writing-analysis` pulls
`regex`, `thiserror`, and `unicode-segmentation` (~150 KB total).

## New Scoring Category: Text Quality (10 pts)

### Readability Grade (4 pts)

Run both crates and cross-validate:

| `writing-analysis` | `textstat` | Confidence | Action |
|---|---|---|---|
| Grade 7–10 | Grade 7–10 | High | ✅ Full points |
| Grade 7–10 | Grade 14+ | Low | ⚠️ Half points, flag for review |
| Grade 14+ | Grade 14+ | High | ❌ Zero points, suggest simplification |
| Grade <5 | Grade <5 | High | ⚠️ Half points, may be too simplistic |

Use Flesch-Kincaid Grade Level as the primary metric. If the two crates disagree
by more than 3 grade levels, mark the check as low-confidence and award partial
credit.

### Cliché Detection (−2 pts penalty)

`writing-analysis` ships with 85 built-in English clichés. Scan the README for:

- "think outside the box"
- "at the end of the day"
- "move the needle"
- "low-hanging fruit"
- "circle back"
- ...and 80 more

Each cliché found deducts 0.5 pts from the anti-pattern penalty pool (max −2).

### Passive Voice (−2 pts penalty)

`writing-analysis` detects passive voice via be-verb + past participle patterns
with 90+ irregular verbs. Calculate the percentage of passive sentences.

| Passive % | Penalty |
|-----------|---------|
| 0–10% | None |
| 10–20% | −1 pt |
| 20%+ | −2 pts |

### Filler Word Density (−1 pt penalty)

Detect filler words (just, really, very, basically, actually, quite, rather,
pretty, somewhat, totally). If >5% of words are fillers, deduct 1 pt.

### Sentiment Analysis (2 pts bonus)

`writing-analysis` provides AFINN-111 lexicon sentiment scoring (2400+ words).
A positive sentiment score (>0.2 comparative) earns a 2 pt bonus. Neutral or
negative scores get 0.

### Sentence Variety (2 pts)

`writing-analysis` measures sentence starter diversity and length variance.
A structure variety score >0.5 earns 2 pts. Below 0.3 gets 0.

## Implementation Notes

- Extract plain text from the README by stripping markdown formatting and HTML
  tags (the `html_stripped` field already exists on `ReadmeStructure`).
- Run both readability crates on the stripped text.
- Add a `TextQualityResult` struct to `src/score/` alongside the existing
  `ContentResult`, `VisualResult`, etc.
- Wire into `engine.rs` scoring pipeline.
- The `textstat` comparison is purely for confidence — only `writing-analysis`
  scores count toward the final grade.

## Files to Create/Modify

| File | Action |
|------|--------|
| `Cargo.toml` | Add `writing-analysis`, `textstat` |
| `src/score/text_quality.rs` | New module — `TextQualityResult`, `analyze()` |
| `src/score/engine.rs` | Wire `text_quality::analyze()` into pipeline |
| `src/score/report.rs` | Add `text_quality` field to `ScoredReport` |
| `src/score/mod.rs` | Register `text_quality` module |
