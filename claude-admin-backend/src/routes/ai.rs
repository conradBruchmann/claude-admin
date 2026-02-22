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
    let client = {
        let guard = state.anthropic_client.read().map_err(|_| {
            ApiError::Internal("Lock poisoned".to_string())
        })?;
        guard.as_ref().cloned().ok_or_else(|| {
            ApiError::BadRequest("API-Key nicht konfiguriert. Bitte unter Settings → API Key eintragen.".to_string())
        })?
    };

    let response = claude_api::get_suggestions(&client, &req).await?;
    Ok(Json(response))
}

pub async fn validate(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SuggestionRequest>,
) -> Result<Json<SuggestionResponse>, ApiError> {
    let client = {
        let guard = state.anthropic_client.read().map_err(|_| {
            ApiError::Internal("Lock poisoned".to_string())
        })?;
        guard.as_ref().cloned().ok_or_else(|| {
            ApiError::BadRequest("API-Key nicht konfiguriert. Bitte unter Settings → API Key eintragen.".to_string())
        })?
    };

    let response = claude_api::validate_content(&client, &req).await?;
    Ok(Json(response))
}
