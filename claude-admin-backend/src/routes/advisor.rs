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

    let client = state
        .anthropic_client
        .as_ref()
        .ok_or_else(|| ApiError::BadRequest("ANTHROPIC_API_KEY nicht konfiguriert. Setze die Umgebungsvariable ANTHROPIC_API_KEY um den Project Advisor zu nutzen.".to_string()))?;

    let report = advisor::analyze_project(&state, client, &project_path).await?;
    Ok(Json(report))
}
