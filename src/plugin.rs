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
    /// An Open Plugin (https://open-plugins.com/) with .plugin/plugin.json manifest.
    OpenPlugin(OpenPluginMeta),
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

/// Metadata from an Open Plugin manifest (.plugin/plugin.json).
#[derive(Debug, Clone)]
pub struct OpenPluginMeta {
    pub name: String,
    pub version: Option<String>,
    pub description: Option<String>,
    #[allow(dead_code)]
    pub author: Option<String>,
    #[allow(dead_code)]
    pub license: Option<String>,
    /// Skills discovered in the plugin's skills/ directory.
    pub skills: Vec<PiSkillMeta>,
    /// Path to the skills directory within the plugin.
    pub skills_dir: Option<std::path::PathBuf>,
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

/// Structure of an Open Plugin manifest (.plugin/plugin.json).
#[derive(Debug, Deserialize)]
struct OpenPluginManifest {
    name: String,
    version: Option<String>,
    description: Option<String>,
    author: Option<OpenPluginAuthor>,
    license: Option<String>,
    #[allow(dead_code)]
    skills: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenPluginAuthor {
    name: Option<String>,
    #[allow(dead_code)]
    url: Option<String>,
}

/// Detect what kind of plugin is in the given directory.
pub fn detect(dir: &Path) -> Result<PluginType> {
    // Check for Open Plugin format first (.plugin/plugin.json)
    let open_plugin_json = dir.join(".plugin").join("plugin.json");
    if open_plugin_json.exists() {
        return detect_open_plugin(dir, &open_plugin_json);
    }

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
            anyhow::bail!(
                "No SKILL.md, .claude-plugin.json, or .plugin/plugin.json found in {}",
                dir.display()
            );
        }
    }
}

/// Detect an Open Plugin and discover its skills.
fn detect_open_plugin(dir: &Path, manifest_path: &Path) -> Result<PluginType> {
    let content =
        std::fs::read_to_string(manifest_path).context("Failed to read .plugin/plugin.json")?;

    let manifest: OpenPluginManifest =
        serde_json::from_str(&content).context("Failed to parse Open Plugin manifest")?;

    // Discover skills in the skills/ directory
    let skills_dir = dir.join("skills");
    let mut skills = Vec::new();

    if skills_dir.exists() && skills_dir.is_dir() {
        for entry in std::fs::read_dir(&skills_dir).context("Failed to read skills directory")? {
            let entry = entry?;
            let skill_path = entry.path();
            if skill_path.is_dir() {
                let skill_md = skill_path.join("SKILL.md");
                if skill_md.exists() {
                    if let Ok(meta) = parse_skill_md(&skill_md) {
                        skills.push(meta);
                    }
                }
            }
        }
    }

    // Also check for root SKILL.md (single-skill plugin)
    if skills.is_empty() {
        let root_skill = dir.join("SKILL.md");
        if root_skill.exists() {
            if let Ok(meta) = parse_skill_md(&root_skill) {
                skills.push(meta);
            }
        }
    }

    let author = manifest.author.and_then(|a| a.name);

    Ok(PluginType::OpenPlugin(OpenPluginMeta {
        name: manifest.name,
        version: manifest.version,
        description: manifest.description,
        author,
        license: manifest.license,
        skills,
        skills_dir: if skills_dir.exists() {
            Some(skills_dir)
        } else {
            None
        },
    }))
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
            PluginType::OpenPlugin(m) => &m.name,
        }
    }

    pub fn description(&self) -> &str {
        match self {
            PluginType::PiSkill(m) => &m.description,
            PluginType::ClaudePlugin(m) => m.description.as_deref().unwrap_or("No description"),
            PluginType::Both { pi, .. } => &pi.description,
            PluginType::OpenPlugin(m) => m.description.as_deref().unwrap_or("No description"),
        }
    }
}
