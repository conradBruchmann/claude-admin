use std::path::Path;

use crate::domain::errors::ApiError;
use claude_admin_shared::*;

/// Get GitHub overview from gh CLI and ~/.claude.json.
pub fn get_github_overview(claude_json_path: &Path) -> Result<GitHubOverview, ApiError> {
    // Check gh auth status
    let auth_output = std::process::Command::new("gh")
        .args(["auth", "status"])
        .output();

    let (auth_status, username) = match auth_output {
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let combined = format!("{}{}", stdout, stderr);

            if output.status.success() {
                // Extract username from output
                let username = combined
                    .lines()
                    .find(|l| l.contains("Logged in to") || l.contains("account"))
                    .and_then(|l| {
                        l.split_whitespace()
                            .find(|w| !w.contains('.') && !w.starts_with('(') && w.len() > 2)
                    })
                    .map(|s| s.trim().to_string());

                ("authenticated".to_string(), username)
            } else {
                ("not_authenticated".to_string(), None)
            }
        }
        Err(_) => ("gh_not_installed".to_string(), None),
    };

    // Get linked repos from ~/.claude.json
    let json_content = std::fs::read_to_string(claude_json_path).unwrap_or_default();
    let json: serde_json::Value = serde_json::from_str(&json_content).unwrap_or_default();

    let linked_repos: Vec<GitHubRepo> = json
        .get("githubRepoPaths")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str())
                .map(|path| {
                    let name = Path::new(path)
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();

                    GitHubRepo {
                        path: path.to_string(),
                        name,
                        recent_commits: vec![], // Could be fetched on demand
                        open_prs: 0,
                    }
                })
                .collect()
        })
        .unwrap_or_default();

    Ok(GitHubOverview {
        auth_status,
        username,
        linked_repos,
    })
}
