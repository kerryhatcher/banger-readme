use anyhow::{Context, Result};
use colored::*;
use std::fs;
use std::path::Path;

use crate::config::InstallPaths;
use crate::git;
use crate::plugin::PluginType;

/// Install the bundled rmb plugin from a local directory to Pi and/or Claude Code.
pub fn install_from_dir(
    dir: &Path,
    plugin: &PluginType,
    paths: &InstallPaths,
    git_url: Option<&str>,
    commit_sha: Option<&str>,
) -> Result<()> {
    match plugin {
        PluginType::PiSkill(meta) => {
            install_pi_skill(dir, meta, paths)?;
        }
        PluginType::ClaudePlugin(meta) => {
            install_claude_plugin(dir, meta, paths, git_url, commit_sha)?;
        }
        PluginType::Both { pi, claude } => {
            install_pi_skill(dir, pi, paths)?;
            install_claude_plugin(dir, claude, paths, git_url, commit_sha)?;
        }
        PluginType::OpenPlugin(meta) => {
            install_open_plugin(dir, meta, paths, git_url, commit_sha)?;
        }
    }
    Ok(())
}

/// Install a Pi skill by copying the directory to ~/.pi/agent/skills/<name>/
fn install_pi_skill(
    dir: &Path,
    meta: &crate::plugin::PiSkillMeta,
    paths: &InstallPaths,
) -> Result<()> {
    let dest = paths.pi_skills.join(&meta.name);

    println!(
        "{} Installing Pi skill '{}' to {}",
        "→".cyan().bold(),
        meta.name.green(),
        dest.display().to_string().dimmed()
    );

    if dest.exists() {
        println!(
            "  {} Existing installation found, replacing...",
            "⚠".yellow()
        );
        fs::remove_dir_all(&dest).context(format!(
            "Failed to remove existing skill at {}",
            dest.display()
        ))?;
    }

    copy_dir_recursive(dir, &dest)
        .context(format!("Failed to copy skill to {}", dest.display()))?;

    println!(
        "  {} Pi skill '{}' installed successfully",
        "✓".green().bold(),
        meta.name.green()
    );

    Ok(())
}

/// Install a Claude Code plugin by copying to the cache and registering it.
fn install_claude_plugin(
    dir: &Path,
    meta: &crate::plugin::ClaudePluginMeta,
    paths: &InstallPaths,
    git_url: Option<&str>,
    commit_sha: Option<&str>,
) -> Result<()> {
    let source_name = git_url
        .and_then(git::repo_slug_from_url)
        .map(|slug| {
            let (owner, _repo) = slug.split_once('/').unwrap_or(("unknown", &slug));
            owner.to_string()
        })
        .unwrap_or_else(|| "local".to_string());

    let version = meta.version.as_deref().unwrap_or("unknown");
    let dest = paths
        .claude_plugins_cache
        .join(&source_name)
        .join(&meta.name)
        .join(version);

    println!(
        "{} Installing Claude Code plugin '{}' to {}",
        "→".cyan().bold(),
        meta.name.green(),
        dest.display().to_string().dimmed()
    );

    if dest.exists() {
        println!(
            "  {} Existing installation found, replacing...",
            "⚠".yellow()
        );
        fs::remove_dir_all(&dest).context(format!(
            "Failed to remove existing plugin at {}",
            dest.display()
        ))?;
    }

    fs::create_dir_all(&dest).context(format!(
        "Failed to create plugin directory: {}",
        dest.display()
    ))?;

    // Copy plugin files
    copy_dir_recursive(dir, &dest)
        .context(format!("Failed to copy plugin to {}", dest.display()))?;

    // Register in installed_plugins.json
    register_claude_plugin(paths, &source_name, meta, &dest, commit_sha)?;

    // If the plugin has a skills/ subdirectory, symlink into ~/.claude/skills/
    let skills_src = dest.join("skills");
    if skills_src.exists() && skills_src.is_dir() {
        install_claude_skills(&skills_src, paths)?;
    }

    println!(
        "  {} Claude Code plugin '{}' installed successfully",
        "✓".green().bold(),
        meta.name.green()
    );

    Ok(())
}

/// Symlink skills from a Claude Code plugin into ~/.claude/skills/
fn install_claude_skills(skills_dir: &Path, paths: &InstallPaths) -> Result<()> {
    fs::create_dir_all(&paths.claude_skills).context("Failed to create Claude skills directory")?;

    for entry in fs::read_dir(skills_dir).context(format!(
        "Failed to read skills directory: {}",
        skills_dir.display()
    ))? {
        let entry = entry?;
        let skill_name = entry.file_name();
        let dest = paths.claude_skills.join(&skill_name);

        if dest.exists() {
            if dest.is_symlink() {
                fs::remove_file(&dest).ok();
            } else {
                println!(
                    "  {} Skipping skill '{}' — destination already exists and is not a symlink",
                    "⚠".yellow(),
                    skill_name.to_string_lossy()
                );
                continue;
            }
        }

        std::os::unix::fs::symlink(entry.path(), &dest).context(format!(
            "Failed to symlink skill '{}'",
            skill_name.to_string_lossy()
        ))?;

        println!(
            "  {} Linked skill '{}'",
            "✓".green(),
            skill_name.to_string_lossy().dimmed()
        );
    }

    Ok(())
}

/// Register a Claude Code plugin in installed_plugins.json.
fn register_claude_plugin(
    paths: &InstallPaths,
    source_name: &str,
    meta: &crate::plugin::ClaudePluginMeta,
    install_path: &Path,
    commit_sha: Option<&str>,
) -> Result<()> {
    use serde_json::{json, Value};

    let plugin_key = format!("{meta_name}@{source_name}", meta_name = meta.name);

    let mut registry: Value = if paths.claude_installed_json.exists() {
        let content = fs::read_to_string(&paths.claude_installed_json)
            .context("Failed to read installed_plugins.json")?;
        serde_json::from_str(&content).unwrap_or_else(|_| {
            json!({
                "version": 2,
                "plugins": {}
            })
        })
    } else {
        json!({
            "version": 2,
            "plugins": {}
        })
    };

    let now = chrono_now();

    let entry = json!([{
        "scope": "user",
        "installPath": install_path.to_string_lossy(),
        "version": meta.version.as_deref().unwrap_or("unknown"),
        "installedAt": now,
        "lastUpdated": now,
        "gitCommitSha": commit_sha.unwrap_or("unknown")
    }]);

    registry["plugins"][&plugin_key] = entry;

    let content =
        serde_json::to_string_pretty(&registry).context("Failed to serialize plugin registry")?;
    fs::write(&paths.claude_installed_json, content)
        .context("Failed to write installed_plugins.json")?;

    Ok(())
}

/// Get current timestamp in ISO 8601 format.
fn chrono_now() -> String {
    use std::time::SystemTime;
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();
    // Simple ISO 8601 without pulling in chrono crate
    let secs = now.as_secs();
    // Convert to a reasonable ISO format
    let days = secs / 86400;
    let time_secs = secs % 86400;
    let hours = time_secs / 3600;
    let minutes = (time_secs % 3600) / 60;
    let seconds = time_secs % 60;

    // Calculate year/month/day from days since epoch (approximate, good enough for metadata)
    let mut y = 1970i64;
    let mut remaining = days as i64;
    loop {
        let days_in_year = if is_leap(y) { 366 } else { 365 };
        if remaining < days_in_year {
            break;
        }
        remaining -= days_in_year;
        y += 1;
    }
    let month_days = if is_leap(y) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };
    let mut m = 0;
    for (i, &md) in month_days.iter().enumerate() {
        if remaining < md as i64 {
            m = i + 1;
            break;
        }
        remaining -= md as i64;
    }
    let d = remaining + 1;

    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.000Z",
        y, m, d, hours, minutes, seconds
    )
}

fn is_leap(y: i64) -> bool {
    (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0)
}

/// Install an Open Plugin by copying its skills to Pi and registering with Claude Code.
fn install_open_plugin(
    dir: &Path,
    meta: &crate::plugin::OpenPluginMeta,
    paths: &InstallPaths,
    git_url: Option<&str>,
    commit_sha: Option<&str>,
) -> Result<()> {
    println!(
        "{} Installing Open Plugin '{}' (v{})...",
        "→".cyan().bold(),
        meta.name.green(),
        meta.version.as_deref().unwrap_or("unknown")
    );

    // Install each skill to Pi
    if let Some(ref skills_dir) = meta.skills_dir {
        for skill_meta in &meta.skills {
            let skill_src = skills_dir.join(&skill_meta.name);
            if skill_src.exists() {
                install_pi_skill(&skill_src, skill_meta, paths)?;
            }
        }
    } else if !meta.skills.is_empty() {
        // Root SKILL.md — install the whole directory as a single skill
        install_pi_skill(dir, &meta.skills[0], paths)?;
    }

    // Also install as Claude Code plugin
    let claude_meta = crate::plugin::ClaudePluginMeta {
        name: meta.name.clone(),
        version: meta.version.clone(),
        description: meta.description.clone(),
        components: vec!["skills".to_string()],
    };
    install_claude_plugin(dir, &claude_meta, paths, git_url, commit_sha)?;

    println!(
        "  {} Open Plugin '{}' installed successfully",
        "✓".green().bold(),
        meta.name.green()
    );

    Ok(())
}

/// Recursively copy a directory.
fn copy_dir_recursive(src: &Path, dest: &Path) -> Result<()> {
    fs::create_dir_all(dest)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if file_type.is_dir() {
            // Skip .git directories
            if entry.file_name() == ".git" {
                continue;
            }
            copy_dir_recursive(&src_path, &dest_path)?;
        } else if file_type.is_symlink() {
            let target = fs::read_link(&src_path)?;
            std::os::unix::fs::symlink(&target, &dest_path)?;
        } else {
            fs::copy(&src_path, &dest_path)?;
        }
    }

    Ok(())
}
