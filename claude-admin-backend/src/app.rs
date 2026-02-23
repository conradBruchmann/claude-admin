use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::{
    routing::{delete, get, post},
    Json, Router,
};
use rust_embed::Embed;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use tokio::sync::broadcast;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};

use crate::infra::auth::TokenStore;
use crate::infra::rate_limit::{create_rate_limiter, RateLimiter};
use crate::infra::{config::Config, cors::create_cors_layer};
use crate::routes;
use crate::services::claude_api::AnthropicClient;
use crate::services::watcher::FileChangeEvent;

#[derive(Clone)]
#[allow(dead_code)]
pub struct AppState {
    pub config: Config,
    pub claude_home: PathBuf,
    pub claude_json_path: PathBuf,
    pub claude_desktop_config_path: Option<PathBuf>,
    pub anthropic_client: Arc<RwLock<Option<AnthropicClient>>>,
    pub token_store: TokenStore,
    pub rate_limiter: RateLimiter,
    pub file_change_tx: Arc<broadcast::Sender<FileChangeEvent>>,
}

#[derive(Embed)]
#[folder = "../claude-admin-frontend/dist/"]
struct FrontendAssets;

/// Middleware that blocks requests containing ".." path segments (path traversal).
pub async fn block_path_traversal(request: Request<Body>, next: Next) -> Response {
    let path = request.uri().path();
    if path.contains("..") {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "Invalid path: directory traversal not allowed" })),
        )
            .into_response();
    }
    next.run(request).await
}

/// Middleware that enforces Bearer token authentication when CLAUDE_ADMIN_TOKEN is set.
/// Supports both master tokens and session tokens.
pub async fn auth_middleware(request: Request<Body>, next: Next) -> Response {
    let token = std::env::var("CLAUDE_ADMIN_TOKEN").unwrap_or_default();
    if token.is_empty() {
        return next.run(request).await;
    }

    // Exempt health check endpoint
    if request.uri().path() == "/api/v1/health" {
        return next.run(request).await;
    }

    // Exempt non-API paths (frontend assets)
    if !request.uri().path().starts_with("/api/") {
        return next.run(request).await;
    }

    // Exempt login endpoint
    if request.uri().path() == "/api/v1/auth/login" {
        return next.run(request).await;
    }

    // Exempt docs endpoints
    if request.uri().path().starts_with("/api/v1/docs") {
        return next.run(request).await;
    }

    // Get token store from extensions
    let token_store = request.extensions().get::<TokenStore>().cloned();

    match request.headers().get(header::AUTHORIZATION) {
        Some(auth_value) => {
            if let Ok(auth_str) = auth_value.to_str() {
                let bearer = auth_str.strip_prefix("Bearer ").unwrap_or(auth_str);

                // Check master token
                if bearer == token {
                    return next.run(request).await;
                }

                // Check session token
                if let Some(ref store) = token_store {
                    if store.validate(bearer) {
                        return next.run(request).await;
                    }
                }
            }
            (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({ "error": "Invalid authentication token" })),
            )
                .into_response()
        }
        None => (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({ "error": "Authentication required. Set Authorization: Bearer <token> header." })),
        )
            .into_response(),
    }
}

/// Middleware that adds security headers to every response.
pub async fn security_headers(request: Request<Body>, next: Next) -> Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();
    headers.insert(header::X_CONTENT_TYPE_OPTIONS, "nosniff".parse().unwrap());
    headers.insert(header::X_FRAME_OPTIONS, "DENY".parse().unwrap());
    headers.insert(
        header::CONTENT_SECURITY_POLICY,
        "default-src 'self'; style-src 'self' 'unsafe-inline'; script-src 'self' 'unsafe-inline' 'wasm-unsafe-eval'; connect-src 'self' https://unpkg.com"
            .parse()
            .unwrap(),
    );
    response
}

/// Login route: exchange master token for a session token with TTL.
pub async fn login(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    crate::domain::extractors::AppJson(req): crate::domain::extractors::AppJson<
        claude_admin_shared::LoginRequest,
    >,
) -> Result<Json<claude_admin_shared::LoginResponse>, crate::domain::errors::ApiError> {
    let master_token = std::env::var("CLAUDE_ADMIN_TOKEN").unwrap_or_default();
    if master_token.is_empty() || req.token != master_token {
        return Err(crate::domain::errors::ApiError::Unauthorized(
            "Invalid token".to_string(),
        ));
    }

    let (session_token, expires_at) = state.token_store.create_session();
    Ok(Json(claude_admin_shared::LoginResponse {
        session_token,
        expires_at: expires_at.to_rfc3339(),
    }))
}

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

    let anthropic_client = AnthropicClient::from_env_or_config(&claude_home);
    let token_store = TokenStore::new(8); // 8 hour TTL
    let rate_limiter = create_rate_limiter();

    // File watcher broadcast channel
    let (file_change_tx, _) = broadcast::channel::<FileChangeEvent>(100);
    let file_change_tx = Arc::new(file_change_tx);

    // Start file watcher
    let _watcher = crate::services::watcher::start_watcher(
        claude_home.clone(),
        file_change_tx.clone(),
    );

    // Spawn background tasks
    crate::infra::auth::spawn_token_purge_task(token_store.clone());
    crate::services::backups::spawn_backup_prune_task(claude_home.clone());

    let state = Arc::new(AppState {
        config: config.clone(),
        claude_home,
        claude_json_path,
        claude_desktop_config_path,
        anthropic_client: Arc::new(RwLock::new(anthropic_client)),
        token_store,
        rate_limiter,
        file_change_tx,
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
        // Permissions & Config Health
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
        // Skill Browser
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
        // Settings Hierarchy & Hook Builder
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
        // API Key management
        .route(
            "/api/v1/settings/api-key",
            get(routes::settings::get_api_key_status).put(routes::settings::set_api_key),
        )
        // Analytics
        .route(
            "/api/v1/analytics/overview",
            get(routes::analytics::get_analytics_overview),
        )
        .route(
            "/api/v1/analytics/projects",
            get(routes::analytics::get_project_analytics),
        )
        .route(
            "/api/v1/analytics/export",
            get(routes::analytics::export_analytics),
        )
        // Sessions
        .route("/api/v1/sessions", get(routes::sessions::list_sessions))
        .route(
            "/api/v1/sessions/search",
            get(routes::sessions::search_history),
        )
        .route("/api/v1/sessions/:id", get(routes::sessions::get_session))
        .route(
            "/api/v1/sessions/:id/transcript",
            get(routes::sessions::get_transcript),
        )
        // System Info & GitHub
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
        )
        // Backups
        .route("/api/v1/backups", get(routes::backups::list_backups))
        .route(
            "/api/v1/backups/prune",
            post(routes::backups::prune_backups),
        )
        .route(
            "/api/v1/backups/:name/restore",
            post(routes::backups::restore_backup),
        )
        .route(
            "/api/v1/backups/:name/diff",
            get(routes::backups::get_backup_diff),
        )
        .route(
            "/api/v1/backups/:name",
            delete(routes::backups::delete_backup),
        )
        // Export/Import
        .route("/api/v1/export", get(routes::export::export_bundle))
        .route("/api/v1/import", post(routes::export::import_bundle))
        // Search
        .route("/api/v1/search", get(routes::search::search))
        // Templates
        .route("/api/v1/templates", get(routes::templates::list_templates))
        .route(
            "/api/v1/templates/:name/apply",
            post(routes::templates::apply_template),
        )
        // Permission Optimizer
        .route(
            "/api/v1/permissions/:project_id/optimize",
            get(routes::permissions::optimize_permissions),
        )
        // === New routes ===
        // Auth (login for session tokens)
        .route("/api/v1/auth/login", post(login))
        // Audit Log
        .route("/api/v1/audit", get(routes::audit::get_audit_log))
        // Budgets
        .route(
            "/api/v1/budgets",
            get(routes::budgets::get_budget_status).put(routes::budgets::update_budget),
        )
        // Preview (Markdown + Syntax Highlighting)
        .route(
            "/api/v1/preview/markdown",
            post(routes::preview::render_markdown),
        )
        .route(
            "/api/v1/preview/highlight",
            post(routes::preview::highlight_code),
        )
        // SSE Events
        .route("/api/v1/events", get(routes::events::sse_events))
        // Webhooks
        .route(
            "/api/v1/webhooks",
            get(routes::webhooks::list_webhooks).post(routes::webhooks::create_webhook),
        )
        .route(
            "/api/v1/webhooks/:id",
            axum::routing::put(routes::webhooks::update_webhook)
                .delete(routes::webhooks::delete_webhook),
        )
        // Sync
        .route(
            "/api/v1/sync/manifest",
            get(routes::sync::get_manifest),
        )
        .route("/api/v1/sync/push", post(routes::sync::push_files))
        .route("/api/v1/sync/pull", post(routes::sync::pull_files))
        .route("/api/v1/sync/file", get(routes::sync::get_file))
        .route(
            "/api/v1/sync/receive",
            post(routes::sync::receive_file),
        )
        // API Docs
        .route("/api/v1/docs", get(routes::docs::swagger_ui))
        .route(
            "/api/v1/docs/openapi.json",
            get(routes::docs::openapi_spec),
        );

    let app = Router::new()
        .merge(api_routes)
        .fallback(serve_frontend)
        .layer(middleware::from_fn(block_path_traversal))
        .layer(middleware::from_fn(auth_middleware))
        .layer(middleware::from_fn(security_headers))
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(create_cors_layer(&config.allowed_origins))
        .with_state(state);

    Ok(app)
}

/// Test-friendly version of serve_frontend that always returns JSON 404 for API paths.
pub async fn serve_frontend_test(uri: axum::http::Uri) -> Response {
    let path = uri.path().trim_start_matches('/');
    if path.starts_with("api/") {
        return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "API endpoint not found" })),
        )
            .into_response();
    }
    (StatusCode::NOT_FOUND, "Not Found").into_response()
}

/// Serve embedded frontend assets, falling back to index.html for SPA routing.
/// Returns JSON 404 for unmatched /api/ paths (BUG-004).
async fn serve_frontend(uri: axum::http::Uri) -> Response {
    let path = uri.path().trim_start_matches('/');

    // API catch-all: return JSON 404 for unmatched /api/ routes
    if path.starts_with("api/") {
        return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "API endpoint not found" })),
        )
            .into_response();
    }

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
