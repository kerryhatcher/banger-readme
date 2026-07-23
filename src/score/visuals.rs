use crate::score::content::ReadmeStructure;
use crate::score::rules::*;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct VisualResult {
    pub score: f64,
    pub max: f64,
    pub checks: Vec<super::content::Check>,
}

pub fn analyze(structure: &ReadmeStructure) -> VisualResult {
    use super::content::Check;
    let mut checks = Vec::new();
    let mut score: f64 = 0.0;
    let max = 25.0;

    // 1. Has logo image (3 pts)
    checks.push(Check {
        name: "Has logo image",
        passed: structure.has_logo,
        points: if structure.has_logo { 3.0 } else { 0.0 },
        max_points: 3.0,
            confidence: 1.0,
    });
    score += if structure.has_logo { 3.0 } else { 0.0 };

    // 2. Badge organization (3 pts)
    checks.push(Check {
        name: "Badge organization",
        passed: structure.badges_organized,
        points: if structure.badges_organized { 3.0 } else { 0.0 },
        max_points: 3.0,
            confidence: 1.0,
    });
    score += if structure.badges_organized { 3.0 } else { 0.0 };

    // 3. Header hierarchy — no h4 (2 pts)
    let has_h4 = structure.headings.iter().any(|(lvl, _)| *lvl >= 4);
    checks.push(Check {
        name: "Header hierarchy (no h4+)",
        passed: !has_h4,
        points: if !has_h4 { 2.0 } else { 0.0 },
        max_points: 2.0,
            confidence: 1.0,
    });
    score += if !has_h4 { 2.0 } else { 0.0 };

    // 4. Emoji in headers (2 pts) — ≥3 headers with emoji
    let emoji_headers = structure
        .headings
        .iter()
        .filter(|(_, text)| RE_EMOJI.is_match(text))
        .count();
    let has_emoji_headers = emoji_headers >= 3;
    checks.push(Check {
        name: "Emoji in headers",
        passed: has_emoji_headers,
        points: if has_emoji_headers { 2.0 } else { 0.0 },
        max_points: 2.0,
            confidence: 1.0,
    });
    score += if has_emoji_headers { 2.0 } else { 0.0 };

    // 5. Image/GIF count (3 pts) — ≥2 images beyond logo
    let image_count = structure.image_count + structure.img_tag_count;
    let has_images = image_count >= 2;
    checks.push(Check {
        name: "Image/GIF count (≥2)",
        passed: has_images,
        points: if has_images { 3.0 } else { 0.0 },
        max_points: 3.0,
            confidence: 1.0,
    });
    score += if has_images { 3.0 } else { 0.0 };

    // 6. Collapsible sections (2 pts)
    let has_collapsible = structure.details_count >= 1;
    checks.push(Check {
        name: "Collapsible sections",
        passed: has_collapsible,
        points: if has_collapsible { 2.0 } else { 0.0 },
        max_points: 2.0,
            confidence: 1.0,
    });
    score += if has_collapsible { 2.0 } else { 0.0 };

    // 7. Code syntax highlighting (2 pts) — ≥50% of code blocks have language
    let has_highlight = if structure.code_block_count > 0 {
        (structure.lang_code_blocks as f64 / structure.code_block_count as f64) >= 0.5
    } else {
        false
    };
    checks.push(Check {
        name: "Code syntax highlighting",
        passed: has_highlight,
        points: if has_highlight { 2.0 } else { 0.0 },
        max_points: 2.0,
            confidence: 1.0,
    });
    score += if has_highlight { 2.0 } else { 0.0 };

    // 8. Link density (2 pts) — ≥1 link per 200 chars
    let char_count = structure.raw.len().max(1);
    let link_density = structure.link_count as f64 / (char_count as f64 / 300.0);
    let has_link_density = link_density >= 1.0;
    checks.push(Check {
        name: "Link density",
        passed: has_link_density,
        points: if has_link_density { 2.0 } else { 0.0 },
        max_points: 2.0,
            confidence: 1.0,
    });
    score += if has_link_density { 2.0 } else { 0.0 };

    // 9. Dark/light mode support (2 pts)
    let has_mode_support = structure.has_dark_mode || structure.has_light_mode;
    checks.push(Check {
        name: "Dark/light mode support",
        passed: has_mode_support,
        points: if has_mode_support { 2.0 } else { 0.0 },
        max_points: 2.0,
            confidence: 1.0,
    });
    score += if has_mode_support { 2.0 } else { 0.0 };

    // 10. Table usage (2 pts)
    let has_tables = structure.table_rows >= 1;
    checks.push(Check {
        name: "Table usage",
        passed: has_tables,
        points: if has_tables { 2.0 } else { 0.0 },
        max_points: 2.0,
            confidence: 1.0,
    });
    score += if has_tables { 2.0 } else { 0.0 };

    // 11. Line length discipline (2 pts)
    checks.push(Check {
        name: "Line length discipline (≤80)",
        passed: structure.line_length_ok,
        points: if structure.line_length_ok { 2.0 } else { 0.0 },
        max_points: 2.0,
            confidence: 1.0,
    });
    score += if structure.line_length_ok { 2.0 } else { 0.0 };

    VisualResult {
        score: score.min(max),
        max,
        checks,
    }
}
