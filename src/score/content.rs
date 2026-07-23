use crate::score::rules::*;
use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag, TagEnd};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ContentResult {
    pub score: f64,
    pub max: f64,
    pub checks: Vec<Check>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Check {
    pub name: &'static str,
    pub passed: bool,
    pub points: f64,
    pub max_points: f64,
    /// Confidence: 1.0 = exact match, 0.5 = implicit/partial, 0.0 = absent
    #[serde(skip)]
    pub confidence: f64,
}

/// Parsed structure extracted from the README.
pub struct ReadmeStructure {
    pub headings: Vec<(usize, String)>,
    pub heading_lines: Vec<usize>,
    pub raw: String,
    pub lines: Vec<String>,
    pub code_block_count: usize,
    pub lang_code_blocks: usize,
    pub image_count: usize,
    pub img_tag_count: usize,
    pub badge_count: usize,
    pub toc_link_count: usize,
    pub emoji_count: usize,
    pub details_count: usize,
    pub table_rows: usize,
    pub has_dark_mode: bool,
    pub has_light_mode: bool,
    pub link_count: usize,
    pub first_h1_is_title: bool,
    pub placeholder_count: usize,
    pub all_caps_headers: usize,
    pub images_no_alt: usize,
    pub has_gif: bool,
    pub has_logo: bool,
    pub badges_organized: bool,
    pub line_length_ok: bool,
    /// Code block appears in first 30 lines (implicit quick start)
    pub early_code_block: bool,
    /// HTML-stripped text for content checks
    #[allow(dead_code)]
    pub html_stripped: String,
    /// HTML headings extracted from <h1>-<h4> tags
    pub html_headings: Vec<(usize, String)>,
}

pub fn parse_structure(raw: &str) -> ReadmeStructure {
    let lines: Vec<String> = raw.lines().map(String::from).collect();

    // Build HTML-stripped version for content checks
    let html_stripped = strip_html_tags(raw);
    let html_headings = extract_html_headings(raw);

    let mut structure = ReadmeStructure {
        headings: Vec::new(),
        heading_lines: Vec::new(),
        raw: raw.to_string(),
        lines: lines.clone(),
        code_block_count: 0,
        lang_code_blocks: 0,
        image_count: 0,
        img_tag_count: 0,
        badge_count: 0,
        toc_link_count: 0,
        emoji_count: 0,
        details_count: 0,
        table_rows: 0,
        has_dark_mode: false,
        has_light_mode: false,
        link_count: 0,
        first_h1_is_title: false,
        placeholder_count: 0,
        all_caps_headers: 0,
        images_no_alt: 0,
        has_gif: false,
        has_logo: false,
        badges_organized: false,
        line_length_ok: false,
        early_code_block: false,
        html_stripped,
        html_headings,
    };

    // Parse with pulldown-cmark
    let parser = Parser::new(raw);
    let mut in_table_head = false;
    let _current_header_text = String::new();
    let _in_heading = false;

    let mut current_header_level: usize = 0;
    let mut current_header_text = String::new();
    let mut in_heading = false;

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(kind)) => {
                structure.code_block_count += 1;
                if let CodeBlockKind::Fenced(lang) = kind {
                    if !lang.is_empty() {
                        structure.lang_code_blocks += 1;
                    }
                }
            }
            Event::End(TagEnd::CodeBlock) => {}
            Event::Start(Tag::Heading { level, .. }) => {
                current_header_level = level as usize;
                current_header_text.clear();
                in_heading = true;
            }
            Event::End(TagEnd::Heading(_)) => {
                if in_heading && !current_header_text.is_empty() {
                    structure
                        .headings
                        .push((current_header_level, current_header_text.clone()));
                    structure.heading_lines.push(0);
                    if current_header_text
                        .chars()
                        .all(|c| c.is_uppercase() || !c.is_alphabetic())
                        && current_header_text.chars().any(|c| c.is_alphabetic())
                    {
                        structure.all_caps_headers += 1;
                    }
                }
                in_heading = false;
            }
            Event::Start(Tag::Image { .. }) => {
                structure.image_count += 1;
            }
            Event::Start(Tag::Table(_)) => {
                in_table_head = true;
            }
            Event::End(TagEnd::Table) => {
                in_table_head = false;
            }
            Event::Start(Tag::TableRow) => {
                if !in_table_head {
                    structure.table_rows += 1;
                }
            }
            Event::End(TagEnd::TableRow) => {}
            Event::Start(Tag::TableHead) => {
                in_table_head = true;
            }
            Event::End(TagEnd::TableHead) => {
                in_table_head = false;
            }
            Event::Text(text) | Event::Code(text) => {
                if in_heading {
                    current_header_text.push_str(&text);
                }
                structure.emoji_count += RE_EMOJI.find_iter(&text).count();
            }
            _ => {}
        }
    }

    // Merge HTML headings into the headings list
    for (level, text) in &structure.html_headings {
        structure.headings.push((*level, text.clone()));
    }

    // Regex-based checks on raw text
    structure.img_tag_count = RE_IMAGE_TAG.find_iter(raw).count();
    structure.badge_count = RE_BADGE_URL.find_iter(raw).count();
    structure.toc_link_count = RE_TOC_LINK.find_iter(raw).count();
    structure.details_count = RE_DETAILS_TAG.find_iter(raw).count();
    structure.has_dark_mode =
        RE_DARK_MODE.is_match(raw) || raw.contains("prefers-color-scheme: dark");
    structure.has_light_mode =
        RE_LIGHT_MODE.is_match(raw) || raw.contains("prefers-color-scheme: light");
    structure.has_gif = RE_GIF.is_match(raw);
    structure.placeholder_count = RE_PLACEHOLDER.find_iter(raw).count();

    // Count HTML <code> and <pre> blocks as code blocks
    let html_code_re = regex::Regex::new(r"<code[^>]*>|</code>|<pre[^>]*>|</pre>").unwrap();
    let html_code_count = html_code_re.find_iter(raw).count() / 2; // pairs
    if structure.code_block_count == 0 && html_code_count > 0 {
        structure.code_block_count = html_code_count;
        structure.lang_code_blocks = html_code_count;
    }

    // Count fenced code blocks via regex as fallback
    let fenced_re = regex::Regex::new(r"```(?s).*?```").unwrap();
    let fenced_count = fenced_re.find_iter(raw).count();
    if structure.code_block_count == 0 && fenced_count > 0 {
        structure.code_block_count = fenced_count;
        structure.lang_code_blocks = fenced_count;
    }

    // Count links
    let link_re = regex::Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").unwrap();
    structure.link_count = link_re.find_iter(raw).count();

    // Count table rows via regex as fallback (pulldown-cmark table parsing varies by version)
    if structure.table_rows == 0 {
        let table_row_re = regex::Regex::new(r"^\|.*\|.*\|$").unwrap();
        let regex_table_rows = lines.iter().filter(|l| table_row_re.is_match(l)).count();
        // Subtract header separator lines (|---|---|)
        let separator_re = regex::Regex::new(r"^\|[\s\-:]+\|$").unwrap();
        let separators = lines.iter().filter(|l| separator_re.is_match(l)).count();
        structure.table_rows = regex_table_rows.saturating_sub(separators);
    }

    // Check for logo (image in first 20 lines)
    let first_twenty: String = lines
        .iter()
        .take(20)
        .cloned()
        .collect::<Vec<_>>()
        .join("\n");
    let has_html_logo = RE_IMAGE_TAG.is_match(&first_twenty)
        && (first_twenty.contains("height")
            || first_twenty.contains("width")
            || first_twenty.to_lowercase().contains("logo"));
    let has_md_logo = RE_MD_IMAGE.is_match(&first_twenty)
        && (first_twenty.to_lowercase().contains("logo") || first_twenty.contains(".svg"));
    structure.has_logo = has_html_logo || has_md_logo;

    // Check badge organization (relaxed: only fail if >5 on a single line)
    structure.badges_organized = check_badge_organization(&lines);

    // Check line length
    structure.line_length_ok = check_line_length(&lines);

    // Check for early code block (implicit quick start)
    structure.early_code_block = check_early_code_block(&lines);

    // Check images without alt text
    structure.images_no_alt = RE_MD_IMAGE
        .captures_iter(raw)
        .filter(|cap| cap.get(1).is_none_or(|m| m.as_str().trim().is_empty()))
        .count();

    // Check if first heading is an H1 (redundant with repo name)
    // Only flag actual H1 headings, not H2-H4
    if let Some((level, text)) = structure.headings.first() {
        if *level == 1 && text.len() > 3 {
            structure.first_h1_is_title = true;
        }
    }

    structure
}

/// Strip HTML tags to extract plain text content.
fn strip_html_tags(raw: &str) -> String {
    let tag_re = regex::Regex::new(r"<[^>]+>").unwrap();
    let mut result = tag_re.replace_all(raw, "").to_string();
    // Collapse multiple blank lines
    let blank_re = regex::Regex::new(r"\n{3,}").unwrap();
    result = blank_re.replace_all(&result, "\n\n").to_string();
    result
}

/// Extract headings from HTML <h1>-<h4> tags.
fn extract_html_headings(raw: &str) -> Vec<(usize, String)> {
    let mut headings = Vec::new();
    let inner_tag_re = regex::Regex::new(r"<[^>]+>").unwrap();
    for level in 1..=4 {
        let tag = format!("h{}", level);
        let re = regex::Regex::new(&format!(r"<{}[^>]*>\s*(.*?)\s*</{}>", tag, tag)).unwrap();
        for cap in re.captures_iter(raw) {
            let text = cap.get(1).unwrap().as_str().trim().to_string();
            // Strip inner HTML tags from the heading text
            let clean = inner_tag_re.replace_all(&text, "").to_string();
            if !clean.is_empty() {
                headings.push((level, clean));
            }
        }
    }
    headings
}

fn check_badge_organization(lines: &[String]) -> bool {
    // Relaxed: only fail if any single line has >5 badges
    for line in lines {
        let count = RE_BADGE_URL.find_iter(line).count();
        if count > 5 {
            return false;
        }
    }
    true
}

fn check_line_length(lines: &[String]) -> bool {
    let mut ok = 0;
    let mut total = 0;
    for line in lines {
        if line.starts_with("    ") || line.starts_with('\t') || line.starts_with("```") {
            continue;
        }
        if line.contains("http") && line.len() > 80 {
            continue;
        }
        total += 1;
        if line.len() <= 80 {
            ok += 1;
        }
    }
    if total == 0 {
        return true;
    }
    (ok as f64 / total as f64) >= 0.8
}

/// Check if a fenced code block appears in the first 30 lines (implicit quick start).
fn check_early_code_block(lines: &[String]) -> bool {
    let first_30: String = lines
        .iter()
        .take(30)
        .cloned()
        .collect::<Vec<_>>()
        .join("\n");
    first_30.contains("```")
}

/// Analyze content completeness with confidence-weighted scoring.
pub fn analyze(structure: &ReadmeStructure) -> ContentResult {
    let mut checks = Vec::new();
    let mut score: f64 = 0.0;
    let max = 35.0;

    // Helper to add a check with confidence
    let mut add_check = |name: &'static str, passed: bool, max_pts: f64, confidence: f64| {
        let pts = if passed { max_pts * confidence } else { 0.0 };
        checks.push(Check {
            name,
            passed,
            points: pts,
            max_points: max_pts,
            confidence,
        });
        pts
    };

    // 1. Logo/Banner (3 pts)
    score += add_check("Logo/Banner", structure.has_logo, 3.0, 1.0);

    // 2. Badges (3 pts)
    score += add_check("Badges", structure.badge_count >= 2, 3.0, 1.0);

    // 3. One-liner (3 pts)
    let oneliner_conf = check_oneliner_confidence(&structure.raw);
    score += add_check("One-liner", oneliner_conf > 0.0, 3.0, oneliner_conf);

    // 4. Demo GIF/Screenshot (4 pts)
    let has_demo = structure.has_gif
        || has_section_merged(structure, DEMO_PATTERNS)
        || (structure.image_count + structure.img_tag_count) >= 2;
    score += add_check("Demo GIF/Screenshot", has_demo, 4.0, 1.0);

    // 5. Feature Highlights (3 pts)
    let has_features = has_section_merged(structure, FEATURE_PATTERNS);
    score += add_check("Feature Highlights", has_features, 3.0, 1.0);

    // 6. Quick Start (3 pts) — explicit heading OR implicit (early code block)
    let has_explicit_qs = has_section_merged(structure, QUICKSTART_PATTERNS);
    let has_implicit_qs = structure.early_code_block;
    if has_explicit_qs {
        score += add_check("Quick Start", true, 3.0, 1.0);
    } else if has_implicit_qs {
        score += add_check("Quick Start", true, 3.0, 0.5);
    } else {
        score += add_check("Quick Start", false, 3.0, 0.0);
    }

    // 7. Table of Contents (2 pts)
    let has_toc = has_section_merged(structure, TOC_PATTERNS) || structure.toc_link_count >= 5;
    score += add_check("Table of Contents", has_toc, 2.0, 1.0);

    // 8. The "Why" (2 pts)
    let has_why = has_section_merged(structure, WHY_PATTERNS);
    score += add_check("The \"Why\"", has_why, 2.0, 1.0);

    // 9. Installation (3 pts) — explicit OR link-heavy install section
    let has_install = has_section_merged(structure, INSTALL_PATTERNS);
    let install_conf = if has_install {
        // Check if install section is mostly links (external docs)
        if is_link_heavy_section(structure, INSTALL_PATTERNS) {
            0.7 // half credit for external docs
        } else {
            1.0
        }
    } else {
        0.0
    };
    score += add_check("Installation", has_install, 3.0, install_conf);

    // 10. Usage/Examples (3 pts)
    let has_usage = has_section_merged(structure, USAGE_PATTERNS);
    score += add_check("Usage/Examples", has_usage, 3.0, 1.0);

    // 11. API Reference (2 pts) — explicit OR docs link section
    let has_api = has_section_merged(structure, API_PATTERNS)
        || has_section_merged(structure, &["documentation", "docs"]);
    score += add_check(
        "API Reference",
        has_api,
        2.0,
        if has_api { 1.0 } else { 0.0 },
    );

    // 12. Contributing (2 pts)
    let has_contrib = has_section_merged(structure, CONTRIBUTING_PATTERNS);
    score += add_check("Contributing", has_contrib, 2.0, 1.0);

    // 13. License (1 pt)
    let has_license = has_section_merged(structure, LICENSE_PATTERNS);
    score += add_check("License", has_license, 1.0, 1.0);

    // 14. Acknowledgements (1 pt)
    let has_acks = has_section_merged(structure, ACKNOWLEDGE_PATTERNS);
    score += add_check("Acknowledgements", has_acks, 1.0, 1.0);

    // Bonus: multi-language
    let has_multilang = structure.raw.contains("README-")
        || structure.raw.contains("中文")
        || structure.raw.contains("日本語");
    if has_multilang {
        score += 2.0;
        checks.push(Check {
            name: "Multi-language (bonus)",
            passed: true,
            points: 2.0,
            max_points: 2.0,
            confidence: 1.0,
        });
    }

    // Bonus: live demo link
    let has_live_demo = structure.raw.to_lowercase().contains("live demo")
        || structure.raw.to_lowercase().contains("playground");
    if has_live_demo {
        score += 1.0;
        checks.push(Check {
            name: "Live demo link (bonus)",
            passed: true,
            points: 1.0,
            max_points: 1.0,
            confidence: 1.0,
        });
    }

    ContentResult {
        score: score.min(max),
        max,
        checks,
    }
}

/// Check if a section exists in either markdown or HTML headings.
fn has_section_merged(structure: &ReadmeStructure, patterns: &[&str]) -> bool {
    has_section(&structure.headings, patterns)
}

fn has_section(headings: &[(usize, String)], patterns: &[&str]) -> bool {
    headings
        .iter()
        .any(|(_, text)| heading_matches(text, patterns))
}

/// Check if a section's content is primarily links (external docs).
fn is_link_heavy_section(structure: &ReadmeStructure, patterns: &[&str]) -> bool {
    // Find the section, then check if the content after it is mostly links
    for (_, text) in structure.headings.iter() {
        if heading_matches(text, patterns) {
            // Look at the next ~200 chars after this heading in the raw text
            if let Some(pos) = structure.raw.to_lowercase().find(&text.to_lowercase()) {
                let after = &structure.raw[pos..];
                let snippet: String = after.chars().take(400).collect();
                let link_count = snippet.matches("http").count();
                let text_len = snippet.len().max(1);
                // If >2 links per 400 chars, it's link-heavy
                return link_count as f64 / (text_len as f64 / 400.0) > 2.0;
            }
            break;
        }
    }
    false
}

/// Returns confidence 1.0 for clear one-liner, 0.7 for HTML-wrapped, 0.0 for none.
fn check_oneliner_confidence(raw: &str) -> f64 {
    // Try HTML paragraph tags first
    let html_p_re = regex::Regex::new(r"<p[^>]*>\s*([^<]{30,400})\s*</p>").unwrap();
    if let Some(cap) = html_p_re.captures(raw) {
        let text = cap.get(1).unwrap().as_str().trim();
        let text_only = RE_EMOJI.replace_all(text, "").trim().to_string();
        if text_only.len() >= 30
            && !text.starts_with("<")
            && !text.starts_with("!")
            && !text.starts_with("[")
        {
            return 0.7; // HTML-wrapped is slightly less ideal
        }
    }

    // Fall back to plain paragraph detection
    let after_header = if let Some(pos) = raw.find("\n\n") {
        &raw[pos + 2..]
    } else {
        raw
    };

    for paragraph in after_header.split("\n\n") {
        let trimmed = paragraph.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with('[') {
            continue;
        }
        if trimmed.starts_with('!') || trimmed.starts_with('<') {
            continue;
        }
        let text_only = RE_EMOJI.replace_all(trimmed, "").trim().to_string();
        let len = text_only.len();
        if (30..=400).contains(&len)
            && !trimmed.contains('\n')
            && !trimmed.starts_with('-')
            && !trimmed.starts_with('*')
        {
            return 1.0; // Clean markdown paragraph = full confidence
        }
        // Continue checking — don't break on first non-match
    }
    0.0
}
