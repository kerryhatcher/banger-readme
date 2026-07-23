use crate::score::content::ReadmeStructure;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct TextQualityResult {
    pub score: f64,
    pub max: f64,
    pub checks: Vec<super::content::Check>,
    /// Cross-validation detail between the two readability engines
    pub consensus: ReadabilityConsensus,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReadabilityConsensus {
    /// writing-analysis Flesch-Kincaid grade
    pub wa_grade: Option<f64>,
    /// textstat Flesch-Kincaid grade
    pub ts_grade: Option<f64>,
    /// Whether the two engines agree (within 3 grade levels)
    pub agree: bool,
    /// Confidence level: "high" | "low"
    pub confidence: String,
}

pub fn analyze(structure: &ReadmeStructure) -> TextQualityResult {
    use super::content::Check;
    let mut checks = Vec::new();
    let mut score: f64 = 0.0;
    let max = 10.0;

    // Extract plain text from the README (strip markdown + HTML)
    let plain_text = extract_plain_text(&structure.raw);

    // Skip analysis if there's not enough text
    if plain_text.split_whitespace().count() < 20 {
        return TextQualityResult {
            score: 0.0,
            max,
            checks,
            consensus: ReadabilityConsensus {
                wa_grade: None,
                ts_grade: None,
                agree: true,
                confidence: "skipped — not enough text".into(),
            },
        };
    }

    // ── 1. Readability Grade (4 pts) ──────────────────────────────────────
    let wa_readability = writing_analysis::analyze_readability(&plain_text).ok();
    let wa_grade = wa_readability.as_ref().map(|r| r.flesch_kincaid_grade);

    let ts_grade = {
        let grade = textstat::flesch_kincaid_grade(&plain_text);
        if grade.is_finite() {
            Some(grade)
        } else {
            None
        }
    };

    let (readability_passed, readability_conf, consensus) =
        evaluate_readability(wa_grade, ts_grade);

    let readability_pts = if readability_passed {
        4.0 * readability_conf
    } else {
        0.0
    };

    checks.push(Check {
        name: "Readability grade (7–10)",
        passed: readability_passed,
        points: readability_pts,
        max_points: 4.0,
        confidence: readability_conf,
    });
    score += readability_pts;

    // ── 2. Cliché Detection (−2 pts penalty, scored as bonus for clean copy) ──
    let cliche_result = writing_analysis::detect_cliches(&plain_text).ok();
    let cliche_free = cliche_result.map_or(true, |r| r.count == 0);
    let cliche_pts = if cliche_free { 2.0 } else { 0.0 };

    checks.push(Check {
        name: "Cliché-free copy",
        passed: cliche_free,
        points: cliche_pts,
        max_points: 2.0,
        confidence: 1.0,
    });
    score += cliche_pts;

    // ── 3. Passive Voice (−2 pts penalty, scored as bonus for active voice) ──
    let passive_result =
        writing_analysis::detect_passive_voice(&plain_text).ok();
    let passive_ok = passive_result.map_or(true, |r| r.percentage <= 20.0);
    let passive_pts = if passive_ok { 2.0 } else { 0.0 };

    checks.push(Check {
        name: "Active voice (≤20% passive)",
        passed: passive_ok,
        points: passive_pts,
        max_points: 2.0,
        confidence: 1.0,
    });
    score += passive_pts;

    // ── 4. Sentiment (1 pt bonus for positive tone) ───────────────────────
    let sentiment_result = writing_analysis::analyze_sentiment(&plain_text).ok();
    let positive_tone = sentiment_result.map_or(false, |r| r.comparative > 0.1);
    let sentiment_pts = if positive_tone { 1.0 } else { 0.0 };

    checks.push(Check {
        name: "Positive/welcoming tone",
        passed: positive_tone,
        points: sentiment_pts,
        max_points: 1.0,
        confidence: 1.0,
    });
    score += sentiment_pts;

    // ── 5. Sentence Variety (1 pt) ────────────────────────────────────────
    let variety_result =
        writing_analysis::analyze_sentence_variety(&plain_text).ok();
    let has_variety = variety_result.map_or(false, |r| r.structure_variety > 0.4);
    let variety_pts = if has_variety { 1.0 } else { 0.0 };

    checks.push(Check {
        name: "Sentence variety",
        passed: has_variety,
        points: variety_pts,
        max_points: 1.0,
        confidence: 1.0,
    });
    score += variety_pts;

    TextQualityResult {
        score: score.min(max),
        max,
        checks,
        consensus,
    }
}

/// Strip markdown formatting to extract clean plain text for NLP analysis.
/// Uses pulldown-cmark to extract only text content, preserving sentence boundaries.
fn extract_plain_text(raw: &str) -> String {
    use pulldown_cmark::{Event, Parser, Tag, TagEnd};

    let parser = Parser::new(raw);
    let mut text = String::new();
    let mut last_was_text = false;

    for event in parser {
        match event {
            Event::Text(t) | Event::Code(t) => {
                if last_was_text {
                    text.push(' ');
                }
                text.push_str(&t);
                last_was_text = true;
            }
            Event::SoftBreak | Event::HardBreak => {
                text.push(' ');
                last_was_text = false;
            }
            Event::Start(Tag::Paragraph)
            | Event::Start(Tag::Heading { .. })
            | Event::Start(Tag::BlockQuote(_)) => {
                if !text.is_empty() && !text.ends_with('.') && !text.ends_with('\n') {
                    text.push('.');
                }
                last_was_text = false;
            }
            Event::End(TagEnd::Paragraph)
            | Event::End(TagEnd::Heading(_))
            | Event::End(TagEnd::BlockQuote(_)) => {
                if last_was_text && !text.ends_with('.') {
                    text.push('.');
                }
                text.push(' ');
                last_was_text = false;
            }
            Event::Start(Tag::List(_)) | Event::End(TagEnd::List(_)) => {
                last_was_text = false;
            }
            Event::Start(Tag::Item) => {
                last_was_text = false;
            }
            Event::End(TagEnd::Item) => {
                if last_was_text && !text.ends_with('.') {
                    text.push('.');
                }
                text.push(' ');
                last_was_text = false;
            }
            _ => {}
        }
    }

    // Collapse multiple spaces
    let multi_space_re = regex::Regex::new(r"  +").unwrap();
    text = multi_space_re.replace_all(&text, " ").to_string();

    text.trim().to_string()
}

/// Cross-validate readability between the two engines.
fn evaluate_readability(
    wa_grade: Option<f64>,
    ts_grade: Option<f64>,
) -> (bool, f64, ReadabilityConsensus) {
    let (agree, confidence) = match (wa_grade, ts_grade) {
        (Some(wa), Some(ts)) => {
            let diff = (wa - ts).abs();
            if diff <= 3.0 {
                (true, "high")
            } else {
                (false, "low")
            }
        }
        (Some(_), None) => (true, "low"), // only one engine produced a result
        (None, Some(_)) => (true, "low"),
        (None, None) => (false, "low"),
    };

    // Determine if the consensus grade is in the 7–10 sweet spot
    let avg_grade = match (wa_grade, ts_grade) {
        (Some(wa), Some(ts)) => Some((wa + ts) / 2.0),
        (Some(wa), None) => Some(wa),
        (None, Some(ts)) => Some(ts),
        (None, None) => None,
    };

    let in_range = avg_grade.map_or(false, |g| (7.0..=10.0).contains(&g));

    let conf_value = if confidence == "high" { 1.0 } else { 0.5 };

    (
        in_range,
        conf_value,
        ReadabilityConsensus {
            wa_grade,
            ts_grade,
            agree,
            confidence: confidence.into(),
        },
    )
}
