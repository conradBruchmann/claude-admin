use leptos::*;

use crate::i18n::t;

#[component]
pub fn MarkdownEditor(
    #[prop(into)] content: RwSignal<String>,
    #[prop(into)] on_save: Callback<String>,
    #[prop(optional)] label: &'static str,
) -> impl IntoView {
    let saving = create_rw_signal(false);

    let handle_save = move |_| {
        saving.set(true);
        on_save.call(content.get());
        // Reset after a moment (in real app, wait for response)
        set_timeout(
            move || saving.set(false),
            std::time::Duration::from_millis(500),
        );
    };

    view! {
        <div class="editor-container">
            <div class="editor-toolbar">
                <span>{label}</span>
                <button class="btn btn-primary btn-sm" on:click=handle_save disabled=saving>
                    {move || if saving.get() { t("component.editor.saving") } else { t("common.save") }}
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
