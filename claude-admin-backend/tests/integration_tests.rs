use axum::body::Body;
use axum::http::{header, Method, Request, StatusCode};
use http_body_util::BodyExt;
use std::path::PathBuf;
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
    });

    // Build the router the same way as production but with test state
    use axum::routing::{delete, get, post};

    let api = axum::Router::new()
        .route(
            "/api/v1/health",
            get(claude_admin_backend::routes::health::health_check),
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
            "/api/v1/backups",
            get(claude_admin_backend::routes::backups::list_backups),
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
            "/api/v1/permissions/:project_id/optimize",
            get(claude_admin_backend::routes::permissions::optimize_permissions),
        )
        .fallback(claude_admin_backend::app::serve_frontend_test)
        .layer(axum::middleware::from_fn(
            claude_admin_backend::app::security_headers,
        ))
        .with_state(state);

    (api, tmp)
}

async fn body_string(resp: axum::response::Response) -> String {
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    String::from_utf8(bytes.to_vec()).unwrap()
}

// === Health Check ===

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

// === Security Headers ===

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

// === API Catch-All: JSON 404 ===

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

// === Name Validation ===

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

// === Content-Type check ===

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

    // Should be 415 or 400 without Content-Type
    assert!(
        resp.status() == StatusCode::UNSUPPORTED_MEDIA_TYPE
            || resp.status() == StatusCode::BAD_REQUEST
    );
}

// === Skills CRUD ===

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

// === Rules CRUD ===

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

// === MCP Create: Wrapper + Flat Format ===

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

// === Plans CRUD ===

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

// === Settings GET/PUT ===

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

// === Error Response Format ===

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
    // Should not contain internal details like line/column numbers
    assert!(!body.contains("line"));
    assert!(!body.contains("column"));
}

// === Backups ===

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

// === Export ===

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

// === Search ===

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

// === Templates ===

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

// === GitHub Username Parsing (unit test) ===

#[test]
fn test_github_username_unit() {
    // This tests the shared crate type deserialization
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
