use claude_admin_shared::{BrowsableSkill, SkillInstallRequest};
use leptos::*;

use crate::api;
use crate::i18n::t;

#[component]
pub fn SkillBrowserPage() -> impl IntoView {
    let active_tab = create_rw_signal("official".to_string());
    let search_query = create_rw_signal(String::new());
    let install_status = create_rw_signal::<Option<(String, String)>>(None);

    let official = create_resource(
        || (),
        |_| async move { api::get::<Vec<BrowsableSkill>>("/skill-browser/official").await },
    );

    let community = create_resource(
        || (),
        |_| async move { api::get::<Vec<BrowsableSkill>>("/skill-browser/community").await },
    );

    view! {
        <div class="page-header">
            <h2>{t("skill_browser.title")}</h2>
            <p>{t("skill_browser.subtitle")}</p>
        </div>

        <div class="tabs">
            <button
                class=move || if active_tab.get() == "official" { "tab active" } else { "tab" }
                on:click=move |_| active_tab.set("official".to_string())
            >{t("skill_browser.tab_official")}</button>
            <button
                class=move || if active_tab.get() == "community" { "tab active" } else { "tab" }
                on:click=move |_| active_tab.set("community".to_string())
            >{t("skill_browser.tab_community")}</button>
        </div>

        <div style="margin-bottom: 1rem;">
            <input
                type="text"
                placeholder=t("skill_browser.search_placeholder")
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
                        view! { <span>"'" {name} "' " {t("skill_browser.install_success")}</span> }.into_view()
                    } else {
                        view! { <span>{t("skill_browser.install_failed")} " '" {name} "': " {status}</span> }.into_view()
                    }}
                </span>
            </div>
        })}

        <Suspense fallback=move || view! { <div class="loading">{t("skill_browser.loading")}</div> }>
            {move || {
                let tab = active_tab.get();
                let query = search_query.get().to_lowercase();

                let skills_result = if tab == "official" {
                    official.get()
                } else {
                    community.get()
                };

                skills_result.map(|result| match result {
                    Ok(skills) => {
                        let filtered: Vec<BrowsableSkill> = skills
                            .into_iter()
                            .filter(|s| {
                                if query.is_empty() {
                                    true
                                } else {
                                    s.name.to_lowercase().contains(&query)
                                        || s.description.to_lowercase().contains(&query)
                                }
                            })
                            .collect();

                        if filtered.is_empty() {
                            view! {
                                <div class="empty-state"><p>{t("skill_browser.no_results")}</p></div>
                            }.into_view()
                        } else {
                            view! {
                                <div class="skill-grid">
                                    {filtered.into_iter().map(|skill| {
                                        let name = skill.name.clone();
                                        let name_for_install = skill.name.clone();
                                        let installed = skill.installed;

                                        view! {
                                            <div class="card skill-card">
                                                <div style="display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 0.5rem;">
                                                    <div>
                                                        <span style="font-weight: 600; font-size: 1rem;">{name}</span>
                                                        {skill.category.map(|cat| view! {
                                                            <span class="badge badge-muted" style="margin-left: 0.5rem;">{cat}</span>
                                                        })}
                                                    </div>
                                                    {if installed {
                                                        view! { <span class="badge badge-success">{t("skill_browser.installed")}</span> }.into_view()
                                                    } else {
                                                        view! {
                                                            <button
                                                                class="btn btn-primary btn-sm"
                                                                on:click=move |_| {
                                                                    let name = name_for_install.clone();
                                                                    install_status.set(None);
                                                                    spawn_local(async move {
                                                                        let req = SkillInstallRequest {
                                                                            name: name.clone(),
                                                                            content: format!(
                                                                                "---\ndescription: Installed via ClaudeAdmin\nuser_invocable: true\n---\n\n# {}\n\nInstalled from Skill Browser.\n",
                                                                                name
                                                                            ),
                                                                        };
                                                                        match api::post::<serde_json::Value, _>(
                                                                            "/skill-browser/install",
                                                                            &req
                                                                        ).await {
                                                                            Ok(_) => {
                                                                                install_status.set(Some((name, "installed".to_string())));
                                                                            }
                                                                            Err(e) => {
                                                                                install_status.set(Some((name, e)));
                                                                            }
                                                                        }
                                                                    });
                                                                }
                                                            >
                                                                {t("skill_browser.install")}
                                                            </button>
                                                        }.into_view()
                                                    }}
                                                </div>
                                                <p style="color: var(--text-secondary); font-size: 0.875rem; line-height: 1.5;">
                                                    {skill.description}
                                                </p>
                                                <div style="margin-top: 0.5rem; font-size: 0.75rem; color: var(--text-muted);">
                                                    {skill.repo}
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
