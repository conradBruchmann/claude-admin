use crate::components::language_selector::LanguageSelector;
use crate::i18n::t;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <aside class="sidebar">
            <div class="sidebar-header">
                <h1>{t("app.title")}</h1>
                <div class="subtitle">{t("app.subtitle")}</div>
                <div style="margin-top: 0.5rem;">
                    <LanguageSelector/>
                </div>
            </div>
            <nav>
                <div class="nav-section">{t("sidebar.overview")}</div>
                <A href="" class="nav-link" exact=true>
                    <span class="nav-icon">"#"</span>
                    {t("sidebar.dashboard")}
                </A>
                <A href="/analytics" class="nav-link">
                    <span class="nav-icon">"~"</span>
                    {t("sidebar.analytics")}
                </A>

                <div class="nav-section">{t("sidebar.manage")}</div>
                <A href="/projects" class="nav-link">
                    <span class="nav-icon">"P"</span>
                    {t("sidebar.projects")}
                </A>
                <A href="/skills" class="nav-link">
                    <span class="nav-icon">"S"</span>
                    {t("sidebar.global_skills")}
                </A>
                <A href="/skill-browser" class="nav-link">
                    <span class="nav-icon">"+"</span>
                    {t("sidebar.skill_browser")}
                </A>
                <A href="/rules" class="nav-link">
                    <span class="nav-icon">"R"</span>
                    {t("sidebar.global_rules")}
                </A>
                <A href="/plans" class="nav-link">
                    <span class="nav-icon">">"</span>
                    {t("sidebar.plans")}
                </A>
                <A href="/mcp" class="nav-link">
                    <span class="nav-icon">"M"</span>
                    {t("sidebar.mcp_servers")}
                </A>
                <A href="/mcp-browser" class="nav-link">
                    <span class="nav-icon">"^"</span>
                    {t("sidebar.mcp_browser")}
                </A>

                <div class="nav-section">{t("sidebar.security")}</div>
                <A href="/permissions" class="nav-link">
                    <span class="nav-icon">"!"</span>
                    {t("sidebar.permissions")}
                </A>
                <A href="/health" class="nav-link">
                    <span class="nav-icon">"H"</span>
                    {t("sidebar.config_health")}
                </A>

                <div class="nav-section">{t("sidebar.system")}</div>
                <A href="/settings" class="nav-link">
                    <span class="nav-icon">"*"</span>
                    {t("sidebar.settings")}
                </A>
                <A href="/sessions" class="nav-link">
                    <span class="nav-icon">"@"</span>
                    {t("sidebar.sessions")}
                </A>
                <A href="/github" class="nav-link">
                    <span class="nav-icon">"G"</span>
                    {t("sidebar.github")}
                </A>

                <div class="nav-section">{t("sidebar.learn")}</div>
                <A href="/docs" class="nav-link">
                    <span class="nav-icon">"D"</span>
                    {t("sidebar.docs")}
                </A>
                <A href="/help" class="nav-link">
                    <span class="nav-icon">"?"</span>
                    {t("sidebar.help")}
                </A>
                <A href="/licenses" class="nav-link">
                    <span class="nav-icon">"L"</span>
                    {t("sidebar.licenses")}
                </A>
            </nav>
        </aside>
    }
}
