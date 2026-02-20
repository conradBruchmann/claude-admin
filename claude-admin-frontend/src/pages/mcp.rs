use claude_admin_shared::{
    McpHealthResult, McpServerCreateRequest, McpServerDetail, McpServerUpdateRequest,
};
use leptos::*;

use crate::api;
use crate::i18n::t;

#[component]
pub fn McpServersPage() -> impl IntoView {
    let active_tab = create_rw_signal("servers".to_string());

    view! {
        <div class="page-header">
            <h2>{t("mcp.title")}</h2>
            <p>{t("mcp.subtitle")}</p>
        </div>

        <div class="tabs">
            <button
                class=move || if active_tab.get() == "servers" { "tab active" } else { "tab" }
                on:click=move |_| active_tab.set("servers".to_string())
            >{t("mcp.tab_servers")}</button>
            <button
                class=move || if active_tab.get() == "health" { "tab active" } else { "tab" }
                on:click=move |_| active_tab.set("health".to_string())
            >{t("mcp.tab_health")}</button>
            <button
                class=move || if active_tab.get() == "add" { "tab active" } else { "tab" }
                on:click=move |_| active_tab.set("add".to_string())
            >{t("mcp.tab_add")}</button>
        </div>

        {move || match active_tab.get().as_str() {
            "servers" => view! { <ServersTab/> }.into_view(),
            "health" => view! { <HealthCheckTab/> }.into_view(),
            "add" => view! { <AddServerTab/> }.into_view(),
            _ => view! { <ServersTab/> }.into_view(),
        }}
    }
}

/// Return CSS for source badge color.
fn source_badge_style(source: &str) -> &'static str {
    if source == "claude_code" {
        "background: var(--success); color: #fff;"
    } else if source == "claude_desktop" {
        "background: var(--warning); color: #000;"
    } else {
        "background: var(--text-muted); color: #fff;"
    }
}

/// Return label for source badge.
fn source_badge_label(source: &str) -> String {
    if source == "claude_code" {
        "Claude Code".to_string()
    } else if source == "claude_desktop" {
        "Claude Desktop".to_string()
    } else if let Some(project) = source.strip_prefix("project:") {
        format!("Project: {}", project)
    } else {
        source.to_string()
    }
}

// ─────────────────────────────────────────────
// Servers Tab
// ─────────────────────────────────────────────

#[component]
fn ServersTab() -> impl IntoView {
    let servers = create_resource(
        || (),
        |_| async move { api::get::<Vec<McpServerDetail>>("/mcp").await },
    );
    let selected = create_rw_signal::<Option<String>>(None);
    let selected_source = create_rw_signal::<Option<String>>(None);
    let editor_content = create_rw_signal(String::new());
    let save_status = create_rw_signal::<Option<(String, bool)>>(None);

    view! {
        <Suspense fallback=move || view! { <div class="loading">{t("mcp.loading")}</div> }>
            {move || servers.get().map(|result| match result {
                Ok(list) => {
                    if list.is_empty() {
                        view! {
                            <div class="empty-state">
                                <p>{t("mcp.no_servers")}</p>
                                <p style="color: var(--text-muted); font-size: 0.875rem;">
                                    {t("mcp.no_servers_hint")}
                                </p>
                            </div>
                        }.into_view()
                    } else {
                        let list_clone = list.clone();
                        view! {
                            <div style="display: grid; grid-template-columns: 280px 1fr; gap: 1.5rem; min-height: 400px;">
                                // Left: server list
                                <div class="card" style="padding: 0; overflow: hidden;">
                                    <div style="padding: 0.75rem 1rem; border-bottom: 1px solid var(--border); font-weight: 600; font-size: 0.875rem;">
                                        {format!("{} Server{}", list.len(), if list.len() == 1 { "" } else { "s" })}
                                    </div>
                                    {list.into_iter().map(|s| {
                                        let name = s.name.clone();
                                        let name_click = s.name.clone();
                                        let source = s.source.clone();
                                        let source_click = s.source.clone();
                                        let config_json = serde_json::to_string_pretty(&s.raw_config).unwrap_or_default();
                                        let server_type = if s.command.is_empty() { "sse" } else { "stdio" };
                                        let badge_style = source_badge_style(&source);
                                        let badge_label = source_badge_label(&source);

                                        view! {
                                            <div
                                                style="padding: 0.75rem 1rem; border-bottom: 1px solid var(--border); cursor: pointer; transition: background 0.15s;"
                                                class="hover-highlight"
                                                on:click=move |_| {
                                                    selected.set(Some(name_click.clone()));
                                                    selected_source.set(Some(source_click.clone()));
                                                    editor_content.set(config_json.clone());
                                                    save_status.set(None);
                                                }
                                            >
                                                <div style="font-weight: 500; font-size: 0.875rem;">{name}</div>
                                                <div style="display: flex; gap: 0.35rem; margin-top: 0.25rem;">
                                                    <span class="badge badge-muted" style="font-size: 0.7rem;">
                                                        {server_type}
                                                    </span>
                                                    <span class="badge" style=format!("font-size: 0.65rem; padding: 0.1rem 0.4rem; border-radius: 4px; {}", badge_style)>
                                                        {badge_label}
                                                    </span>
                                                </div>
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>

                                // Right: editor
                                <div>
                                    {move || {
                                        if let Some(name) = selected.get() {
                                            let is_editable = selected_source.get().as_deref() == Some("claude_code");
                                            let name_save = name.clone();
                                            let name_delete = name.clone();
                                            let name_health = name.clone();

                                            view! {
                                                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem;">
                                                    <h3 style="margin: 0;">{name}</h3>
                                                    <div style="display: flex; gap: 0.5rem;">
                                                        <button
                                                            class="btn btn-sm"
                                                            on:click=move |_| {
                                                                let n = name_health.clone();
                                                                spawn_local(async move {
                                                                    match api::get::<McpHealthResult>(&format!("/mcp/{}/health", n)).await {
                                                                        Ok(r) => {
                                                                            let ok = r.error.is_none();
                                                                            let msg = if let Some(e) = r.error {
                                                                                format!("Error: {}", e)
                                                                            } else {
                                                                                format!("OK - {} tools, {}ms", r.tools.len(), r.duration_ms)
                                                                            };
                                                                            save_status.set(Some((msg, ok)));
                                                                        }
                                                                        Err(e) => save_status.set(Some((format!("Error: {}", e), false))),
                                                                    }
                                                                });
                                                            }
                                                        >{t("mcp.check_health")}</button>
                                                        {if is_editable {
                                                            view! {
                                                                <button
                                                                    class="btn btn-primary btn-sm"
                                                                    on:click=move |_| {
                                                                        let n = name_save.clone();
                                                                        let content = editor_content.get();
                                                                        spawn_local(async move {
                                                                            match serde_json::from_str::<serde_json::Value>(&content) {
                                                                                Ok(config) => {
                                                                                    let req = McpServerUpdateRequest { config };
                                                                                    match api::put::<McpServerDetail, _>(&format!("/mcp/{}", n), &req).await {
                                                                                        Ok(_) => save_status.set(Some(("Saved!".to_string(), true))),
                                                                                        Err(e) => save_status.set(Some((format!("Error: {}", e), false))),
                                                                                    }
                                                                                }
                                                                                Err(e) => save_status.set(Some((format!("Invalid JSON: {}", e), false))),
                                                                            }
                                                                        });
                                                                    }
                                                                >{t("mcp.save")}</button>
                                                                <button
                                                                    class="btn btn-sm"
                                                                    style="color: var(--error);"
                                                                    on:click=move |_| {
                                                                        let n = name_delete.clone();
                                                                        spawn_local(async move {
                                                                            match api::delete(&format!("/mcp/{}", n)).await {
                                                                                Ok(_) => {
                                                                                    selected.set(None);
                                                                                    save_status.set(Some(("Deleted!".to_string(), true)));
                                                                                    servers.refetch();
                                                                                }
                                                                                Err(e) => save_status.set(Some((format!("Error: {}", e), false))),
                                                                            }
                                                                        });
                                                                    }
                                                                >{t("mcp.delete")}</button>
                                                            }.into_view()
                                                        } else {
                                                            view! {
                                                                <span class="badge badge-muted" style="font-size: 0.75rem; padding: 0.3rem 0.6rem;">
                                                                    {t("mcp.read_only")}
                                                                </span>
                                                            }.into_view()
                                                        }}
                                                    </div>
                                                </div>

                                                {move || {
                                                    let is_ro = selected_source.get().as_deref() != Some("claude_code");
                                                    if is_ro {
                                                        Some(view! {
                                                            <div class="card" style="margin-bottom: 1rem; padding: 0.5rem 1rem; border-left: 3px solid var(--warning); font-size: 0.85rem; color: var(--text-secondary);">
                                                                {t("mcp.read_only_hint")}
                                                            </div>
                                                        })
                                                    } else {
                                                        None
                                                    }
                                                }}

                                                {move || save_status.get().map(|(msg, ok)| view! {
                                                    <div class="card" style=format!(
                                                        "margin-bottom: 1rem; padding: 0.5rem 1rem; border-left: 3px solid {};",
                                                        if ok { "var(--success)" } else { "var(--error)" }
                                                    )>
                                                        <span style="font-size: 0.875rem;">{msg}</span>
                                                    </div>
                                                })}

                                                <div class="editor-container">
                                                    <textarea
                                                        class="editor-textarea"
                                                        style="min-height: 300px; font-family: monospace; font-size: 0.85rem;"
                                                        prop:value=move || editor_content.get()
                                                        prop:disabled=move || selected_source.get().as_deref() != Some("claude_code")
                                                        on:input=move |ev| editor_content.set(event_target_value(&ev))
                                                    />
                                                </div>
                                            }.into_view()
                                        } else {
                                            let first_name = list_clone.first().map(|s| s.name.clone());
                                            view! {
                                                <div class="empty-state" style="min-height: 300px; display: flex; align-items: center; justify-content: center;">
                                                    <p style="color: var(--text-muted);">
                                                        {if first_name.is_some() {
                                                            t("mcp.select_server")
                                                        } else {
                                                            t("mcp.no_servers_configured")
                                                        }}
                                                    </p>
                                                </div>
                                            }.into_view()
                                        }
                                    }}
                                </div>
                            </div>
                        }.into_view()
                    }
                }
                Err(e) => view! {
                    <div class="empty-state"><p>{t("common.error_prefix")} {e}</p></div>
                }.into_view(),
            })}
        </Suspense>
    }
}

// ─────────────────────────────────────────────
// Health Check Tab
// ─────────────────────────────────────────────

#[component]
fn HealthCheckTab() -> impl IntoView {
    let results = create_rw_signal::<Option<Result<Vec<McpHealthResult>, String>>>(None);
    let loading = create_rw_signal(false);

    view! {
        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem;">
            <h3 style="margin: 0;">{t("mcp.health.title")}</h3>
            <button
                class="btn btn-primary"
                prop:disabled=move || loading.get()
                on:click=move |_| {
                    loading.set(true);
                    results.set(None);
                    spawn_local(async move {
                        let res = api::get::<Vec<McpHealthResult>>("/mcp/health").await;
                        results.set(Some(res));
                        loading.set(false);
                    });
                }
            >
                {move || if loading.get() { t("mcp.health.checking") } else { t("mcp.health.check_all") }}
            </button>
        </div>

        <p style="color: var(--text-secondary); font-size: 0.875rem; margin-bottom: 1.5rem;">
            {t("mcp.health.description")}
        </p>

        {move || results.get().map(|res| match res {
            Ok(checks) => {
                if checks.is_empty() {
                    view! {
                        <div class="empty-state"><p>{t("mcp.no_servers")}</p></div>
                    }.into_view()
                } else {
                    view! {
                        <div class="table-container">
                            <table>
                                <thead>
                                    <tr>
                                        <th>{t("mcp.health.col_name")}</th>
                                        <th>{t("mcp.health.col_source")}</th>
                                        <th>{t("mcp.health.col_status")}</th>
                                        <th>{t("mcp.health.col_server_info")}</th>
                                        <th>{t("mcp.health.col_tools")}</th>
                                        <th>{t("mcp.health.col_duration")}</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {checks.into_iter().map(|r| {
                                        let (badge_class, status_text) = match r.status {
                                            claude_admin_shared::McpServerStatus::Running => ("badge badge-success", "Running"),
                                            claude_admin_shared::McpServerStatus::Error => ("badge badge-error", "Error"),
                                            claude_admin_shared::McpServerStatus::Timeout => ("badge badge-warning", "Timeout"),
                                            claude_admin_shared::McpServerStatus::Unknown => ("badge badge-muted", "Unknown"),
                                        };
                                        let src_style = source_badge_style(&r.source);
                                        let src_label = source_badge_label(&r.source);

                                        view! {
                                            <tr>
                                                <td style="font-weight: 500;">{r.name}</td>
                                                <td>
                                                    <span class="badge" style=format!("font-size: 0.65rem; padding: 0.1rem 0.4rem; border-radius: 4px; {}", src_style)>
                                                        {src_label}
                                                    </span>
                                                </td>
                                                <td><span class=badge_class>{status_text}</span></td>
                                                <td style="font-size: 0.85rem; color: var(--text-secondary);">
                                                    {r.server_info.unwrap_or_else(|| "-".to_string())}
                                                </td>
                                                <td>{r.tools.len().to_string()}</td>
                                                <td style="font-size: 0.85rem; color: var(--text-muted);">
                                                    {format!("{}ms", r.duration_ms)}
                                                </td>
                                            </tr>
                                            {r.error.map(|e| view! {
                                                <tr>
                                                    <td colspan="6">
                                                        <div class="card" style="margin: 0.25rem 0; padding: 0.5rem; border-left: 3px solid var(--error); font-size: 0.8rem; color: var(--error);">
                                                            {e}
                                                        </div>
                                                    </td>
                                                </tr>
                                            })}
                                        }
                                    }).collect_view()}
                                </tbody>
                            </table>
                        </div>
                    }.into_view()
                }
            }
            Err(e) => view! {
                <div class="empty-state"><p>{t("common.error_prefix")} {e}</p></div>
            }.into_view(),
        })}
    }
}

// ─────────────────────────────────────────────
// Add Server Tab
// ─────────────────────────────────────────────

#[component]
fn AddServerTab() -> impl IntoView {
    let name = create_rw_signal(String::new());
    let config_json = create_rw_signal(
        "{\n  \"command\": \"npx\",\n  \"args\": [\"-y\", \"@modelcontextprotocol/server-name\"],\n  \"env\": {}\n}".to_string()
    );
    let status = create_rw_signal::<Option<(String, bool)>>(None);

    view! {
        <h3 style="margin-bottom: 1rem;">{t("mcp.add.title")}</h3>
        <p style="color: var(--text-secondary); font-size: 0.875rem; margin-bottom: 1.5rem;">
            {t("mcp.add.description")}
        </p>

        {move || status.get().map(|(msg, ok)| view! {
            <div class="card" style=format!(
                "margin-bottom: 1rem; padding: 0.5rem 1rem; border-left: 3px solid {};",
                if ok { "var(--success)" } else { "var(--error)" }
            )>
                <span style="font-size: 0.875rem;">{msg}</span>
            </div>
        })}

        <div class="card" style="padding: 1.5rem;">
            <div style="margin-bottom: 1rem;">
                <label style="display: block; font-weight: 500; margin-bottom: 0.5rem; font-size: 0.875rem;">{t("mcp.add.name_label")}</label>
                <input
                    type="text"
                    placeholder="e.g. my-server"
                    style="max-width: 400px;"
                    prop:value=move || name.get()
                    on:input=move |ev| name.set(event_target_value(&ev))
                />
            </div>

            <div style="margin-bottom: 1rem;">
                <label style="display: block; font-weight: 500; margin-bottom: 0.5rem; font-size: 0.875rem;">{t("mcp.add.config_label")}</label>
                <div class="editor-container">
                    <textarea
                        class="editor-textarea"
                        style="min-height: 200px; font-family: monospace; font-size: 0.85rem;"
                        prop:value=move || config_json.get()
                        on:input=move |ev| config_json.set(event_target_value(&ev))
                    />
                </div>
            </div>

            <button
                class="btn btn-primary"
                on:click=move |_| {
                    let n = name.get();
                    let c = config_json.get();
                    if n.trim().is_empty() {
                        status.set(Some(("Please enter a server name".to_string(), false)));
                        return;
                    }
                    spawn_local(async move {
                        match serde_json::from_str::<serde_json::Value>(&c) {
                            Ok(config) => {
                                let req = McpServerCreateRequest {
                                    name: n.clone(),
                                    config,
                                };
                                match api::post::<McpServerDetail, _>("/mcp", &req).await {
                                    Ok(_) => {
                                        status.set(Some((format!("'{}' added successfully!", n), true)));
                                        name.set(String::new());
                                    }
                                    Err(e) => status.set(Some((format!("Error: {}", e), false))),
                                }
                            }
                            Err(e) => status.set(Some((format!("Invalid JSON: {}", e), false))),
                        }
                    });
                }
            >{t("mcp.add.submit")}</button>
        </div>
    }
}
