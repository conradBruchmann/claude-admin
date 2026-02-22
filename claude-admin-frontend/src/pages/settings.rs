use claude_admin_shared::{HookTemplate, SettingsOverview, SettingsUpdateRequest, StorageInfo};
use leptos::*;
use serde::{Deserialize, Serialize};

use crate::api;
use crate::i18n::t;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct AuthStatus {
    configured: bool,
    source: String,
    has_config_key: bool,
}

#[derive(Clone, Serialize)]
struct SetApiKeyRequest {
    api_key: String,
}

#[component]
pub fn SettingsPage() -> impl IntoView {
    let active_tab = create_rw_signal("overview".to_string());

    view! {
        <div class="page-header">
            <h2>{t("settings.title")}</h2>
            <p>{t("settings.subtitle")}</p>
        </div>

        <div class="tabs">
            <button
                class=move || if active_tab.get() == "overview" { "tab active" } else { "tab" }
                on:click=move |_| active_tab.set("overview".to_string())
            >{t("settings.tab_overview")}</button>
            <button
                class=move || if active_tab.get() == "hooks" { "tab active" } else { "tab" }
                on:click=move |_| active_tab.set("hooks".to_string())
            >{t("settings.tab_hooks")}</button>
            <button
                class=move || if active_tab.get() == "storage" { "tab active" } else { "tab" }
                on:click=move |_| active_tab.set("storage".to_string())
            >{t("settings.tab_storage")}</button>
            <button
                class=move || if active_tab.get() == "api_key" { "tab active" } else { "tab" }
                on:click=move |_| active_tab.set("api_key".to_string())
            >"🔑 API Key"</button>
        </div>

        {move || match active_tab.get().as_str() {
            "overview" => view! { <SettingsOverviewTab/> }.into_view(),
            "hooks" => view! { <HookBuilderTab/> }.into_view(),
            "storage" => view! { <StorageTab/> }.into_view(),
            "api_key" => view! { <ApiKeyTab/> }.into_view(),
            _ => view! { <SettingsOverviewTab/> }.into_view(),
        }}
    }
}

#[component]
fn SettingsOverviewTab() -> impl IntoView {
    let settings = create_resource(
        || (),
        |_| async move { api::get::<SettingsOverview>("/settings/global").await },
    );

    view! {
        <Suspense fallback=move || view! { <div class="loading">{t("settings.loading")}</div> }>
            {move || settings.get().map(|result| match result {
                Ok(data) => {
                    let formatted = serde_json::to_string_pretty(&data.global_settings)
                        .unwrap_or_default();

                    view! {
                        // Hooks section
                        <h3 style="margin-bottom: 1rem;">{t("settings.hooks_title")}</h3>

                        <div style="margin-bottom: 1.5rem;">
                            <h4 style="font-size: 0.875rem; color: var(--text-secondary); margin-bottom: 0.5rem;">{t("settings.pre_tool_use")}</h4>
                            {if data.hooks.pre_tool_use.is_empty() {
                                view! { <p style="color: var(--text-muted);">{t("settings.no_hooks")}</p> }.into_view()
                            } else {
                                view! {
                                    <div class="table-container">
                                        <table>
                                            <thead><tr><th>{t("settings.matcher")}</th><th>{t("settings.command")}</th></tr></thead>
                                            <tbody>
                                                {data.hooks.pre_tool_use.into_iter().map(|h| {
                                                    let cmds = h.hooks.iter()
                                                        .map(|c| c.command.clone())
                                                        .collect::<Vec<_>>()
                                                        .join("; ");
                                                    view! {
                                                        <tr>
                                                            <td><code>{h.matcher}</code></td>
                                                            <td style="font-size: 0.75rem; font-family: monospace; word-break: break-all;">{cmds}</td>
                                                        </tr>
                                                    }
                                                }).collect_view()}
                                            </tbody>
                                        </table>
                                    </div>
                                }.into_view()
                            }}
                        </div>

                        <div style="margin-bottom: 1.5rem;">
                            <h4 style="font-size: 0.875rem; color: var(--text-secondary); margin-bottom: 0.5rem;">{t("settings.post_tool_use")}</h4>
                            {if data.hooks.post_tool_use.is_empty() {
                                view! { <p style="color: var(--text-muted);">{t("settings.no_hooks")}</p> }.into_view()
                            } else {
                                view! {
                                    <div class="table-container">
                                        <table>
                                            <thead><tr><th>{t("settings.matcher")}</th><th>{t("settings.command")}</th><th>"Timeout"</th></tr></thead>
                                            <tbody>
                                                {data.hooks.post_tool_use.into_iter().map(|h| {
                                                    let matcher = h.matcher.clone();
                                                    h.hooks.into_iter().map(move |c| {
                                                        let m = matcher.clone();
                                                        view! {
                                                            <tr>
                                                                <td><code>{m}</code></td>
                                                                <td style="font-size: 0.75rem; font-family: monospace; word-break: break-all;">{c.command}</td>
                                                                <td>{c.timeout.map(|t| format!("{}s", t)).unwrap_or("-".into())}</td>
                                                            </tr>
                                                        }
                                                    }).collect_view()
                                                }).collect_view()}
                                            </tbody>
                                        </table>
                                    </div>
                                }.into_view()
                            }}
                        </div>

                        <h3 style="margin-bottom: 1rem;">"Raw Settings"</h3>
                        <div class="editor-container">
                            <pre class="editor-textarea" style="min-height: 200px; white-space: pre-wrap;">
                                {formatted}
                            </pre>
                        </div>
                    }.into_view()
                }
                Err(e) => view! {
                    <div class="empty-state"><p>{t("common.error_prefix")} {e}</p></div>
                }.into_view(),
            })}
        </Suspense>
    }
}

#[component]
fn HookBuilderTab() -> impl IntoView {
    let templates = create_resource(
        || (),
        |_| async move { api::get::<Vec<HookTemplate>>("/settings/hook-templates").await },
    );

    let install_status = create_rw_signal::<Option<String>>(None);

    view! {
        <h3 style="margin-bottom: 1rem;">{t("settings.hook_templates_title")}</h3>
        <p style="color: var(--text-secondary); font-size: 0.875rem; margin-bottom: 1.5rem;">
            {t("settings.hook_templates_desc")}
        </p>

        {move || install_status.get().map(|s| view! {
            <div class="card" style="margin-bottom: 1rem; border-left: 3px solid var(--success);">
                <span style="font-size: 0.875rem;">{s}</span>
            </div>
        })}

        <Suspense fallback=move || view! { <div class="loading">{t("settings.hook_templates_loading")}</div> }>
            {move || templates.get().map(|result| match result {
                Ok(tmpls) => view! {
                    <div style="display: flex; flex-direction: column; gap: 1rem;">
                        {tmpls.into_iter().map(|tmpl| {
                            let name = tmpl.name.clone();
                            let event = tmpl.event.clone();
                            let matcher = tmpl.matcher.clone();
                            let command = tmpl.command.clone();
                            let command_display = command.clone();

                            view! {
                                <div class="card">
                                    <div style="display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 0.5rem;">
                                        <div>
                                            <span style="font-weight: 600;">{name}</span>
                                            <span class="badge badge-muted" style="margin-left: 0.5rem;">{event.clone()}</span>
                                            {matcher.clone().map(|m| view! {
                                                <span class="badge badge-muted" style="margin-left: 0.25rem;">{m}</span>
                                            })}
                                        </div>
                                        <button
                                            class="btn btn-primary btn-sm"
                                            on:click=move |_| {
                                                let event = event.clone();
                                                let matcher = matcher.clone();
                                                let command = command.clone();
                                                spawn_local(async move {
                                                    // First get current settings
                                                    match api::get::<SettingsOverview>("/settings/global").await {
                                                        Ok(current) => {
                                                            let mut settings = current.global_settings;
                                                            // Add hook to the appropriate event
                                                            let hooks = settings
                                                                .as_object_mut()
                                                                .unwrap()
                                                                .entry("hooks")
                                                                .or_insert(serde_json::json!({}));
                                                            let event_hooks = hooks
                                                                .as_object_mut()
                                                                .unwrap()
                                                                .entry(&event)
                                                                .or_insert(serde_json::json!([]));
                                                            if let Some(arr) = event_hooks.as_array_mut() {
                                                                arr.push(serde_json::json!({
                                                                    "matcher": matcher.unwrap_or_else(|| "*".to_string()),
                                                                    "hooks": [{
                                                                        "type": "command",
                                                                        "command": command
                                                                    }]
                                                                }));
                                                            }

                                                            let req = SettingsUpdateRequest { settings };
                                                            match api::put::<SettingsOverview, _>("/settings/global", &req).await {
                                                                Ok(_) => install_status.set(Some("Hook added successfully!".to_string())),
                                                                Err(e) => install_status.set(Some(format!("Error: {}", e))),
                                                            }
                                                        }
                                                        Err(e) => install_status.set(Some(format!("Error reading settings: {}", e))),
                                                    }
                                                });
                                            }
                                        >{t("settings.add_hook")}</button>
                                    </div>
                                    <p style="color: var(--text-secondary); font-size: 0.875rem; margin-bottom: 0.5rem;">
                                        {tmpl.description}
                                    </p>
                                    <pre style="
                                        padding: 0.5rem 0.75rem;
                                        background: var(--bg-primary);
                                        border-radius: 0.375rem;
                                        font-size: 0.8rem;
                                        color: var(--text-secondary);
                                        overflow-x: auto;
                                    ">{command_display}</pre>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                }.into_view(),
                Err(e) => view! {
                    <div class="empty-state"><p>{t("common.error_prefix")} {e}</p></div>
                }.into_view(),
            })}
        </Suspense>
    }
}

#[component]
fn StorageTab() -> impl IntoView {
    let storage = create_resource(
        || (),
        |_| async move { api::get::<StorageInfo>("/settings/storage").await },
    );

    view! {
        <h3 style="margin-bottom: 1rem;">{t("settings.storage_title")}</h3>
        <p style="color: var(--text-secondary); font-size: 0.875rem; margin-bottom: 1.5rem;">
            "Disk usage of ~/.claude/ directories"
        </p>

        <Suspense fallback=move || view! { <div class="loading">{t("settings.storage_loading")}</div> }>
            {move || storage.get().map(|result| match result {
                Ok(data) => {
                    let total = format_bytes(data.total_bytes);
                    let max_bytes = data.directories.first().map(|d| d.bytes).unwrap_or(1);

                    view! {
                        <div class="card" style="margin-bottom: 1.5rem;">
                            <div class="card-value">{total}</div>
                            <div class="card-label">{t("settings.storage_total")} " ~/.claude/"</div>
                        </div>

                        <div class="card">
                            {data.directories.into_iter().map(|d| {
                                let pct = (d.bytes as f64 / max_bytes as f64 * 100.0) as u64;
                                view! {
                                    <div style="margin-bottom: 1rem;">
                                        <div style="display: flex; justify-content: space-between; font-size: 0.875rem; margin-bottom: 0.25rem;">
                                            <span style="font-weight: 500;">{d.name}</span>
                                            <span style="color: var(--text-muted);">{format_bytes(d.bytes)}</span>
                                        </div>
                                        <div class="progress-bar">
                                            <div class="progress-fill" style=format!("width: {}%;", pct)/>
                                        </div>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    }.into_view()
                }
                Err(e) => view! {
                    <div class="empty-state"><p>{t("common.error_prefix")} {e}</p></div>
                }.into_view(),
            })}
        </Suspense>
    }
}

#[component]
fn ApiKeyTab() -> impl IntoView {
    let status = create_resource(
        || (),
        |_| async move { api::get::<AuthStatus>("/settings/api-key").await },
    );

    let api_key_input = create_rw_signal(String::new());
    let save_status = create_rw_signal::<Option<(bool, String)>>(None);
    let is_saving = create_rw_signal(false);

    let on_save = move |_| {
        let key = api_key_input.get();
        is_saving.set(true);
        save_status.set(None);
        spawn_local(async move {
            let req = SetApiKeyRequest { api_key: key };
            match api::put::<AuthStatus, _>("/settings/api-key", &req).await {
                Ok(_status) => {
                    save_status.set(Some((true, "API Key gespeichert und aktiviert!".to_string())));
                    api_key_input.set(String::new());
                    status.refetch();
                }
                Err(e) => {
                    save_status.set(Some((false, format!("Fehler: {}", e))));
                }
            }
            is_saving.set(false);
        });
    };

    let on_remove = move |_| {
        is_saving.set(true);
        save_status.set(None);
        spawn_local(async move {
            let req = SetApiKeyRequest { api_key: String::new() };
            match api::put::<AuthStatus, _>("/settings/api-key", &req).await {
                Ok(_) => {
                    save_status.set(Some((true, "API Key entfernt.".to_string())));
                    status.refetch();
                }
                Err(e) => {
                    save_status.set(Some((false, format!("Fehler: {}", e))));
                }
            }
            is_saving.set(false);
        });
    };

    view! {
        <h3 style="margin-bottom: 1rem;">"Anthropic API Key"</h3>
        <p style="color: var(--text-secondary); font-size: 0.875rem; margin-bottom: 1.5rem;">
            "Konfiguriere deinen Anthropic API Key für AI-Features (Suggestions, Validierung, Project Advisor). "
            "Auf macOS wird alternativ der OAuth-Token aus Claude Code verwendet."
        </p>

        // Status display
        <Suspense fallback=move || view! { <div class="loading">"Lade Status..."</div> }>
            {move || status.get().map(|result| match result {
                Ok(data) => {
                    let (icon, label, detail) = match data.source.as_str() {
                        "env" => ("✅", "Aktiv via Umgebungsvariable", "ANTHROPIC_API_KEY ist gesetzt. Diese hat Vorrang vor allen anderen Quellen."),
                        "config" => ("✅", "Aktiv via gespeichertem Key", "API Key wurde in den ClaudeAdmin-Einstellungen hinterlegt."),
                        "keychain" => ("✅", "Aktiv via macOS Keychain", "OAuth-Token von Claude Code wird verwendet."),
                        _ => ("❌", "Nicht konfiguriert", "Kein API Key gefunden. Bitte unten eingeben."),
                    };

                    view! {
                        <div class="card" style="margin-bottom: 1.5rem;">
                            <div style="display: flex; align-items: center; gap: 0.75rem;">
                                <span style="font-size: 1.25rem;">{icon}</span>
                                <div>
                                    <div style="font-weight: 600;">{label}</div>
                                    <div style="font-size: 0.8rem; color: var(--text-muted);">{detail}</div>
                                </div>
                            </div>
                        </div>
                    }.into_view()
                }
                Err(e) => view! {
                    <div class="card" style="margin-bottom: 1.5rem; border-left: 3px solid var(--danger);">
                        <span style="font-size: 0.875rem; color: var(--danger);">"Fehler: " {e}</span>
                    </div>
                }.into_view(),
            })}
        </Suspense>

        // Save status message
        {move || save_status.get().map(|(success, msg)| {
            let border_color = if success { "var(--success)" } else { "var(--danger)" };
            view! {
                <div class="card" style=format!("margin-bottom: 1rem; border-left: 3px solid {};", border_color)>
                    <span style="font-size: 0.875rem;">{msg}</span>
                </div>
            }
        })}

        // Input form
        <div class="card">
            <h4 style="font-size: 0.875rem; color: var(--text-secondary); margin-bottom: 0.75rem;">
                "API Key eingeben"
            </h4>
            <div style="display: flex; gap: 0.5rem; margin-bottom: 0.75rem;">
                <input
                    type="password"
                    class="editor-textarea"
                    placeholder="sk-ant-..."
                    style="flex: 1; padding: 0.5rem 0.75rem; font-family: monospace; font-size: 0.85rem;"
                    prop:value=move || api_key_input.get()
                    on:input=move |ev| api_key_input.set(event_target_value(&ev))
                />
                <button
                    class="btn btn-primary"
                    prop:disabled=move || api_key_input.get().is_empty() || is_saving.get()
                    on:click=on_save
                >"Speichern"</button>
            </div>
            <div style="display: flex; justify-content: space-between; align-items: center;">
                <p style="font-size: 0.75rem; color: var(--text-muted);">
                    "Der Key wird in ~/.claude/claude-admin.json gespeichert."
                </p>
                {move || {
                    status.get().and_then(|r| r.ok()).filter(|s| s.has_config_key).map(|_| {
                        view! {
                            <button
                                class="btn btn-sm"
                                style="color: var(--danger); border-color: var(--danger);"
                                prop:disabled=move || is_saving.get()
                                on:click=on_remove
                            >"Key entfernen"</button>
                        }
                    })
                }}
            </div>
        </div>
    }
}

fn format_bytes(bytes: u64) -> String {
    if bytes >= 1_073_741_824 {
        format!("{:.1} GB", bytes as f64 / 1_073_741_824.0)
    } else if bytes >= 1_048_576 {
        format!("{:.1} MB", bytes as f64 / 1_048_576.0)
    } else if bytes >= 1_024 {
        format!("{:.1} KB", bytes as f64 / 1_024.0)
    } else {
        format!("{} B", bytes)
    }
}
