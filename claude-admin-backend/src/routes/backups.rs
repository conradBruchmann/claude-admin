use axum::extract::{Path, State};
use axum::Json;
use std::sync::Arc;

use crate::app::AppState;
use crate::domain::errors::ApiError;
use crate::services::backups;
use claude_admin_shared::BackupEntry;

pub async fn list_backups(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<BackupEntry>>, ApiError> {
    let entries = backups::list_backups(&state.claude_home).await?;
    Ok(Json(entries))
}

pub async fn restore_backup(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // Name validation is relaxed for backups (they contain timestamps/underscores)
    if name.contains("..") || name.contains('/') || name.contains('\\') || name.contains('\0') {
        return Err(ApiError::BadRequest("Invalid backup name".to_string()));
    }
    backups::restore_backup(&state.claude_home, &name).await?;
    Ok(Json(
        serde_json::json!({ "status": "restored", "name": name }),
    ))
}

pub async fn delete_backup(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Result<Json<serde_json::Value>, ApiError> {
    if name.contains("..") || name.contains('/') || name.contains('\\') || name.contains('\0') {
        return Err(ApiError::BadRequest("Invalid backup name".to_string()));
    }
    backups::delete_backup(&state.claude_home, &name).await?;
    Ok(Json(
        serde_json::json!({ "status": "deleted", "name": name }),
    ))
}
