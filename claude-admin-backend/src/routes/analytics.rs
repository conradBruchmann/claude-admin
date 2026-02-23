use axum::extract::{Query, State};
use axum::http::{header, StatusCode};
use axum::response::Response;
use axum::Json;
use std::sync::Arc;

use crate::app::AppState;
use crate::domain::errors::ApiError;
use crate::services::analytics;
use claude_admin_shared::*;

#[derive(serde::Deserialize)]
pub struct AnalyticsQuery {
    pub from: Option<String>,
    pub to: Option<String>,
}

pub async fn get_analytics_overview(
    State(state): State<Arc<AppState>>,
    Query(query): Query<AnalyticsQuery>,
) -> Result<Json<AnalyticsOverview>, ApiError> {
    let mut overview = analytics::get_analytics_overview(&state.claude_home)?;

    // Apply date filtering if specified
    if query.from.is_some() || query.to.is_some() {
        let from = query.from.as_deref().unwrap_or("0000-00-00");
        let to = query.to.as_deref().unwrap_or("9999-99-99");

        overview
            .daily_activity
            .retain(|d| d.date.as_str() >= from && d.date.as_str() <= to);

        // Recalculate totals for filtered range
        let filtered_messages: u64 = overview
            .daily_activity
            .iter()
            .map(|d| d.message_count)
            .sum();
        let filtered_sessions: u64 = overview
            .daily_activity
            .iter()
            .map(|d| d.session_count)
            .sum();
        if filtered_messages > 0 {
            overview.total_messages = filtered_messages;
        }
        if filtered_sessions > 0 {
            overview.total_sessions = filtered_sessions;
        }
    }

    Ok(Json(overview))
}

pub async fn get_project_analytics(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<ProjectAnalytics>>, ApiError> {
    let projects = analytics::get_project_analytics(&state.claude_home, &state.claude_json_path)?;
    Ok(Json(projects))
}

#[derive(serde::Deserialize)]
pub struct ExportQuery {
    pub format: Option<String>,
}

pub async fn export_analytics(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ExportQuery>,
) -> Result<Response, ApiError> {
    let overview = analytics::get_analytics_overview(&state.claude_home)?;
    let format = query.format.as_deref().unwrap_or("json");

    match format {
        "csv" => {
            let mut csv = String::from("date,message_count,session_count,tool_call_count\n");
            for d in &overview.daily_activity {
                csv.push_str(&format!(
                    "{},{},{},{}\n",
                    d.date, d.message_count, d.session_count, d.tool_call_count
                ));
            }

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "text/csv")
                .header(
                    header::CONTENT_DISPOSITION,
                    "attachment; filename=\"analytics.csv\"",
                )
                .body(axum::body::Body::from(csv))
                .unwrap())
        }
        "json" => {
            let json = serde_json::to_string_pretty(&overview)
                .map_err(|e| ApiError::Internal(format!("Serialize error: {}", e)))?;

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/json")
                .header(
                    header::CONTENT_DISPOSITION,
                    "attachment; filename=\"analytics.json\"",
                )
                .body(axum::body::Body::from(json))
                .unwrap())
        }
        other => Err(ApiError::BadRequest(format!(
            "Unsupported export format '{}'. Use 'json' or 'csv'.",
            other
        ))),
    }
}
