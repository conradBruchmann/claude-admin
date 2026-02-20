use claude_admin_shared::{RuleFile, RuleUpdateRequest};
use leptos::*;

use crate::api;
use crate::components::markdown_editor::MarkdownEditor;
use crate::i18n::t;

#[component]
pub fn GlobalRulesPage() -> impl IntoView {
    let rules = create_resource(
        || (),
        |_| async move { api::get::<Vec<RuleFile>>("/rules").await },
    );

    let selected_rule = create_rw_signal::<Option<RuleFile>>(None);

    view! {
        <div class="page-header">
            <h2>{t("global_rules.title")}</h2>
            <p>{t("global_rules.subtitle")}</p>
        </div>

        <Suspense fallback=move || view! { <div class="loading">{t("global_rules.loading")}</div> }>
            {move || rules.get().map(|result| match result {
                Ok(data) => {
                    if data.is_empty() {
                        return view! {
                            <div class="empty-state">
                                <p>{t("global_rules.no_rules")}</p>
                                <p style="font-size: 0.8rem;">{t("global_rules.no_rules_hint")}</p>
                            </div>
                        }.into_view();
                    }

                    view! {
                        <div style="display: flex; gap: 1.5rem;">
                            <div style="width: 280px; flex-shrink: 0;">
                                <div class="table-container">
                                    <table>
                                        <thead><tr><th>{t("global_rules.col_rule")}</th></tr></thead>
                                        <tbody>
                                            {data.into_iter().map(|r| {
                                                let rule = r.clone();
                                                view! {
                                                    <tr
                                                        style="cursor: pointer;"
                                                        on:click=move |_| selected_rule.set(Some(rule.clone()))
                                                    >
                                                        <td>{r.name}</td>
                                                    </tr>
                                                }
                                            }).collect_view()}
                                        </tbody>
                                    </table>
                                </div>
                            </div>

                            <div style="flex: 1;">
                                {move || {
                                    if let Some(rule) = selected_rule.get() {
                                        let content_signal = create_rw_signal(rule.content.clone());
                                        let rule_name = rule.name.clone();

                                        view! {
                                            <h3 style="margin-bottom: 1rem;">{t("global_rules.editing")} " " {rule_name.clone()}</h3>
                                            <MarkdownEditor
                                                content=content_signal
                                                on_save=Callback::new(move |content: String| {
                                                    let name = rule_name.clone();
                                                    spawn_local(async move {
                                                        let req = RuleUpdateRequest { content };
                                                        let _ = api::put::<RuleFile, _>(
                                                            &format!("/rules/global/{}", name),
                                                            &req,
                                                        ).await;
                                                    });
                                                })
                                                label="Rule Content"
                                            />
                                        }.into_view()
                                    } else {
                                        view! {
                                            <div class="empty-state"><p>{t("global_rules.select_rule")}</p></div>
                                        }.into_view()
                                    }
                                }}
                            </div>
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
