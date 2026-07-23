use crate::score::content::ReadmeStructure;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct AntipatternResult {
    pub penalty: f64,
    pub max_penalty: f64,
    pub findings: Vec<Finding>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Finding {
    pub name: &'static str,
    pub detected: bool,
    pub penalty: f64,
    pub detail: String,
}

pub fn analyze(structure: &ReadmeStructure) -> AntipatternResult {
    let mut findings = Vec::new();
    let mut penalty: f64 = 0.0;
    let max_penalty = 15.0;

    // 1. "Coming soon" / "TODO" placeholders (−3)
    let has_placeholders = structure.placeholder_count > 0;
    findings.push(Finding {
        name: "Placeholder text",
        detected: has_placeholders,
        penalty: if has_placeholders { 3.0 } else { 0.0 },
        detail: if has_placeholders {
            format!(
                "Found {} placeholder(s) like 'Coming soon', 'TODO', or 'TBD'",
                structure.placeholder_count
            )
        } else {
            String::new()
        },
    });
    if has_placeholders {
        penalty += 3.0;
    }

    // 2. Duplicate title — H1 matches repo name (−1)
    findings.push(Finding {
        name: "Duplicate title (H1 = repo name)",
        detected: structure.first_h1_is_title,
        penalty: if structure.first_h1_is_title { 1.0 } else { 0.0 },
        detail: if structure.first_h1_is_title {
            "First heading is an H1 — consider starting with a logo instead".into()
        } else {
            String::new()
        },
    });
    if structure.first_h1_is_title {
        penalty += 1.0;
    }

    // 3. No headers at all — wall of text (−5)
    let no_headers = structure.headings.is_empty();
    findings.push(Finding {
        name: "No headers (wall of text)",
        detected: no_headers,
        penalty: if no_headers { 5.0 } else { 0.0 },
        detail: if no_headers {
            "No markdown headers found — break content into sections".into()
        } else {
            String::new()
        },
    });
    if no_headers {
        penalty += 5.0;
    }

    // 4. Excessive length — kitchen sink (−2)
    let too_long = structure.lines.len() > 2000;
    findings.push(Finding {
        name: "Excessive length (>2000 lines)",
        detected: too_long,
        penalty: if too_long { 2.0 } else { 0.0 },
        detail: if too_long {
            format!(
                "README is {} lines — consider moving detailed content to separate docs",
                structure.lines.len()
            )
        } else {
            String::new()
        },
    });
    if too_long {
        penalty += 2.0;
    }

    // 5. Badge abuse — >15 badges (−2)
    let badge_abuse = structure.badge_count > 15;
    findings.push(Finding {
        name: "Badge abuse (>15 badges)",
        detected: badge_abuse,
        penalty: if badge_abuse { 2.0 } else { 0.0 },
        detail: if badge_abuse {
            format!(
                "Found {} badge URLs — consider keeping only the most important 5–10",
                structure.badge_count
            )
        } else {
            String::new()
        },
    });
    if badge_abuse {
        penalty += 2.0;
    }

    // 6. No code blocks (−2)
    let no_code = structure.code_block_count == 0;
    findings.push(Finding {
        name: "No code blocks",
        detected: no_code,
        penalty: if no_code { 2.0 } else { 0.0 },
        detail: if no_code {
            "No fenced code blocks found — add usage examples".into()
        } else {
            String::new()
        },
    });
    if no_code {
        penalty += 2.0;
    }

    // 7. Missing alt text on images (−1)
    let missing_alt = structure.images_no_alt > 0;
    findings.push(Finding {
        name: "Missing alt text on images",
        detected: missing_alt,
        penalty: if missing_alt { 1.0 } else { 0.0 },
        detail: if missing_alt {
            format!(
                "{} image(s) without alt text — add descriptive alt text for accessibility",
                structure.images_no_alt
            )
        } else {
            String::new()
        },
    });
    if missing_alt {
        penalty += 1.0;
    }

    // 8. All-caps headers (−1)
    let total_headers = structure.headings.len().max(1);
    let caps_ratio = structure.all_caps_headers as f64 / total_headers as f64;
    let caps_abuse = caps_ratio > 0.5;
    findings.push(Finding {
        name: "All-caps headers",
        detected: caps_abuse,
        penalty: if caps_abuse { 1.0 } else { 0.0 },
        detail: if caps_abuse {
            format!(
                "{} of {} headers are ALL CAPS — use sentence case or Title Case instead",
                structure.all_caps_headers, total_headers
            )
        } else {
            String::new()
        },
    });
    if caps_abuse {
        penalty += 1.0;
    }

    AntipatternResult {
        penalty: penalty.min(max_penalty),
        max_penalty,
        findings,
    }
}
