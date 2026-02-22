use claude_admin_shared::BackupEntry;
use leptos::*;

use crate::api;
use crate::i18n::t;

fn format_size(bytes: u64) -> String {
    if bytes >= 1_048_576 {
        format!("{:.1} MB", bytes as f64 / 1_048_576.0)
    } else if bytes >= 1_024 {
        format!("{:.1} KB", bytes as f64 / 1_024.0)
    } else {
        format!("{} B", bytes)
    }
}

#[component]
pub fn BackupsPage() -> impl IntoView {
    let backups = create_resource(
        || (),
        |_| async move { api::get::<Vec<BackupEntry>>("/backups").await },
    );

    let action_status = create_rw_signal::<Option<(String, bool)>>(None); // (message, is_error)

    view! {
        <div class="page-header">
            <h2>{t("backups.title")}</h2>
            <p>{t("backups.subtitle")}</p>
        </div>

        {move || action_status.get().map(|(msg, is_error)| view! {
            <div style=move || format!(
                "margin-bottom: 1rem; padding: 0.75rem 1rem; border-radius: 6px; font-size: 0.875rem; {}",
                if is_error {
                    "background: var(--error-bg, #fee2e2); color: var(--error, #ef4444); border: 1px solid var(--error, #ef4444);"
                } else {
                    "background: var(--success-bg, #dcfce7); color: var(--success, #22c55e); border: 1px solid var(--success, #22c55e);"
                }
            )>
                {msg}
            </div>
        })}

        <Suspense fallback=move || view! { <div class="loading">{t("backups.loading")}</div> }>
            {move || backups.get().map(|result| match result {
                Ok(data) => {
                    if data.is_empty() {
                        return view! {
                            <div class="empty-state">
                                <p>{t("backups.no_backups")}</p>
                            </div>
                        }.into_view();
                    }

                    view! {
                        <div class="table-container">
                            <table>
                                <thead>
                                    <tr>
                                        <th>{t("backups.col_name")}</th>
                                        <th>{t("backups.col_size")}</th>
                                        <th>{t("backups.col_created")}</th>
                                        <th>{t("backups.col_original")}</th>
                                        <th>{t("backups.col_actions")}</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {data.into_iter().map(|entry| {
                                        let name_restore = entry.name.clone();
                                        let name_delete = entry.name.clone();
                                        let size_display = format_size(entry.size_bytes);

                                        view! {
                                            <tr>
                                                <td style="font-family: monospace; font-size: 0.8rem; word-break: break-all; max-width: 260px;">
                                                    {entry.name.clone()}
                                                </td>
                                                <td style="white-space: nowrap; color: var(--text-muted); font-size: 0.875rem;">
                                                    {size_display}
                                                </td>
                                                <td style="white-space: nowrap; font-size: 0.875rem; color: var(--text-muted);">
                                                    {entry.created}
                                                </td>
                                                <td style="font-family: monospace; font-size: 0.75rem; word-break: break-all; max-width: 300px; color: var(--text-secondary);">
                                                    {entry.original_path}
                                                </td>
                                                <td style="white-space: nowrap;">
                                                    <div style="display: flex; gap: 0.5rem;">
                                                        <button
                                                            class="btn btn-sm btn-secondary"
                                                            on:click=move |_| {
                                                                let name = name_restore.clone();
                                                                action_status.set(None);
                                                                spawn_local(async move {
                                                                    match api::post::<serde_json::Value, ()>(
                                                                        &format!("/backups/{}/restore", name),
                                                                        &(),
                                                                    ).await {
                                                                        Ok(_) => action_status.set(Some((
                                                                            t("backups.restored").get_untracked(),
                                                                            false,
                                                                        ))),
                                                                        Err(e) => action_status.set(Some((
                                                                            format!("{} {}", t("common.error_prefix").get_untracked(), e),
                                                                            true,
                                                                        ))),
                                                                    }
                                                                });
                                                            }
                                                        >
                                                            {t("backups.restore")}
                                                        </button>
                                                        <button
                                                            class="btn btn-sm btn-danger"
                                                            on:click=move |_| {
                                                                let name = name_delete.clone();
                                                                action_status.set(None);
                                                                spawn_local(async move {
                                                                    match api::delete(
                                                                        &format!("/backups/{}", name),
                                                                    ).await {
                                                                        Ok(_) => action_status.set(Some((
                                                                            t("backups.deleted").get_untracked(),
                                                                            false,
                                                                        ))),
                                                                        Err(e) => action_status.set(Some((
                                                                            format!("{} {}", t("common.error_prefix").get_untracked(), e),
                                                                            true,
                                                                        ))),
                                                                    }
                                                                });
                                                            }
                                                        >
                                                            {t("backups.delete")}
                                                        </button>
                                                    </div>
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
