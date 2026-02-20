use claude_admin_shared::{
    HealthOverview, PermissionDeleteRequest, ProjectPermissionSummary, ProjectPermissions,
};
use leptos::*;
use leptos_router::*;

use crate::api;
use crate::i18n::t;

#[component]
pub fn PermissionsPage() -> impl IntoView {
    let permissions = create_resource(
        || (),
        |_| async move { api::get::<Vec<ProjectPermissionSummary>>("/permissions").await },
    );

    view! {
        <div class="page-header">
            <h2>{t("permissions.title")}</h2>
            <p>{t("permissions.subtitle_manage")}</p>
        </div>

        <Suspense fallback=move || view! { <div class="loading">{t("permissions.loading")}</div> }>
            {move || permissions.get().map(|result| match result {
                Ok(data) => {
                    if data.is_empty() {
                        view! {
                            <div class="empty-state">
                                <p>{t("permissions.no_permissions")}</p>
                            </div>
                        }.into_view()
                    } else {
                        view! {
                            <div class="table-container">
                                <table>
                                    <thead>
                                        <tr>
                                            <th>{t("permissions.col_project")}</th>
                                            <th>{t("permissions.col_entries")}</th>
                                            <th>{t("permissions.col_security_issues")}</th>
                                            <th>{t("permissions.col_fragmented")}</th>
                                            <th>{t("permissions.col_actions")}</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {data.into_iter().map(|p| {
                                            let href = format!("/permissions/{}", p.project_id);
                                            let has_security = p.security_issues > 0;
                                            let has_fragments = p.fragmented_entries > 0;
                                            view! {
                                                <tr class={if has_security { "row-warning" } else { "" }}>
                                                    <td>
                                                        <div>{p.project_name}</div>
                                                        <div style="font-size: 0.75rem; color: var(--text-muted);">{p.path}</div>
                                                    </td>
                                                    <td><span class="badge badge-muted">{p.total_entries}</span></td>
                                                    <td>
                                                        {if has_security {
                                                            view! { <span class="badge badge-danger">{p.security_issues}</span> }.into_view()
                                                        } else {
                                                            view! { <span class="badge badge-success">"0"</span> }.into_view()
                                                        }}
                                                    </td>
                                                    <td>
                                                        {if has_fragments {
                                                            view! { <span class="badge badge-warning">{p.fragmented_entries}</span> }.into_view()
                                                        } else {
                                                            view! { <span class="badge badge-muted">"0"</span> }.into_view()
                                                        }}
                                                    </td>
                                                    <td>
                                                        <a class="btn btn-sm btn-secondary" href=href>{t("permissions.details")}</a>
                                                    </td>
                                                </tr>
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
        </Suspense>
    }
}

#[component]
pub fn PermissionDetailPage() -> impl IntoView {
    let params = use_params_map();
    let project_id = move || params.with(|p| p.get("id").cloned().unwrap_or_default());

    let permissions = create_resource(project_id, |id| async move {
        api::get::<ProjectPermissions>(&format!("/permissions/{}", id)).await
    });

    let selected = create_rw_signal::<Vec<usize>>(vec![]);
    let delete_status = create_rw_signal::<Option<String>>(None);

    view! {
        <div class="page-header">
            <h2>{t("permissions.detail_title")}</h2>
            <p>{t("permissions.detail_subtitle")}</p>
        </div>

        <Suspense fallback=move || view! { <div class="loading">{t("permissions.detail_loading")}</div> }>
            {move || permissions.get().map(|result| match result {
                Ok(data) => {
                    let warnings = data.security_warnings.clone();
                    let entries = data.entries.clone();

                    view! {
                        {if !warnings.is_empty() {
                            view! {
                                <div class="card" style="margin-bottom: 1.5rem; border-left: 3px solid var(--error);">
                                    <h4 style="color: var(--error); margin-bottom: 0.5rem;">
                                        {t("permissions.detail_warnings_count")} " (" {warnings.len()} ")"
                                    </h4>
                                    {warnings.into_iter().map(|w| view! {
                                        <div style="padding: 0.25rem 0; font-size: 0.875rem;">
                                            <span class="badge badge-danger" style="margin-right: 0.5rem;">{w.severity}</span>
                                            {w.message}
                                            <span style="color: var(--text-muted); margin-left: 0.5rem;">"(" {t("permissions.detail_entry")} " #" {w.index} ")"</span>
                                        </div>
                                    }).collect_view()}
                                </div>
                            }.into_view()
                        } else {
                            view! {}.into_view()
                        }}

                        <div style="display: flex; gap: 0.5rem; margin-bottom: 1rem;">
                            <button
                                class="btn btn-danger btn-sm"
                                disabled=move || selected.get().is_empty()
                                on:click=move |_| {
                                    let indices = selected.get();
                                    let pid = project_id();
                                    delete_status.set(Some("Deleting...".to_string()));
                                    spawn_local(async move {
                                        let req = PermissionDeleteRequest { indices };
                                        match api::delete_with_body::<ProjectPermissions, _>(
                                            &format!("/permissions/{}/entries", pid),
                                            &req
                                        ).await {
                                            Ok(_) => {
                                                delete_status.set(Some("Deleted! Reloading...".to_string()));
                                                selected.set(vec![]);
                                            }
                                            Err(e) => delete_status.set(Some(format!("Error: {}", e))),
                                        }
                                    });
                                }
                            >
                                {t("permissions.detail_delete_count")} " (" {move || selected.get().len()} ")"
                            </button>

                            {move || delete_status.get().map(|s| view! {
                                <span style="color: var(--text-muted); font-size: 0.8125rem; align-self: center;">{s}</span>
                            })}
                        </div>

                        <div class="table-container">
                            <table>
                                <thead>
                                    <tr>
                                        <th style="width: 30px;"></th>
                                        <th>{t("permissions.detail_col_index")}</th>
                                        <th>{t("permissions.detail_col_tool")}</th>
                                        <th>{t("permissions.detail_col_command")}</th>
                                        <th>{t("permissions.detail_col_status")}</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {entries.into_iter().map(|e| {
                                        let idx = e.index;
                                        let has_issue = e.security_issue.is_some();
                                        let is_frag = e.is_fragmented;
                                        let issue_text = e.security_issue.clone().unwrap_or_default();

                                        view! {
                                            <tr class={if has_issue { "row-warning" } else if is_frag { "row-fragment" } else { "" }}>
                                                <td>
                                                    <input
                                                        type="checkbox"
                                                        on:change=move |ev| {
                                                            let checked = event_target_checked(&ev);
                                                            selected.update(|v| {
                                                                if checked {
                                                                    v.push(idx);
                                                                } else {
                                                                    v.retain(|&x| x != idx);
                                                                }
                                                            });
                                                        }
                                                    />
                                                </td>
                                                <td style="color: var(--text-muted);">{idx}</td>
                                                <td><code>{e.tool}</code></td>
                                                <td style="font-family: monospace; font-size: 0.8rem; word-break: break-all; max-width: 500px;">
                                                    {e.command}
                                                </td>
                                                <td>
                                                    {if has_issue {
                                                        view! { <span class="badge badge-danger">{issue_text}</span> }.into_view()
                                                    } else if is_frag {
                                                        view! { <span class="badge badge-warning">{t("permissions.detail_fragment")}</span> }.into_view()
                                                    } else {
                                                        view! { <span class="badge badge-success">{t("permissions.detail_ok")}</span> }.into_view()
                                                    }}
                                                </td>
                                            </tr>
                                        }
                                    }).collect_view()}
                                </tbody>
                            </table>
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
pub fn ConfigHealthPage() -> impl IntoView {
    let health = create_resource(
        || (),
        |_| async move { api::get::<HealthOverview>("/health/overview").await },
    );

    view! {
        <div class="page-header">
            <h2>{t("permissions.health_title")}</h2>
            <p>{t("permissions.health_subtitle_scores")}</p>
        </div>

        <Suspense fallback=move || view! { <div class="loading">{t("permissions.health_loading")}</div> }>
            {move || health.get().map(|result| match result {
                Ok(data) => {
                    let avg = data.average_score;
                    let color = score_color(avg);

                    view! {
                        <div class="card-grid">
                            <div class="card">
                                <div class="card-value" style=format!("color: {};", color)>{avg}</div>
                                <div class="card-label">{t("permissions.health_avg_score")}</div>
                            </div>
                            <div class="card">
                                <div class="card-value">{data.projects.len()}</div>
                                <div class="card-label">{t("permissions.health_projects_analyzed")}</div>
                            </div>
                        </div>

                        <div class="table-container">
                            <table>
                                <thead>
                                    <tr>
                                        <th>{t("permissions.health_col_project")}</th>
                                        <th>{t("permissions.health_col_score")}</th>
                                        <th>{t("permissions.health_col_issues")}</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {data.projects.into_iter().map(|p| {
                                        let color = score_color(p.score);
                                        view! {
                                            <tr>
                                                <td>{p.name}</td>
                                                <td>
                                                    <span class="badge" style=format!(
                                                        "background: {}20; color: {};", color, color
                                                    )>
                                                        {p.score} "/100"
                                                    </span>
                                                </td>
                                                <td>
                                                    {if p.issues.is_empty() {
                                                        view! { <span style="color: var(--success);">{t("permissions.health_no_issues")}</span> }.into_view()
                                                    } else {
                                                        view! {
                                                            <ul style="margin: 0; padding-left: 1.25rem; font-size: 0.8125rem; color: var(--text-secondary);">
                                                                {p.issues.into_iter().map(|issue| view! {
                                                                    <li>{issue}</li>
                                                                }).collect_view()}
                                                            </ul>
                                                        }.into_view()
                                                    }}
                                                </td>
                                            </tr>
                                        }
                                    }).collect_view()}
                                </tbody>
                            </table>
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

fn score_color(score: u8) -> &'static str {
    if score >= 70 {
        "#22c55e"
    } else if score >= 40 {
        "#eab308"
    } else {
        "#ef4444"
    }
}

fn event_target_checked(ev: &web_sys::Event) -> bool {
    use wasm_bindgen::JsCast;
    ev.target()
        .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
        .map(|input| input.checked())
        .unwrap_or(false)
}
