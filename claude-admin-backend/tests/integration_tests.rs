use axum::body::Body;
use axum::http::{header, Method, Request, StatusCode};
use http_body_util::BodyExt;
use std::sync::{Arc, RwLock};
use tempfile::TempDir;
use tower::ServiceExt;

/// Helper: create an app backed by a temp directory.
async fn create_test_app() -> (axum::Router, TempDir) {
    let tmp = TempDir::new().unwrap();
    let claude_home = tmp.path().join(".claude");
    std::fs::create_dir_all(&claude_home).unwrap();

    // Create minimal ~/.claude.json
    let claude_json = tmp.path().join(".claude.json");
    std::fs::write(
        &claude_json,
        r#"{"projects":{"/tmp/test-project":{}},"mcpServers":{}}"#,
    )
    .unwrap();

    let (file_change_tx, _) = tokio::sync::broadcast::channel(100);

    let state = Arc::new(claude_admin_backend::app::AppState {
        config: claude_admin_backend::infra::config::Config {
            host: "127.0.0.1".to_string(),
            port: 9099,
            allowed_origins: vec![],
            anthropic_api_key: None,
        },
        claude_home: claude_home.clone(),
        claude_json_path: claude_json,
        claude_desktop_config_path: None,
        anthropic_client: Arc::new(RwLock::new(None)),
        token_store: claude_admin_backend::infra::auth::TokenStore::new(8),
        rate_limiter: claude_admin_backend::infra::rate_limit::create_rate_limiter(),
        file_change_tx: Arc::new(file_change_tx),
    });

    // Build the router the same way as production but with test state
    use axum::routing::{delete, get, post};

    let api = axum::Router::new()
        .route(
            "/api/v1/health",
            get(claude_admin_backend::routes::health::health_check),
        )
        .route(
            "/api/v1/dashboard",
            get(claude_admin_backend::routes::dashboard::get_dashboard),
        )
        .route(
            "/api/v1/skills",
            get(claude_admin_backend::routes::skills::list_skills)
                .post(claude_admin_backend::routes::skills::create_skill),
        )
        .route(
            "/api/v1/skills/:scope/:name",
            get(claude_admin_backend::routes::skills::get_skill)
                .put(claude_admin_backend::routes::skills::update_skill)
                .delete(claude_admin_backend::routes::skills::delete_skill),
        )
        .route(
            "/api/v1/rules",
            get(claude_admin_backend::routes::rules::list_rules)
                .post(claude_admin_backend::routes::rules::create_rule),
        )
        .route(
            "/api/v1/rules/:scope/:name",
            get(claude_admin_backend::routes::rules::get_rule)
                .put(claude_admin_backend::routes::rules::update_rule)
                .delete(claude_admin_backend::routes::rules::delete_rule),
        )
        .route(
            "/api/v1/plans",
            get(claude_admin_backend::routes::plans::list_plans),
        )
        .route(
            "/api/v1/plans/:name",
            get(claude_admin_backend::routes::plans::get_plan)
                .put(claude_admin_backend::routes::plans::update_plan)
                .delete(claude_admin_backend::routes::plans::delete_plan),
        )
        .route(
            "/api/v1/memory/:project",
            get(claude_admin_backend::routes::memory::get_memory)
                .put(claude_admin_backend::routes::memory::put_memory),
        )
        .route(
            "/api/v1/memory/:project/topics/:name",
            get(claude_admin_backend::routes::memory::get_topic)
                .put(claude_admin_backend::routes::memory::put_topic),
        )
        .route(
            "/api/v1/settings/global",
            get(claude_admin_backend::routes::settings::get_global_settings)
                .put(claude_admin_backend::routes::settings::put_global_settings),
        )
        .route(
            "/api/v1/mcp",
            get(claude_admin_backend::routes::mcp::list_mcp_servers)
                .post(claude_admin_backend::routes::mcp::create_mcp_server),
        )
        .route(
            "/api/v1/mcp/:name",
            delete(claude_admin_backend::routes::mcp::delete_mcp_server),
        )
        .route(
            "/api/v1/backups",
            get(claude_admin_backend::routes::backups::list_backups),
        )
        .route(
            "/api/v1/backups/:name",
            delete(claude_admin_backend::routes::backups::delete_backup),
        )
        .route(
            "/api/v1/export",
            get(claude_admin_backend::routes::export::export_bundle),
        )
        .route(
            "/api/v1/import",
            post(claude_admin_backend::routes::export::import_bundle),
        )
        .route(
            "/api/v1/search",
            get(claude_admin_backend::routes::search::search),
        )
        .route(
            "/api/v1/templates",
            get(claude_admin_backend::routes::templates::list_templates),
        )
        .route(
            "/api/v1/templates/:name/apply",
            post(claude_admin_backend::routes::templates::apply_template),
        )
        .route(
            "/api/v1/permissions",
            get(claude_admin_backend::routes::permissions::list_permissions),
        )
        .route(
            "/api/v1/permissions/:project_id",
            get(claude_admin_backend::routes::permissions::get_project_permissions),
        )
        .route(
            "/api/v1/permissions/:project_id/optimize",
            get(claude_admin_backend::routes::permissions::optimize_permissions),
        )
        .route(
            "/api/v1/sessions",
            get(claude_admin_backend::routes::sessions::list_sessions),
        )
        .route(
            "/api/v1/sessions/search",
            get(claude_admin_backend::routes::sessions::search_history),
        )
        .route(
            "/api/v1/analytics/overview",
            get(claude_admin_backend::routes::analytics::get_analytics_overview),
        )
        .route(
            "/api/v1/analytics/export",
            get(claude_admin_backend::routes::analytics::export_analytics),
        )
        // Budgets
        .route(
            "/api/v1/budgets",
            get(claude_admin_backend::routes::budgets::get_budget_status)
                .put(claude_admin_backend::routes::budgets::update_budget),
        )
        // Webhooks
        .route(
            "/api/v1/webhooks",
            get(claude_admin_backend::routes::webhooks::list_webhooks)
                .post(claude_admin_backend::routes::webhooks::create_webhook),
        )
        .route(
            "/api/v1/webhooks/:id",
            axum::routing::put(claude_admin_backend::routes::webhooks::update_webhook)
                .delete(claude_admin_backend::routes::webhooks::delete_webhook),
        )
        // Audit
        .route(
            "/api/v1/audit",
            get(claude_admin_backend::routes::audit::get_audit_log),
        )
        // SSE Events
        .route(
            "/api/v1/events",
            get(claude_admin_backend::routes::events::sse_events),
        )
        .fallback(claude_admin_backend::app::serve_frontend_test)
        .layer(axum::middleware::from_fn(
            claude_admin_backend::app::block_path_traversal,
        ))
        .layer(axum::middleware::from_fn(
            claude_admin_backend::app::security_headers,
        ))
        .with_state(state);

    (api, tmp)
}

// Note: Auth middleware tests are not included in integration tests because
// they rely on env vars (CLAUDE_ADMIN_TOKEN) which cause race conditions
// in parallel test execution. The auth middleware is tested manually.

async fn body_string(resp: axum::response::Response) -> String {
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    String::from_utf8(bytes.to_vec()).unwrap()
}

// ================================================================
// Health Check
// ================================================================

#[tokio::test]
async fn test_health_check() {
    let (app, _tmp) = create_test_app().await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_string(resp).await;
    assert!(body.contains("\"status\":\"ok\""));
}

// ================================================================
// Security Headers
// ================================================================

#[tokio::test]
async fn test_security_headers_present() {
    let (app, _tmp) = create_test_app().await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        resp.headers().get("x-content-type-options").unwrap(),
        "nosniff"
    );
    assert_eq!(resp.headers().get("x-frame-options").unwrap(), "DENY");
    assert!(resp
        .headers()
        .get("content-security-policy")
        .unwrap()
        .to_str()
        .unwrap()
        .contains("default-src 'self'"));
}

// ================================================================
// Path Traversal Blocking (BUG-v2-001)
// ================================================================

#[tokio::test]
async fn test_path_traversal_blocked_in_url() {
    let (app, _tmp) = create_test_app().await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/skills/../../../etc/passwd")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    let body = body_string(resp).await;
    assert!(body.contains("directory traversal"));
}

#[tokio::test]
async fn test_path_traversal_blocked_double_dots() {
    let (app, _tmp) = create_test_app().await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/../api/v1/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

// ================================================================
// Authentication Middleware (SEC-v2-001)
// ================================================================

// Auth middleware tests omitted: env var CLAUDE_ADMIN_TOKEN causes race
// conditions in parallel tests. The middleware is verified via UAT.

// ================================================================
// API Catch-All: JSON 404
// ================================================================

#[tokio::test]
async fn test_api_catch_all_returns_json_404() {
    let (app, _tmp) = create_test_app().await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/nonexistent")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    let body = body_string(resp).await;
    assert!(body.contains("\"error\""));
}

// ================================================================
// Name Validation
// ================================================================

#[tokio::test]
async fn test_validation_path_traversal() {
    let (app, _tmp) = create_test_app().await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/skills/global/..%2F..%2Fetc%2Fpasswd")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_validation_null_byte() {
    let (app, _tmp) = create_test_app().await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/rules/global/test%00evil")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_validation_empty_name() {
    let (app, _tmp) = create_test_app().await;
    let body = serde_json::json!({
        "name": "",
        "scope": "global",
        "content": "test"
    });
    let resp = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/rules")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_validation_too_long_name() {
    let (app, _tmp) = create_test_app().await;
    let long_name = "a".repeat(200);
    let body = serde_json::json!({
        "name": long_name,
        "scope": "global",
        "content": "test"
    });
    let resp = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/rules")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

// ================================================================
// Content-Type Check
// ================================================================

#[tokio::test]
async fn test_content_type_required_for_json_body() {
    let (app, _tmp) = create_test_app().await;
    let resp = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/skills")
                .body(Body::from(
                    r#"{"name":"test","scope":"global","frontmatter":{},"content":"x"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        resp.status() == StatusCode::UNSUPPORTED_MEDIA_TYPE
            || resp.status() == StatusCode::BAD_REQUEST
    );
}

// ================================================================
// Dashboard
// ================================================================

#[tokio::test]
async fn test_dashboard_returns_stats() {
    let (app, _tmp) = create_test_app().await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/dashboard")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_string(resp).await;
    assert!(body.contains("global_skills_count"));
    assert!(body.contains("global_rules_count"));
    assert!(body.contains("projects_count"));
}

// ================================================================
// Skills CRUD
// ================================================================

#[tokio::test]
async fn test_skills_list_empty() {
    let (app, _tmp) = create_test_app().await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/skills")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_string(resp).await;
    assert_eq!(body, "[]");
}

#[tokio::test]
async fn test_skills_crud() {
    let (app, _tmp) = create_test_app().await;
    let body = serde_json::json!({
        "name": "test-skill",
        "scope": "global",
        "frontmatter": {"description": "A test skill"},
        "content": "# Test Skill\nHello"
    });

    // Create
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/skills")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Get
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/skills/global/test-skill")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // List
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/skills")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_string(resp).await;
    assert!(body.contains("test-skill"));

    // Delete
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::DELETE)
                .uri("/api/v1/skills/global/test-skill")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

// ================================================================
// Rules CRUD
// ================================================================

#[tokio::test]
async fn test_rules_list_empty() {
    let (app, _tmp) = create_test_app().await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/rules")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_rules_crud() {
    let (app, _tmp) = create_test_app().await;
    let body = serde_json::json!({
        "name": "test-rule",
        "scope": "global",
        "content": "Always use tests."
    });

    // Create
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/rules")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Get
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/rules/global/test-rule")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Update
    let update = serde_json::json!({"content": "Updated rule."});
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::PUT)
                .uri("/api/v1/rules/global/test-rule")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&update).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Delete
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::DELETE)
                .uri("/api/v1/rules/global/test-rule")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

// ================================================================
// MCP Create: Wrapper + Flat Format
// ================================================================

#[tokio::test]
async fn test_mcp_create_wrapper_format() {
    let (app, _tmp) = create_test_app().await;
    let body = serde_json::json!({
        "name": "test-mcp",
        "config": {
            "command": "echo",
            "args": ["hello"]
        }
    });

    let resp = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/mcp")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_mcp_create_flat_format() {
    let (app, _tmp) = create_test_app().await;
    let body = serde_json::json!({
        "name": "test-mcp-flat",
        "command": "echo",
        "args": ["hello"],
        "env": {}
    });

    let resp = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/mcp")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_mcp_crud_full_lifecycle() {
    let (app, _tmp) = create_test_app().await;

    // Create
    let body = serde_json::json!({
        "name": "lifecycle-mcp",
        "command": "echo",
        "args": ["test"],
        "env": {}
    });
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/mcp")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // List - should contain the new server
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/mcp")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_string(resp).await;
    assert!(body.contains("lifecycle-mcp"));

    // Delete
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::DELETE)
                .uri("/api/v1/mcp/lifecycle-mcp")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

// ================================================================
// Plans CRUD
// ================================================================

#[tokio::test]
async fn test_plans_list_empty() {
    let (app, _tmp) = create_test_app().await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/plans")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_string(resp).await;
    assert_eq!(body, "[]");
}

#[tokio::test]
async fn test_plans_not_found() {
    let (app, _tmp) = create_test_app().await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/plans/nonexistent")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

// ================================================================
// Memory
// ================================================================

#[tokio::test]
async fn test_memory_get_returns_response() {
    let (app, tmp) = create_test_app().await;

    // Use the dash-encoding used in ~/.claude/projects/
    let encoded_dir_name = "-tmp-test-project";

    // Create the project memory directory
    let memory_dir = tmp
        .path()
        .join(".claude/projects")
        .join(encoded_dir_name)
        .join("memory");
    std::fs::create_dir_all(&memory_dir).unwrap();
    std::fs::write(memory_dir.join("MEMORY.md"), "# Test Memory").unwrap();

    // Encode the path as base64url for the API URL
    let encoded_id =
        claude_admin_backend::services::project_resolver::encode_project_id("/tmp/test-project");

    let resp = app
        .oneshot(
            Request::builder()
                .uri(&format!("/api/v1/memory/{}", encoded_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_string(resp).await;
    assert!(body.contains("MEMORY.md"));
}

// ================================================================
// Settings GET/PUT
// ================================================================

#[tokio::test]
async fn test_settings_get() {
    let (app, _tmp) = create_test_app().await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/settings/global")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_settings_put() {
    let (app, _tmp) = create_test_app().await;
    let body = serde_json::json!({
        "settings": {"permissions": {"allow": []}}
    });

    let resp = app
        .oneshot(
            Request::builder()
                .method(Method::PUT)
                .uri("/api/v1/settings/global")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

// ================================================================
// Sessions
// ================================================================

#[tokio::test]
async fn test_sessions_list_empty() {
    let (app, _tmp) = create_test_app().await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/sessions?offset=0&limit=5")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_string(resp).await;
    assert!(body.contains("\"sessions\""));
    assert!(body.contains("\"total\""));
}

#[tokio::test]
async fn test_sessions_search_empty() {
    let (app, _tmp) = create_test_app().await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/sessions/search?q=test&limit=5")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

// ================================================================
// Analytics
// ================================================================

#[tokio::test]
async fn test_analytics_overview() {
    let (app, _tmp) = create_test_app().await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/analytics/overview")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_string(resp).await;
    assert!(body.contains("total_sessions"));
    assert!(body.contains("estimated_total_cost_usd"));
}

// ================================================================
// Permissions
// ================================================================

#[tokio::test]
async fn test_permissions_list() {
    let (app, _tmp) = create_test_app().await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/permissions")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_permissions_optimize_nonexistent() {
    let (app, _tmp) = create_test_app().await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/permissions/nonexistent/optimize")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    // Should return OK with empty optimization or a meaningful error
    assert!(
        resp.status() == StatusCode::OK
            || resp.status() == StatusCode::NOT_FOUND
            || resp.status() == StatusCode::BAD_REQUEST
    );
}

// ================================================================
// Error Response Format
// ================================================================

#[tokio::test]
async fn test_error_response_is_json() {
    let (app, _tmp) = create_test_app().await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/skills/global/nonexistent")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    let body = body_string(resp).await;
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert!(json.get("error").is_some());
}

#[tokio::test]
async fn test_error_no_internal_details() {
    let (app, _tmp) = create_test_app().await;
    let resp = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/skills")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(r#"{"invalid json"#))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = body_string(resp).await;
    assert!(!body.contains("line"));
    assert!(!body.contains("column"));
}

// ================================================================
// Backups
// ================================================================

#[tokio::test]
async fn test_backups_list_empty() {
    let (app, _tmp) = create_test_app().await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/backups")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_string(resp).await;
    assert_eq!(body, "[]");
}

#[tokio::test]
async fn test_backup_delete_nonexistent() {
    let (app, _tmp) = create_test_app().await;
    let resp = app
        .oneshot(
            Request::builder()
                .method(Method::DELETE)
                .uri("/api/v1/backups/nonexistent-backup")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

// ================================================================
// Export / Import
// ================================================================

#[tokio::test]
async fn test_export_bundle() {
    let (app, _tmp) = create_test_app().await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/export")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_string(resp).await;
    assert!(body.contains("\"version\""));
    assert!(body.contains("\"exported_at\""));
}

#[tokio::test]
async fn test_import_empty_bundle() {
    let (app, _tmp) = create_test_app().await;
    let body = serde_json::json!({
        "version": "1.0",
        "exported_at": "2026-01-01T00:00:00Z",
        "skills": [],
        "rules": [],
        "settings": {},
        "mcp_servers": []
    });
    let resp = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/import")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_string(resp).await;
    assert!(body.contains("skills_imported"));
}

// ================================================================
// Search
// ================================================================

#[tokio::test]
async fn test_search_empty_query() {
    let (app, _tmp) = create_test_app().await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/search?q=")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_string(resp).await;
    assert_eq!(body, "[]");
}

#[tokio::test]
async fn test_search_with_term_returns_ok() {
    let (app, _tmp) = create_test_app().await;

    // Search for any term - should return OK with valid JSON array
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/search?q=test")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_string(resp).await;
    // Should be a valid JSON array (empty or not)
    let _: Vec<serde_json::Value> = serde_json::from_str(&body).unwrap();
}

// ================================================================
// Templates
// ================================================================

#[tokio::test]
async fn test_templates_list() {
    let (app, _tmp) = create_test_app().await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/templates")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_string(resp).await;
    assert!(body.contains("rust-developer"));
    assert!(body.contains("fullstack-js"));
}

#[tokio::test]
async fn test_template_apply() {
    let (app, _tmp) = create_test_app().await;
    let resp = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/templates/rust-developer/apply")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_string(resp).await;
    assert!(body.contains("rules_created"));
}

// ================================================================
// GitHub Username Parsing (unit tests)
// ================================================================

#[test]
fn test_github_username_unit() {
    let json = r#"{"name":"test","command":"echo","args":["hello"]}"#;
    let req: claude_admin_shared::McpServerCreateRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.name, "test");
    assert!(req.config.get("command").is_some());
}

#[test]
fn test_mcp_create_request_wrapper_format() {
    let json = r#"{"name":"test","config":{"command":"echo"}}"#;
    let req: claude_admin_shared::McpServerCreateRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.name, "test");
    assert!(req.config.get("command").is_some());
}

// ================================================================
// Budget Validation (GAP-v4-008)
// ================================================================

#[tokio::test]
async fn test_budget_reject_negative_daily() {
    let (app, _tmp) = create_test_app().await;
    let body = serde_json::json!({
        "daily_budget_usd": -10.0,
        "weekly_budget_usd": null,
        "monthly_budget_usd": null
    });

    let resp = app
        .oneshot(
            Request::builder()
                .method(Method::PUT)
                .uri("/api/v1/budgets")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    let body = body_string(resp).await;
    assert!(body.contains("negative"));
}

#[tokio::test]
async fn test_budget_reject_negative_weekly() {
    let (app, _tmp) = create_test_app().await;
    let body = serde_json::json!({
        "daily_budget_usd": null,
        "weekly_budget_usd": -5.0,
        "monthly_budget_usd": null
    });

    let resp = app
        .oneshot(
            Request::builder()
                .method(Method::PUT)
                .uri("/api/v1/budgets")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    let body = body_string(resp).await;
    assert!(body.contains("negative"));
}

#[tokio::test]
async fn test_budget_accept_zero() {
    let (app, _tmp) = create_test_app().await;
    let body = serde_json::json!({
        "daily_budget_usd": 0.0,
        "weekly_budget_usd": null,
        "monthly_budget_usd": null
    });

    let resp = app
        .oneshot(
            Request::builder()
                .method(Method::PUT)
                .uri("/api/v1/budgets")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
}

// ================================================================
// Webhook CRUD (GAP-v4-006)
// ================================================================

#[tokio::test]
async fn test_webhook_crud() {
    let (app, _tmp) = create_test_app().await;

    // Create webhook
    let body = serde_json::json!({
        "url": "https://example.com/webhook",
        "events": ["skill.created", "rule.updated"]
    });
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/webhooks")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let resp_body = body_string(resp).await;
    let created: serde_json::Value = serde_json::from_str(&resp_body).unwrap();
    let webhook_id = created["id"].as_str().unwrap().to_string();

    // List webhooks — should contain the new one
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/webhooks")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let resp_body = body_string(resp).await;
    assert!(resp_body.contains(&webhook_id));

    // Update webhook
    let update_body = serde_json::json!({
        "url": "https://example.com/webhook-v2",
        "active": false
    });
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::PUT)
                .uri(&format!("/api/v1/webhooks/{}", webhook_id))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&update_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let resp_body = body_string(resp).await;
    assert!(resp_body.contains("webhook-v2"));

    // Delete webhook
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::DELETE)
                .uri(&format!("/api/v1/webhooks/{}", webhook_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Verify deletion — list should be empty
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/webhooks")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let resp_body = body_string(resp).await;
    assert_eq!(resp_body, "[]");
}

// ================================================================
// Audit Log (GAP-v4-006)
// ================================================================

#[tokio::test]
async fn test_audit_log_after_writes() {
    let (app, _tmp) = create_test_app().await;

    // Create a skill to generate an audit entry
    let body = serde_json::json!({
        "name": "audit-test-skill",
        "scope": "global",
        "frontmatter": {"description": "audit test"},
        "content": "# Audit Test"
    });
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/skills")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Check audit log
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/audit?limit=10")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body_str = body_string(resp).await;
    assert!(body_str.contains("\"entries\""));
}

// ================================================================
// Export Format Validation (GAP-v4-006)
// ================================================================

#[tokio::test]
async fn test_export_format_invalid() {
    let (app, _tmp) = create_test_app().await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/analytics/export?format=xml")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    let body = body_string(resp).await;
    assert!(body.contains("Unsupported"));
}

#[tokio::test]
async fn test_export_format_csv() {
    let (app, _tmp) = create_test_app().await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/analytics/export?format=csv")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(resp.headers().get("content-type").unwrap(), "text/csv");
}

// ================================================================
// SSE Events Endpoint (GAP-v4-007)
// ================================================================

#[tokio::test]
async fn test_sse_events_endpoint() {
    let (app, _tmp) = create_test_app().await;
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/events")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let content_type = resp
        .headers()
        .get("content-type")
        .unwrap()
        .to_str()
        .unwrap();
    assert!(content_type.contains("text/event-stream"));
}
