use axum::extract::{Path, State};
use axum::Json;
use std::sync::Arc;

use crate::app::AppState;
use crate::domain::errors::ApiError;
use crate::services::{file_ops, fs_scanner, project_resolver};
use claude_admin_shared::{
    ClaudeMdContent, ClaudeMdUpdateRequest, ProjectDetail, ProjectStatus, ProjectSummaryLite,
};

/// Instant project list - no filesystem checks.
pub async fn list_projects(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<ProjectSummaryLite>>, ApiError> {
    let projects = fs_scanner::scan_projects_lite(&state.claude_json_path).await?;
    Ok(Json(projects))
}

/// JIT status for a single project - loaded on demand.
pub async fn get_project_status(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<ProjectStatus>, ApiError> {
    let project_path = project_resolver::decode_project_id(&id)?;
    let status = fs_scanner::scan_project_status(&state.claude_home, &project_path).await?;
    Ok(Json(status))
}

pub async fn get_project(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<ProjectDetail>, ApiError> {
    let project_path = project_resolver::decode_project_id(&id)?;
    let detail = fs_scanner::scan_project_detail(&state.claude_home, &project_path).await?;
    Ok(Json(detail))
}

pub async fn get_claude_md(
    State(_state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<ClaudeMdContent>, ApiError> {
    let project_path = project_resolver::decode_project_id(&id)?;
    let claude_md_path = std::path::Path::new(&project_path).join("CLAUDE.md");

    if !tokio::fs::try_exists(&claude_md_path)
        .await
        .unwrap_or(false)
    {
        return Err(ApiError::NotFound(format!(
            "CLAUDE.md not found for project {}",
            project_path
        )));
    }

    let content = tokio::fs::read_to_string(&claude_md_path).await?;
    Ok(Json(ClaudeMdContent {
        content,
        path: claude_md_path.to_string_lossy().to_string(),
    }))
}

pub async fn put_claude_md(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<ClaudeMdUpdateRequest>,
) -> Result<Json<ClaudeMdContent>, ApiError> {
    let project_path = project_resolver::decode_project_id(&id)?;
    let claude_md_path = std::path::Path::new(&project_path).join("CLAUDE.md");

    file_ops::write_with_backup(&state.claude_home, &claude_md_path, &req.content).await?;

    Ok(Json(ClaudeMdContent {
        content: req.content,
        path: claude_md_path.to_string_lossy().to_string(),
    }))
}
