use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::Path;

/// Represents a detected plugin type and its metadata.
#[derive(Debug, Clone)]
pub enum PluginType {
    /// A Pi skill with a SKILL.md file.
    PiSkill(PiSkillMeta),
    /// A Claude Code plugin with plugin metadata.
    ClaudePlugin(ClaudePluginMeta),
    /// Both Pi and Claude Code compatible.
    Both {
        pi: PiSkillMeta,
        claude: ClaudePluginMeta,
    },
}

#[derive(Debug, Clone, Deserialize)]
pub struct PiSkillMeta {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ClaudePluginMeta {
    pub name: String,
    pub version: Option<String>,
    pub description: Option<String>,
    /// Subdirectories to install (commands, agents, skills, hooks, etc.)
    #[allow(dead_code)]
    pub components: Vec<String>,
}

/// Frontmatter from a SKILL.md file.
#[derive(Debug, Deserialize)]
struct SkillFrontmatter {
    name: Option<String>,
    description: Option<String>,
}

/// Structure of a .claude-plugin.json or plugin.json manifest.
#[derive(Debug, Deserialize)]
struct ClaudePluginManifest {
    name: String,
    version: Option<String>,
    description: Option<String>,
    #[serde(default)]
    components: Vec<String>,
}

/// Detect what kind of plugin is in the given directory.
pub fn detect(dir: &Path) -> Result<PluginType> {
    let skill_md = dir.join("SKILL.md");
    let claude_plugin_json = dir.join(".claude-plugin.json");
    let plugin_json = dir.join("plugin.json");

    let has_skill = skill_md.exists();
    let has_claude_manifest = claude_plugin_json.exists() || plugin_json.exists();

    match (has_skill, has_claude_manifest) {
        (true, true) => {
            let pi = parse_skill_md(&skill_md)?;
            let manifest_path = if claude_plugin_json.exists() {
                &claude_plugin_json
            } else {
                &plugin_json
            };
            let claude = parse_claude_manifest(manifest_path)?;
            Ok(PluginType::Both { pi, claude })
        }
        (true, false) => {
            let pi = parse_skill_md(&skill_md)?;
            Ok(PluginType::PiSkill(pi))
        }
        (false, true) => {
            let manifest_path = if claude_plugin_json.exists() {
                &claude_plugin_json
            } else {
                &plugin_json
            };
            let claude = parse_claude_manifest(manifest_path)?;
            Ok(PluginType::ClaudePlugin(claude))
        }
        (false, false) => {
            // Try to infer: if there's a SKILL.md anywhere, treat as Pi skill
            // Otherwise, treat as a generic Claude Code plugin
            anyhow::bail!(
                "No SKILL.md or .claude-plugin.json/plugin.json found in {}",
                dir.display()
            );
        }
    }
}

fn parse_skill_md(path: &Path) -> Result<PiSkillMeta> {
    let content = std::fs::read_to_string(path).context("Failed to read SKILL.md")?;

    // Extract YAML frontmatter between --- markers
    let frontmatter = extract_frontmatter(&content)?;

    let fm: SkillFrontmatter =
        serde_yaml::from_str(&frontmatter).context("Failed to parse SKILL.md frontmatter")?;

    let name = fm.name.unwrap_or_else(|| {
        path.parent()
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string())
    });

    let description = fm
        .description
        .unwrap_or_else(|| "No description provided".to_string());

    Ok(PiSkillMeta { name, description })
}

fn parse_claude_manifest(path: &Path) -> Result<ClaudePluginMeta> {
    let content =
        std::fs::read_to_string(path).context(format!("Failed to read {}", path.display()))?;

    let manifest: ClaudePluginManifest =
        serde_json::from_str(&content).context("Failed to parse plugin manifest")?;

    Ok(ClaudePluginMeta {
        name: manifest.name,
        version: manifest.version,
        description: manifest.description,
        components: manifest.components,
    })
}

/// Extract YAML frontmatter from a markdown file (content between --- markers).
fn extract_frontmatter(content: &str) -> Result<String> {
    let lines: Vec<&str> = content.lines().collect();

    if lines.is_empty() || lines[0].trim() != "---" {
        anyhow::bail!("No frontmatter found in SKILL.md");
    }

    let mut fm_lines = Vec::new();
    for line in lines.iter().skip(1) {
        if line.trim() == "---" {
            break;
        }
        fm_lines.push(*line);
    }

    Ok(fm_lines.join("\n"))
}

impl PluginType {
    pub fn name(&self) -> &str {
        match self {
            PluginType::PiSkill(m) => &m.name,
            PluginType::ClaudePlugin(m) => &m.name,
            PluginType::Both { pi, .. } => &pi.name,
        }
    }

    pub fn description(&self) -> &str {
        match self {
            PluginType::PiSkill(m) => &m.description,
            PluginType::ClaudePlugin(m) => m.description.as_deref().unwrap_or("No description"),
            PluginType::Both { pi, .. } => &pi.description,
        }
    }
}
