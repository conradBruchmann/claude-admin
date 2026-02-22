use std::path::Path;

use crate::domain::errors::ApiError;
use claude_admin_shared::BackupEntry;

/// List all backups from ~/.claude/backups/
pub async fn list_backups(claude_home: &Path) -> Result<Vec<BackupEntry>, ApiError> {
    let backup_dir = claude_home.join("backups");
    if !tokio::fs::try_exists(&backup_dir).await.unwrap_or(false) {
        return Ok(vec![]);
    }

    let mut entries = Vec::new();
    let mut dir = tokio::fs::read_dir(&backup_dir).await?;
    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let metadata = entry.metadata().await?;
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let created = metadata
            .modified()
            .ok()
            .and_then(|t| {
                t.duration_since(std::time::UNIX_EPOCH)
                    .ok()
                    .map(|d| d.as_secs())
            })
            .map(|secs| {
                chrono::DateTime::from_timestamp(secs as i64, 0)
                    .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                    .unwrap_or_default()
            })
            .unwrap_or_default();

        // Parse original path from backup name:
        // Format: YYYYMMDD_HHMMSS_relative_path
        let original_path = parse_original_path(&name);

        entries.push(BackupEntry {
            name,
            size_bytes: metadata.len(),
            created,
            original_path,
        });
    }

    entries.sort_by(|a, b| b.created.cmp(&a.created));
    Ok(entries)
}

/// Restore a backup by copying its content back to the original path.
pub async fn restore_backup(claude_home: &Path, backup_name: &str) -> Result<(), ApiError> {
    let backup_path = claude_home.join("backups").join(backup_name);
    if !tokio::fs::try_exists(&backup_path).await.unwrap_or(false) {
        return Err(ApiError::NotFound(format!(
            "Backup '{}' not found",
            backup_name
        )));
    }

    let content = tokio::fs::read_to_string(&backup_path).await?;

    // Reconstruct original path from backup name
    let original_relative = parse_original_path(backup_name);
    if original_relative.is_empty() {
        return Err(ApiError::BadRequest(
            "Cannot determine original file path from backup name".to_string(),
        ));
    }

    let original_path = claude_home.join(original_relative.replace('_', "/"));

    // Create a backup of current state before restoring
    if tokio::fs::try_exists(&original_path).await.unwrap_or(false) {
        let current_content = tokio::fs::read_to_string(&original_path).await?;
        crate::services::file_ops::create_backup(claude_home, &original_path, &current_content)
            .await?;
    }

    // Ensure parent dir exists and write
    if let Some(parent) = original_path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    tokio::fs::write(&original_path, &content).await?;

    Ok(())
}

/// Delete a backup file.
pub async fn delete_backup(claude_home: &Path, backup_name: &str) -> Result<(), ApiError> {
    let backup_path = claude_home.join("backups").join(backup_name);
    if !tokio::fs::try_exists(&backup_path).await.unwrap_or(false) {
        return Err(ApiError::NotFound(format!(
            "Backup '{}' not found",
            backup_name
        )));
    }
    tokio::fs::remove_file(&backup_path).await?;
    Ok(())
}

/// Extract relative path from backup name.
/// Format: YYYYMMDD_HHMMSS_relative_path
fn parse_original_path(name: &str) -> String {
    // Skip timestamp prefix: "20240101_120000_"
    let parts: Vec<&str> = name.splitn(3, '_').collect();
    if parts.len() >= 3 {
        parts[2].to_string()
    } else {
        name.to_string()
    }
}
