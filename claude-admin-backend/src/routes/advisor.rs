use axum::extract::{Path, State};
use axum::Json;
use std::sync::Arc;

use crate::app::AppState;
use crate::domain::errors::ApiError;
use crate::services::{advisor, project_resolver};
use claude_admin_shared::AdvisorReport;

pub async fn get_advisor_report(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<AdvisorReport>, ApiError> {
    let project_path = project_resolver::decode_project_id(&id)?;

    let client = {
        let guard = state.anthropic_client.read().map_err(|_| {
            ApiError::Internal("Lock poisoned".to_string())
        })?;
        guard.as_ref().cloned().ok_or_else(|| {
            ApiError::BadRequest("API-Key nicht konfiguriert. Bitte unter Settings → API Key eintragen.".to_string())
        })?
    };

    let report = advisor::analyze_project(&state, &client, &project_path).await?;
    Ok(Json(report))
}
