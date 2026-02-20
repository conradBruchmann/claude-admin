use claude_admin_shared::{BrowsableMcpServer, McpInstallRequest, McpServerDetail};
use leptos::*;

use crate::api;
use crate::i18n::t;

#[component]
pub fn McpBrowserPage() -> impl IntoView {
    let search_query = create_rw_signal(String::new());
    let install_status = create_rw_signal::<Option<(String, String)>>(None);

    let catalog = create_resource(
        || (),
        |_| async move { api::get::<Vec<BrowsableMcpServer>>("/mcp-browser").await },
    );

    view! {
        <div class="page-header">
            <h2>{t("mcp_browser.title")}</h2>
            <p>{t("mcp_browser.subtitle")}</p>
        </div>

        <div style="margin-bottom: 1rem;">
            <input
                type="text"
                placeholder="Search MCP servers..."
                style="max-width: 400px;"
                prop:value=move || search_query.get()
                on:input=move |ev| search_query.set(event_target_value(&ev))
            />
        </div>

        {move || install_status.get().map(|(name, status)| view! {
            <div class="card" style=format!(
                "margin-bottom: 1rem; border-left: 3px solid {};",
                if status == "installed" { "var(--success)" } else { "var(--error)" }
            )>
                <span style="font-size: 0.875rem;">
                    {if status == "installed" {
                        format!("'{}' installed successfully!", name)
                    } else {
                        format!("Failed to install '{}': {}", name, status)
                    }}
                </span>
            </div>
        })}

        <Suspense fallback=move || view! { <div class="loading">{t("mcp_browser.loading")}</div> }>
            {move || {
                let query = search_query.get().to_lowercase();

                catalog.get().map(|result| match result {
                    Ok(servers) => {
                        let filtered: Vec<BrowsableMcpServer> = servers
                            .into_iter()
                            .filter(|s| {
                                if query.is_empty() {
                                    true
                                } else {
                                    s.name.to_lowercase().contains(&query)
                                        || s.description.to_lowercase().contains(&query)
                                        || s.category.to_lowercase().contains(&query)
                                }
                            })
                            .collect();

                        if filtered.is_empty() {
                            view! {
                                <div class="empty-state"><p>{t("mcp_browser.no_results")}</p></div>
                            }.into_view()
                        } else {
                            view! {
                                <div class="skill-grid">
                                    {filtered.into_iter().map(|server| {
                                        let name = server.name.clone();
                                        let name_for_install = server.name.clone();
                                        let installed = server.installed;
                                        let default_config = server.default_config.clone();
                                        let has_env = server.default_config
                                            .get("env")
                                            .and_then(|v| v.as_object())
                                            .map(|o| !o.is_empty())
                                            .unwrap_or(false);

                                        view! {
                                            <div class="card skill-card">
                                                <div style="display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 0.5rem;">
                                                    <div>
                                                        <span style="font-weight: 600; font-size: 1rem;">{name}</span>
                                                        <span class="badge badge-muted" style="margin-left: 0.5rem;">{server.category.clone()}</span>
                                                    </div>
                                                    {if installed {
                                                        view! { <span class="badge badge-success">{t("mcp_browser.installed")}</span> }.into_view()
                                                    } else {
                                                        view! {
                                                            <button
                                                                class="btn btn-primary btn-sm"
                                                                on:click=move |_| {
                                                                    let name = name_for_install.clone();
                                                                    let config = default_config.clone();
                                                                    install_status.set(None);
                                                                    spawn_local(async move {
                                                                        let req = McpInstallRequest {
                                                                            name: name.clone(),
                                                                            config,
                                                                        };
                                                                        match api::post::<McpServerDetail, _>(
                                                                            "/mcp-browser/install",
                                                                            &req
                                                                        ).await {
                                                                            Ok(_) => {
                                                                                install_status.set(Some((name, "installed".to_string())));
                                                                                catalog.refetch();
                                                                            }
                                                                            Err(e) => {
                                                                                install_status.set(Some((name, e)));
                                                                            }
                                                                        }
                                                                    });
                                                                }
                                                            >
                                                                {t("mcp_browser.install")}
                                                            </button>
                                                        }.into_view()
                                                    }}
                                                </div>
                                                <p style="color: var(--text-secondary); font-size: 0.875rem; line-height: 1.5;">
                                                    {server.description}
                                                </p>
                                                <div style="margin-top: 0.5rem; display: flex; justify-content: space-between; align-items: center;">
                                                    <span style="font-size: 0.75rem; color: var(--text-muted); font-family: monospace;">
                                                        {server.npm_package}
                                                    </span>
                                                    {if has_env {
                                                        view! {
                                                            <span class="badge badge-warning" style="font-size: 0.65rem;">{t("mcp_browser.needs_api_key")}</span>
                                                        }.into_view()
                                                    } else {
                                                        view! { <span/> }.into_view()
                                                    }}
                                                </div>
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>
                            }.into_view()
                        }
                    }
                    Err(e) => view! {
                        <div class="empty-state"><p>{t("common.error_prefix")} {e}</p></div>
                    }.into_view(),
                })
            }}
        </Suspense>
    }
}
