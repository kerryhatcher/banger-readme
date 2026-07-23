use std::path::PathBuf;

/// Paths used by the installer for Pi and Claude Code plugin directories.
pub struct InstallPaths {
    /// Pi skills directory: ~/.pi/agent/skills/
    pub pi_skills: PathBuf,
    /// Claude Code plugins cache: ~/.claude/plugins/cache/
    pub claude_plugins_cache: PathBuf,
    /// Claude Code installed plugins registry: ~/.claude/plugins/installed_plugins.json
    pub claude_installed_json: PathBuf,
    /// Claude Code skills directory: ~/.claude/skills/
    pub claude_skills: PathBuf,
}

impl InstallPaths {
    pub fn detect() -> Self {
        let home = dirs::home_dir().expect("Could not determine home directory");

        Self {
            pi_skills: home.join(".pi").join("agent").join("skills"),
            claude_plugins_cache: home.join(".claude").join("plugins").join("cache"),
            claude_installed_json: home
                .join(".claude")
                .join("plugins")
                .join("installed_plugins.json"),
            claude_skills: home.join(".claude").join("skills"),
        }
    }
}
