use axum::extract::{Path, State};
use axum::Json;
use std::sync::Arc;

use crate::app::AppState;
use crate::domain::errors::ApiError;
use crate::domain::frontmatter;
use crate::services::{file_ops, fs_scanner};
use claude_admin_shared::{ConfigScope, SkillCreateRequest, SkillFile, SkillUpdateRequest};

pub async fn list_skills(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<SkillFile>>, ApiError> {
    let skills = fs_scanner::scan_skills(&state.claude_home, &ConfigScope::Global).await?;
    Ok(Json(skills))
}

pub async fn get_skill(
    State(state): State<Arc<AppState>>,
    Path((scope, name)): Path<(String, String)>,
) -> Result<Json<SkillFile>, ApiError> {
    let scope = parse_scope(&scope)?;
    let skill_path = skill_path(&state.claude_home, &scope, &name);
    let content = tokio::fs::read_to_string(&skill_path)
        .await
        .map_err(|_| ApiError::NotFound(format!("Skill '{}' not found", name)))?;

    let (fm, body) = frontmatter::parse_frontmatter(&content);

    Ok(Json(SkillFile {
        name: name.clone(),
        path: skill_path.to_string_lossy().to_string(),
        scope,
        frontmatter: fm.unwrap_or_default(),
        content: body,
    }))
}

pub async fn create_skill(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SkillCreateRequest>,
) -> Result<Json<SkillFile>, ApiError> {
    let skill_dir = skill_dir(&state.claude_home, &req.scope, &req.name);
    tokio::fs::create_dir_all(&skill_dir).await?;

    let skill_path = skill_dir.join("SKILL.md");
    let content = frontmatter::serialize_frontmatter(&req.frontmatter, &req.content);
    file_ops::write_with_backup(&state.claude_home, &skill_path, &content).await?;

    Ok(Json(SkillFile {
        name: req.name,
        path: skill_path.to_string_lossy().to_string(),
        scope: req.scope,
        frontmatter: req.frontmatter,
        content: req.content,
    }))
}

pub async fn update_skill(
    State(state): State<Arc<AppState>>,
    Path((scope, name)): Path<(String, String)>,
    Json(req): Json<SkillUpdateRequest>,
) -> Result<Json<SkillFile>, ApiError> {
    let scope = parse_scope(&scope)?;
    let skill_path = skill_path(&state.claude_home, &scope, &name);

    if !tokio::fs::try_exists(&skill_path).await.unwrap_or(false) {
        return Err(ApiError::NotFound(format!("Skill '{}' not found", name)));
    }

    let content = frontmatter::serialize_frontmatter(&req.frontmatter, &req.content);
    file_ops::write_with_backup(&state.claude_home, &skill_path, &content).await?;

    Ok(Json(SkillFile {
        name,
        path: skill_path.to_string_lossy().to_string(),
        scope,
        frontmatter: req.frontmatter,
        content: req.content,
    }))
}

pub async fn delete_skill(
    State(state): State<Arc<AppState>>,
    Path((scope, name)): Path<(String, String)>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let scope = parse_scope(&scope)?;
    let dir = skill_dir(&state.claude_home, &scope, &name);

    if !tokio::fs::try_exists(&dir).await.unwrap_or(false) {
        return Err(ApiError::NotFound(format!("Skill '{}' not found", name)));
    }

    // Backup before delete
    let skill_path = dir.join("SKILL.md");
    if tokio::fs::try_exists(&skill_path).await.unwrap_or(false) {
        let content = tokio::fs::read_to_string(&skill_path).await?;
        file_ops::create_backup(&state.claude_home, &skill_path, &content).await?;
    }

    tokio::fs::remove_dir_all(&dir).await?;
    Ok(Json(serde_json::json!({"deleted": name})))
}

fn parse_scope(s: &str) -> Result<ConfigScope, ApiError> {
    match s {
        "global" => Ok(ConfigScope::Global),
        "project" => Ok(ConfigScope::Project),
        _ => Err(ApiError::BadRequest(format!("Invalid scope: {}", s))),
    }
}

fn skill_dir(claude_home: &std::path::Path, scope: &ConfigScope, name: &str) -> std::path::PathBuf {
    match scope {
        ConfigScope::Global => claude_home.join("skills").join(name),
        ConfigScope::Project => claude_home.join("skills").join(name), // TODO: project-scoped path
    }
}

fn skill_path(
    claude_home: &std::path::Path,
    scope: &ConfigScope,
    name: &str,
) -> std::path::PathBuf {
    skill_dir(claude_home, scope, name).join("SKILL.md")
}
