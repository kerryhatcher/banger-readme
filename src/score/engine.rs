use crate::score::antipatterns;
use crate::score::content;
use crate::score::funnel;
use crate::score::hygiene;
use crate::score::report::ScoredReport;
use crate::score::rules;
use crate::score::visuals;
use anyhow::{Context, Result};
use std::path::Path;

/// Fetch README content from a URL or local path.
pub fn fetch_readme(target: &str) -> Result<(String, Option<String>)> {
    if target.starts_with("http://") || target.starts_with("https://") {
        // Fetch from URL
        let client = reqwest::blocking::Client::builder()
            .user_agent("rmb-readme-scorer/0.1")
            .build()
            .context("Failed to build HTTP client")?;

        let resp = client
            .get(target)
            .send()
            .context(format!("Failed to fetch URL: {target}"))?;

        let url = resp.url().to_string();
        let body = resp
            .text()
            .context(format!("Failed to read response body from {target}"))?;

        Ok((body, Some(url)))
    } else {
        // Read from local file
        let path = Path::new(target);
        let body = std::fs::read_to_string(path)
            .context(format!("Failed to read file: {}", path.display()))?;
        Ok((body, None))
    }
}

/// Find README in a directory.
pub fn find_readme(dir: &Path) -> Option<std::path::PathBuf> {
    for name in &["README.md", "README", "readme.md", "Readme.md", "README.MD"] {
        let path = dir.join(name);
        if path.exists() {
            return Some(path);
        }
    }
    None
}

/// Run the full scoring pipeline against a README.
pub fn score_readme(raw: &str, repo_dir: Option<&Path>) -> ScoredReport {
    let structure = content::parse_structure(raw);

    let content_result = content::analyze(&structure);
    let visual_result = visuals::analyze(&structure);
    let hygiene_result = hygiene::analyze(&structure, repo_dir);
    let funnel_result = funnel::analyze(&structure);
    let antipattern_result = antipatterns::analyze(&structure);

    let raw_score =
        content_result.score + visual_result.score + hygiene_result.score + funnel_result.score;
    let penalty = antipattern_result.penalty;
    let final_score = (raw_score - penalty).max(0.0);
    let (grade, label) = rules::grade(final_score);

    ScoredReport {
        score: final_score,
        grade: grade.to_string(),
        label: label.to_string(),
        content: content_result,
        visuals: visual_result,
        hygiene: hygiene_result,
        funnel: funnel_result,
        antipatterns: antipattern_result,
    }
}
