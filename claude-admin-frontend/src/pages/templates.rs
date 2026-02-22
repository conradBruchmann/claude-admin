use claude_admin_shared::{ConfigTemplate, TemplateApplyResult};
use leptos::*;

use crate::api;
use crate::i18n::t;

#[component]
pub fn TemplatesPage() -> impl IntoView {
    let apply_status =
        create_rw_signal::<Option<(String, Result<TemplateApplyResult, String>)>>(None);

    let templates = create_resource(
        || (),
        |_| async move { api::get::<Vec<ConfigTemplate>>("/templates").await },
    );

    view! {
        <div class="page-header">
            <h2>{t("templates.title")}</h2>
            <p>{t("templates.subtitle")}</p>
        </div>

        // Apply status banner
        {move || apply_status.get().map(|(name, result)| {
            let (border_color, message) = match &result {
                Ok(r) => (
                    "var(--success)",
                    format!(
                        "'{}' {}  ({} rules, {} skills{})",
                        name,
                        "applied",
                        r.rules_created,
                        r.skills_created,
                        if r.claude_md_updated { ", CLAUDE.md updated" } else { "" },
                    ),
                ),
                Err(e) => (
                    "var(--error)",
                    format!("Failed to apply '{}': {}", name, e),
                ),
            };
            view! {
                <div class="card" style=format!("margin-bottom: 1.5rem; border-left: 3px solid {};", border_color)>
                    <span style="font-size: 0.875rem;">{message}</span>
                </div>
            }
        })}

        <Suspense fallback=move || view! { <div class="loading">{t("templates.loading")}</div> }>
            {move || templates.get().map(|result| match result {
                Ok(data) => {
                    if data.is_empty() {
                        view! {
                            <div class="empty-state">
                                <p>{t("search.no_results")}</p>
                            </div>
                        }.into_view()
                    } else {
                        view! {
                            <div class="skill-grid">
                                {data.into_iter().map(|tmpl| {
                                    let name_for_apply = tmpl.name.clone();
                                    let name_display = tmpl.name.clone();

                                    view! {
                                        <div class="card skill-card">
                                            // Header row: name + category badge + Apply button
                                            <div style="display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 0.5rem;">
                                                <div>
                                                    <span style="font-weight: 600; font-size: 1rem;">{name_display.clone()}</span>
                                                    <span class="badge badge-muted" style="margin-left: 0.5rem;">{tmpl.category.clone()}</span>
                                                </div>
                                                <button
                                                    class="btn btn-primary btn-sm"
                                                    on:click=move |_| {
                                                        let name = name_for_apply.clone();
                                                        if !web_sys::window()
                                                            .and_then(|w| w.confirm_with_message(&t("templates.confirm").get()).ok())
                                                            .unwrap_or(false)
                                                        {
                                                            return;
                                                        }
                                                        apply_status.set(None);
                                                        spawn_local(async move {
                                                            let outcome = api::post::<TemplateApplyResult, ()>(
                                                                &format!("/templates/{}/apply", name),
                                                                &(),
                                                            )
                                                            .await;
                                                            apply_status.set(Some((name, outcome)));
                                                            templates.refetch();
                                                        });
                                                    }
                                                >
                                                    {t("templates.apply")}
                                                </button>
                                            </div>

                                            // Description
                                            <p style="color: var(--text-secondary); font-size: 0.875rem; line-height: 1.5; margin-bottom: 0.75rem;">
                                                {tmpl.description}
                                            </p>

                                            // Content summary: rules + skills counts
                                            <div style="display: flex; gap: 0.5rem; flex-wrap: wrap; font-size: 0.75rem; color: var(--text-muted);">
                                                {if !tmpl.rules.is_empty() {
                                                    view! {
                                                        <span class="badge badge-muted">
                                                            {tmpl.rules.len()} " rules"
                                                        </span>
                                                    }.into_view()
                                                } else {
                                                    view! {}.into_view()
                                                }}
                                                {if !tmpl.skills.is_empty() {
                                                    view! {
                                                        <span class="badge badge-muted">
                                                            {tmpl.skills.len()} " skills"
                                                        </span>
                                                    }.into_view()
                                                } else {
                                                    view! {}.into_view()
                                                }}
                                                {if tmpl.claude_md_snippet.is_some() {
                                                    view! {
                                                        <span class="badge badge-muted">"CLAUDE.md snippet"</span>
                                                    }.into_view()
                                                } else {
                                                    view! {}.into_view()
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
            })}
        </Suspense>
    }
}
