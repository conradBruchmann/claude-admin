use claude_admin_shared::{
    AdvisorCategory, AdvisorReport, ClaudeMdContent, ClaudeMdUpdateRequest, ProjectDetail,
    ProjectHealth, ProjectPermissions,
};
use leptos::*;
use leptos_router::*;

use crate::api;
use crate::components::markdown_editor::MarkdownEditor;
use crate::i18n::t;

#[component]
pub fn ProjectDetailPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").cloned().unwrap_or_default());

    let project = create_resource(id, |id| async move {
        api::get::<ProjectDetail>(&format!("/projects/{}", id)).await
    });

    let active_tab = create_rw_signal("advisor".to_string());

    view! {
        <Suspense fallback=move || view! { <div class="loading">{t("project_detail.loading")}</div> }>
            {move || project.get().map(|result| match result {
                Ok(detail) => {
                    let name = detail.summary.name.clone();
                    let path = detail.summary.path.clone();

                    view! {
                        <div class="page-header">
                            <h2>{name}</h2>
                            <p>{path}</p>
                        </div>

                        <div class="tabs">
                            <button
                                class=move || if active_tab.get() == "advisor" { "tab active" } else { "tab" }
                                on:click=move |_| active_tab.set("advisor".to_string())
                            >{t("project_detail.tab_advisor")}</button>
                            <button
                                class=move || if active_tab.get() == "claude-md" { "tab active" } else { "tab" }
                                on:click=move |_| active_tab.set("claude-md".to_string())
                            >{t("project_detail.tab_claude_md")}</button>
                            <button
                                class=move || if active_tab.get() == "skills" { "tab active" } else { "tab" }
                                on:click=move |_| active_tab.set("skills".to_string())
                            >{t("project_detail.tab_skills")}</button>
                            <button
                                class=move || if active_tab.get() == "rules" { "tab active" } else { "tab" }
                                on:click=move |_| active_tab.set("rules".to_string())
                            >{t("project_detail.tab_rules")}</button>
                            <button
                                class=move || if active_tab.get() == "memory" { "tab active" } else { "tab" }
                                on:click=move |_| active_tab.set("memory".to_string())
                            >{t("project_detail.tab_memory")}</button>
                            <button
                                class=move || if active_tab.get() == "permissions" { "tab active" } else { "tab" }
                                on:click=move |_| active_tab.set("permissions".to_string())
                            >{t("project_detail.tab_permissions")}</button>
                            <button
                                class=move || if active_tab.get() == "health" { "tab active" } else { "tab" }
                                on:click=move |_| active_tab.set("health".to_string())
                            >{t("project_detail.tab_health")}</button>
                        </div>

                        {move || {
                            let tab = active_tab.get();
                            match tab.as_str() {
                                "advisor" => {
                                    let pid = id();
                                    view! { <AdvisorTab project_id=pid/> }.into_view()
                                }
                                "claude-md" => {
                                    let content_signal = create_rw_signal(
                                        detail.claude_md.clone().unwrap_or_default()
                                    );
                                    let project_id = id();
                                    view! {
                                        <MarkdownEditor
                                            content=content_signal
                                            on_save=Callback::new(move |content: String| {
                                                let pid = project_id.clone();
                                                spawn_local(async move {
                                                    let req = ClaudeMdUpdateRequest { content };
                                                    let _ = api::put::<ClaudeMdContent, _>(
                                                        &format!("/projects/{}/claude-md", pid),
                                                        &req
                                                    ).await;
                                                });
                                            })
                                            label="CLAUDE.md"
                                        />
                                    }.into_view()
                                }
                                "skills" => {
                                    let skills = detail.skills.clone();
                                    view! {
                                        <div class="table-container">
                                            <table>
                                                <thead>
                                                    <tr>
                                                        <th>{t("project_detail.skills_col_name")}</th>
                                                        <th>{t("project_detail.skills_col_description")}</th>
                                                        <th>{t("project_detail.skills_col_invocable")}</th>
                                                    </tr>
                                                </thead>
                                                <tbody>
                                                    {skills.into_iter().map(|s| view! {
                                                        <tr>
                                                            <td>{s.name}</td>
                                                            <td>{s.frontmatter.description.unwrap_or_default()}</td>
                                                            <td>{if s.frontmatter.user_invocable.unwrap_or(false) {
                                                                view! { <span class="badge badge-success">{t("common.yes")}</span> }.into_view()
                                                            } else {
                                                                view! { <span class="badge badge-muted">{t("common.no")}</span> }.into_view()
                                                            }}</td>
                                                        </tr>
                                                    }).collect_view()}
                                                </tbody>
                                            </table>
                                        </div>
                                    }.into_view()
                                }
                                "rules" => {
                                    let rules = detail.rules.clone();
                                    if rules.is_empty() {
                                        view! { <div class="empty-state"><p>{t("project_detail.no_rules")}</p></div> }.into_view()
                                    } else {
                                        view! {
                                            <div class="table-container">
                                                <table>
                                                    <thead><tr><th>{t("project_detail.rules_col_name")}</th><th>{t("project_detail.rules_col_path")}</th></tr></thead>
                                                    <tbody>
                                                        {rules.into_iter().map(|r| view! {
                                                            <tr>
                                                                <td>{r.name}</td>
                                                                <td style="font-size: 0.8rem; color: var(--text-muted);">{r.path}</td>
                                                            </tr>
                                                        }).collect_view()}
                                                    </tbody>
                                                </table>
                                            </div>
                                        }.into_view()
                                    }
                                }
                                "memory" => {
                                    let memory = detail.memory_files.clone();
                                    if memory.is_empty() {
                                        view! { <div class="empty-state"><p>{t("project_detail.no_memory")}</p></div> }.into_view()
                                    } else {
                                        view! {
                                            <div class="table-container">
                                                <table>
                                                    <thead><tr><th>{t("project_detail.memory_col_file")}</th><th>{t("project_detail.memory_col_size")}</th></tr></thead>
                                                    <tbody>
                                                        {memory.into_iter().map(|m| {
                                                            let size = m.content.len();
                                                            view! {
                                                                <tr>
                                                                    <td>{m.name}</td>
                                                                    <td>{format!("{} ", size)} {t("project_detail.bytes")}</td>
                                                                </tr>
                                                            }
                                                        }).collect_view()}
                                                    </tbody>
                                                </table>
                                            </div>
                                        }.into_view()
                                    }
                                }
                                "permissions" => {
                                    let pid = id();
                                    view! { <PermissionsTab project_id=pid/> }.into_view()
                                }
                                "health" => {
                                    let pid = id();
                                    view! { <HealthTab project_id=pid/> }.into_view()
                                }
                                _ => view! { <div>{t("project_detail.unknown_tab")}</div> }.into_view(),
                            }
                        }}
                    }.into_view()
                }
                Err(e) => view! {
                    <div class="empty-state">
                        <p>{t("common.error_prefix")} {e}</p>
                    </div>
                }.into_view(),
            })}
        </Suspense>
    }
}

#[component]
fn AdvisorTab(#[prop(into)] project_id: String) -> impl IntoView {
    let pid = project_id.clone();
    let advisor_data = create_rw_signal::<Option<Result<AdvisorReport, String>>>(None);
    let loading = create_rw_signal(false);
    let action_status = create_rw_signal::<Option<(usize, String)>>(None);

    let fetch_advisor = move |_| {
        let pid = pid.clone();
        loading.set(true);
        advisor_data.set(None);
        spawn_local(async move {
            let result = api::get::<AdvisorReport>(&format!("/projects/{}/advisor", pid)).await;
            advisor_data.set(Some(result));
            loading.set(false);
        });
    };

    view! {
        <div style="margin-bottom: 1.5rem;">
            <div style="display: flex; align-items: center; gap: 1rem; margin-bottom: 1rem;">
                <button
                    class="btn btn-primary"
                    on:click=fetch_advisor
                    disabled=loading
                >
                    {move || if loading.get() { t("project_detail.advisor_analyzing").get() } else { t("project_detail.advisor_analyze").get() }}
                </button>
                <span style="color: var(--text-muted); font-size: 0.8125rem;">
                    {t("project_detail.advisor_description")}
                </span>
            </div>

            {move || if loading.get() {
                view! {
                    <div class="loading">{t("project_detail.advisor_loading")}</div>
                }.into_view()
            } else {
                view! {}.into_view()
            }}

            {move || advisor_data.get().map(|result| match result {
                Ok(report) => {
                    let summary = report.project_summary.clone();
                    view! {
                        // Project summary
                        <div class="card" style="margin-bottom: 1.5rem; border-left: 3px solid var(--accent);">
                            <div style="font-size: 0.75rem; text-transform: uppercase; letter-spacing: 0.05em; color: var(--text-muted); margin-bottom: 0.5rem;">
                                {t("project_detail.advisor_summary")}
                            </div>
                            <div style="font-size: 0.9375rem; line-height: 1.5;">{summary}</div>
                        </div>

                        // Recommendations
                        <div style="display: flex; flex-direction: column; gap: 1rem;">
                            {report.recommendations.into_iter().enumerate().map(|(idx, rec)| {
                                let category_label = category_label(&rec.category);
                                let category_color = category_color(&rec.category);
                                let title = rec.title.clone();
                                let description = rec.description.clone();
                                let has_action = rec.action.is_some();
                                let action_label = rec.action.as_ref().map(|a| a.label.clone()).unwrap_or_default();
                                let action_payload = rec.action.as_ref().map(|a| a.payload.clone()).unwrap_or_default();
                                let action_type = rec.action.as_ref().map(|a| a.action_type.clone());
                                let project_id_for_action = project_id.clone();

                                view! {
                                    <div class="card" style="border-left: 3px solid; border-left-color: var(--border);">
                                        <div style="display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 0.5rem;">
                                            <div>
                                                <span
                                                    class="badge"
                                                    style=format!("background: {}20; color: {};", category_color, category_color)
                                                >
                                                    {category_label}
                                                </span>
                                                <span style="font-weight: 600; margin-left: 0.5rem;">{title}</span>
                                            </div>
                                        </div>
                                        <p style="color: var(--text-secondary); font-size: 0.875rem; line-height: 1.6; white-space: pre-line;">
                                            {description}
                                        </p>

                                        {if has_action {
                                            let pid = project_id_for_action.clone();
                                            let payload = action_payload.clone();
                                            let at = action_type.clone();
                                            view! {
                                                <div style="margin-top: 0.75rem; display: flex; align-items: center; gap: 0.75rem;">
                                                    <button
                                                        class="btn btn-primary btn-sm"
                                                        on:click=move |_| {
                                                            let pid = pid.clone();
                                                            let payload = payload.clone();
                                                            let at = at.clone();
                                                            action_status.set(Some((idx, "running".to_string())));
                                                            spawn_local(async move {
                                                                let result = execute_action(&pid, at.as_ref(), &payload).await;
                                                                match result {
                                                                    Ok(_) => action_status.set(Some((idx, "done".to_string()))),
                                                                    Err(e) => action_status.set(Some((idx, format!("error: {}", e)))),
                                                                }
                                                            });
                                                        }
                                                    >
                                                        {action_label}
                                                    </button>
                                                    {move || action_status.get().and_then(|(i, status)| {
                                                        if i == idx {
                                                            Some(if status == "done" {
                                                                view! { <span style="color: var(--success); font-size: 0.8125rem;">{t("project_detail.advisor_done")}</span> }.into_view()
                                                            } else if status == "running" {
                                                                view! { <span style="color: var(--text-muted); font-size: 0.8125rem;">"..."</span> }.into_view()
                                                            } else {
                                                                view! { <span style="color: var(--error); font-size: 0.8125rem;">{status}</span> }.into_view()
                                                            })
                                                        } else {
                                                            None
                                                        }
                                                    })}
                                                </div>
                                            }.into_view()
                                        } else {
                                            view! {}.into_view()
                                        }}

                                        // Show payload preview if there's an action with content
                                        {if !action_payload.is_empty() && action_payload.len() > 20 {
                                            view! {
                                                <details style="margin-top: 0.75rem;">
                                                    <summary style="cursor: pointer; color: var(--text-muted); font-size: 0.8125rem;">
                                                        {t("project_detail.advisor_preview")}
                                                    </summary>
                                                    <pre style="
                                                        margin-top: 0.5rem;
                                                        padding: 0.75rem;
                                                        background: var(--bg-primary);
                                                        border-radius: 0.375rem;
                                                        font-size: 0.8rem;
                                                        line-height: 1.5;
                                                        overflow-x: auto;
                                                        white-space: pre-wrap;
                                                        color: var(--text-secondary);
                                                    ">
                                                        {action_payload}
                                                    </pre>
                                                </details>
                                            }.into_view()
                                        } else {
                                            view! {}.into_view()
                                        }}
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    }.into_view()
                }
                Err(e) => view! {
                    <div class="card" style="border-left: 3px solid var(--error);">
                        <p style="color: var(--error);">{t("common.error_prefix")} {e}</p>
                    </div>
                }.into_view(),
            })}
        </div>
    }
}

fn category_label(cat: &AdvisorCategory) -> &'static str {
    match cat {
        AdvisorCategory::GlobalSkill => "Skill",
        AdvisorCategory::GlobalRule => "Rule",
        AdvisorCategory::ClaudeMd => "CLAUDE.md",
        AdvisorCategory::Memory => "Memory",
        AdvisorCategory::Hooks => "Hooks",
        AdvisorCategory::General => "Tipp",
    }
}

fn category_color(cat: &AdvisorCategory) -> &'static str {
    match cat {
        AdvisorCategory::GlobalSkill => "#f97316",
        AdvisorCategory::GlobalRule => "#8b5cf6",
        AdvisorCategory::ClaudeMd => "#22c55e",
        AdvisorCategory::Memory => "#3b82f6",
        AdvisorCategory::Hooks => "#eab308",
        AdvisorCategory::General => "#94a3b8",
    }
}

use claude_admin_shared::AdvisorActionType;

async fn execute_action(
    project_id: &str,
    action_type: Option<&AdvisorActionType>,
    payload: &str,
) -> Result<(), String> {
    match action_type {
        Some(AdvisorActionType::CreateClaudeMd) | Some(AdvisorActionType::UpdateClaudeMd) => {
            let req = ClaudeMdUpdateRequest {
                content: payload.to_string(),
            };
            api::put::<ClaudeMdContent, _>(&format!("/projects/{}/claude-md", project_id), &req)
                .await
                .map(|_| ())
        }
        Some(AdvisorActionType::InitMemory) => {
            let req = claude_admin_shared::MemoryUpdateRequest {
                content: payload.to_string(),
            };
            api::put::<claude_admin_shared::MemoryFile, _>(&format!("/memory/{}", project_id), &req)
                .await
                .map(|_| ())
        }
        _ => {
            // Other action types not yet implemented
            Ok(())
        }
    }
}

#[component]
fn PermissionsTab(#[prop(into)] project_id: String) -> impl IntoView {
    let pid = project_id.clone();
    let pid_for_link = project_id.clone();
    let permissions = create_resource(
        move || pid.clone(),
        |id| async move { api::get::<ProjectPermissions>(&format!("/permissions/{}", id)).await },
    );

    let link_href = store_value(format!("/permissions/{}", pid_for_link));

    view! {
        <Suspense fallback=move || view! { <div class="loading">{t("project_detail.permissions_loading")}</div> }>
            {move || {
                permissions.get().map(|result| match result {
                Ok(data) => {
                    if data.entries.is_empty() {
                        view! {
                            <div class="empty-state"><p>{t("project_detail.permissions_no_entries")}</p></div>
                        }.into_view()
                    } else {
                        view! {
                            {if !data.security_warnings.is_empty() {
                                view! {
                                    <div class="card" style="margin-bottom: 1rem; border-left: 3px solid var(--error);">
                                        <span style="color: var(--error); font-weight: 600;">
                                            {data.security_warnings.len()} " " {t("project_detail.permissions_security_warnings")}
                                        </span>
                                    </div>
                                }.into_view()
                            } else {
                                view! {}.into_view()
                            }}

                            <div class="table-container">
                                <table>
                                    <thead><tr><th>{t("project_detail.permissions_col_index")}</th><th>{t("project_detail.permissions_col_tool")}</th><th>{t("project_detail.permissions_col_command")}</th><th>{t("project_detail.permissions_col_status")}</th></tr></thead>
                                    <tbody>
                                        {data.entries.into_iter().map(|e| {
                                            let has_issue = e.security_issue.is_some();
                                            let is_frag = e.is_fragmented;
                                            view! {
                                                <tr class={if has_issue { "row-warning" } else { "" }}>
                                                    <td style="color: var(--text-muted);">{e.index}</td>
                                                    <td><code>{e.tool}</code></td>
                                                    <td style="font-family: monospace; font-size: 0.8rem; word-break: break-all; max-width: 400px;">{e.command}</td>
                                                    <td>
                                                        {if has_issue {
                                                            view! { <span class="badge badge-danger">{e.security_issue.unwrap_or_default()}</span> }.into_view()
                                                        } else if is_frag {
                                                            view! { <span class="badge badge-warning">{t("project_detail.permissions_fragment")}</span> }.into_view()
                                                        } else {
                                                            view! { <span class="badge badge-success">{t("project_detail.permissions_ok")}</span> }.into_view()
                                                        }}
                                                    </td>
                                                </tr>
                                            }
                                        }).collect_view()}
                                    </tbody>
                                </table>
                            </div>

                            <div style="margin-top: 1rem;">
                                <a class="btn btn-secondary btn-sm" href=link_href.get_value()>
                                    {t("project_detail.permissions_manage")}
                                </a>
                            </div>
                        }.into_view()
                    }
                }
                Err(e) => view! {
                    <div class="empty-state"><p>{t("common.error_prefix")} {e}</p></div>
                }.into_view(),
            })}}
        </Suspense>
    }
}

#[component]
fn HealthTab(#[prop(into)] project_id: String) -> impl IntoView {
    let pid = project_id.clone();
    let health = create_resource(
        move || pid.clone(),
        |id| async move { api::get::<ProjectHealth>(&format!("/health/{}", id)).await },
    );

    view! {
        <Suspense fallback=move || view! { <div class="loading">{t("project_detail.health_loading")}</div> }>
            {move || health.get().map(|result| match result {
                Ok(data) => {
                    let color = if data.score >= 70 { "var(--success)" } else if data.score >= 40 { "var(--warning)" } else { "var(--error)" };
                    view! {
                        <div class="card-grid">
                            <div class="card">
                                <div class="card-value" style=format!("color: {};", color)>{data.score}</div>
                                <div class="card-label">{t("project_detail.health_score")}</div>
                            </div>
                            <div class="card">
                                <div class="card-value">{if data.has_claude_md { t("common.yes").get() } else { t("common.no").get() }}</div>
                                <div class="card-label">{t("project_detail.health_claude_md")}</div>
                            </div>
                            <div class="card">
                                <div class="card-value">{if data.has_memory { t("common.yes").get() } else { t("common.no").get() }}</div>
                                <div class="card-label">{t("project_detail.health_memory")}</div>
                            </div>
                            <div class="card">
                                <div class="card-value">{data.permission_count}</div>
                                <div class="card-label">{t("project_detail.health_permission_entries")}</div>
                            </div>
                        </div>

                        {if !data.security_issues.is_empty() {
                            view! {
                                <h4 style="margin-bottom: 0.5rem; color: var(--error);">{t("project_detail.health_security_issues")}</h4>
                                <div class="card" style="margin-bottom: 1rem; border-left: 3px solid var(--error);">
                                    {data.security_issues.into_iter().map(|w| view! {
                                        <div style="padding: 0.25rem 0; font-size: 0.875rem;">
                                            <span class="badge badge-danger" style="margin-right: 0.5rem;">{w.severity}</span>
                                            {w.message}
                                        </div>
                                    }).collect_view()}
                                </div>
                            }.into_view()
                        } else {
                            view! {
                                <div class="card" style="border-left: 3px solid var(--success); margin-bottom: 1rem;">
                                    <span style="color: var(--success);">{t("project_detail.health_no_security_issues")}</span>
                                </div>
                            }.into_view()
                        }}

                        {if !data.duplicated_rules.is_empty() {
                            view! {
                                <h4 style="margin-bottom: 0.5rem;">{t("project_detail.health_duplicated_rules")}</h4>
                                <div class="table-container">
                                    <table>
                                        <thead><tr><th>{t("project_detail.health_col_text")}</th><th>{t("project_detail.health_col_found_in")}</th><th>{t("project_detail.health_col_also_in")}</th></tr></thead>
                                        <tbody>
                                            {data.duplicated_rules.into_iter().map(|d| view! {
                                                <tr>
                                                    <td style="font-size: 0.8125rem; max-width: 300px; overflow: hidden; text-overflow: ellipsis;">{d.text}</td>
                                                    <td>{d.found_in_project}</td>
                                                    <td>{d.also_in_global}</td>
                                                </tr>
                                            }).collect_view()}
                                        </tbody>
                                    </table>
                                </div>
                            }.into_view()
                        } else {
                            view! {}.into_view()
                        }}
                    }.into_view()
                }
                Err(e) => view! {
                    <div class="empty-state"><p>{t("common.error_prefix")} {e}</p></div>
                }.into_view(),
            })}
        </Suspense>
    }
}
