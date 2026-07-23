use crate::score::content::ReadmeStructure;
use serde::Serialize;
use std::path::Path;
use std::time::Duration;

#[derive(Debug, Clone, Serialize)]
pub struct LinkCheckResult {
    pub score: f64,
    pub max: f64,
    pub checks: Vec<super::content::Check>,
    /// Details about each link checked
    pub links: Vec<LinkStatus>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LinkStatus {
    pub url: String,
    pub status: String, // "ok", "broken", "redirect", "timeout", "error"
    pub detail: String,
}

pub fn analyze(structure: &ReadmeStructure, repo_dir: Option<&Path>) -> LinkCheckResult {
    use super::content::Check;
    let mut checks = Vec::new();
    let mut score: f64 = 0.0;
    let max = 5.0;
    let mut link_statuses = Vec::new();

    // Extract all HTTP(S) URLs from the README
    let urls = extract_urls(&structure.raw);

    if urls.is_empty() {
        return LinkCheckResult {
            score: 0.0,
            max,
            checks,
            links: vec![],
        };
    }

    // Check each URL
    let client = reqwest::blocking::Client::builder()
        .user_agent("rmb-readme-scorer/0.7")
        .timeout(Duration::from_secs(10))
        .build()
        .ok();

    let mut broken_count = 0;
    let mut redirect_count = 0;

    for url in &urls {
        if let Some(ref client) = client {
            match check_url(client, url) {
                Ok(status) => {
                    if status.contains("404") || status.contains("500") || status.contains("timeout") {
                        broken_count += 1;
                    }
                    if status.contains("301") || status.contains("302") {
                        redirect_count += 1;
                    }
                    link_statuses.push(LinkStatus {
                        url: url.clone(),
                        status: if broken_count > 0 && status.contains("404") {
                            "broken".into()
                        } else if status.contains("timeout") {
                            "timeout".into()
                        } else if status.contains("301") || status.contains("302") {
                            "redirect".into()
                        } else {
                            "ok".into()
                        },
                        detail: status,
                    });
                }
                Err(e) => {
                    broken_count += 1;
                    link_statuses.push(LinkStatus {
                        url: url.clone(),
                        status: "error".into(),
                        detail: e,
                    });
                }
            }
        }
    }

    // ── 1. Broken Link Detection (3 pts) ──────────────────────────────────
    let no_broken = broken_count == 0;
    let broken_pts = if no_broken {
        3.0
    } else if broken_count == 1 {
        1.0
    } else {
        0.0
    };

    checks.push(Check {
        name: "No broken links",
        passed: no_broken,
        points: broken_pts,
        max_points: 3.0,
        confidence: 1.0,
    });
    score += broken_pts;

    // ── 2. Redirect Chain Detection (1 pt) ────────────────────────────────
    let no_excessive_redirects = redirect_count <= 2;
    checks.push(Check {
        name: "No excessive redirects",
        passed: no_excessive_redirects,
        points: if no_excessive_redirects { 1.0 } else { 0.0 },
        max_points: 1.0,
        confidence: 1.0,
    });
    score += if no_excessive_redirects { 1.0 } else { 0.0 };

    // ── 3. Anchor Fragment Validation (1 pt) ───────────────────────────────
    if let Some(dir) = repo_dir {
        let anchors_ok = check_anchors(structure, dir);
        checks.push(Check {
            name: "Valid anchor fragments",
            passed: anchors_ok,
            points: if anchors_ok { 1.0 } else { 0.0 },
            max_points: 1.0,
            confidence: 1.0,
        });
        score += if anchors_ok { 1.0 } else { 0.0 };
    }

    LinkCheckResult {
        score: score.min(max),
        max,
        checks,
        links: link_statuses,
    }
}

/// Extract all HTTP(S) URLs from markdown and HTML, cleaning trailing punctuation.
fn extract_urls(raw: &str) -> Vec<String> {
    let mut urls = Vec::new();

    // Markdown links: [text](url)
    let md_link_re = regex::Regex::new(r"\[([^\]]*)\]\(([^)]+)\)").unwrap();
    for cap in md_link_re.captures_iter(raw) {
        let url = cap.get(2).unwrap().as_str();
        if url.starts_with("http://") || url.starts_with("https://") {
            urls.push(clean_url(url));
        }
    }

    // HTML href: <a href="url">
    let html_href_re = regex::Regex::new(r#"href\s*=\s*["']([^"']+)["']"#).unwrap();
    for cap in html_href_re.captures_iter(raw) {
        let url = cap.get(1).unwrap().as_str();
        if url.starts_with("http://") || url.starts_with("https://") {
            urls.push(clean_url(url));
        }
    }

    // HTML img src: <img src="url">
    let html_img_re = regex::Regex::new(r#"src\s*=\s*["']([^"']+)["']"#).unwrap();
    for cap in html_img_re.captures_iter(raw) {
        let url = cap.get(1).unwrap().as_str();
        if url.starts_with("http://") || url.starts_with("https://") {
            urls.push(clean_url(url));
        }
    }

    // Deduplicate
    urls.sort();
    urls.dedup();
    urls
}

/// Clean trailing punctuation and quotes from a URL.
fn clean_url(url: &str) -> String {
    url.trim_end_matches(')')
        .trim_end_matches('"')
        .trim_end_matches('\'')
        .trim_end_matches('.')
        .trim_end_matches(',')
        .trim_end_matches(';')
        .to_string()
}

/// Check a single URL with a HEAD request, falling back to GET.
fn check_url(client: &reqwest::blocking::Client, url: &str) -> Result<String, String> {
    // Skip badge/image service URLs that don't respond well to HEAD
    if url.contains("img.shields.io") || url.contains("contrib.rocks") {
        return Ok("badge/image URL (skipped)".into());
    }

    // Try HEAD first
    match client.head(url).send() {
        Ok(resp) => {
            let status = resp.status().as_u16();
            if status == 405 || status == 403 {
                // HEAD not allowed, try GET
                match client.get(url).send() {
                    Ok(get_resp) => {
                        let s = get_resp.status().as_u16();
                        Ok(format!("HTTP {}", s))
                    }
                    Err(e) => Err(format!("HTTP error: {}", e)),
                }
            } else if (300..400).contains(&status) {
                // Redirect
                let location = resp
                    .headers()
                    .get("location")
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("unknown");
                Ok(format!("HTTP {} → {}", status, location))
            } else {
                Ok(format!("HTTP {}", status))
            }
        }
        Err(e) => {
            if e.is_timeout() {
                Err("timeout".into())
            } else if e.is_connect() {
                Err("connection failed".into())
            } else {
                Err(format!("error: {}", e))
            }
        }
    }
}

/// Check that anchor fragments in local links point to real headings.
fn check_anchors(structure: &ReadmeStructure, dir: &Path) -> bool {
    // Find links with #fragments pointing to local files
    let fragment_re = regex::Regex::new(r"\[([^\]]*)\]\(([^)]*#([^)]+))\)").unwrap();

    let mut all_valid = true;

    for cap in fragment_re.captures_iter(&structure.raw) {
        let path = cap.get(2).unwrap().as_str();
        let fragment = cap.get(3).unwrap().as_str();

        // Skip external URLs
        if path.starts_with("http://") || path.starts_with("https://") {
            continue;
        }

        // Determine the target file
        let target_file = if path.starts_with('#') {
            // Same-file anchor
            None
        } else {
            // Cross-file anchor
            let file_path = if let Some(clean) = path.split('#').next() {
                dir.join(clean)
            } else {
                continue;
            };
            if file_path.exists() {
                Some(file_path)
            } else {
                all_valid = false;
                continue;
            }
        };

        // Check if the fragment matches a heading
        let headings = if let Some(ref file) = target_file {
            // Read the target file and extract headings
            match std::fs::read_to_string(file) {
                Ok(content) => extract_headings(&content),
                Err(_) => {
                    all_valid = false;
                    continue;
                }
            }
        } else {
            // Use the current README's headings
            structure
                .headings
                .iter()
                .map(|(_, text)| text.clone())
                .collect()
        };

        let fragment_lower = fragment.to_lowercase().replace('-', " ");
        let found = headings.iter().any(|h| {
            let h_lower = h.to_lowercase();
            h_lower == fragment_lower
                || h_lower.replace(' ', "-") == fragment.to_lowercase()
        });

        if !found {
            all_valid = false;
        }
    }

    all_valid
}

/// Extract heading text from markdown content.
fn extract_headings(raw: &str) -> Vec<String> {
    let heading_re = regex::Regex::new(r"(?m)^#{1,4}\s+(.+)$").unwrap();
    heading_re
        .captures_iter(raw)
        .map(|c| c.get(1).unwrap().as_str().to_string())
        .collect()
}
