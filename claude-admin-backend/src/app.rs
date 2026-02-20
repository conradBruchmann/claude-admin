use axum::body::Body;
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::{
    routing::{delete, get, post},
    Router,
};
use rust_embed::Embed;
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};

use crate::infra::{config::Config, cors::create_cors_layer};
use crate::routes;
use crate::services::claude_api::AnthropicClient;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub claude_home: PathBuf,
    pub claude_json_path: PathBuf,
    pub claude_desktop_config_path: Option<PathBuf>,
    pub anthropic_client: Option<AnthropicClient>,
}

#[derive(Embed)]
#[folder = "../claude-admin-frontend/dist/"]
struct FrontendAssets;

pub async fn create_app(config: Config) -> Result<Router, Box<dyn std::error::Error>> {
    let claude_home = dirs_home().join(".claude");
    let claude_json_path = dirs_home().join(".claude.json");

    let desktop_config =
        dirs_home().join("Library/Application Support/Claude/claude_desktop_config.json");
    let claude_desktop_config_path = if desktop_config.exists() {
        Some(desktop_config)
    } else {
        None
    };

    let anthropic_client = AnthropicClient::from_env();

    let state = Arc::new(AppState {
        config: config.clone(),
        claude_home,
        claude_json_path,
        claude_desktop_config_path,
        anthropic_client,
    });

    let api_routes = Router::new()
        .route("/api/v1/health", get(routes::health::health_check))
        .route("/api/v1/dashboard", get(routes::dashboard::get_dashboard))
        .route(
            "/api/v1/dashboard/health",
            get(routes::dashboard::get_dashboard_health),
        )
        .route("/api/v1/projects", get(routes::projects::list_projects))
        .route("/api/v1/projects/:id", get(routes::projects::get_project))
        .route(
            "/api/v1/projects/:id/status",
            get(routes::projects::get_project_status),
        )
        .route(
            "/api/v1/projects/:id/claude-md",
            get(routes::projects::get_claude_md).put(routes::projects::put_claude_md),
        )
        .route(
            "/api/v1/skills",
            get(routes::skills::list_skills).post(routes::skills::create_skill),
        )
        .route(
            "/api/v1/skills/:scope/:name",
            get(routes::skills::get_skill)
                .put(routes::skills::update_skill)
                .delete(routes::skills::delete_skill),
        )
        .route(
            "/api/v1/rules",
            get(routes::rules::list_rules).post(routes::rules::create_rule),
        )
        .route(
            "/api/v1/rules/:scope/:name",
            get(routes::rules::get_rule)
                .put(routes::rules::update_rule)
                .delete(routes::rules::delete_rule),
        )
        .route(
            "/api/v1/memory/:project",
            get(routes::memory::get_memory).put(routes::memory::put_memory),
        )
        .route(
            "/api/v1/memory/:project/topics/:name",
            get(routes::memory::get_topic).put(routes::memory::put_topic),
        )
        .route(
            "/api/v1/settings/global",
            get(routes::settings::get_global_settings).put(routes::settings::put_global_settings),
        )
        .route(
            "/api/v1/settings/claude-json",
            get(routes::settings::get_claude_json),
        )
        .route("/api/v1/plans", get(routes::plans::list_plans))
        .route(
            "/api/v1/plans/:name",
            get(routes::plans::get_plan)
                .put(routes::plans::update_plan)
                .delete(routes::plans::delete_plan),
        )
        .route(
            "/api/v1/projects/:id/advisor",
            get(routes::advisor::get_advisor_report),
        )
        .route(
            "/api/v1/ai/suggest",
            axum::routing::post(routes::ai::suggest),
        )
        .route(
            "/api/v1/ai/validate",
            axum::routing::post(routes::ai::validate),
        )
        // Phase 8: Permissions & Config Health
        .route(
            "/api/v1/permissions",
            get(routes::permissions::list_permissions),
        )
        .route(
            "/api/v1/permissions/:project_id",
            get(routes::permissions::get_project_permissions),
        )
        .route(
            "/api/v1/permissions/:project_id/entries",
            delete(routes::permissions::delete_permission_entries),
        )
        .route(
            "/api/v1/health/overview",
            get(routes::permissions::get_health_overview),
        )
        .route(
            "/api/v1/health/:project_id",
            get(routes::permissions::get_project_health),
        )
        // Phase 9: Skill Browser
        .route(
            "/api/v1/skill-browser/official",
            get(routes::skill_browser::list_official_skills),
        )
        .route(
            "/api/v1/skill-browser/community",
            get(routes::skill_browser::list_community_skills),
        )
        .route(
            "/api/v1/skill-browser/install",
            post(routes::skill_browser::install_skill),
        )
        // Phase 10: Settings Hierarchy & Hook Builder
        .route(
            "/api/v1/settings/hierarchy/:project_id",
            get(routes::settings::get_settings_hierarchy),
        )
        .route(
            "/api/v1/settings/hook-templates",
            get(routes::settings::get_hook_templates),
        )
        .route(
            "/api/v1/settings/storage",
            get(routes::settings::get_storage),
        )
        // Phase 11: Analytics
        .route(
            "/api/v1/analytics/overview",
            get(routes::analytics::get_analytics_overview),
        )
        .route(
            "/api/v1/analytics/projects",
            get(routes::analytics::get_project_analytics),
        )
        // Phase 12: Sessions
        .route("/api/v1/sessions", get(routes::sessions::list_sessions))
        .route(
            "/api/v1/sessions/search",
            get(routes::sessions::search_history),
        )
        .route("/api/v1/sessions/:id", get(routes::sessions::get_session))
        // Phase 13: System Info & GitHub
        .route(
            "/api/v1/system/info",
            get(routes::system_info::get_system_info),
        )
        .route(
            "/api/v1/system/storage",
            get(routes::system_info::get_storage_info),
        )
        .route("/api/v1/github", get(routes::github::get_github_overview))
        // Licenses
        .route("/api/v1/licenses", get(routes::licenses::get_licenses))
        // MCP Server Management
        .route(
            "/api/v1/mcp",
            get(routes::mcp::list_mcp_servers).post(routes::mcp::create_mcp_server),
        )
        .route("/api/v1/mcp/health", get(routes::mcp::health_check_all))
        .route(
            "/api/v1/mcp/:name",
            get(routes::mcp::get_mcp_server)
                .put(routes::mcp::update_mcp_server)
                .delete(routes::mcp::delete_mcp_server),
        )
        .route(
            "/api/v1/mcp/:name/health",
            get(routes::mcp::health_check_server),
        )
        .route("/api/v1/mcp-browser", get(routes::mcp::get_mcp_catalog))
        .route(
            "/api/v1/mcp-browser/install",
            post(routes::mcp::install_mcp_server),
        );

    let app = Router::new()
        .merge(api_routes)
        .fallback(serve_frontend)
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(create_cors_layer(&config.allowed_origins))
        .with_state(state);

    Ok(app)
}

/// Serve embedded frontend assets, falling back to index.html for SPA routing.
async fn serve_frontend(uri: axum::http::Uri) -> Response {
    let path = uri.path().trim_start_matches('/');

    // Try to serve the exact file
    if let Some(file) = FrontendAssets::get(path) {
        let mime = mime_guess::from_path(path)
            .first_or_octet_stream()
            .to_string();
        return Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, mime)
            .body(Body::from(file.data.to_vec()))
            .unwrap();
    }

    // SPA fallback: serve index.html for all non-API, non-file routes
    if let Some(index) = FrontendAssets::get("index.html") {
        return Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/html")
            .body(Body::from(index.data.to_vec()))
            .unwrap();
    }

    (StatusCode::NOT_FOUND, "Not Found").into_response()
}

fn dirs_home() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/tmp"))
}
