mod config;
mod git;
mod install;
mod plugin;
mod score;

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;

/// rmb — Create and rate great READMEs.
///
/// Scores READMEs (and their associated project files) against research-backed
/// best practices, and bootstraps the rmb skill into your coding agents.
#[derive(Parser)]
#[command(name = "rmb", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Bootstrap the rmb skill into your coding agents (Pi and Claude Code).
    ///
    /// Fetches the bundled banger-readme plugin and installs it so your agents
    /// can use rmb's README skill. Installs to both agents by default.
    Install {
        /// Only install for the Pi coding harness.
        #[arg(long, conflicts_with = "claude_only")]
        pi_only: bool,

        /// Only install for Claude Code.
        #[arg(long, conflicts_with = "pi_only")]
        claude_only: bool,
    },

    /// Score a README against best-practice criteria.
    Score {
        /// Path to README.md, URL, or local directory containing a README.
        target: String,

        /// Output machine-readable JSON.
        #[arg(long)]
        json: bool,

        /// Exit with non-zero code if score is below threshold.
        #[arg(long)]
        check: bool,

        /// Minimum acceptable score (default: 70).
        #[arg(long, default_value = "70")]
        threshold: u32,

        /// Skip project hygiene checks (for remote URLs without repo access).
        #[arg(long)]
        no_hygiene: bool,

        /// Enable deep analysis (image similarity comparisons).
        #[arg(long)]
        deep: bool,

        /// Enable link checking (broken link detection).
        #[arg(long)]
        links: bool,

        /// Enable multi-language README scoring.
        #[arg(long)]
        multi_lang: bool,
    },
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Install {
            pi_only,
            claude_only,
        } => {
            cmd_install(pi_only, claude_only)?;
        }
        Commands::Score {
            target,
            json,
            check,
            threshold,
            no_hygiene,
            deep,
            links,
            multi_lang,
        } => {
            cmd_score(&target, json, check, threshold, no_hygiene, deep, links, multi_lang)?;
        }
    }

    Ok(())
}

const SELF_INSTALL_REPO: &str = "https://github.com/kerryhatcher/banger-readme";

/// Bootstrap the rmb skill into Pi and/or Claude Code by fetching the bundled
/// banger-readme plugin and installing it.
fn cmd_install(pi_only: bool, claude_only: bool) -> Result<()> {
    println!(
        "{} Fetching banger-readme plugin from GitHub...",
        "→".cyan().bold()
    );

    // Clone the repo to a temp directory
    let temp = git::clone_to_temp(SELF_INSTALL_REPO, None)?;
    let plugin_dir = temp.path().join("plugin");

    if !plugin_dir.exists() {
        anyhow::bail!("Plugin directory not found in cloned repo");
    }

    println!("{} Detecting plugin type...", "→".cyan().bold());

    let plugin = plugin::detect(&plugin_dir)?;

    println!(
        "{} Detected: {} — {}",
        "✓".green().bold(),
        plugin.name().green(),
        plugin.description().dimmed()
    );

    let paths = config::InstallPaths::detect();

    // Get git metadata
    let repo = git2::Repository::open(temp.path()).ok();
    let commit_sha = repo.as_ref().and_then(|r| git::head_sha(r).ok());

    match &plugin {
        plugin::PluginType::PiSkill(_) if claude_only => {
            println!(
                "{} Skipping Pi skill (--claude-only specified)",
                "⚠".yellow()
            );
        }
        plugin::PluginType::ClaudePlugin(_) if pi_only => {
            println!(
                "{} Skipping Claude Code plugin (--pi-only specified)",
                "⚠".yellow()
            );
        }
        _ => {
            install::install_from_dir(
                &plugin_dir,
                &plugin,
                &paths,
                Some(SELF_INSTALL_REPO),
                commit_sha.as_deref(),
            )?;
        }
    }

    println!("\n{} Done!", "🎉".green().bold());

    Ok(())
}

fn cmd_score(
    target: &str,
    json: bool,
    check: bool,
    threshold: u32,
    no_hygiene: bool,
    deep: bool,
    links: bool,
    multi_lang: bool,
) -> Result<()> {
    use std::path::Path;

    // Resolve target: URL, file, or directory
    let (raw, repo_dir) = if target.starts_with("http://") || target.starts_with("https://") {
        println!("{} Fetching {}...", "→".cyan().bold(), target.dimmed());
        let (body, _url) = score::engine::fetch_readme(target)?;
        (body, None)
    } else {
        let path = Path::new(target);
        if path.is_dir() {
            match score::engine::find_readme(path) {
                Some(readme_path) => {
                    println!(
                        "{} Found {}...",
                        "→".cyan().bold(),
                        readme_path.display().to_string().dimmed()
                    );
                    let body = std::fs::read_to_string(&readme_path)?;
                    let repo_dir = if no_hygiene { None } else { Some(path) };
                    (body, repo_dir)
                }
                None => {
                    anyhow::bail!("No README found in directory: {}", path.display());
                }
            }
        } else if path.is_file() {
            let body = std::fs::read_to_string(path)?;
            let repo_dir = if no_hygiene { None } else { path.parent() };
            (body, repo_dir)
        } else {
            anyhow::bail!("Target not found: {target}");
        }
    };

    let report = score::engine::score_readme(&raw, repo_dir.map(|p| p as &Path), deep, links, multi_lang);

    if json {
        report.print_json();
    } else {
        report.print_terminal();
    }

    if check && (report.score as u32) < threshold {
        std::process::exit(1);
    }

    Ok(())
}
