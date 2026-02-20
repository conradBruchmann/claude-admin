use claude_admin_shared::{ProjectStatus, ProjectSummaryLite};
use leptos::*;

use crate::api;
use crate::i18n::t;

#[component]
pub fn ProjectsPage() -> impl IntoView {
    let projects = create_resource(
        || (),
        |_| async move { api::get::<Vec<ProjectSummaryLite>>("/projects").await },
    );

    view! {
        <div class="page-header">
            <h2>{t("projects.title")}</h2>
            <p>{t("projects.subtitle")}</p>
        </div>

        <Suspense fallback=move || view! { <div class="loading">{t("projects.loading")}</div> }>
            {move || projects.get().map(|result| match result {
                Ok(data) => view! {
                    <div class="table-container">
                        <table>
                            <thead>
                                <tr>
                                    <th>{t("projects.col_name")}</th>
                                    <th>{t("projects.col_path")}</th>
                                    <th>{t("projects.col_claude_md")}</th>
                                    <th>{t("projects.col_skills")}</th>
                                    <th>{t("projects.col_rules")}</th>
                                    <th>{t("projects.col_memory")}</th>
                                </tr>
                            </thead>
                            <tbody>
                                {data.into_iter().map(|p| {
                                    view! { <ProjectRow project=p/> }
                                }).collect_view()}
                            </tbody>
                        </table>
                    </div>
                }.into_view(),
                Err(e) => view! {
                    <div class="empty-state">
                        <p>{t("projects.error_loading")} {e}</p>
                    </div>
                }.into_view(),
            })}
        </Suspense>
    }
}

/// Single project row - loads status badges JIT.
#[component]
fn ProjectRow(project: ProjectSummaryLite) -> impl IntoView {
    let encoded = project.encoded_path.clone();
    let href = format!("/projects/{}", project.encoded_path);
    let path = project.path.clone();

    let status = create_resource(
        move || encoded.clone(),
        |id| async move { api::get::<ProjectStatus>(&format!("/projects/{}/status", id)).await },
    );

    view! {
        <tr>
            <td>
                <a class="table-link" href=href>{project.name}</a>
            </td>
            <td style="color: var(--text-muted); font-size: 0.8rem;">{path}</td>
            {move || match status.get() {
                Some(Ok(s)) => view! {
                    <td>{badge(s.has_claude_md)}</td>
                    <td>{badge(s.has_skills)}</td>
                    <td>{badge(s.has_rules)}</td>
                    <td>{badge(s.has_memory)}</td>
                }.into_view(),
                Some(Err(_)) => view! {
                    <td>{badge_placeholder("?")}</td>
                    <td>{badge_placeholder("?")}</td>
                    <td>{badge_placeholder("?")}</td>
                    <td>{badge_placeholder("?")}</td>
                }.into_view(),
                None => view! {
                    <td>{badge_loading()}</td>
                    <td>{badge_loading()}</td>
                    <td>{badge_loading()}</td>
                    <td>{badge_loading()}</td>
                }.into_view(),
            }}
        </tr>
    }
}

fn badge(has: bool) -> impl IntoView {
    if has {
        view! { <span class="badge badge-success">{t("projects.yes")}</span> }
    } else {
        view! { <span class="badge badge-muted">"-"</span> }
    }
}

fn badge_placeholder(text: &'static str) -> impl IntoView {
    view! { <span class="badge badge-muted">{text}</span> }
}

fn badge_loading() -> impl IntoView {
    view! { <span class="badge badge-muted" style="opacity: 0.5;">"..."</span> }
}
