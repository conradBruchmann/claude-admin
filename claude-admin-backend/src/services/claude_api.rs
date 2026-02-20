use crate::domain::errors::ApiError;
use claude_admin_shared::{ClaudeFileType, SuggestionRequest, SuggestionResponse};

#[derive(Clone)]
pub struct AnthropicClient {
    token: String,
    auth_mode: AuthMode,
    client: reqwest::Client,
}

#[derive(Clone)]
enum AuthMode {
    ApiKey,
    OAuthBearer,
}

impl AnthropicClient {
    /// Try to create a client: first check ANTHROPIC_API_KEY env var,
    /// then fall back to reading the OAuth token from the macOS Keychain.
    pub fn from_env() -> Option<Self> {
        // 1. Explicit API key
        if let Ok(api_key) = std::env::var("ANTHROPIC_API_KEY") {
            tracing::info!("Using ANTHROPIC_API_KEY for Claude API");
            return Some(Self {
                token: api_key,
                auth_mode: AuthMode::ApiKey,
                client: reqwest::Client::new(),
            });
        }

        // 2. OAuth token from macOS Keychain (Claude Code subscription)
        if let Some(oauth_token) = read_oauth_from_keychain() {
            tracing::info!("Using OAuth token from macOS Keychain for Claude API");
            return Some(Self {
                token: oauth_token,
                auth_mode: AuthMode::OAuthBearer,
                client: reqwest::Client::new(),
            });
        }

        tracing::warn!("No Claude API credentials found (no ANTHROPIC_API_KEY, no Keychain token)");
        None
    }
}

/// Read the Claude OAuth access token from the macOS Keychain.
fn read_oauth_from_keychain() -> Option<String> {
    let output = std::process::Command::new("security")
        .args([
            "find-generic-password",
            "-s",
            "Claude Code-credentials",
            "-w",
        ])
        .output()
        .ok()?;

    if !output.status.success() {
        tracing::debug!("Keychain lookup for 'Claude Code-credentials' failed");
        return None;
    }

    let raw = String::from_utf8(output.stdout).ok()?;
    let raw = raw.trim();

    // Parse the JSON to extract the OAuth access token
    let json: serde_json::Value = serde_json::from_str(raw).ok()?;
    let access_token = json
        .get("claudeAiOauth")
        .and_then(|o| o.get("accessToken"))
        .and_then(|t| t.as_str())?;

    Some(access_token.to_string())
}

pub async fn get_suggestions(
    client: &AnthropicClient,
    req: &SuggestionRequest,
) -> Result<SuggestionResponse, ApiError> {
    let system_prompt = build_system_prompt(&req.file_type);
    let user_prompt = format!(
        "Analyze and suggest improvements for this {}:\n\n```\n{}\n```\n\n{}",
        file_type_label(&req.file_type),
        req.content,
        req.context.as_deref().unwrap_or("")
    );

    let response = call_claude(client, &system_prompt, &user_prompt).await?;
    parse_suggestion_response(&response)
}

pub async fn validate_content(
    client: &AnthropicClient,
    req: &SuggestionRequest,
) -> Result<SuggestionResponse, ApiError> {
    let system_prompt = format!(
        "You are a Claude Code configuration validator. Check the following {} for issues, \
         best practices violations, and potential problems. Return a JSON object with keys: \
         'validation_issues' (array of strings), 'suggestions' (array of strings).",
        file_type_label(&req.file_type)
    );

    let user_prompt = format!("Validate this content:\n\n```\n{}\n```", req.content);

    let response = call_claude(client, &system_prompt, &user_prompt).await?;
    parse_suggestion_response(&response)
}

pub async fn call_claude_raw(
    client: &AnthropicClient,
    system: &str,
    user: &str,
) -> Result<String, ApiError> {
    call_claude(client, system, user).await
}

async fn call_claude(
    client: &AnthropicClient,
    system: &str,
    user: &str,
) -> Result<String, ApiError> {
    let body = serde_json::json!({
        "model": "claude-sonnet-4-20250514",
        "max_tokens": 4096,
        "system": system,
        "messages": [
            {"role": "user", "content": user}
        ]
    });

    let mut req = client
        .client
        .post("https://api.anthropic.com/v1/messages")
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json");

    // Use the right auth header depending on mode
    req = match &client.auth_mode {
        AuthMode::ApiKey => req.header("x-api-key", &client.token),
        AuthMode::OAuthBearer => req
            .header("Authorization", format!("Bearer {}", client.token))
            .header("anthropic-beta", "oauth-2025-04-20"),
    };

    let resp = req
        .json(&body)
        .send()
        .await
        .map_err(|e| ApiError::Internal(format!("Claude API request failed: {}", e)))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();

        // If OAuth token expired, hint at restarting
        if status.as_u16() == 401 {
            return Err(ApiError::Internal(
                "Claude API: Token abgelaufen oder ungültig. Starte ClaudeAdmin neu um einen frischen Token zu laden.".to_string()
            ));
        }

        return Err(ApiError::Internal(format!(
            "Claude API error {}: {}",
            status, text
        )));
    }

    let json: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to parse Claude response: {}", e)))?;

    let text = json["content"][0]["text"]
        .as_str()
        .unwrap_or("")
        .to_string();

    Ok(text)
}

fn build_system_prompt(file_type: &ClaudeFileType) -> String {
    match file_type {
        ClaudeFileType::Skill => {
            "You are an expert at writing Claude Code skills. Skills are markdown files with YAML \
             frontmatter (description, user_invocable). Suggest improvements for clarity, \
             completeness, and effectiveness. Return JSON with 'suggestions' (array of strings), \
             'improved_content' (string with the improved skill), and 'validation_issues' (array)."
                .to_string()
        }
        ClaudeFileType::Rule => {
            "You are an expert at writing Claude Code rules. Rules guide Claude's behavior in \
             specific contexts. Suggest improvements for specificity and actionability. Return JSON \
             with 'suggestions', 'improved_content', and 'validation_issues'."
                .to_string()
        }
        ClaudeFileType::ClaudeMd => {
            "You are an expert at writing CLAUDE.md files for Claude Code projects. These files \
             provide project context, coding conventions, and instructions. Return JSON with \
             'suggestions', 'improved_content', and 'validation_issues'."
                .to_string()
        }
        ClaudeFileType::Memory => {
            "You are an expert at writing Claude Code memory files. These persist knowledge across \
             sessions. Suggest improvements for organization and usefulness. Return JSON with \
             'suggestions', 'improved_content', and 'validation_issues'."
                .to_string()
        }
        _ => {
            "You are an expert at Claude Code configuration. Analyze and suggest improvements. \
             Return JSON with 'suggestions', 'improved_content', and 'validation_issues'."
                .to_string()
        }
    }
}

fn file_type_label(file_type: &ClaudeFileType) -> &str {
    match file_type {
        ClaudeFileType::Skill => "skill",
        ClaudeFileType::Rule => "rule",
        ClaudeFileType::ClaudeMd => "CLAUDE.md",
        ClaudeFileType::Memory => "memory file",
        ClaudeFileType::Settings => "settings",
        ClaudeFileType::Plan => "plan",
    }
}

fn parse_suggestion_response(text: &str) -> Result<SuggestionResponse, ApiError> {
    // Try to parse as JSON
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(text) {
        return Ok(SuggestionResponse {
            suggestions: json["suggestions"]
                .as_array()
                .map(|a| {
                    a.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default(),
            improved_content: json["improved_content"].as_str().map(String::from),
            validation_issues: json["validation_issues"]
                .as_array()
                .map(|a| {
                    a.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default(),
        });
    }

    // If not JSON, treat entire response as a suggestion
    Ok(SuggestionResponse {
        suggestions: vec![text.to_string()],
        improved_content: None,
        validation_issues: vec![],
    })
}
