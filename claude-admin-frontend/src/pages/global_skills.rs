use claude_admin_shared::{SkillFile, SkillUpdateRequest};
use leptos::*;

use crate::api;
use crate::components::frontmatter_form::FrontmatterForm;
use crate::components::markdown_editor::MarkdownEditor;
use crate::i18n::t;

#[component]
pub fn GlobalSkillsPage() -> impl IntoView {
    let skills = create_resource(
        || (),
        |_| async move { api::get::<Vec<SkillFile>>("/skills").await },
    );

    let selected_skill = create_rw_signal::<Option<SkillFile>>(None);

    view! {
        <div class="page-header">
            <h2>{t("global_skills.title")}</h2>
            <p>{t("global_skills.subtitle")}</p>
        </div>

        <Suspense fallback=move || view! { <div class="loading">{t("global_skills.loading")}</div> }>
            {move || skills.get().map(|result| match result {
                Ok(data) => {
                    view! {
                        <div style="display: flex; gap: 1.5rem;">
                            // Skill list
                            <div style="width: 280px; flex-shrink: 0;">
                                <div class="table-container">
                                    <table>
                                        <thead><tr><th>{t("global_skills.col_skill")}</th><th>{t("global_skills.col_invocable")}</th></tr></thead>
                                        <tbody>
                                            {data.into_iter().map(|s| {
                                                let skill = s.clone();
                                                view! {
                                                    <tr
                                                        style="cursor: pointer;"
                                                        on:click=move |_| selected_skill.set(Some(skill.clone()))
                                                    >
                                                        <td>{s.name.clone()}</td>
                                                        <td>{if s.frontmatter.user_invocable.unwrap_or(false) {
                                                            view! { <span class="badge badge-success">{t("common.yes")}</span> }.into_view()
                                                        } else {
                                                            view! { <span class="badge badge-muted">{t("common.no")}</span> }.into_view()
                                                        }}</td>
                                                    </tr>
                                                }
                                            }).collect_view()}
                                        </tbody>
                                    </table>
                                </div>
                            </div>

                            // Editor panel
                            <div style="flex: 1;">
                                {move || {
                                    if let Some(skill) = selected_skill.get() {
                                        let fm_signal = create_rw_signal(skill.frontmatter.clone());
                                        let content_signal = create_rw_signal(skill.content.clone());
                                        let skill_name = skill.name.clone();

                                        view! {
                                            <h3 style="margin-bottom: 1rem;">{t("global_skills.editing")} " " {skill_name.clone()}</h3>
                                            <FrontmatterForm frontmatter=fm_signal/>
                                            <MarkdownEditor
                                                content=content_signal
                                                on_save=Callback::new(move |content: String| {
                                                    let name = skill_name.clone();
                                                    let fm = fm_signal.get();
                                                    spawn_local(async move {
                                                        let req = SkillUpdateRequest {
                                                            frontmatter: fm,
                                                            content,
                                                        };
                                                        let _ = api::put::<SkillFile, _>(
                                                            &format!("/skills/global/{}", name),
                                                            &req,
                                                        ).await;
                                                    });
                                                })
                                                label="Skill Content"
                                            />
                                        }.into_view()
                                    } else {
                                        view! {
                                            <div class="empty-state">
                                                <p>{t("global_skills.select_skill")}</p>
                                            </div>
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
