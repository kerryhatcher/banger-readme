use crate::score::content::ReadmeStructure;
use serde::Serialize;
use std::path::Path;

#[derive(Debug, Clone, Serialize)]
pub struct ImageSimilarityResult {
    pub score: f64,
    pub max: f64,
    pub checks: Vec<super::content::Check>,
}

pub fn analyze(
    structure: &ReadmeStructure,
    repo_dir: Option<&Path>,
) -> ImageSimilarityResult {
    use super::content::Check;
    let mut checks = Vec::new();
    let mut score: f64 = 0.0;
    let max = 3.0;

    let dir = match repo_dir {
        Some(d) => d,
        None => {
            return ImageSimilarityResult {
                score: 0.0,
                max,
                checks,
            };
        }
    };

    // Find dark/light mode image pairs
    let (dark_path, light_path) = find_mode_pair(structure, dir);

    // ── 1. Dark/Light Mode Distinctiveness (2 pts) ─────────────────────────
    if let (Some(dark), Some(light)) = (&dark_path, &light_path) {
        let distinct = check_mode_distinctiveness(dark, light);

        checks.push(Check {
            name: "Dark/light mode distinctiveness",
            passed: distinct,
            points: if distinct { 2.0 } else { 0.0 },
            max_points: 2.0,
            confidence: 1.0,
        });
        score += if distinct { 2.0 } else { 0.0 };
    }

    // ── 2. Placeholder Detection (1 pt penalty, scored as bonus for clean) ──
    let image_paths = super::image_heuristics::extract_local_image_paths(&structure.raw);
    let all_real = image_paths.iter().all(|p| {
        let full = dir.join(p);
        !is_placeholder(&full)
    });

    checks.push(Check {
        name: "No placeholder images",
        passed: all_real,
        points: if all_real { 1.0 } else { 0.0 },
        max_points: 1.0,
        confidence: 1.0,
    });
    score += if all_real { 1.0 } else { 0.0 };

    ImageSimilarityResult {
        score: score.min(max),
        max,
        checks,
    }
}

/// Find dark/light mode image pairs from the README.
/// Returns (dark_path, light_path) if both exist.
fn find_mode_pair(structure: &ReadmeStructure, dir: &Path) -> (Option<std::path::PathBuf>, Option<std::path::PathBuf>) {
    let raw = &structure.raw;

    // Look for <picture> with dark/light source media
    let dark_src = extract_picture_source(raw, "dark");
    let light_src = extract_picture_source(raw, "light");

    let dark_path = dark_src.and_then(|p| {
        let full = dir.join(&p);
        if full.exists() { Some(full) } else { None }
    });
    let light_path = light_src.and_then(|p| {
        let full = dir.join(&p);
        if full.exists() { Some(full) } else { None }
    });

    if dark_path.is_some() && light_path.is_some() {
        return (dark_path, light_path);
    }

    // Fall back: look for files with "dark" and "light" in their names
    let image_paths = super::image_heuristics::extract_local_image_paths(raw);
    let dark = image_paths.iter().find(|p| {
        let lower = p.to_lowercase();
        lower.contains("dark") || lower.contains("_d.") || lower.contains("-d.")
    }).and_then(|p| {
        let full = dir.join(p);
        if full.exists() { Some(full) } else { None }
    });
    let light = image_paths.iter().find(|p| {
        let lower = p.to_lowercase();
        lower.contains("light") || lower.contains("_l.") || lower.contains("-l.")
    }).and_then(|p| {
        let full = dir.join(p);
        if full.exists() { Some(full) } else { None }
    });

    (dark, light)
}

/// Extract the srcset from a <source> tag with a specific media query.
fn extract_picture_source(raw: &str, mode: &str) -> Option<String> {
    let pattern = format!(
        r#"<source[^>]+media\s*=\s*["'][^"']*prefers-color-scheme:\s*{}[^"']*["'][^>]+srcset\s*=\s*["']([^"']+)["']"#,
        mode
    );
    let re = regex::Regex::new(&pattern).ok()?;
    re.captures(raw)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string())
}

/// Check if dark and light mode images are actually different using SSIM.
fn check_mode_distinctiveness(dark: &Path, light: &Path) -> bool {
    // For SVG files, compare the text content directly
    let dark_ext = dark.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
    let light_ext = light.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();

    if dark_ext == "svg" && light_ext == "svg" {
        return check_svg_distinctiveness(dark, light);
    }

    // For raster images, use SSIM
    check_raster_distinctiveness(dark, light)
}

/// Compare two SVG files by checking if their content is substantially different.
fn check_svg_distinctiveness(dark: &Path, light: &Path) -> bool {
    let dark_content = match std::fs::read_to_string(dark) {
        Ok(c) => c,
        Err(_) => return true, // Can't read, assume they're different
    };
    let light_content = match std::fs::read_to_string(light) {
        Ok(c) => c,
        Err(_) => return true,
    };

    // Extract color-related attributes and compare
    let color_re = regex::Regex::new(r#"(?:fill|stroke|stop-color|color)\s*=\s*["']([^"']+)["']"#).unwrap();

    let dark_colors: Vec<String> = color_re
        .captures_iter(&dark_content)
        .map(|c| c.get(1).unwrap().as_str().to_lowercase())
        .collect();
    let light_colors: Vec<String> = color_re
        .captures_iter(&light_content)
        .map(|c| c.get(1).unwrap().as_str().to_lowercase())
        .collect();

    // If color attributes differ, the images are distinct
    if dark_colors != light_colors {
        return true;
    }

    // If the files are byte-identical, they're not distinct
    dark_content != light_content
}

/// Compare two raster images to check if they're substantially different.
fn check_raster_distinctiveness(dark: &Path, light: &Path) -> bool {
    let dark_img = match image::open(dark) {
        Ok(img) => img,
        Err(_) => return true,
    };
    let light_img = match image::open(light) {
        Ok(img) => img,
        Err(_) => return true,
    };

    let (dw, dh) = (dark_img.width(), dark_img.height());
    let (lw, lh) = (light_img.width(), light_img.height());

    if dw != lw || dh != lh {
        return true; // Different dimensions = clearly distinct
    }

    let dark_rgba = dark_img.to_rgba8();
    let light_rgba = light_img.to_rgba8();

    let dark_pixels = dark_rgba.as_raw();
    let light_pixels = light_rgba.as_raw();

    // Count pixels that differ by more than a small threshold
    let total_pixels = (dw * dh) as usize;
    let mut different_pixels = 0usize;

    for i in (0..dark_pixels.len()).step_by(4) {
        let dr = dark_pixels[i] as i32;
        let dg = dark_pixels[i + 1] as i32;
        let db = dark_pixels[i + 2] as i32;
        let lr = light_pixels[i] as i32;
        let lg = light_pixels[i + 1] as i32;
        let lb = light_pixels[i + 2] as i32;

        // Per-channel difference > 10 is "visibly different"
        if (dr - lr).abs() > 10 || (dg - lg).abs() > 10 || (db - lb).abs() > 10 {
            different_pixels += 1;
        }
    }

    // If >5% of pixels differ, images are distinct
    let diff_ratio = different_pixels as f64 / total_pixels as f64;
    diff_ratio > 0.05
}

/// Check if an image is a placeholder (solid color, placeholder service URL, etc.).
fn is_placeholder(path: &Path) -> bool {
    // Check filename for placeholder indicators
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_lowercase();
    if name.contains("placeholder") || name.contains("dummy") || name.contains("temp") {
        return true;
    }

    // Check if it's a solid-color image
    let img = match image::open(path) {
        Ok(img) => img,
        Err(_) => return false,
    };

    let rgba = img.to_rgba8();
    let pixels = rgba.as_raw();

    if pixels.len() < 4 {
        return false;
    }

    // Sample the first pixel
    let first = &pixels[..4];
    // Check if >95% of pixels match the first pixel
    let matching = pixels
        .chunks(4)
        .filter(|p| *p == first)
        .count();
    let total = pixels.len() / 4;

    (matching as f64 / total as f64) > 0.95
}
