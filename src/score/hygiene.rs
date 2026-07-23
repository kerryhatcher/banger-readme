use crate::score::content::ReadmeStructure;
use std::path::Path;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct HygieneResult {
    pub score: f64,
    pub max: f64,
    pub checks: Vec<super::content::Check>,
}

/// Check if a companion file exists in the given directory.
fn file_exists(dir: &Path, names: &[&str]) -> bool {
    names.iter().any(|name| dir.join(name).exists())
}

pub fn analyze(structure: &ReadmeStructure, repo_dir: Option<&Path>) -> HygieneResult {
    use super::content::Check;
    let mut checks = Vec::new();
    let mut score: f64 = 0.0;
    let max = 20.0;

    // If no repo directory is provided, skip file-based checks
    let has_repo = repo_dir.is_some();
    let dir = repo_dir.unwrap_or_else(|| Path::new("."));

    // 1. LICENSE file (3 pts)
    let has_license = if has_repo {
        file_exists(
            dir,
            &[
                "LICENSE",
                "LICENSE.md",
                "LICENSE.txt",
                "LICENCE",
                "LICENCE.md",
            ],
        )
    } else {
        // Fall back to checking if README mentions license
        structure.raw.to_lowercase().contains("license")
    };
    checks.push(Check {
        name: "LICENSE file",
        passed: has_license,
        points: if has_license { 3.0 } else { 0.0 },
        max_points: 3.0,
        confidence: 1.0,
    });
    score += if has_license { 3.0 } else { 0.0 };

    // 2. CONTRIBUTING.md (3 pts)
    let has_contrib = if has_repo {
        file_exists(dir, &["CONTRIBUTING.md", "CONTRIBUTING.rst"])
    } else {
        structure.raw.to_lowercase().contains("contributing.md")
    };
    checks.push(Check {
        name: "CONTRIBUTING.md",
        passed: has_contrib,
        points: if has_contrib { 3.0 } else { 0.0 },
        max_points: 3.0,
        confidence: 1.0,
    });
    score += if has_contrib { 3.0 } else { 0.0 };

    // 3. CODE_OF_CONDUCT.md (2 pts)
    let has_coc = if has_repo {
        file_exists(dir, &["CODE_OF_CONDUCT.md", "CODE-OF-CONDUCT.md"])
    } else {
        structure.raw.to_lowercase().contains("code of conduct")
    };
    checks.push(Check {
        name: "CODE_OF_CONDUCT.md",
        passed: has_coc,
        points: if has_coc { 2.0 } else { 0.0 },
        max_points: 2.0,
        confidence: 1.0,
    });
    score += if has_coc { 2.0 } else { 0.0 };

    // 4. SECURITY.md (2 pts)
    let has_security = if has_repo {
        file_exists(dir, &["SECURITY.md"])
    } else {
        structure.raw.to_lowercase().contains("security.md")
    };
    checks.push(Check {
        name: "SECURITY.md",
        passed: has_security,
        points: if has_security { 2.0 } else { 0.0 },
        max_points: 2.0,
        confidence: 1.0,
    });
    score += if has_security { 2.0 } else { 0.0 };

    // 5. CHANGELOG.md (1 pt)
    let has_changelog = if has_repo {
        file_exists(dir, &["CHANGELOG.md", "CHANGES.md"])
    } else {
        structure.raw.to_lowercase().contains("changelog")
    };
    checks.push(Check {
        name: "CHANGELOG.md",
        passed: has_changelog,
        points: if has_changelog { 1.0 } else { 0.0 },
        max_points: 1.0,
        confidence: 1.0,
    });
    score += if has_changelog { 1.0 } else { 0.0 };

    // 6. CI badge present (3 pts)
    let has_ci = structure.raw.contains("actions/workflows")
        || structure.raw.contains("travis-ci")
        || structure.raw.contains("circleci")
        || structure.raw.to_lowercase().contains("ci");
    checks.push(Check {
        name: "CI badge present",
        passed: has_ci,
        points: if has_ci { 3.0 } else { 0.0 },
        max_points: 3.0,
        confidence: 1.0,
    });
    score += if has_ci { 3.0 } else { 0.0 };

    // 7. Test mention (2 pts)
    let has_tests = structure.raw.to_lowercase().contains("test")
        && (structure.raw.to_lowercase().contains("cargo test")
            || structure.raw.to_lowercase().contains("npm test")
            || structure.raw.to_lowercase().contains("pytest")
            || structure.raw.to_lowercase().contains("run test")
            || structure.raw.to_lowercase().contains("testing"));
    checks.push(Check {
        name: "Test mention",
        passed: has_tests,
        points: if has_tests { 2.0 } else { 0.0 },
        max_points: 2.0,
        confidence: 1.0,
    });
    score += if has_tests { 2.0 } else { 0.0 };

    // 8. Version badge (2 pts)
    let has_version_badge = structure.raw.contains("crates.io")
        || structure.raw.contains("npmjs.com")
        || structure.raw.contains("pypi.org")
        || structure.raw.contains("github.com") && structure.raw.contains("/v/")
        || structure.raw.contains("img.shields.io")
            && structure.raw.to_lowercase().contains("version");
    checks.push(Check {
        name: "Version badge",
        passed: has_version_badge,
        points: if has_version_badge { 2.0 } else { 0.0 },
        max_points: 2.0,
        confidence: 1.0,
    });
    score += if has_version_badge { 2.0 } else { 0.0 };

    // 9. Support channels (2 pts)
    let has_support = structure.raw.to_lowercase().contains("discord")
        || structure.raw.to_lowercase().contains("slack")
        || structure.raw.contains("matrix")
        || structure.raw.to_lowercase().contains("discussions")
        || structure.raw.to_lowercase().contains("irc");
    checks.push(Check {
        name: "Support channels",
        passed: has_support,
        points: if has_support { 2.0 } else { 0.0 },
        max_points: 2.0,
        confidence: 1.0,
    });
    score += if has_support { 2.0 } else { 0.0 };

    HygieneResult {
        score: score.min(max),
        max,
        checks,
    }
}
