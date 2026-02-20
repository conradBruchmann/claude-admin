use leptos::*;

use crate::i18n::t;

#[component]
pub fn JsonEditor(
    #[prop(into)] content: RwSignal<String>,
    #[prop(into)] on_save: Callback<String>,
    #[prop(optional)] label: &'static str,
) -> impl IntoView {
    let is_valid =
        create_memo(move |_| serde_json::from_str::<serde_json::Value>(&content.get()).is_ok());

    let handle_save = move |_| {
        if is_valid.get() {
            on_save.call(content.get());
        }
    };

    view! {
        <div class="editor-container">
            <div class="editor-toolbar">
                <div>
                    <span>{label}</span>
                    {move || if !is_valid.get() {
                        view! { <span style="color: var(--error); margin-left: 0.5rem;">" " {t("component.json_editor.invalid")}</span> }.into_view()
                    } else {
                        view! { <span style="color: var(--success); margin-left: 0.5rem;">" " {t("component.json_editor.valid")}</span> }.into_view()
                    }}
                </div>
                <button
                    class="btn btn-primary btn-sm"
                    on:click=handle_save
                    disabled=move || !is_valid.get()
                >
                    {t("common.save")}
                </button>
            </div>
            <textarea
                class="editor-textarea"
                prop:value=move || content.get()
                on:input=move |ev| {
                    content.set(event_target_value(&ev));
                }
            />
        </div>
    }
}
