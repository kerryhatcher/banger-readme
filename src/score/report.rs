use crate::score::antipatterns::AntipatternResult;
use crate::score::content::ContentResult;
use crate::score::funnel::FunnelResult;
use crate::score::hygiene::HygieneResult;
use crate::score::image_heuristics::ImageHeuristicsResult;
#[cfg(feature = "deep")]
use crate::score::image_similarity::ImageSimilarityResult;
#[cfg(feature = "links")]
use crate::score::link_check::LinkCheckResult;
#[cfg(feature = "multi-lang")]
use crate::score::multi_lang::MultiLangResult;
use crate::score::text_quality::TextQualityResult;
use crate::score::visuals::VisualResult;
use colored::*;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ScoredReport {
    pub score: f64,
    pub grade: String,
    pub label: String,
    pub content: ContentResult,
    pub visuals: VisualResult,
    pub hygiene: HygieneResult,
    pub funnel: FunnelResult,
    pub antipatterns: AntipatternResult,
    pub text_quality: TextQualityResult,
    pub image_heuristics: ImageHeuristicsResult,
    #[cfg(feature = "deep")]
    pub image_similarity: Option<ImageSimilarityResult>,
    #[cfg(not(feature = "deep"))]
    pub image_similarity: Option<()>,
    #[cfg(feature = "links")]
    pub link_check: Option<LinkCheckResult>,
    #[cfg(not(feature = "links"))]
    pub link_check: Option<()>,
    #[cfg(feature = "multi-lang")]
    pub multi_lang: Option<MultiLangResult>,
    #[cfg(not(feature = "multi-lang"))]
    pub multi_lang: Option<()>,
}

impl ScoredReport {
    /// Print a human-readable terminal report.
    pub fn print_terminal(&self) {
        let percentile = benchmark_percentile(self.score);
        // Header
        println!();
        println!("╔══════════════════════════════════════════════════╗");
        println!(
            "║         README Score: {:<3}/100  [{}]           ║",
            self.score as u32,
            self.grade.bold()
        );
        println!("║         {:^38} ║", format!("\"{}\"", self.label).dimmed());
        println!(
            "║         Top {:.0}% of open source READMEs       ║",
            (100.0 - percentile)
        );
        println!("╚══════════════════════════════════════════════════╝");
        println!();

        // Content
        self.print_category(
            "Content Completeness",
            self.content.score,
            self.content.max,
            &self.content.checks,
        );

        // Visuals
        self.print_category(
            "Visual Design",
            self.visuals.score,
            self.visuals.max,
            &self.visuals.checks,
        );

        // Hygiene
        self.print_category(
            "Project Hygiene",
            self.hygiene.score,
            self.hygiene.max,
            &self.hygiene.checks,
        );

        // Funnel
        self.print_category(
            "Cognitive Funneling",
            self.funnel.score,
            self.funnel.max,
            &self.funnel.checks,
        );

        // Text Quality
        self.print_category(
            "Text Quality",
            self.text_quality.score,
            self.text_quality.max,
            &self.text_quality.checks,
        );

        // Image Heuristics
        self.print_category(
            "Image Heuristics",
            self.image_heuristics.score,
            self.image_heuristics.max,
            &self.image_heuristics.checks,
        );

        // Image Similarity (only when --deep is used)
        if let Some(ref img_sim) = self.image_similarity {
            #[cfg(feature = "deep")]
            {
                self.print_category(
                    "Image Similarity",
                    img_sim.score,
                    img_sim.max,
                    &img_sim.checks,
                );
            }
            #[cfg(not(feature = "deep"))]
            let _ = img_sim;
        }

        // Link Check (only when --links is used)
        if let Some(ref lc) = self.link_check {
            #[cfg(feature = "links")]
            {
                self.print_category(
                    "Link Check",
                    lc.score,
                    lc.max,
                    &lc.checks,
                );
                // Print link details
                if !lc.links.is_empty() {
                    println!("│ Link details:");
                    for link in &lc.links {
                        let icon = match link.status.as_str() {
                            "ok" => "✅",
                            "broken" => "❌",
                            "redirect" => "🔀",
                            "timeout" => "⏱️",
                            _ => "⚠️",
                        };
                        println!("│   {} {} — {}", icon, link.url.dimmed(), link.detail);
                    }
                    println!("└──────────────────────────────────────────┘");
                    println!();
                }
            }
            #[cfg(not(feature = "links"))]
            let _ = lc;
        }

        // Multi-Language (only when --multi-lang is used)
        if let Some(ref ml) = self.multi_lang {
            #[cfg(feature = "multi-lang")]
            {
                self.print_category(
                    "Multi-Language",
                    ml.score,
                    ml.max,
                    &ml.checks,
                );
                if !ml.translations.is_empty() {
                    println!("│ Translations found:");
                    for t in &ml.translations {
                        let grade_str = t.grade.map_or("N/A".into(), |g| format!("grade {:.1}", g));
                        let match_str = if t.sections_match { "✅" } else { "⚠️" };
                        println!(
                            "│   {} {} ({}) — {} sections {}",
                            match_str,
                            t.path.dimmed(),
                            t.language,
                            grade_str,
                            if t.sections_match {
                                "match".dimmed()
                            } else {
                                "mismatch".red()
                            }
                        );
                    }
                    println!("└──────────────────────────────────────────┘");
                    println!();
                }
            }
            #[cfg(not(feature = "multi-lang"))]
            let _ = ml;
        }

        // Anti-patterns
        if !self.antipatterns.findings.is_empty() {
            let has_issues = self.antipatterns.findings.iter().any(|f| f.detected);
            if has_issues {
                println!(
                    "┌─ Anti-Patterns ─────────────── {:>3} ─────┐",
                    format!("−{}", self.antipatterns.penalty as u32).red()
                );
                for finding in &self.antipatterns.findings {
                    if finding.detected {
                        println!("│ {} {}", "⚠".yellow(), finding.name.red());
                        if !finding.detail.is_empty() {
                            println!("│   {}", finding.detail.dimmed());
                        }
                    }
                }
                println!("└──────────────────────────────────────────┘");
                println!();
            }
        }

        // Recommendations
        println!("{}", "─".repeat(44));
        println!("{} Top Recommendations (by impact):", "🎯".bold());

        let recs = self.generate_recommendations();
        for (i, rec) in recs.iter().enumerate() {
            println!(
                "  {}. {} — +{} pts, {} impact",
                i + 1,
                rec.name.green(),
                rec.points,
                rec.impact
            );
            println!("     {}", rec.message.dimmed());
        }
        println!("{}", "─".repeat(44));
        println!();
    }

    fn print_category(
        &self,
        name: &str,
        score: f64,
        max: f64,
        checks: &[crate::score::content::Check],
    ) {
        println!("┌─ {} ────── {:.0}/{:.0} ─────┐", name, score, max);
        for check in checks {
            if check.passed && check.confidence >= 1.0 {
                println!("│ {} {}", "✅".green(), check.name);
            } else if check.passed && check.confidence > 0.0 {
                println!(
                    "│ {} {} (partial, {:.0}% credit)",
                    "⚠".yellow(),
                    check.name,
                    check.confidence * 100.0
                );
            } else {
                println!(
                    "│ {} {} {} +{} pts",
                    "❌".red(),
                    check.name,
                    "←".dimmed(),
                    check.max_points as u32
                );
            }
        }
        println!("└──────────────────────────────────────────┘");
        println!();
    }

    /// Print JSON output.
    pub fn print_json(&self) {
        let json = serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".into());
        println!("{json}");
    }

    /// Generate actionable recommendations sorted by impact.
    fn generate_recommendations(&self) -> Vec<Recommendation> {
        let mut recs = Vec::new();

        // Collect failed checks from all categories
        for check in &self.content.checks {
            if !check.passed {
                recs.push(Recommendation {
                    name: check.name.to_string(),
                    points: check.max_points,
                    impact: if check.max_points >= 3.0 {
                        "high"
                    } else {
                        "medium"
                    },
                    message: recommendation_message(check.name),
                });
            }
        }

        for check in &self.visuals.checks {
            if !check.passed {
                recs.push(Recommendation {
                    name: check.name.to_string(),
                    points: check.max_points,
                    impact: if check.max_points >= 3.0 {
                        "high"
                    } else {
                        "medium"
                    },
                    message: recommendation_message(check.name),
                });
            }
        }

        for check in &self.hygiene.checks {
            if !check.passed {
                recs.push(Recommendation {
                    name: check.name.to_string(),
                    points: check.max_points,
                    impact: if check.max_points >= 3.0 {
                        "high"
                    } else {
                        "medium"
                    },
                    message: recommendation_message(check.name),
                });
            }
        }

        for check in &self.funnel.checks {
            if !check.passed {
                recs.push(Recommendation {
                    name: check.name.to_string(),
                    points: check.max_points,
                    impact: "medium",
                    message: recommendation_message(check.name),
                });
            }
        }

        for check in &self.text_quality.checks {
            if !check.passed {
                recs.push(Recommendation {
                    name: check.name.to_string(),
                    points: check.max_points,
                    impact: if check.max_points >= 3.0 {
                        "high"
                    } else {
                        "medium"
                    },
                    message: recommendation_message(check.name),
                });
            }
        }

        for check in &self.image_heuristics.checks {
            if !check.passed {
                recs.push(Recommendation {
                    name: check.name.to_string(),
                    points: check.max_points,
                    impact: "medium",
                    message: recommendation_message(check.name),
                });
            }
        }

        #[cfg(feature = "deep")]
        if let Some(ref img_sim) = self.image_similarity {
            for check in &img_sim.checks {
                if !check.passed {
                    recs.push(Recommendation {
                        name: check.name.to_string(),
                        points: check.max_points,
                        impact: "medium",
                        message: recommendation_message(check.name),
                    });
                }
            }
        }

        #[cfg(feature = "links")]
        if let Some(ref lc) = self.link_check {
            for check in &lc.checks {
                if !check.passed {
                    recs.push(Recommendation {
                        name: check.name.to_string(),
                        points: check.max_points,
                        impact: if check.max_points >= 3.0 {
                            "high"
                        } else {
                            "medium"
                        },
                        message: recommendation_message(check.name),
                    });
                }
            }
        }

        #[cfg(feature = "multi-lang")]
        if let Some(ref ml) = self.multi_lang {
            for check in &ml.checks {
                if !check.passed {
                    recs.push(Recommendation {
                        name: check.name.to_string(),
                        points: check.max_points,
                        impact: "medium",
                        message: recommendation_message(check.name),
                    });
                }
            }
        }

        // Sort by points descending (highest impact first)
        recs.sort_by(|a, b| {
            b.points
                .partial_cmp(&a.points)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        recs.truncate(5);
        recs
    }
}

struct Recommendation {
    name: String,
    points: f64,
    impact: &'static str,
    message: String,
}

fn recommendation_message(name: &str) -> String {
    match name {
        "Logo/Banner" => "Add a project logo or banner image at the top of your README".into(),
        "Badges" => "Add CI, version, and license badges for instant trust signals".into(),
        "One-liner" => "Add a clear one-sentence description of what your project does".into(),
        "Demo GIF/Screenshot" => {
            "Add an animated GIF or screenshot showing your project in action — highest ROI change"
                .into()
        }
        "Feature Highlights" => {
            "Add a bulleted list of key features that set your project apart".into()
        }
        "Quick Start" => {
            "Add a quick start section with copy-paste commands to get running in 60 seconds".into()
        }
        "Table of Contents" => {
            "Add a table of contents for easy navigation (for READMEs longer than 2 scrolls)".into()
        }
        "The \"Why\"" => {
            "Add a section explaining the problem your project solves and why it exists".into()
        }
        "Installation" => {
            "Add clear installation instructions covering all supported platforms".into()
        }
        "Usage/Examples" => "Add code examples showing real-world usage of your project".into(),
        "API Reference" => "Add an API reference section or link to full documentation".into(),
        "Contributing" => "Add a contributing section or link to CONTRIBUTING.md".into(),
        "License" => "Add a license section or ensure a LICENSE file exists in the repo".into(),
        "Acknowledgements" => {
            "Add an acknowledgements section crediting contributors and inspirations".into()
        }
        "Has logo image" => "Add a project logo image (under 100px height) at the top".into(),
        "Badge organization" => {
            "Organize badges in groups of ≤5 per line, separated by blank lines".into()
        }
        "Header hierarchy (no h4+)" => {
            "Avoid h4+ headers — restructure content to use at most 3 levels".into()
        }
        "Emoji in headers" => {
            "Add emoji to section headers for visual navigation and personality".into()
        }
        "Image/GIF count (≥2)" => {
            "Add more screenshots or GIFs to visually demonstrate your project".into()
        }
        "Collapsible sections" => {
            "Use <details> tags for long content to keep the README scannable".into()
        }
        "Code syntax highlighting" => {
            "Add language identifiers to code blocks for syntax highlighting".into()
        }
        "Link density" => "Add more links to related projects, docs, and references".into(),
        "Dark/light mode support" => {
            "Add dark/light mode image variants using #gh-dark-mode-only and #gh-light-mode-only"
                .into()
        }
        "Table usage" => {
            "Use markdown tables for structured data like platform support or comparisons".into()
        }
        "Line length discipline (≤80)" => {
            "Keep non-code lines under 80 characters for better readability".into()
        }
        "LICENSE file" => "Add a LICENSE file to the repository root".into(),
        "CONTRIBUTING.md" => {
            "Add a CONTRIBUTING.md with development setup and PR guidelines".into()
        }
        "CODE_OF_CONDUCT.md" => "Add a CODE_OF_CONDUCT.md (use Contributor Covenant)".into(),
        "SECURITY.md" => "Add a SECURITY.md with vulnerability reporting instructions".into(),
        "CHANGELOG.md" => "Add a CHANGELOG.md or use GitHub Releases for version history".into(),
        "CI badge present" => "Add a CI status badge to show build/test health at a glance".into(),
        "Test mention" => "Document how to run tests in the README or CONTRIBUTING.md".into(),
        "Version badge" => "Add a version badge (npm, crates.io, PyPI, or GitHub release)".into(),
        "Support channels" => {
            "Document where users can get help (Discord, Slack, Discussions, etc.)".into()
        }
        "One-liner before installation" => {
            "Move your project description above the installation section".into()
        }
        "Demo before API docs" => "Show a demo or screenshot before diving into API details".into(),
        "Features before contributing" => "List features before the contributing section".into(),
        "Quick start before detailed install" => {
            "Add a quick start section before detailed installation instructions".into()
        }
        "Install before usage" => "Place installation instructions before usage examples".into(),
        "License at bottom" => "Move the license section to the end of the README".into(),
        "Readability grade (7–10)" => {
            "Simplify language — target a 7th–10th grade reading level for broad accessibility"
                .into()
        }
        "Cliché-free copy" => {
            "Replace clichés like 'think outside the box' with specific, original language".into()
        }
        "Active voice (≤20% passive)" => {
            "Rewrite passive sentences in active voice for clearer, more direct communication"
                .into()
        }
        "Positive/welcoming tone" => {
            "Use more positive, welcoming language to make the project feel approachable".into()
        }
        "Sentence variety" => {
            "Vary sentence length and structure — mix short punchy lines with longer explanations"
                .into()
        }
        "Logo dimensions (≥128×128)" => {
            "Use a logo at least 128×128px for sharp display on HiDPI/Retina screens".into()
        }
        "Banner width (≥1200px)" => {
            "Use a header banner at least 1200px wide to fill the GitHub README container"
                .into()
        }
        "Image format optimized" => {
            "Optimize images — use SVG for logos, PNG for screenshots, keep files under 1MB"
                .into()
        }
        "Dark/light mode distinctiveness" => {
            "Your dark and light mode images look identical — adapt colors for each mode"
                .into()
        }
        "No placeholder images" => {
            "Replace placeholder images with real screenshots or project graphics".into()
        }
        "No broken links" => {
            "Fix broken links — check that all URLs return 2xx status codes".into()
        }
        "No excessive redirects" => {
            "Update redirected URLs to point directly to their final destinations".into()
        }
        "Valid anchor fragments" => {
            "Fix broken anchor links — headings may have been renamed or removed".into()
        }
        "Translation readability" => {
            "Ensure translated READMEs maintain accessible reading levels for their audience"
                .into()
        }
        "Translation completeness" => {
            "Keep translations in sync — update non-English READMEs when sections change"
                .into()
        }
        _ => "Review and improve this aspect of your README".into(),
    }
}

/// Benchmark percentiles derived from scoring known open source READMEs.
/// These are approximate, based on our research sample of 30+ projects.
fn benchmark_percentile(score: f64) -> f64 {
    // Benchmark data points (score, cumulative_percentile)
    let benchmarks: [(f64, f64); 10] = [
        (85.0, 95.0), // meilisearch-tier = top 5%
        (75.0, 85.0), // dioxus-tier = top 15%
        (68.0, 70.0), // above average
        (62.0, 55.0), // average
        (55.0, 40.0), // below average
        (50.0, 30.0), // rustdesk/helix tier
        (45.0, 20.0),
        (38.0, 10.0),
        (30.0, 5.0),
        (20.0, 2.0),
    ];

    for (bench_score, percentile) in &benchmarks {
        if score >= *bench_score {
            return *percentile;
        }
    }
    1.0 // bottom 1%
}
