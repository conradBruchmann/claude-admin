use axum::extract::State;
use axum::Json;
use std::sync::Arc;

use crate::app::AppState;
use crate::domain::errors::ApiError;
use crate::services::claude_api;
use claude_admin_shared::{SuggestionRequest, SuggestionResponse};

pub async fn suggest(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SuggestionRequest>,
) -> Result<Json<SuggestionResponse>, ApiError> {
    let client = state
        .anthropic_client
        .as_ref()
        .ok_or_else(|| ApiError::BadRequest("ANTHROPIC_API_KEY not configured".to_string()))?;

    let response = claude_api::get_suggestions(client, &req).await?;
    Ok(Json(response))
}

pub async fn validate(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SuggestionRequest>,
) -> Result<Json<SuggestionResponse>, ApiError> {
    let client = state
        .anthropic_client
        .as_ref()
        .ok_or_else(|| ApiError::BadRequest("ANTHROPIC_API_KEY not configured".to_string()))?;

    let response = claude_api::validate_content(client, &req).await?;
    Ok(Json(response))
}
