use crate::score::content::ReadmeStructure;
use serde::Serialize;
use std::path::Path;

#[derive(Debug, Clone, Serialize)]
pub struct MultiLangResult {
    pub score: f64,
    pub max: f64,
    pub checks: Vec<super::content::Check>,
    /// Details about each non-English README found
    pub translations: Vec<TranslationStatus>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TranslationStatus {
    pub path: String,
    pub language: String,
    pub grade: Option<f64>,
    pub sections_match: bool,
}

pub fn analyze(structure: &ReadmeStructure, repo_dir: Option<&Path>) -> MultiLangResult {
    use super::content::Check;
    let mut checks = Vec::new();
    let mut score: f64 = 0.0;
    let max = 5.0;
    let mut translations = Vec::new();

    let dir = match repo_dir {
        Some(d) => d,
        None => {
            return MultiLangResult {
                score: 0.0,
                max,
                checks,
                translations,
            };
        }
    };

    // Find non-English README variants
    let variants = find_readme_variants(dir);

    if variants.is_empty() {
        return MultiLangResult {
            score: 0.0,
            max,
            checks,
            translations,
        };
    }

    // ── 1. Per-Language Readability (3 pts per language, max 3) ───────────
    let mut lang_score: f64 = 0.0;
    for (path, lang_code) in &variants {
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let grade = check_readability(&content, lang_code);
        let in_range = grade.map_or(false, |g| (7.0..=12.0).contains(&g));

        translations.push(TranslationStatus {
            path: path.display().to_string(),
            language: lang_code.clone(),
            grade,
            sections_match: true, // checked below
        });

        if in_range {
            lang_score += 1.0;
        }
    }

    let lang_pts = lang_score.min(3.0);
    checks.push(Check {
        name: "Translation readability",
        passed: lang_pts > 0.0,
        points: lang_pts,
        max_points: 3.0,
        confidence: 1.0,
    });
    score += lang_pts;

    // ── 2. Translation Completeness (2 pts) ───────────────────────────────
    let primary_sections = count_sections(&structure.headings);
    let mut all_complete = true;

    for (path, _) in &variants {
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => {
                all_complete = false;
                continue;
            }
        };
        let trans_headings = extract_headings(&content);
        let trans_sections = count_sections(&trans_headings);

        // Translation should have at least 80% of the primary sections
        if primary_sections > 0 {
            let ratio = trans_sections as f64 / primary_sections as f64;
            if ratio < 0.8 {
                all_complete = false;
            }
        }

        // Update translation status
        if let Some(t) = translations.iter_mut().find(|t| t.path == path.display().to_string()) {
            t.sections_match = primary_sections == 0 || trans_sections as f64 / primary_sections as f64 >= 0.8;
        }
    }

    let complete_pts = if all_complete { 2.0 } else { 0.0 };
    checks.push(Check {
        name: "Translation completeness",
        passed: all_complete,
        points: complete_pts,
        max_points: 2.0,
        confidence: 1.0,
    });
    score += complete_pts;

    MultiLangResult {
        score: score.min(max),
        max,
        checks,
        translations,
    }
}

/// Find non-English README variants in the repository.
fn find_readme_variants(dir: &Path) -> Vec<(std::path::PathBuf, String)> {
    let mut variants = Vec::new();

    // Common language code patterns
    let patterns = [
        ("README-zh.md", "zh"),
        ("README-ja.md", "ja"),
        ("README-ko.md", "ko"),
        ("README-ru.md", "ru"),
        ("README-de.md", "de"),
        ("README-fr.md", "fr"),
        ("README-es.md", "es"),
        ("README-it.md", "it"),
        ("README-pt.md", "pt"),
        ("README-ar.md", "ar"),
        ("README-hi.md", "hi"),
        ("README-zh-CN.md", "zh"),
        ("README-zh-TW.md", "zh"),
        ("README_zh.md", "zh"),
        ("README_ja.md", "ja"),
        ("readme-zh.md", "zh"),
        ("readme-ja.md", "ja"),
    ];

    for (filename, lang) in &patterns {
        let path = dir.join(filename);
        if path.exists() {
            variants.push((path, lang.to_string()));
        }
    }

    variants
}

/// Check readability of non-English text using readsight.
fn check_readability(content: &str, lang_code: &str) -> Option<f64> {
    let engine = readsight::ReadSight::new(lang_code).ok()?;

    // Try Flesch-Kincaid first, fall back to Gunning Fog
    if let Ok(result) = engine.score("flesch_kincaid_grade_level", content) {
        return Some(result.score);
    }

    if let Ok(result) = engine.score("gunning_fog", content) {
        return Some(result.score);
    }

    None
}

/// Count the number of unique sections in headings.
fn count_sections(headings: &[(usize, String)]) -> usize {
    headings.len()
}

/// Extract headings from markdown content.
fn extract_headings(raw: &str) -> Vec<(usize, String)> {
    let heading_re = regex::Regex::new(r"(?m)^(#{1,4})\s+(.+)$").unwrap();
    heading_re
        .captures_iter(raw)
        .map(|c| {
            let level = c.get(1).unwrap().as_str().len();
            let text = c.get(2).unwrap().as_str().to_string();
            (level, text)
        })
        .collect()
}
