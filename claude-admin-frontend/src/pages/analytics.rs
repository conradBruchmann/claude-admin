use claude_admin_shared::AnalyticsOverview;
use leptos::*;

use crate::api;
use crate::i18n::t;

#[component]
pub fn AnalyticsPage() -> impl IntoView {
    let analytics = create_resource(
        || (),
        |_| async move { api::get::<AnalyticsOverview>("/analytics/overview").await },
    );

    view! {
        <div class="page-header">
            <h2>{t("analytics.title")}</h2>
            <p>{t("analytics.subtitle")}</p>
        </div>

        <Suspense fallback=move || view! { <div class="loading">{t("analytics.loading")}</div> }>
            {move || analytics.get().map(|result| match result {
                Ok(data) => {
                    let daily = data.daily_activity.clone();
                    let hours = data.hour_distribution.clone();
                    let models = data.model_usage.clone();
                    let tools = data.tool_ranking.clone();
                    let langs = data.language_breakdown.clone();
                    let outcomes = data.outcome_distribution.clone();

                    view! {
                        // Summary cards
                        <div class="card-grid">
                            <div class="card">
                                <div class="card-value">{data.total_sessions}</div>
                                <div class="card-label">{t("analytics.total_sessions")}</div>
                            </div>
                            <div class="card">
                                <div class="card-value">{format_number(data.total_messages)}</div>
                                <div class="card-label">{t("analytics.total_messages")}</div>
                            </div>
                            <div class="card">
                                <div class="card-value">{data.total_git_commits}</div>
                                <div class="card-label">{t("analytics.git_commits")}</div>
                            </div>
                            <div class="card">
                                <div class="card-value">{format_number(data.total_lines_added)}</div>
                                <div class="card-label">{t("analytics.lines_added")}</div>
                            </div>
                        </div>

                        // Activity Heatmap
                        <h3 style="margin-bottom: 1rem;">{t("analytics.activity_heatmap")}</h3>
                        <div class="card" style="margin-bottom: 2rem; overflow-x: auto;">
                            <div class="heatmap-grid">
                                {daily.iter().map(|d| {
                                    let intensity = heatmap_intensity(d.message_count);
                                    let title = format!("{}: {} messages, {} sessions", d.date, d.message_count, d.session_count);
                                    view! {
                                        <div
                                            class="heatmap-cell"
                                            style=format!("background: rgba(249, 115, 22, {});", intensity)
                                            title=title
                                        />
                                    }
                                }).collect_view()}
                            </div>
                        </div>

                        // Hour distribution
                        <h3 style="margin-bottom: 1rem;">{t("analytics.hourly_distribution")}</h3>
                        <div class="card" style="margin-bottom: 2rem;">
                            <div class="hour-chart">
                                {hours.iter().map(|(hour, count)| {
                                    let max_count = hours.iter().map(|(_, c)| *c).max().unwrap_or(1);
                                    let pct = (*count as f64 / max_count as f64 * 100.0) as u64;
                                    view! {
                                        <div class="hour-bar-container">
                                            <div class="hour-bar" style=format!("height: {}%;", pct)/>
                                            <div class="hour-label">{format!("{:02}", hour)}</div>
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        </div>

                        // Model usage
                        <h3 style="margin-bottom: 1rem;">{t("analytics.model_usage")}</h3>
                        <div class="table-container" style="margin-bottom: 2rem;">
                            <table>
                                <thead>
                                    <tr>
                                        <th>{t("analytics.col_model")}</th>
                                        <th>{t("analytics.col_input_tokens")}</th>
                                        <th>{t("analytics.col_output_tokens")}</th>
                                        <th>{t("analytics.col_cache_read")}</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {models.into_iter().map(|m| view! {
                                        <tr>
                                            <td>{m.model}</td>
                                            <td>{format_number(m.input_tokens)}</td>
                                            <td>{format_number(m.output_tokens)}</td>
                                            <td>{format_number(m.cache_read_tokens)}</td>
                                        </tr>
                                    }).collect_view()}
                                </tbody>
                            </table>
                        </div>

                        // Two-column layout for tools and languages
                        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 2rem; margin-bottom: 2rem;">
                            // Tool ranking
                            <div>
                                <h3 style="margin-bottom: 1rem;">{t("analytics.tool_usage_top10")}</h3>
                                <div class="card">
                                    {tools.iter().take(10).map(|(name, count)| {
                                        let max = tools.first().map(|(_, c)| *c).unwrap_or(1);
                                        let pct = (*count as f64 / max as f64 * 100.0) as u64;
                                        view! {
                                            <div style="margin-bottom: 0.75rem;">
                                                <div style="display: flex; justify-content: space-between; font-size: 0.8125rem; margin-bottom: 0.25rem;">
                                                    <span>{name}</span>
                                                    <span style="color: var(--text-muted);">{format_number(*count)}</span>
                                                </div>
                                                <div class="progress-bar">
                                                    <div class="progress-fill" style=format!("width: {}%;", pct)/>
                                                </div>
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>
                            </div>

                            // Language breakdown
                            <div>
                                <h3 style="margin-bottom: 1rem;">{t("analytics.languages")}</h3>
                                <div class="card">
                                    {langs.iter().take(10).map(|(name, count)| {
                                        let max = langs.first().map(|(_, c)| *c).unwrap_or(1);
                                        let pct = (*count as f64 / max as f64 * 100.0) as u64;
                                        view! {
                                            <div style="margin-bottom: 0.75rem;">
                                                <div style="display: flex; justify-content: space-between; font-size: 0.8125rem; margin-bottom: 0.25rem;">
                                                    <span>{name}</span>
                                                    <span style="color: var(--text-muted);">{format_number(*count)}</span>
                                                </div>
                                                <div class="progress-bar">
                                                    <div class="progress-fill progress-blue" style=format!("width: {}%;", pct)/>
                                                </div>
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>
                            </div>
                        </div>

                        // Outcome distribution
                        {if !outcomes.is_empty() {
                            view! {
                                <h3 style="margin-bottom: 1rem;">{t("analytics.session_outcomes")}</h3>
                                <div class="card">
                                    {outcomes.into_iter().map(|(outcome, count)| view! {
                                        <div style="display: flex; justify-content: space-between; padding: 0.375rem 0; border-bottom: 1px solid var(--border);">
                                            <span style="font-size: 0.875rem;">{outcome}</span>
                                            <span class="badge badge-muted">{count}</span>
                                        </div>
                                    }).collect_view()}
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

fn format_number(n: u64) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}

fn heatmap_intensity(count: u64) -> f64 {
    if count == 0 {
        0.05
    } else if count < 10 {
        0.2
    } else if count < 50 {
        0.4
    } else if count < 200 {
        0.6
    } else if count < 500 {
        0.8
    } else {
        1.0
    }
}
