use leptos::*;

/// Onboarding wizard shown when the user has no rules and few skills.
/// Detects: rules_count == 0 && skills_count < 3
#[component]
pub fn OnboardingBanner(rules_count: usize, skills_count: usize) -> impl IntoView {
    let show = create_rw_signal(rules_count == 0 && skills_count < 3);
    let dismissed = create_rw_signal(false);

    // Check localStorage for dismissal
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if storage
                .get_item("claude_admin_onboarding_dismissed")
                .ok()
                .flatten()
                .as_deref()
                == Some("true")
            {
                dismissed.set(true);
            }
        }
    }

    let dismiss = move |_| {
        show.set(false);
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("claude_admin_onboarding_dismissed", "true");
            }
        }
    };

    view! {
        {move || {
            if show.get() && !dismissed.get() {
                view! {
                    <div class="onboarding-banner">
                        <div class="onboarding-header">
                            <h3>"Getting Started with Claude Code"</h3>
                            <button class="btn btn-sm" on:click=dismiss>"Dismiss"</button>
                        </div>
                        <div class="onboarding-steps">
                            <div class="onboarding-step">
                                <div class="onboarding-step-number">"1"</div>
                                <div>
                                    <strong>"Create a CLAUDE.md"</strong>
                                    <p>"Add project instructions to guide Claude's behavior."</p>
                                    <a href="/projects" class="btn btn-sm btn-primary" style="margin-top: 0.5rem;">"Go to Projects"</a>
                                </div>
                            </div>
                            <div class="onboarding-step">
                                <div class="onboarding-step-number">"2"</div>
                                <div>
                                    <strong>"Add a Rule"</strong>
                                    <p>"Create coding conventions or constraints."</p>
                                    <a href="/rules" class="btn btn-sm btn-primary" style="margin-top: 0.5rem;">"Go to Rules"</a>
                                </div>
                            </div>
                            <div class="onboarding-step">
                                <div class="onboarding-step-number">"3"</div>
                                <div>
                                    <strong>"Install a Skill"</strong>
                                    <p>"Browse and install reusable skills."</p>
                                    <a href="/skill-browser" class="btn btn-sm btn-primary" style="margin-top: 0.5rem;">"Skill Browser"</a>
                                </div>
                            </div>
                        </div>
                    </div>
                }.into_view()
            } else {
                view! {}.into_view()
            }
        }}
    }
}
