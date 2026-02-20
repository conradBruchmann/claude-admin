use std::path::Path;

use crate::domain::errors::ApiError;
use crate::services::project_resolver;
use claude_admin_shared::*;

/// Compute health overview for all projects.
pub async fn overall_health(
    claude_home: &Path,
    claude_json_path: &Path,
) -> Result<HealthOverview, ApiError> {
    let projects =
        crate::services::fs_scanner::scan_projects(claude_home, claude_json_path).await?;
    let mut summaries = Vec::new();
    let mut total_score: u64 = 0;

    for project in &projects {
        let health = compute_health_score(claude_home, &project.path).await?;
        let mut issues = Vec::new();

        if !health.has_claude_md {
            issues.push("Missing CLAUDE.md".to_string());
        }
        if !health.has_memory {
            issues.push("No memory files".to_string());
        }
        if health.permission_count > 50 {
            issues.push(format!(
                "{} permission entries (bloated)",
                health.permission_count
            ));
        }
        if !health.security_issues.is_empty() {
            issues.push(format!("{} security issues", health.security_issues.len()));
        }
        if !health.duplicated_rules.is_empty() {
            issues.push(format!(
                "{} duplicated rules",
                health.duplicated_rules.len()
            ));
        }

        total_score += health.score as u64;
        summaries.push(ProjectHealthSummary {
            project_id: project.encoded_path.clone(),
            name: project.name.clone(),
            score: health.score,
            issues,
        });
    }

    let average_score = if summaries.is_empty() {
        0
    } else {
        (total_score / summaries.len() as u64) as u8
    };

    summaries.sort_by(|a, b| a.score.cmp(&b.score));

    Ok(HealthOverview {
        projects: summaries,
        average_score,
    })
}

/// Compute health score for a single project (0-100).
pub async fn compute_health_score(
    claude_home: &Path,
    project_path: &str,
) -> Result<ProjectHealth, ApiError> {
    let project_fs = Path::new(project_path);
    let encoded = project_resolver::encode_project_path(project_path);
    let claude_project_dir = claude_home.join("projects").join(&encoded);

    let has_claude_md = tokio::fs::try_exists(project_fs.join("CLAUDE.md"))
        .await
        .unwrap_or(false);

    let has_memory_dir = tokio::fs::try_exists(claude_project_dir.join("memory"))
        .await
        .unwrap_or(false);
    let has_memory = if has_memory_dir {
        if let Ok(mut dir) = tokio::fs::read_dir(claude_project_dir.join("memory")).await {
            dir.next_entry().await.unwrap_or(None).is_some()
        } else {
            false
        }
    } else {
        false
    };

    // Count permissions
    let settings_local = project_fs.join(".claude").join("settings.local.json");
    let (permission_count, security_issues) = if tokio::fs::try_exists(&settings_local)
        .await
        .unwrap_or(false)
    {
        let perms = crate::services::permissions::scan_project_permissions(project_path).await?;
        let sec_issues = perms.security_warnings;
        (perms.entries.len(), sec_issues)
    } else {
        (0, vec![])
    };

    // Check for project-specific skills/rules
    let has_project_config = tokio::fs::try_exists(claude_project_dir.join("skills"))
        .await
        .unwrap_or(false)
        || tokio::fs::try_exists(claude_project_dir.join("rules"))
            .await
            .unwrap_or(false)
        || tokio::fs::try_exists(project_fs.join(".claude").join("skills"))
            .await
            .unwrap_or(false)
        || tokio::fs::try_exists(project_fs.join(".claude").join("rules"))
            .await
            .unwrap_or(false);

    // Find duplicated rules
    let duplicated_rules = find_duplicated_rules(claude_home, project_path).await;

    // Calculate score
    let mut score: u8 = 0;

    // Has CLAUDE.md? (+20)
    if has_claude_md {
        score += 20;
    }

    // Has Memory? (+10)
    if has_memory {
        score += 10;
    }

    // Permission list manageable (<50)? (+15)
    if permission_count < 50 {
        score += 15;
    }

    // No security issues? (+25)
    if security_issues.is_empty() {
        score += 25;
    }

    // No duplicated rules? (+15)
    if duplicated_rules.is_empty() {
        score += 15;
    }

    // Uses project-specific config? (+15)
    if has_project_config {
        score += 15;
    }

    Ok(ProjectHealth {
        score,
        has_claude_md,
        has_memory,
        permission_count,
        security_issues,
        duplicated_rules,
    })
}

/// Find rules that are duplicated between project CLAUDE.md and global rules/skills.
async fn find_duplicated_rules(claude_home: &Path, project_path: &str) -> Vec<DuplicatedRule> {
    let mut duplicates = Vec::new();

    // Read project CLAUDE.md
    let project_claude_md = Path::new(project_path).join("CLAUDE.md");
    let project_content = match tokio::fs::read_to_string(&project_claude_md).await {
        Ok(c) => c,
        Err(_) => return duplicates,
    };

    // Read global rules
    let rules_dir = claude_home.join("rules");
    if tokio::fs::try_exists(&rules_dir).await.unwrap_or(false) {
        if let Ok(mut dir) = tokio::fs::read_dir(&rules_dir).await {
            while let Ok(Some(entry)) = dir.next_entry().await {
                let path = entry.path();
                if path.extension().map_or(false, |e| e == "md") {
                    if let Ok(rule_content) = tokio::fs::read_to_string(&path).await {
                        // Check if significant portions overlap (simple line-based check)
                        let rule_lines: Vec<&str> = rule_content
                            .lines()
                            .filter(|l| l.trim().len() > 20)
                            .collect();
                        for line in &rule_lines {
                            if project_content.contains(line.trim()) {
                                duplicates.push(DuplicatedRule {
                                    text: line.trim().chars().take(100).collect(),
                                    found_in_project: "CLAUDE.md".to_string(),
                                    also_in_global: format!(
                                        "rules/{}",
                                        path.file_name().unwrap_or_default().to_string_lossy()
                                    ),
                                });
                                break; // One match per file is enough
                            }
                        }
                    }
                }
            }
        }
    }

    duplicates
}
