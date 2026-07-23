use regex::Regex;
use std::sync::LazyLock;

// ---------------------------------------------------------------------------
// Regex patterns — compiled once at first use
// ---------------------------------------------------------------------------

pub static RE_IMAGE_TAG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"<img\s[^>]*?src\s*=\s*["'][^"']+["'][^>]*?>"#).unwrap());

pub static RE_MD_IMAGE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"!\[([^\]]*)\]\(([^)]+)\)").unwrap());

pub static RE_BADGE_URL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"https?://img\.shields\.io/|https?://camo\.githubusercontent\.com/|https?://badge\.fury\.io/|https?://github\.com/[^/]+/[^/]+/actions/workflows/")
        .unwrap()
});

pub static RE_GIF: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\.gif(\b|[?#])").unwrap());

pub static RE_TOC_LINK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[([^\]]+)\]\(#([^)]+)\)").unwrap());

pub static RE_EMOJI: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"[\x{1F300}-\x{1F9FF}\x{1F600}-\x{1F64F}\x{1F680}-\x{1F6FF}\x{2600}-\x{26FF}\x{2700}-\x{27BF}\x{1F900}-\x{1F9FF}\x{1FA00}-\x{1FA6F}\x{1FA70}-\x{1FAFF}]",
    )
    .unwrap()
});

pub static RE_PLACEHOLDER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)coming\s*soon|TODO|TBD|work\s+in\s+progress|under\s+construction")
        .unwrap()
});

pub static RE_DETAILS_TAG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<details[^>]*>").unwrap());

pub static RE_DARK_MODE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"#gh-dark-mode-only").unwrap());

pub static RE_LIGHT_MODE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"#gh-light-mode-only").unwrap());

// ---------------------------------------------------------------------------
// Section heading patterns (case-insensitive)
// ---------------------------------------------------------------------------

pub fn heading_matches(heading: &str, patterns: &[&str]) -> bool {
    let lower = heading.to_lowercase();
    patterns.iter().any(|p| lower.contains(p))
}

pub const FEATURE_PATTERNS: &[&str] = &["feature"];
pub const QUICKSTART_PATTERNS: &[&str] = &["quick", "getting started"];
pub const TOC_PATTERNS: &[&str] = &["table of contents", "contents", "toc"];
pub const WHY_PATTERNS: &[&str] = &["why", "motivation", "background", "philosophy", "problem"];
pub const INSTALL_PATTERNS: &[&str] = &["install", "setup", "build", "download"];
pub const USAGE_PATTERNS: &[&str] = &["usage", "example", "how to"];
pub const API_PATTERNS: &[&str] = &["api", "reference", "documentation"];
pub const CONTRIBUTING_PATTERNS: &[&str] = &["contribut"];
pub const LICENSE_PATTERNS: &[&str] = &["license", "licence"];
pub const ACKNOWLEDGE_PATTERNS: &[&str] = &["acknowledge", "credit", "thanks", "contributor"];
pub const DEMO_PATTERNS: &[&str] = &["demo", "screenshot", "preview"];

// ---------------------------------------------------------------------------
// Weights
// ---------------------------------------------------------------------------


// ---------------------------------------------------------------------------
// Grade scale
// ---------------------------------------------------------------------------

pub fn grade(score: f64) -> (&'static str, &'static str) {
    match score {
        s if s >= 90.0 => ("A+", "Exceptional — ranks with the best in Awesome README"),
        s if s >= 80.0 => ("A", "Excellent — well above average"),
        s if s >= 70.0 => ("B", "Good — solid, with room for improvement"),
        s if s >= 60.0 => ("C", "Adequate — covers basics, misses opportunities"),
        s if s >= 40.0 => ("D", "Underdeveloped — significant gaps"),
        _ => ("F", "Poor — needs fundamental work"),
    }
}
