use axum::extract::State;
use axum::Json;
use std::sync::Arc;

use crate::app::AppState;
use crate::domain::errors::ApiError;
use crate::services::analytics;
use claude_admin_shared::*;

pub async fn get_analytics_overview(
    State(state): State<Arc<AppState>>,
) -> Result<Json<AnalyticsOverview>, ApiError> {
    let overview = analytics::get_analytics_overview(&state.claude_home)?;
    Ok(Json(overview))
}

pub async fn get_project_analytics(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<ProjectAnalytics>>, ApiError> {
    let projects = analytics::get_project_analytics(&state.claude_home, &state.claude_json_path)?;
    Ok(Json(projects))
}
