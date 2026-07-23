use crate::score::content::ReadmeStructure;
use serde::Serialize;
use std::path::Path;

#[derive(Debug, Clone, Serialize)]
pub struct ImageHeuristicsResult {
    pub score: f64,
    pub max: f64,
    pub checks: Vec<super::content::Check>,
}

pub fn analyze(structure: &ReadmeStructure, repo_dir: Option<&Path>) -> ImageHeuristicsResult {
    use super::content::Check;
    let mut checks = Vec::new();
    let mut score: f64 = 0.0;
    let max = 4.0;

    let dir = match repo_dir {
        Some(d) => d,
        None => {
            // Can't check local images without a repo directory
            return ImageHeuristicsResult {
                score: 0.0,
                max,
                checks,
            };
        }
    };

    // Find local image paths in the README
    let image_paths = extract_local_image_paths(&structure.raw);

    if image_paths.is_empty() {
        return ImageHeuristicsResult {
            score: 0.0,
            max,
            checks,
        };
    }

    // ── 1. Logo Dimensions (2 pts) ────────────────────────────────────────
    let logo_path = find_logo(&image_paths, dir);
    let logo_ok = logo_path.as_ref().map_or(false, |p| check_logo_dimensions(p));

    checks.push(Check {
        name: "Logo dimensions (≥128×128)",
        passed: logo_ok,
        points: if logo_ok { 2.0 } else { 0.0 },
        max_points: 2.0,
        confidence: 1.0,
    });
    score += if logo_ok { 2.0 } else { 0.0 };

    // ── 2. Banner Width (1 pt) ────────────────────────────────────────────
    let banner_path = find_banner(&image_paths, dir);
    let banner_ok = banner_path.as_ref().map_or(false, |p| check_banner_width(p));

    checks.push(Check {
        name: "Banner width (≥1200px)",
        passed: banner_ok,
        points: if banner_ok { 1.0 } else { 0.0 },
        max_points: 1.0,
        confidence: 1.0,
    });
    score += if banner_ok { 1.0 } else { 0.0 };

    // ── 3. Image Format Optimization (1 pt) ───────────────────────────────
    let all_optimized = image_paths.iter().all(|p| {
        let full = dir.join(p);
        check_image_format(&full)
    });

    checks.push(Check {
        name: "Image format optimized",
        passed: all_optimized,
        points: if all_optimized { 1.0 } else { 0.0 },
        max_points: 1.0,
        confidence: 1.0,
    });
    score += if all_optimized { 1.0 } else { 0.0 };

    ImageHeuristicsResult {
        score: score.min(max),
        max,
        checks,
    }
}

/// Extract relative local image paths from markdown and HTML.
fn extract_local_image_paths(raw: &str) -> Vec<String> {
    let mut paths = Vec::new();

    // Markdown images: ![alt](path)
    let md_img_re = regex::Regex::new(r"!\[[^\]]*\]\(([^)]+)\)").unwrap();
    for cap in md_img_re.captures_iter(raw) {
        let path = cap.get(1).unwrap().as_str();
        if !path.starts_with("http://") && !path.starts_with("https://") {
            paths.push(path.to_string());
        }
    }

    // HTML img tags: <img src="path"
    let html_img_re = regex::Regex::new(r#"<img[^>]+src\s*=\s*["']([^"']+)["']"#).unwrap();
    for cap in html_img_re.captures_iter(raw) {
        let path = cap.get(1).unwrap().as_str();
        if !path.starts_with("http://") && !path.starts_with("https://") {
            paths.push(path.to_string());
        }
    }

    // HTML source tags: <source srcset="path"
    let source_re = regex::Regex::new(r#"<source[^>]+srcset\s*=\s*["']([^"']+)["']"#).unwrap();
    for cap in source_re.captures_iter(raw) {
        let path = cap.get(1).unwrap().as_str();
        if !path.starts_with("http://") && !path.starts_with("https://") {
            paths.push(path.to_string());
        }
    }

    paths
}

/// Find the logo image (first image, or one with "logo" in the name).
fn find_logo<'a>(paths: &'a [String], dir: &Path) -> Option<std::path::PathBuf> {
    // Prefer images with "logo" in the filename
    for path in paths {
        let lower = path.to_lowercase();
        if lower.contains("logo") || lower.contains("icon") {
            let full = dir.join(path);
            if full.exists() {
                return Some(full);
            }
        }
    }
    // Fall back to the first existing image
    for path in paths {
        let full = dir.join(path);
        if full.exists() {
            return Some(full);
        }
    }
    None
}

/// Find the banner image (wide image, or one with "header" or "banner" in the name).
fn find_banner<'a>(paths: &'a [String], dir: &Path) -> Option<std::path::PathBuf> {
    for path in paths {
        let lower = path.to_lowercase();
        if lower.contains("header") || lower.contains("banner") || lower.contains("hero") {
            let full = dir.join(path);
            if full.exists() {
                return Some(full);
            }
        }
    }
    None
}

/// Check that a logo image is at least 128×128 pixels.
fn check_logo_dimensions(path: &Path) -> bool {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    if ext == "svg" {
        return check_svg_dimensions(path, 128, 128);
    }

    let img = match image::open(path) {
        Ok(img) => img,
        Err(_) => return false,
    };
    let (w, h) = (img.width(), img.height());
    w >= 128 && h >= 128
}

/// Check that a banner image is at least 1200px wide.
fn check_banner_width(path: &Path) -> bool {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    if ext == "svg" {
        return check_svg_dimensions(path, 1200, 1);
    }

    let img = match image::open(path) {
        Ok(img) => img,
        Err(_) => return false,
    };
    img.width() >= 1200
}

/// Parse an SVG file to check its dimensions from viewBox or width/height attributes.
fn check_svg_dimensions(path: &Path, min_width: u32, min_height: u32) -> bool {
    let contents = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return false,
    };

    // Try viewBox first: viewBox="0 0 W H"
    let viewbox_re =
        regex::Regex::new(r#"viewBox\s*=\s*['"]\d+\s+\d+\s+(\d+)\s+(\d+)['"]"#).unwrap();
    if let Some(cap) = viewbox_re.captures(&contents) {
        let w: u32 = cap.get(1).unwrap().as_str().parse().unwrap_or(0);
        let h: u32 = cap.get(2).unwrap().as_str().parse().unwrap_or(0);
        return w >= min_width && h >= min_height;
    }

    // Try width/height attributes
    let width_re = regex::Regex::new(r#"width\s*=\s*['"]?(\d+)['"]?"#).unwrap();
    let height_re = regex::Regex::new(r#"height\s*=\s*['"]?(\d+)['"]?"#).unwrap();

    let w = width_re
        .captures(&contents)
        .and_then(|c| c.get(1))
        .and_then(|m| m.as_str().parse::<u32>().ok())
        .unwrap_or(0);
    let h = height_re
        .captures(&contents)
        .and_then(|c| c.get(1))
        .and_then(|m| m.as_str().parse::<u32>().ok())
        .unwrap_or(0);

    w >= min_width && h >= min_height
}

/// Check that an image uses an optimized format (not BMP or TIFF).
fn check_image_format(path: &Path) -> bool {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    // SVG is always fine (vector)
    if ext == "svg" {
        return true;
    }

    // BMP and TIFF are uncompressed — suggest PNG/WebP
    if ext == "bmp" || ext == "tiff" || ext == "tif" {
        return false;
    }

    // Check file size for raster images
    if let Ok(meta) = std::fs::metadata(path) {
        let size_mb = meta.len() as f64 / (1024.0 * 1024.0);
        if size_mb > 1.0 {
            return false; // >1MB raster image could be optimized
        }
    }

    true
}
