mod config;
mod git;
mod install;
mod plugin;
mod score;

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;

/// rmb — Install plugins for the Pi coding harness and Claude Code.
///
/// Clones a git repository, detects the plugin type (Pi skill, Claude Code plugin, or both),
/// and installs it to the appropriate directories.
#[derive(Parser)]
#[command(name = "rmb", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install a plugin from a git repository, local directory, or the bundled banger-readme plugin.
    ///
    /// When no URL is given, fetches and installs the bundled banger-readme plugin from GitHub.
    Install {
        /// Git URL or local path of the plugin repository (optional — defaults to banger-readme self-install).
        url: Option<String>,

        /// Optional branch or tag to check out.
        #[arg(short, long)]
        branch: Option<String>,

        /// Only install for Pi coding harness.
        #[arg(long, conflicts_with = "claude_only")]
        pi_only: bool,

        /// Only install for Claude Code.
        #[arg(long, conflicts_with = "pi_only")]
        claude_only: bool,

        /// Force reinstall even if already installed.
        #[arg(short, long)]
        force: bool,
    },

    /// List installed plugins.
    List {
        /// Show only Pi skills.
        #[arg(long)]
        pi: bool,

        /// Show only Claude Code plugins.
        #[arg(long)]
        claude: bool,
    },

    /// Remove an installed plugin.
    Remove {
        /// Name of the plugin to remove.
        name: String,

        /// Also remove from Pi.
        #[arg(long)]
        pi: bool,

        /// Also remove from Claude Code.
        #[arg(long)]
        claude: bool,
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
    },
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Install {
            url,
            branch,
            pi_only,
            claude_only,
            force: _force,
        } => {
            match url {
                Some(ref u) => cmd_install(u, branch.as_deref(), pi_only, claude_only)?,
                None => cmd_self_install(pi_only, claude_only)?,
            }
        }
        Commands::List { pi, claude } => {
            cmd_list(pi, claude)?;
        }
        Commands::Remove { name, pi, claude } => {
            cmd_remove(&name, pi, claude)?;
        }
        Commands::Score {
            target,
            json,
            check,
            threshold,
            no_hygiene,
        } => {
            cmd_score(&target, json, check, threshold, no_hygiene)?;
        }
    }

    Ok(())
}

fn cmd_install(url: &str, branch: Option<&str>, pi_only: bool, claude_only: bool) -> Result<()> {
    use std::path::Path;

    // Check if the target is a local directory
    let local_path = Path::new(url);
    let (_temp, dir) = if local_path.is_dir() {
        println!(
            "{} Using local directory {}...",
            "→".cyan().bold(),
            url.dimmed()
        );
        // Create a dummy temp dir that won't be used — we need the TempDir for the lifetime
        let t = tempfile::TempDir::new()?;
        let d = local_path.to_path_buf();
        (Some(t), d)
    } else {
        println!("{} Cloning {}...", "→".cyan().bold(), url.dimmed());
        let t = git::clone_to_temp(url, branch)?;
        let d = t.path().to_path_buf();
        (Some(t), d)
    };

    let dir_ref: &Path = dir.as_ref();

    println!("{} Detecting plugin type...", "→".cyan().bold());

    let plugin = plugin::detect(dir_ref)?;

    println!(
        "{} Detected: {} — {}",
        "✓".green().bold(),
        plugin.name().green(),
        plugin.description().dimmed()
    );

    let paths = config::InstallPaths::detect();

    // Get git metadata
    let repo = git2::Repository::open(dir_ref).ok();
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
            install::install_from_dir(dir_ref, &plugin, &paths, Some(url), commit_sha.as_deref())?;
        }
    }

    println!("\n{} Done!", "🎉".green().bold());

    Ok(())
}

fn cmd_list(show_pi: bool, show_claude: bool) -> Result<()> {
    let paths = config::InstallPaths::detect();
    let show_all = !show_pi && !show_claude;

    if show_all || show_pi {
        println!("{}", "Pi Skills:".bold().underline());
        if paths.pi_skills.exists() {
            let mut found = false;
            if let Ok(entries) = std::fs::read_dir(&paths.pi_skills) {
                for entry in entries.flatten() {
                    let skill_md = entry.path().join("SKILL.md");
                    if skill_md.exists() {
                        found = true;
                        let name = entry.file_name().to_string_lossy().to_string();
                        // Try to read description from SKILL.md
                        if let Ok(content) = std::fs::read_to_string(&skill_md) {
                            if let Ok(fm) = extract_frontmatter_desc(&content) {
                                println!("  {} — {}", name.green(), fm.dimmed());
                                continue;
                            }
                        }
                        println!("  {}", name.green());
                    }
                }
            }
            if !found {
                println!("  (none)");
            }
        } else {
            println!("  (none)");
        }
        println!();
    }

    if show_all || show_claude {
        println!("{}", "Claude Code Plugins:".bold().underline());
        if paths.claude_installed_json.exists() {
            if let Ok(content) = std::fs::read_to_string(&paths.claude_installed_json) {
                if let Ok(registry) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some(plugins) = registry["plugins"].as_object() {
                        if plugins.is_empty() {
                            println!("  (none)");
                        }
                        for (key, entries) in plugins {
                            if let Some(arr) = entries.as_array() {
                                for entry in arr {
                                    let version = entry["version"].as_str().unwrap_or("unknown");
                                    let install_path =
                                        entry["installPath"].as_str().unwrap_or("unknown");
                                    println!(
                                        "  {} (v{}) — {}",
                                        key.green(),
                                        version,
                                        install_path.dimmed()
                                    );
                                }
                            }
                        }
                    } else {
                        println!("  (none)");
                    }
                }
            }
        } else {
            println!("  (none)");
        }
    }

    Ok(())
}

fn cmd_remove(name: &str, remove_pi: bool, remove_claude: bool) -> Result<()> {
    let paths = config::InstallPaths::detect();
    let remove_all = !remove_pi && !remove_claude;

    if remove_all || remove_pi {
        let pi_dir = paths.pi_skills.join(name);
        if pi_dir.exists() {
            println!(
                "{} Removing Pi skill '{}'...",
                "→".cyan().bold(),
                name.green()
            );
            std::fs::remove_dir_all(&pi_dir)?;
            println!("  {} Removed", "✓".green());
        } else if remove_pi {
            println!("{} Pi skill '{}' not found", "⚠".yellow(), name);
        }
    }

    if remove_all || remove_claude {
        // Remove from installed_plugins.json and cache
        if paths.claude_installed_json.exists() {
            let content = std::fs::read_to_string(&paths.claude_installed_json)?;
            let mut registry: serde_json::Value = serde_json::from_str(&content)?;

            let mut removed = false;
            if let Some(plugins) = registry["plugins"].as_object_mut() {
                // Find and remove matching plugin entries
                let keys_to_remove: Vec<String> = plugins
                    .iter()
                    .filter(|(key, _)| key.contains(name))
                    .map(|(key, _)| key.clone())
                    .collect();

                for key in &keys_to_remove {
                    if let Some(entries) = plugins.remove(key) {
                        removed = true;
                        // Try to remove the install directory
                        if let Some(arr) = entries.as_array() {
                            for entry in arr {
                                if let Some(install_path) = entry["installPath"].as_str() {
                                    let path = std::path::Path::new(install_path);
                                    if path.exists() {
                                        std::fs::remove_dir_all(path).ok();
                                        println!(
                                            "  {} Removed cache at {}",
                                            "✓".green(),
                                            install_path.dimmed()
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if removed {
                let content = serde_json::to_string_pretty(&registry)?;
                std::fs::write(&paths.claude_installed_json, content)?;
                println!(
                    "{} Removed Claude Code plugin '{}'",
                    "✓".green().bold(),
                    name.green()
                );
            } else if remove_claude {
                println!("{} Claude Code plugin '{}' not found", "⚠".yellow(), name);
            }
        }
    }

    Ok(())
}

/// Extract just the description from SKILL.md frontmatter for listing.
fn extract_frontmatter_desc(content: &str) -> Result<String> {
    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() || lines[0].trim() != "---" {
        anyhow::bail!("No frontmatter");
    }
    for line in lines.iter().skip(1) {
        if line.trim() == "---" {
            break;
        }
        if let Some(desc) = line.strip_prefix("description:") {
            let desc = desc.trim().trim_matches('"').to_string();
            if !desc.is_empty() {
                return Ok(desc);
            }
        }
    }
    anyhow::bail!("No description found")
}

const SELF_INSTALL_REPO: &str = "https://github.com/kerryhatcher/banger-readme";

fn cmd_self_install(pi_only: bool, claude_only: bool) -> Result<()> {
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

fn cmd_score(target: &str, json: bool, check: bool, threshold: u32, no_hygiene: bool) -> Result<()> {
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
                    let repo_dir = if no_hygiene {
                        None
                    } else {
                        Some(path)
                    };
                    (body, repo_dir)
                }
                None => {
                    anyhow::bail!("No README found in directory: {}", path.display());
                }
            }
        } else if path.is_file() {
            let body = std::fs::read_to_string(path)?;
            let repo_dir = if no_hygiene {
                None
            } else {
                path.parent()
            };
            (body, repo_dir)
        } else {
            anyhow::bail!("Target not found: {target}");
        }
    };

    let report = score::engine::score_readme(&raw, repo_dir.map(|p| p as &Path));

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
