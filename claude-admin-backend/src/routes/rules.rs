use axum::extract::{Path, State};
use axum::Json;
use std::sync::Arc;

use crate::app::AppState;
use crate::domain::errors::ApiError;
use crate::domain::extractors::AppJson;
use crate::domain::validation::validate_resource_name;
use crate::services::{file_ops, fs_scanner};
use claude_admin_shared::{ConfigScope, RuleCreateRequest, RuleFile, RuleUpdateRequest};

pub async fn list_rules(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<RuleFile>>, ApiError> {
    let rules = fs_scanner::scan_rules(&state.claude_home, &ConfigScope::Global).await?;
    Ok(Json(rules))
}

pub async fn get_rule(
    State(state): State<Arc<AppState>>,
    Path((scope, name)): Path<(String, String)>,
) -> Result<Json<RuleFile>, ApiError> {
    validate_resource_name(&name, "Rule")?;
    let scope = parse_scope(&scope)?;
    let rule_path = rule_path(&state.claude_home, &scope, &name);
    let content = tokio::fs::read_to_string(&rule_path)
        .await
        .map_err(|_| ApiError::NotFound(format!("Rule '{}' not found", name)))?;

    Ok(Json(RuleFile {
        name,
        path: rule_path.to_string_lossy().to_string(),
        scope,
        content,
    }))
}

pub async fn create_rule(
    State(state): State<Arc<AppState>>,
    AppJson(req): AppJson<RuleCreateRequest>,
) -> Result<Json<RuleFile>, ApiError> {
    validate_resource_name(&req.name, "Rule")?;
    let rules_dir = rules_dir(&state.claude_home, &req.scope);
    tokio::fs::create_dir_all(&rules_dir).await?;

    let filename = if req.name.ends_with(".md") {
        req.name.clone()
    } else {
        format!("{}.md", req.name)
    };
    let rule_path = rules_dir.join(&filename);
    file_ops::write_with_backup(&state.claude_home, &rule_path, &req.content).await?;

    let webhooks = crate::services::webhooks::load_webhooks(&state.claude_home);
    crate::services::webhooks::fire_webhook(
        &webhooks,
        "rule.created",
        serde_json::json!({"name": &req.name, "scope": format!("{:?}", &req.scope)}),
    );

    Ok(Json(RuleFile {
        name: req.name,
        path: rule_path.to_string_lossy().to_string(),
        scope: req.scope,
        content: req.content,
    }))
}

pub async fn update_rule(
    State(state): State<Arc<AppState>>,
    Path((scope, name)): Path<(String, String)>,
    AppJson(req): AppJson<RuleUpdateRequest>,
) -> Result<Json<RuleFile>, ApiError> {
    validate_resource_name(&name, "Rule")?;
    let scope = parse_scope(&scope)?;
    let rule_path = rule_path(&state.claude_home, &scope, &name);

    if !tokio::fs::try_exists(&rule_path).await.unwrap_or(false) {
        return Err(ApiError::NotFound(format!("Rule '{}' not found", name)));
    }

    file_ops::write_with_backup(&state.claude_home, &rule_path, &req.content).await?;

    let webhooks = crate::services::webhooks::load_webhooks(&state.claude_home);
    crate::services::webhooks::fire_webhook(
        &webhooks,
        "rule.updated",
        serde_json::json!({"name": &name, "scope": format!("{:?}", &scope)}),
    );

    Ok(Json(RuleFile {
        name,
        path: rule_path.to_string_lossy().to_string(),
        scope,
        content: req.content,
    }))
}

pub async fn delete_rule(
    State(state): State<Arc<AppState>>,
    Path((scope, name)): Path<(String, String)>,
) -> Result<Json<serde_json::Value>, ApiError> {
    validate_resource_name(&name, "Rule")?;
    let scope = parse_scope(&scope)?;
    let rule_path = rule_path(&state.claude_home, &scope, &name);

    if !tokio::fs::try_exists(&rule_path).await.unwrap_or(false) {
        return Err(ApiError::NotFound(format!("Rule '{}' not found", name)));
    }

    let content = tokio::fs::read_to_string(&rule_path).await?;
    file_ops::create_backup(&state.claude_home, &rule_path, &content).await?;

    tokio::fs::remove_file(&rule_path).await?;

    let webhooks = crate::services::webhooks::load_webhooks(&state.claude_home);
    crate::services::webhooks::fire_webhook(
        &webhooks,
        "rule.deleted",
        serde_json::json!({"name": &name, "scope": format!("{:?}", &scope)}),
    );

    Ok(Json(serde_json::json!({"deleted": name})))
}

fn parse_scope(s: &str) -> Result<ConfigScope, ApiError> {
    match s {
        "global" => Ok(ConfigScope::Global),
        "project" => Ok(ConfigScope::Project),
        _ => Err(ApiError::BadRequest(format!("Invalid scope: {}", s))),
    }
}

fn rules_dir(claude_home: &std::path::Path, scope: &ConfigScope) -> std::path::PathBuf {
    match scope {
        ConfigScope::Global => claude_home.join("rules"),
        ConfigScope::Project => claude_home.join("rules"), // TODO: project-scoped
    }
}

fn rule_path(claude_home: &std::path::Path, scope: &ConfigScope, name: &str) -> std::path::PathBuf {
    let filename = if name.ends_with(".md") {
        name.to_string()
    } else {
        format!("{}.md", name)
    };
    rules_dir(claude_home, scope).join(filename)
}
