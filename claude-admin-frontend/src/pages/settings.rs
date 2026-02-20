use claude_admin_shared::{HookTemplate, SettingsOverview, SettingsUpdateRequest, StorageInfo};
use leptos::*;

use crate::api;
use crate::i18n::t;

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
        </div>

        {move || match active_tab.get().as_str() {
            "overview" => view! { <SettingsOverviewTab/> }.into_view(),
            "hooks" => view! { <HookBuilderTab/> }.into_view(),
            "storage" => view! { <StorageTab/> }.into_view(),
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
