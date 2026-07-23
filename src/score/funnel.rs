use crate::score::content::ReadmeStructure;
use crate::score::rules::*;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct FunnelResult {
    pub score: f64,
    pub max: f64,
    pub checks: Vec<super::content::Check>,
}

/// Find the line index of the first heading matching any of the given patterns.
fn first_heading_line(
    headings: &[(usize, String)],
    _heading_lines: &[usize],
    patterns: &[&str],
) -> Option<usize> {
    for (i, (_, text)) in headings.iter().enumerate() {
        if heading_matches(text, patterns) {
            return Some(i);
        }
    }
    None
}

/// Find the line number of the first image (markdown or HTML) in the raw text.
fn first_image_line(raw: &str) -> Option<usize> {
    for (i, line) in raw.lines().enumerate() {
        if RE_MD_IMAGE.is_match(line) || RE_IMAGE_TAG.is_match(line) {
            return Some(i);
        }
    }
    None
}

/// Find the line number of the first paragraph (non-empty, non-heading, non-image text).
fn first_paragraph_line(raw: &str) -> Option<usize> {
    for (i, line) in raw.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty()
            || trimmed.starts_with('#')
            || trimmed.starts_with('[')
            || trimmed.starts_with('!')
            || trimmed.starts_with('<')
        {
            continue;
        }
        if trimmed.len() > 30 {
            return Some(i);
        }
    }
    None
}

pub fn analyze(structure: &ReadmeStructure) -> FunnelResult {
    use super::content::Check;
    let mut checks = Vec::new();
    let mut score: f64 = 0.0;
    let max = 15.0;

    let headings = &structure.headings;
    let heading_lines = &structure.heading_lines;

    // 1. One-liner before installation (3 pts)
    let oneliner_line = first_paragraph_line(&structure.raw);
    let install_idx = first_heading_line(headings, heading_lines, INSTALL_PATTERNS);
    let oneliner_first = match (oneliner_line, install_idx) {
        (Some(p), Some(_i)) => {
            // The paragraph should appear before the install heading
            // We approximate: if the paragraph is in the first 30 lines, it's before install
            p < 30
        }
        _ => oneliner_line.is_some(),
    };
    checks.push(Check {
        name: "One-liner before installation",
        passed: oneliner_first,
        points: if oneliner_first { 3.0 } else { 0.0 },
        max_points: 3.0,
            confidence: 1.0,
    });
    score += if oneliner_first { 3.0 } else { 0.0 };

    // 2. Demo before API docs (3 pts)
    let demo_line = first_image_line(&structure.raw);
    let api_idx = first_heading_line(headings, heading_lines, API_PATTERNS);
    let demo_first = match (demo_line, api_idx) {
        (Some(d), Some(_a)) => {
            // If there's a demo image, it should be in the first 100 lines
            d < 100
        }
        _ => demo_line.is_some(),
    };
    checks.push(Check {
        name: "Demo before API docs",
        passed: demo_first,
        points: if demo_first { 3.0 } else { 0.0 },
        max_points: 3.0,
            confidence: 1.0,
    });
    score += if demo_first { 3.0 } else { 0.0 };

    // 3. Features before contributing (2 pts)
    let features_idx = first_heading_line(headings, heading_lines, FEATURE_PATTERNS);
    let contrib_idx = first_heading_line(headings, heading_lines, CONTRIBUTING_PATTERNS);
    let features_first = match (features_idx, contrib_idx) {
        (Some(f), Some(c)) => f < c,
        _ => true, // can't determine, give benefit of doubt
    };
    checks.push(Check {
        name: "Features before contributing",
        passed: features_first,
        points: if features_first { 2.0 } else { 0.0 },
        max_points: 2.0,
            confidence: 1.0,
    });
    score += if features_first { 2.0 } else { 0.0 };

    // 4. Quick start before detailed install (3 pts)
    let quick_idx = first_heading_line(headings, heading_lines, QUICKSTART_PATTERNS);
    let quick_first = match (quick_idx, install_idx) {
        (Some(q), Some(i)) => q < i,
        _ => quick_idx.is_some(), // if only quick start exists, that's fine
    };
    checks.push(Check {
        name: "Quick start before detailed install",
        passed: quick_first,
        points: if quick_first { 3.0 } else { 0.0 },
        max_points: 3.0,
            confidence: 1.0,
    });
    score += if quick_first { 3.0 } else { 0.0 };

    // 5. Install before usage (2 pts)
    let usage_idx = first_heading_line(headings, heading_lines, USAGE_PATTERNS);
    let install_first = match (install_idx, usage_idx) {
        (Some(i), Some(u)) => i < u,
        _ => true,
    };
    checks.push(Check {
        name: "Install before usage",
        passed: install_first,
        points: if install_first { 2.0 } else { 0.0 },
        max_points: 2.0,
            confidence: 1.0,
    });
    score += if install_first { 2.0 } else { 0.0 };

    // 6. License at bottom (2 pts) — license section in last 20% of headings
    let license_idx = first_heading_line(headings, heading_lines, LICENSE_PATTERNS);
    let license_at_bottom = match license_idx {
        Some(l) => {
            let total = headings.len().max(1);
            l as f64 >= total as f64 * 0.8
        }
        None => false,
    };
    checks.push(Check {
        name: "License at bottom",
        passed: license_at_bottom,
        points: if license_at_bottom { 2.0 } else { 0.0 },
        max_points: 2.0,
            confidence: 1.0,
    });
    score += if license_at_bottom { 2.0 } else { 0.0 };

    FunnelResult {
        score: score.min(max),
        max,
        checks,
    }
}
