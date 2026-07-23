use anyhow::{Context, Result};
use git2::Repository;
use tempfile::TempDir;

/// Clone a git repository to a temporary directory and return its path.
/// If `branch` is provided, check out that branch after cloning.
pub fn clone_to_temp(url: &str, branch: Option<&str>) -> Result<TempDir> {
    let temp = TempDir::new().context("Failed to create temp directory")?;

    let mut builder = git2::build::RepoBuilder::new();
    if let Some(b) = branch {
        builder.branch(b);
    }

    builder
        .clone(url, temp.path())
        .context(format!("Failed to clone repository: {url}"))?;

    Ok(temp)
}

/// Get the current commit SHA of a repository.
pub fn head_sha(repo: &Repository) -> Result<String> {
    let head = repo.head().context("Failed to get HEAD")?;
    let oid = head.target().context("HEAD does not point to a commit")?;
    Ok(oid.to_string())
}

/// Extract the repo slug (owner/name) from a git URL.
pub fn repo_slug_from_url(url: &str) -> Option<String> {
    // Handle various git URL formats:
    // https://github.com/owner/name.git
    // git@github.com:owner/name.git
    // https://github.com/owner/name

    let url = url.strip_suffix(".git").unwrap_or(url);

    // SSH format: git@github.com:owner/name
    if let Some(rest) = url.strip_prefix("git@") {
        if let Some((_host, path)) = rest.split_once(':') {
            return Some(path.to_string());
        }
    }

    // HTTPS format: https://github.com/owner/name
    if let Some(rest) = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
    {
        let parts: Vec<&str> = rest.split('/').collect();
        if parts.len() >= 3 {
            return Some(format!(
                "{}/{}",
                parts[parts.len() - 2],
                parts[parts.len() - 1]
            ));
        }
    }

    None
}
