use std::collections::HashMap;
use std::sync::OnceLock;

static TRANSLATIONS: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();

pub fn translations() -> &'static HashMap<&'static str, &'static str> {
    TRANSLATIONS.get_or_init(|| {
        let mut m = HashMap::new();

        // ── App ──
        m.insert("app.title", "ClaudeAdmin");
        m.insert("app.subtitle", "Configuration Manager");

        // ── Sidebar ──
        m.insert("sidebar.overview", "Übersicht");
        m.insert("sidebar.dashboard", "Dashboard");
        m.insert("sidebar.analytics", "Analytik");
        m.insert("sidebar.manage", "Verwalten");
        m.insert("sidebar.projects", "Projekte");
        m.insert("sidebar.global_skills", "Globale Skills");
        m.insert("sidebar.skill_browser", "Skill Browser");
        m.insert("sidebar.global_rules", "Globale Regeln");
        m.insert("sidebar.plans", "Pläne");
        m.insert("sidebar.mcp_servers", "MCP Servers");
        m.insert("sidebar.mcp_browser", "MCP Browser");
        m.insert("sidebar.security", "Sicherheit");
        m.insert("sidebar.permissions", "Berechtigungen");
        m.insert("sidebar.config_health", "Config Health");
        m.insert("sidebar.system", "System");
        m.insert("sidebar.settings", "Einstellungen");
        m.insert("sidebar.sessions", "Sitzungen");
        m.insert("sidebar.github", "GitHub");
        m.insert("sidebar.learn", "Lernen");
        m.insert("sidebar.docs", "Dokumentation");
        m.insert("sidebar.help", "System Info");

        // ── Dashboard ──
        m.insert("dashboard.title", "Dashboard");
        m.insert("dashboard.subtitle", "Überblick über deine Claude Code Konfiguration");
        m.insert("dashboard.projects", "Projekte");
        m.insert("dashboard.global_skills", "Globale Skills");
        m.insert("dashboard.global_rules", "Globale Regeln");
        m.insert("dashboard.mcp_servers", "MCP Servers");
        m.insert("dashboard.plans", "Pläne");
        m.insert("dashboard.config_health", "Config Health");
        m.insert("dashboard.recent_projects", "Aktuelle Projekte");
        m.insert("dashboard.loading", "Laden");
        m.insert("dashboard.error_loading", "Fehler beim Laden des Dashboards");
        m.insert("dashboard.col_name", "Name");
        m.insert("dashboard.col_claude_md", "CLAUDE.md");
        m.insert("dashboard.col_skills", "Skills");
        m.insert("dashboard.col_rules", "Regeln");
        m.insert("dashboard.col_memory", "Memory");
        m.insert("dashboard.yes", "Ja");

        // ── MCP ──
        m.insert("mcp.title", "MCP Servers");
        m.insert("mcp.subtitle", "Model Context Protocol Server für Claude Code verwalten");
        m.insert("mcp.tab_servers", "Servers");
        m.insert("mcp.tab_health", "Health Check");
        m.insert("mcp.tab_add", "Neuer Server");
        m.insert("mcp.loading", "MCP Servers laden");
        m.insert("mcp.no_servers", "Keine MCP Server konfiguriert");
        m.insert("mcp.no_servers_hint", "Füge Server über den Tab 'Neuer Server' oder den MCP Browser hinzu.");
        m.insert("mcp.select_server", "Wähle einen Server aus der Liste um die Konfiguration anzuzeigen.");
        m.insert("mcp.no_servers_configured", "Keine Server konfiguriert.");
        m.insert("mcp.check_health", "Health Check");
        m.insert("mcp.save", "Speichern");
        m.insert("mcp.delete", "Löschen");
        m.insert("mcp.saved", "Gespeichert!");
        m.insert("mcp.deleted", "Gelöscht!");
        m.insert("mcp.read_only", "Nur lesen");
        m.insert("mcp.read_only_hint", "Dieser Server wird extern verwaltet und kann hier nicht bearbeitet werden.");
        m.insert("mcp.health.title", "MCP Server Health");
        m.insert("mcp.health.check_all", "Alle Server prüfen");
        m.insert("mcp.health.checking", "Prüfe...");
        m.insert("mcp.health.description", "Startet jeden MCP Server Prozess, sendet JSON-RPC initialize + tools/list und zeigt die Ergebnisse. Timeout: 10 Sekunden pro Server.");
        m.insert("mcp.health.col_name", "Name");
        m.insert("mcp.health.col_source", "Quelle");
        m.insert("mcp.health.col_status", "Status");
        m.insert("mcp.health.col_server_info", "Server Info");
        m.insert("mcp.health.col_tools", "Tools");
        m.insert("mcp.health.col_duration", "Dauer");
        m.insert("mcp.health.running", "Läuft");
        m.insert("mcp.health.error", "Fehler");
        m.insert("mcp.health.timeout", "Timeout");
        m.insert("mcp.health.unknown", "Unbekannt");
        m.insert("mcp.add.title", "MCP Server hinzufügen");
        m.insert("mcp.add.description", "Einen neuen MCP Server zur globalen ~/.claude.json Konfiguration hinzufügen.");
        m.insert("mcp.add.name_label", "Server Name");
        m.insert("mcp.add.name_placeholder", "z.B. my-server");
        m.insert("mcp.add.config_label", "Server Konfiguration (JSON)");
        m.insert("mcp.add.submit", "Server hinzufügen");
        m.insert("mcp.add.name_required", "Bitte gib einen Server-Namen ein");
        m.insert("mcp.source.claude_code", "Claude Code");
        m.insert("mcp.source.claude_desktop", "Claude Desktop");

        // ── MCP Browser ──
        m.insert("mcp_browser.title", "MCP Browser");
        m.insert("mcp_browser.subtitle", "MCP Server für Claude Code entdecken und installieren");
        m.insert("mcp_browser.search_placeholder", "MCP Server suchen...");
        m.insert("mcp_browser.loading", "MCP Katalog laden");
        m.insert("mcp_browser.no_results", "Keine MCP Server gefunden");
        m.insert("mcp_browser.installed", "Installiert");
        m.insert("mcp_browser.install", "Installieren");
        m.insert("mcp_browser.needs_api_key", "Benötigt API Key");
        m.insert("mcp_browser.install_success", "erfolgreich installiert!");
        m.insert("mcp_browser.install_failed", "Installation fehlgeschlagen");

        // ── Projects ──
        m.insert("projects.title", "Projekte");
        m.insert("projects.subtitle", "Alle in ~/.claude.json registrierten Projekte");
        m.insert("projects.loading", "Laden");
        m.insert("projects.error_loading", "Fehler beim Laden der Projekte: ");
        m.insert("projects.col_name", "Name");
        m.insert("projects.col_path", "Pfad");
        m.insert("projects.col_claude_md", "CLAUDE.md");
        m.insert("projects.col_skills", "Skills");
        m.insert("projects.col_rules", "Regeln");
        m.insert("projects.col_memory", "Memory");
        m.insert("projects.yes", "Ja");

        // ── Project Detail ──
        m.insert("project_detail.loading", "Lade Projektdetails");
        m.insert("project_detail.error_loading", "Fehler beim Laden des Projekts");
        m.insert("project_detail.tab_advisor", "Advisor");
        m.insert("project_detail.tab_claude_md", "CLAUDE.md");
        m.insert("project_detail.tab_skills", "Skills");
        m.insert("project_detail.tab_rules", "Regeln");
        m.insert("project_detail.tab_memory", "Memory");
        m.insert("project_detail.tab_permissions", "Berechtigungen");
        m.insert("project_detail.tab_health", "Health");
        m.insert("project_detail.no_claude_md", "Keine CLAUDE.md vorhanden");
        m.insert("project_detail.no_claude_md_hint", "Erstelle eine CLAUDE.md in deinem Projektverzeichnis um Claude Code Anweisungen zu geben.");
        m.insert("project_detail.no_skills", "Keine Skills für dieses Projekt");
        m.insert("project_detail.no_rules", "Keine Regeln für dieses Projekt");
        m.insert("project_detail.no_memory", "Kein Memory für dieses Projekt");
        m.insert("project_detail.save", "Speichern");
        m.insert("project_detail.saved", "Gespeichert!");
        m.insert("project_detail.skill_scope", "Scope");
        m.insert("project_detail.permissions_loading", "Berechtigungen laden...");
        m.insert("project_detail.permissions_error", "Fehler beim Laden der Berechtigungen");
        m.insert("project_detail.permissions_entries", "Einträge");
        m.insert("project_detail.permissions_col_tool", "Tool");
        m.insert("project_detail.permissions_col_command", "Befehl");
        m.insert("project_detail.permissions_no_entries", "Keine Berechtigungseinträge");
        m.insert("project_detail.health_loading", "Health wird berechnet...");
        m.insert("project_detail.health_error", "Fehler beim Laden der Health-Daten");
        m.insert("project_detail.health_score", "Health Score");
        m.insert("project_detail.health_claude_md", "CLAUDE.md vorhanden");
        m.insert("project_detail.health_memory", "Memory vorhanden");
        m.insert("project_detail.health_permissions", "Berechtigungen");
        m.insert("project_detail.health_security_issues", "Sicherheitsprobleme");
        m.insert("project_detail.health_duplicated_rules", "Duplizierte Regeln");
        m.insert("project_detail.health_no_security_issues", "Keine Sicherheitsprobleme gefunden");
        m.insert("project_detail.health_col_text", "Text");
        m.insert("project_detail.health_col_found_in", "Gefunden in");
        m.insert("project_detail.health_col_also_in", "Auch in");
        m.insert("project_detail.health_permission_entries", "Berechtigungseinträge");
        m.insert("project_detail.permissions_col_index", "#");
        m.insert("project_detail.permissions_col_status", "Status");
        m.insert("project_detail.permissions_fragment", "Fragment");
        m.insert("project_detail.permissions_ok", "OK");
        m.insert("project_detail.permissions_security_warnings", "Sicherheitswarnung(en)");
        m.insert("project_detail.permissions_manage", "Berechtigungen verwalten");
        m.insert("project_detail.advisor_analyze", "Projekt analysieren");
        m.insert("project_detail.advisor_analyzing", "Analysiere...");
        m.insert("project_detail.advisor_description", "Claude analysiert dein Projekt und gibt Empfehlungen");
        m.insert("project_detail.advisor_loading", "Claude analysiert dein Projekt");
        m.insert("project_detail.advisor_summary", "Projekt-Einschätzung");
        m.insert("project_detail.advisor_done", "Erledigt!");
        m.insert("project_detail.advisor_preview", "Vorschau anzeigen");
        m.insert("project_detail.advisor_category_tip", "Tipp");
        m.insert("project_detail.skills_col_name", "Name");
        m.insert("project_detail.skills_col_description", "Beschreibung");
        m.insert("project_detail.skills_col_invocable", "Aufrufbar");
        m.insert("project_detail.rules_col_name", "Name");
        m.insert("project_detail.rules_col_path", "Pfad");
        m.insert("project_detail.memory_col_file", "Datei");
        m.insert("project_detail.memory_col_size", "Größe");
        m.insert("project_detail.bytes", "Bytes");
        m.insert("project_detail.unknown_tab", "Unbekannter Tab");

        // ── Global Skills ──
        m.insert("global_skills.title", "Globale Skills");
        m.insert("global_skills.subtitle", "Skills in ~/.claude/skills/ verwalten");
        m.insert("global_skills.loading", "Skills laden");
        m.insert("global_skills.no_skills", "Keine globalen Skills vorhanden");
        m.insert("global_skills.no_skills_hint", "Erstelle Skills in ~/.claude/skills/ oder nutze den Skill Browser.");
        m.insert("global_skills.select_skill", "Wähle einen Skill aus der Liste.");
        m.insert("global_skills.col_skill", "Skill");
        m.insert("global_skills.col_invocable", "Aufrufbar");
        m.insert("global_skills.invocable", "Aufrufbar");
        m.insert("global_skills.not_invocable", "Nicht aufrufbar");
        m.insert("global_skills.editing", "Bearbeiten:");
        m.insert("global_skills.save", "Speichern");
        m.insert("global_skills.saved", "Gespeichert!");
        m.insert("global_skills.delete", "Löschen");
        m.insert("global_skills.deleted", "Gelöscht!");

        // ── Global Rules ──
        m.insert("global_rules.title", "Globale Regeln");
        m.insert("global_rules.subtitle", "Regeln in ~/.claude/rules/ verwalten");
        m.insert("global_rules.loading", "Regeln laden");
        m.insert("global_rules.no_rules", "Keine globalen Regeln vorhanden");
        m.insert("global_rules.no_rules_hint", "Erstelle .md Dateien in ~/.claude/rules/");
        m.insert("global_rules.select_rule", "Wähle eine Regel aus der Liste.");
        m.insert("global_rules.col_rule", "Regel");
        m.insert("global_rules.editing", "Bearbeiten:");
        m.insert("global_rules.save", "Speichern");
        m.insert("global_rules.saved", "Gespeichert!");
        m.insert("global_rules.delete", "Löschen");
        m.insert("global_rules.deleted", "Gelöscht!");

        // ── Plans ──
        m.insert("plans.title", "Pläne");
        m.insert("plans.subtitle", "Plan-Dateien in ~/.claude/plans/ verwalten");
        m.insert("plans.loading", "Pläne laden");
        m.insert("plans.no_plans", "Keine Pläne vorhanden");
        m.insert("plans.no_plans_hint", "Pläne werden von Claude Code beim Planen erstellt.");
        m.insert("plans.select_plan", "Wähle einen Plan aus der Liste.");
        m.insert("plans.col_plan", "Plan");
        m.insert("plans.col_modified", "Geändert");
        m.insert("plans.modified", "Geändert");
        m.insert("plans.plan_label", "Plan:");
        m.insert("plans.save", "Speichern");
        m.insert("plans.saved", "Gespeichert!");
        m.insert("plans.delete", "Löschen");
        m.insert("plans.deleted", "Gelöscht!");

        // ── Settings ──
        m.insert("settings.title", "Einstellungen");
        m.insert("settings.subtitle", "Claude Code Einstellungen und Hooks verwalten");
        m.insert("settings.tab_overview", "Übersicht");
        m.insert("settings.tab_hooks", "Hook Templates");
        m.insert("settings.tab_storage", "Speicher");
        m.insert("settings.loading", "Einstellungen laden");
        m.insert("settings.hooks_title", "Hooks");
        m.insert("settings.no_hooks", "Keine Hooks konfiguriert");
        m.insert("settings.pre_tool_use", "PreToolUse");
        m.insert("settings.post_tool_use", "PostToolUse");
        m.insert("settings.matcher", "Matcher");
        m.insert("settings.command", "Befehl");
        m.insert("settings.hook_templates_title", "Hook Templates");
        m.insert("settings.hook_templates_desc", "Vorgefertigte Hook-Konfigurationen zum Hinzufügen.");
        m.insert("settings.hook_templates_loading", "Templates laden");
        m.insert("settings.add_hook", "Hinzufügen");
        m.insert("settings.storage_title", "Speicherverbrauch");
        m.insert("settings.storage_loading", "Speicher berechnen");
        m.insert("settings.storage_total", "Gesamt");
        m.insert("settings.storage_dir", "Verzeichnis");
        m.insert("settings.storage_size", "Größe");

        // ── Permissions ──
        m.insert("permissions.title", "Berechtigungen");
        m.insert("permissions.subtitle", "Projekt-Berechtigungen prüfen und verwalten");
        m.insert("permissions.loading", "Berechtigungen laden");
        m.insert("permissions.no_permissions", "Keine Berechtigungen gefunden");
        m.insert("permissions.col_project", "Projekt");
        m.insert("permissions.col_entries", "Einträge");
        m.insert("permissions.col_issues", "Probleme");
        m.insert("permissions.col_fragmented", "Fragmentiert");
        m.insert("permissions.detail_title", "Berechtigungen");
        m.insert("permissions.detail_loading", "Berechtigungen laden");
        m.insert("permissions.detail_col_tool", "Tool");
        m.insert("permissions.detail_col_command", "Befehl");
        m.insert("permissions.detail_col_status", "Status");
        m.insert("permissions.detail_fragmented", "Fragmentiert");
        m.insert("permissions.detail_security_issue", "Sicherheitsproblem");
        m.insert("permissions.detail_delete_selected", "Ausgewählte löschen");
        m.insert("permissions.detail_deleted", "Gelöscht!");
        m.insert("permissions.detail_warnings_title", "Sicherheitswarnungen");
        m.insert("permissions.health_title", "Config Health");
        m.insert("permissions.health_subtitle", "Gesundheitsstatus aller Projekte");
        m.insert("permissions.health_loading", "Health berechnen");
        m.insert("permissions.health_col_project", "Projekt");
        m.insert("permissions.health_col_score", "Score");
        m.insert("permissions.health_col_issues", "Probleme");
        m.insert("permissions.health_avg", "Durchschnitt");
        m.insert("permissions.subtitle_manage", "Berechtigungs-Allowlists aller Projekte verwalten");
        m.insert("permissions.col_actions", "Aktionen");
        m.insert("permissions.col_security_issues", "Sicherheitsprobleme");
        m.insert("permissions.details", "Details");
        m.insert("permissions.detail_subtitle", "Berechtigungseinträge prüfen und bereinigen");
        m.insert("permissions.detail_deleting", "Lösche...");
        m.insert("permissions.detail_deleted_reloading", "Gelöscht! Lade neu...");
        m.insert("permissions.detail_delete_count", "Ausgewählte löschen");
        m.insert("permissions.detail_col_index", "#");
        m.insert("permissions.detail_fragment", "Fragment");
        m.insert("permissions.detail_ok", "OK");
        m.insert("permissions.detail_warnings_count", "Sicherheitswarnungen");
        m.insert("permissions.detail_entry", "Eintrag");
        m.insert("permissions.health_subtitle_scores", "Konfigurations-Health-Scores aller Projekte");
        m.insert("permissions.health_avg_score", "Durchschnittlicher Health Score");
        m.insert("permissions.health_projects_analyzed", "Analysierte Projekte");
        m.insert("permissions.health_no_issues", "Keine Probleme");

        // ── Analytics ──
        m.insert("analytics.title", "Analytik");
        m.insert("analytics.subtitle", "Claude Code Nutzungsstatistiken");
        m.insert("analytics.loading", "Analytik laden");
        m.insert("analytics.error_loading", "Fehler beim Laden der Analytik");
        m.insert("analytics.total_sessions", "Sitzungen gesamt");
        m.insert("analytics.total_messages", "Nachrichten gesamt");
        m.insert("analytics.git_commits", "Git Commits");
        m.insert("analytics.lines_added", "Zeilen hinzugefügt");
        m.insert("analytics.lines_removed", "Zeilen entfernt");
        m.insert("analytics.since", "seit");
        m.insert("analytics.activity_heatmap", "Aktivitäts-Heatmap");
        m.insert("analytics.messages", "Nachrichten");
        m.insert("analytics.sessions", "Sitzungen");
        m.insert("analytics.tool_calls", "Tool-Aufrufe");
        m.insert("analytics.hourly_distribution", "Stündliche Verteilung");
        m.insert("analytics.model_usage", "Modell-Nutzung");
        m.insert("analytics.col_model", "Modell");
        m.insert("analytics.col_input_tokens", "Input Tokens");
        m.insert("analytics.col_output_tokens", "Output Tokens");
        m.insert("analytics.col_cache_tokens", "Cache Tokens");
        m.insert("analytics.tool_ranking", "Tool-Ranking");
        m.insert("analytics.col_cache_read", "Cache Read");
        m.insert("analytics.tool_usage_top10", "Tool-Nutzung (Top 10)");
        m.insert("analytics.languages", "Sprachen");
        m.insert("analytics.session_outcomes", "Sitzungs-Ergebnisse");
        m.insert("analytics.outcomes", "Ergebnisse");

        // ── Sessions ──
        m.insert("sessions.title", "Sitzungen");
        m.insert("sessions.subtitle", "Claude Code Sitzungsverlauf durchsuchen");
        m.insert("sessions.loading", "Sitzungen laden");
        m.insert("sessions.search_placeholder", "Sitzungen suchen...");
        m.insert("sessions.no_sessions", "Keine Sitzungen gefunden");
        m.insert("sessions.col_project", "Projekt");
        m.insert("sessions.col_date", "Datum");
        m.insert("sessions.col_duration", "Dauer");
        m.insert("sessions.col_messages", "Nachrichten");
        m.insert("sessions.col_summary", "Zusammenfassung");
        m.insert("sessions.col_outcome", "Ergebnis");
        m.insert("sessions.minutes", "Min");
        m.insert("sessions.load_more", "Mehr laden");
        m.insert("sessions.detail_title", "Sitzungsdetails");
        m.insert("sessions.detail_loading", "Sitzung laden");
        m.insert("sessions.detail_project", "Projekt");
        m.insert("sessions.detail_start", "Start");
        m.insert("sessions.detail_duration", "Dauer");
        m.insert("sessions.detail_messages", "Nachrichten");
        m.insert("sessions.detail_tools", "Tool-Aufrufe");
        m.insert("sessions.detail_tokens", "Tokens");
        m.insert("sessions.detail_git", "Git");
        m.insert("sessions.detail_first_prompt", "Erster Prompt");
        m.insert("sessions.detail_summary", "Zusammenfassung");
        m.insert("sessions.back", "Zurück");
        m.insert("sessions.searching", "Suche...");
        m.insert("sessions.search", "Suchen");
        m.insert("sessions.clear", "Löschen");
        m.insert("sessions.search_results", "Suchergebnisse");
        m.insert("sessions.no_results", "Keine Ergebnisse gefunden");
        m.insert("sessions.col_prompt", "Prompt");
        m.insert("sessions.session_prefix", "Sitzung: ");
        m.insert("sessions.detail_start_time", "Startzeit");
        m.insert("sessions.user_messages", " Benutzer / ");
        m.insert("sessions.assistant_messages", " Assistent");
        m.insert("sessions.tokens_in", " ein / ");
        m.insert("sessions.tokens_out", " aus");
        m.insert("sessions.commits_label", " Commits, +");
        m.insert("sessions.lines_minus", " / -");
        m.insert("sessions.tools_used", "Verwendete Tools");
        m.insert("sessions.outcome_prefix", "Ergebnis: ");
        m.insert("sessions.showing", "Zeige");
        m.insert("sessions.of", "von");
        m.insert("sessions.previous", "Zurück");
        m.insert("sessions.next", "Weiter");

        // ── GitHub ──
        m.insert("github.title", "GitHub");
        m.insert("github.subtitle", "GitHub Integration Status");
        m.insert("github.loading", "GitHub-Daten laden");
        m.insert("github.auth_status", "Auth Status");
        m.insert("github.username", "Benutzername");
        m.insert("github.linked_repos", "Verknüpfte Repos");
        m.insert("github.no_repos", "Keine verknüpften Repos");
        m.insert("github.col_repo", "Repository");
        m.insert("github.col_recent_commits", "Letzte Commits");
        m.insert("github.col_open_prs", "Offene PRs");

        // ── Help / System Info ──
        m.insert("help.title", "System Info");
        m.insert("help.subtitle", "Claude Code Systeminformationen");
        m.insert("help.loading", "Systeminformationen laden");
        m.insert("help.account", "Account");
        m.insert("help.account_name", "Name");
        m.insert("help.account_email", "E-Mail");
        m.insert("help.subscription", "Abo");
        m.insert("help.claude_version", "Claude Code Version");
        m.insert("help.gh_cli", "GitHub CLI");
        m.insert("help.skill_usage", "Skill-Nutzung");
        m.insert("help.no_skill_usage", "Keine Skill-Nutzung erfasst");
        m.insert("help.col_skill", "Skill");
        m.insert("help.col_count", "Aufrufe");
        m.insert("help.what_is_title", "Was ist ClaudeAdmin?");
        m.insert("help.what_is_desc", "ClaudeAdmin ist die visuelle Admin-Konsole für Claude Code. Es bietet eine Web-Oberfläche zur Verwaltung aller Aspekte deiner Claude Code Konfiguration: Projekte, Skills, Regeln, Memory, Einstellungen, Hooks, MCP Server und Pläne.");
        m.insert("help.system_status", "System Status");
        m.insert("help.not_set", "Nicht gesetzt");
        m.insert("help.unknown", "Unbekannt");
        m.insert("help.not_found", "Nicht gefunden");
        m.insert("help.not_installed", "Nicht installiert");
        m.insert("help.concepts_title", "Claude Code Konzepte");
        m.insert("help.concept_skills", "Wiederverwendbare Prompts mit YAML Frontmatter. Gespeichert als SKILL.md in ~/.claude/skills/ (global) oder .claude/skills/ (projekt).");
        m.insert("help.concept_rules", "Einschränkungen und Richtlinien, die Claudes Verhalten formen. Gespeichert als .md Dateien in ~/.claude/rules/ oder projektspezifisch.");
        m.insert("help.concept_memory", "Persistente Notizen pro Projekt. MEMORY.md wird automatisch in System-Prompts geladen. Speichert Muster, Präferenzen und Erkenntnisse.");
        m.insert("help.concept_hooks", "Shell-Befehle, ausgelöst durch Events (PreToolUse, PostToolUse, Stop). Konfiguriert in settings.json für Auto-Formatierung, Linting, etc.");
        m.insert("help.concept_mcp", "Model Context Protocol Server erweitern Claude um externe Tools. Konfiguriert in ~/.claude.json mit command, args und env.");
        m.insert("help.concept_claudemd", "Projektspezifische Anweisungsdatei. Wird automatisch als Kontext geladen. Enthält Projektkonventionen, Stack-Info und Coding-Richtlinien.");
        m.insert("help.disclaimer", "ClaudeAdmin ist ein unabhängiges Community-Projekt. Es ist nicht mit Anthropic verbunden und wird nicht von Anthropic unterstützt oder genehmigt. Claude und Claude Code sind Marken von Anthropic.");

        m.insert("github.subtitle_detail", "GitHub CLI Integration und verknüpfte Repos");
        m.insert("github.linked_repositories", "Verknüpfte Repositories");
        m.insert("github.no_linked_repos", "Keine GitHub Repositories in ~/.claude.json verknüpft");
        m.insert("github.col_name", "Name");
        m.insert("github.col_path", "Pfad");

        // ── Skill Browser ──
        m.insert("skill_browser.title", "Skill Browser");
        m.insert("skill_browser.subtitle", "Offizielle und Community Skills entdecken und installieren");
        m.insert("skill_browser.loading", "Skills laden");
        m.insert("skill_browser.search_placeholder", "Skills suchen...");
        m.insert("skill_browser.no_results", "Keine Skills gefunden");
        m.insert("skill_browser.installed", "Installiert");
        m.insert("skill_browser.install", "Installieren");
        m.insert("skill_browser.official", "Offiziell");
        m.insert("skill_browser.community", "Community");
        m.insert("skill_browser.tab_official", "Offiziell (Anthropic)");
        m.insert("skill_browser.tab_community", "Community");
        m.insert("skill_browser.install_success", "erfolgreich installiert!");
        m.insert("skill_browser.install_failed", "Installation fehlgeschlagen:");

        // ── Docs ──
        m.insert("docs.title", "Dokumentation");
        m.insert("docs.subtitle", "Alles, was du \u{00fc}ber die Claude Code Konfiguration wissen musst");
        m.insert("docs.loading", "Dokumentation laden");

        // ── Docs: Inhaltsverzeichnis ──
        m.insert("docs.toc_contents", "Inhalt");
        m.insert("docs.toc_why_claudeadmin", "Warum ClaudeAdmin?");
        m.insert("docs.toc_capabilities", "Was es kann & nicht kann");
        m.insert("docs.toc_group", "Konzepte");
        m.insert("docs.toc_claude_md", "CLAUDE.md");
        m.insert("docs.toc_rules", "Regeln");
        m.insert("docs.toc_skills", "Skills");
        m.insert("docs.toc_memory", "Memory");
        m.insert("docs.toc_settings", "Einstellungen & Hooks");
        m.insert("docs.toc_mcp", "MCP Server");
        m.insert("docs.toc_plans", "Pl\u{00e4}ne");
        m.insert("docs.toc_scopes", "Global vs. Projekt");
        m.insert("docs.toc_tips", "Tipps & Best Practices");
        m.insert("docs.toc_links", "Offizielle Dokumentation");

        // ── Docs: Gemeinsame Labels ──
        m.insert("docs.tips_heading", "Tipps & Tricks");
        m.insert("docs.scope_global", "Global");
        m.insert("docs.scope_project", "Projekt");
        m.insert("docs.scope_user", "Benutzer");
        m.insert("docs.scope_parent", "\u{00dc}bergeordnet");
        m.insert("docs.scope_managed", "Verwaltet");
        m.insert("docs.scope_local", "Lokal");

        // ── Docs: \u{00dc}bersicht ──
        m.insert("docs.overview_heading", "Warum ClaudeAdmin?");
        m.insert("docs.overview_callout", " ist die zentrale Admin-Konsole f\u{00fc}r deine gesamte Claude Code Konfiguration. Sie ersetzt das manuelle Bearbeiten von Dateien in dutzenden versteckten Verzeichnissen durch eine einzige, visuelle Oberfl\u{00e4}che.");
        m.insert("docs.overview_text1", "Claude Code speichert seine Konfiguration in einer komplexen Hierarchie von Dateien und Verzeichnissen: CLAUDE.md-Dateien in Projekt-Wurzelverzeichnissen, Regeln und Skills verstreut in ~/.claude/-Unterverzeichnissen, Memory-Dateien nach kodierten Projektpfaden, Einstellungen in mehreren JSON-Dateien und MCP-Server-Konfigurationen in ~/.claude.json. Wenn deine Projekte wachsen, wird das manuelle Verwalten all dessen fehleranf\u{00e4}llig und zeitaufwendig.");
        m.insert("docs.overview_text2", "ClaudeAdmin bietet dir:");
        m.insert("docs.overview_li_visibility_label", "\u{00dc}bersicht");
        m.insert("docs.overview_li_visibility", " \u{2013} Alle Projekte, Skills, Regeln und Memory an einem Ort sehen");
        m.insert("docs.overview_li_editing_label", "Bearbeitung");
        m.insert("docs.overview_li_editing", " \u{2013} CLAUDE.md, Regeln, Skills und Memory mit einem richtigen Editor bearbeiten");
        m.insert("docs.overview_li_health_label", "Health Checks");
        m.insert("docs.overview_li_health", " \u{2013} Sicherheitsprobleme bei Berechtigungen, duplizierte Regeln und fehlende Konfigurationen erkennen");
        m.insert("docs.overview_li_analytics_label", "Analytik");
        m.insert("docs.overview_li_analytics", " \u{2013} Verstehen, wie du Claude Code nutzt: Sitzungen, Tokens, Tools, Kosten");
        m.insert("docs.overview_li_advisor_label", "Advisor");
        m.insert("docs.overview_li_advisor", " \u{2013} KI-gest\u{00fc}tzte Empfehlungen zur Verbesserung deiner Projektkonfiguration");

        // ── Docs: F\u{00e4}higkeiten ──
        m.insert("docs.cap_heading", "Was ClaudeAdmin kann & nicht kann");
        m.insert("docs.cap_can_heading", "Was es kann");
        m.insert("docs.cap_can_1", "Alle in ~/.claude.json registrierten Projekte durchsuchen und verwalten");
        m.insert("docs.cap_can_2", "CLAUDE.md-Dateien f\u{00fc}r jedes Projekt anzeigen und bearbeiten");
        m.insert("docs.cap_can_3", "Globale und projektspezifische Skills erstellen, bearbeiten und l\u{00f6}schen");
        m.insert("docs.cap_can_4", "Globale und projektspezifische Regeln erstellen, bearbeiten und l\u{00f6}schen");
        m.insert("docs.cap_can_5", "Projekt-Memory-Dateien anzeigen und bearbeiten (MEMORY.md und Themen)");
        m.insert("docs.cap_can_6", "Die Einstellungshierarchie inspizieren (Global \u{2192} Projekt \u{2192} Lokal)");
        m.insert("docs.cap_can_7", "Berechtigungseintr\u{00e4}ge pr\u{00fc}fen und Sicherheitsprobleme erkennen");
        m.insert("docs.cap_can_8", "MCP-Server-Konfigurationen anzeigen");
        m.insert("docs.cap_can_9", "Sitzungsverlauf, Token-Nutzung und Kosten analysieren");
        m.insert("docs.cap_can_10", "KI-gest\u{00fc}tzte Projektanalyse mit umsetzbaren Empfehlungen durchf\u{00fc}hren");
        m.insert("docs.cap_can_11", "Skills aus Community-Repositories durchsuchen und installieren");
        m.insert("docs.cap_can_12", "Alle Schreibvorg\u{00e4}nge erstellen automatische Backups in ~/.claude/backups/");
        m.insert("docs.cap_cannot_heading", "Was es nicht kann");
        m.insert("docs.cap_cannot_1", "Claude Code Sitzungen ausf\u{00fc}hren \u{2013} es verwaltet Konfiguration, nicht Ausf\u{00fc}hrung");
        m.insert("docs.cap_cannot_2", "Verwaltete Richtlinien \u{00e4}ndern (Enterprise-/Organisations-Einstellungen)");
        m.insert("docs.cap_cannot_3", "Auf Remote-Umgebungen oder SSH-Sitzungen zugreifen");
        m.insert("docs.cap_cannot_4", "Die Claude Code CLI f\u{00fc}r echte Programmierarbeit ersetzen");
        m.insert("docs.cap_cannot_5", ".claude.json MCP-Server direkt bearbeiten (nur lesen, zur Sicherheit)");
        m.insert("docs.cap_cannot_6", "API-Schl\u{00fc}ssel oder Authentifizierungsdaten verwalten");
        m.insert("docs.cap_cannot_callout", "ClaudeAdmin ist ein Konfigurationsmanager, kein Ersatz f\u{00fc}r Claude Code selbst. Stell es dir wie ein Datenbank-Admin-Tool vor: Es hilft beim Inspizieren, Konfigurieren und Warten \u{2013} aber die eigentliche Arbeit findet in Claude Code statt.");

        // ── Docs: CLAUDE.md ──
        m.insert("docs.claudemd_callout", "Die Projektverfassung. CLAUDE.md ist die wichtigste Konfigurationsdatei \u{2013} sie wird automatisch in jede Claude Code Sitzung als persistenter Kontext geladen.");
        m.insert("docs.claudemd_how_heading", "Wie es funktioniert");
        m.insert("docs.claudemd_how_text", "Wenn Claude Code eine Sitzung startet, sucht es rekursiv nach CLAUDE.md-Dateien von deinem aktuellen Arbeitsverzeichnis bis zum Dateisystem-Root. Alle gefundenen Dateien werden geladen und zusammengef\u{00fc}gt, wobei n\u{00e4}here Dateien Vorrang haben. Das bedeutet, du kannst eine CLAUDE.md auf Monorepo-Ebene mit gemeinsamen Konventionen und CLAUDE.md-Dateien auf Paketebene mit spezifischen \u{00dc}berschreibungen haben.");
        m.insert("docs.claudemd_locations_heading", "Speicherorte");
        m.insert("docs.claudemd_loc_project_or", " oder ");
        m.insert("docs.claudemd_loc_parent", "Monorepo-Root, wird f\u{00fc}r alle Unterpakete geladen");
        m.insert("docs.claudemd_loc_user", "Pers\u{00f6}nliche Standardeinstellungen f\u{00fc}r alle Projekte");
        m.insert("docs.claudemd_whatto_heading", "Was rein sollte");
        m.insert("docs.claudemd_whatto_context_label", "Projektkontext");
        m.insert("docs.claudemd_whatto_context", " \u{2013} Tech-Stack, Architekturentscheidungen, wichtige Abh\u{00e4}ngigkeiten");
        m.insert("docs.claudemd_whatto_standards_label", "Coding-Standards");
        m.insert("docs.claudemd_whatto_standards", " \u{2013} Namenskonventionen, Formatierungsregeln, Fehlerbehandlungsmuster");
        m.insert("docs.claudemd_whatto_workflows_label", "Workflows");
        m.insert("docs.claudemd_whatto_workflows", " \u{2013} Build-, Test-, Deploy-Abl\u{00e4}ufe; Branch-Benennung; PR-Konventionen");
        m.insert("docs.claudemd_whatto_dodont_label", "Do/Don\u{2019}t-Regeln");
        m.insert("docs.claudemd_whatto_dodont", " \u{2013} Explizite Einschr\u{00e4}nkungen (z.B. \u{201c}nie any in TypeScript verwenden\u{201d})");
        m.insert("docs.claudemd_whatto_team_label", "Team-Vereinbarungen");
        m.insert("docs.claudemd_whatto_team", " \u{2013} Review-Prozess, Commit-Nachrichtenformat, Modulgrenzen");
        m.insert("docs.claudemd_tip1", "Halte sie unter 500 Zeilen. Claude l\u{00e4}dt die gesamte Datei in den Kontext \u{2013} aufgebl\u{00e4}hte CLAUDE.md-Dateien verschwenden Tokens und verw\u{00e4}ssern wichtige Anweisungen.");
        m.insert("docs.claudemd_tip2", "Verwende klare Abschnitts\u{00fc}berschriften (## Architektur, ## Konventionen). Claude nutzt die Struktur, um relevante Abschnitte zu finden.");
        m.insert("docs.claudemd_tip3", "Setze die wichtigsten Regeln an den Anfang. In langen Dateien erh\u{00e4}lt der Inhalt am Anfang mehr Aufmerksamkeit.");
        m.insert("docs.claudemd_tip4", "Verwende CLAUDE.local.md f\u{00fc}r pers\u{00f6}nliche Pr\u{00e4}ferenzen, die nicht in Git committet werden sollen.");
        m.insert("docs.claudemd_ext_link", "Anthropic Docs: CLAUDE.md \u{2192}");

        // ── Docs: Regeln ──
        m.insert("docs.rules_heading", "Regeln");
        m.insert("docs.rules_callout", "Modulare, thematische Einschr\u{00e4}nkungen, die Claudes Verhalten formen. Anders als CLAUDE.md, das eine gro\u{00df}e Datei ist, sind Regeln separate .md-Dateien \u{2013} jede auf ein bestimmtes Thema fokussiert.");
        m.insert("docs.rules_how_heading", "Wie es funktioniert");
        m.insert("docs.rules_how_text", "Regeln werden beim Sitzungsstart automatisch geladen. Globale Regeln (deine pers\u{00f6}nlichen Pr\u{00e4}ferenzen) werden zuerst geladen, dann \u{00fc}berlagern Projektregeln diese. So kannst du deinen Coding-Stil global definieren, w\u{00e4}hrend Projekte dom\u{00e4}nenspezifische Einschr\u{00e4}nkungen hinzuf\u{00fc}gen.");
        m.insert("docs.rules_locations_heading", "Speicherorte");
        m.insert("docs.rules_loc_global", "Deine pers\u{00f6}nlichen Regeln, gelten f\u{00fc}r alle Projekte");
        m.insert("docs.rules_loc_project", "Projektspezifisch, in Git committet f\u{00fc}r Team-Sharing");
        m.insert("docs.rules_examples_heading", "Beispiele");
        m.insert("docs.rules_example_frontend", " \u{2013} React-Komponentenmuster, State-Management-Regeln");
        m.insert("docs.rules_example_security", " \u{2013} Eingabevalidierung, Auth-Muster, OWASP-Konformit\u{00e4}t");
        m.insert("docs.rules_example_testing", " \u{2013} Teststruktur, Coverage-Erwartungen, Mocking-Strategie");
        m.insert("docs.rules_example_rust", " \u{2013} Fehlerbehandlung mit thiserror, Modulstruktur, Benennung");
        m.insert("docs.rules_tip1", "Ein Thema pro Datei. Mische nicht Frontend- und Backend-Regeln \u{2013} kleinere, fokussierte Dateien sind einfacher zu pflegen und wiederzuverwenden.");
        m.insert("docs.rules_tip2", "Globale Regeln eignen sich hervorragend f\u{00fc}r pers\u{00f6}nliche Stilpr\u{00e4}ferenzen: bevorzugte Sprache, Formatierungstool, Commit-Nachrichtenformat.");
        m.insert("docs.rules_tip3", "Projektregeln \u{00fc}berschreiben globale Regeln. Bei einem Konflikt gewinnt die Projektregel.");
        m.insert("docs.rules_tip4", "Nutze ClaudeAdmins Health Check, um duplizierte Regeln zwischen globalem und Projektlevel zu erkennen.");
        m.insert("docs.rules_ext_link", "Anthropic Docs: Regeln \u{2192}");

        // ── Docs: Skills ──
        m.insert("docs.skills_heading", "Skills");
        m.insert("docs.skills_callout", "Wiederverwendbare, strukturierte Prompts mit Metadaten. Skills sind wie Plugins f\u{00fc}r Claude \u{2013} sie k\u{00f6}nnen automatisch durch Kontext ausgel\u{00f6}st oder manuell \u{00fc}ber Slash-Befehle aufgerufen werden.");
        m.insert("docs.skills_how_heading", "Wie es funktioniert");
        m.insert("docs.skills_how_text", "Jeder Skill lebt in seinem eigenen Verzeichnis mit einer SKILL.md-Datei mit YAML-Frontmatter und einem Markdown-Body. Das Frontmatter definiert Metadaten wie Beschreibung und Ausl\u{00f6}sebedingungen. Der Body enth\u{00e4}lt die eigentlichen Prompt-Anweisungen, Beispiele und Referenzmaterial.");
        m.insert("docs.skills_structure_heading", "Struktur");
        m.insert("docs.skills_locations_heading", "Speicherorte");
        m.insert("docs.skills_loc_global", "In allen Projekten verf\u{00fc}gbar");
        m.insert("docs.skills_loc_project", "Projektspezifische Skills");
        m.insert("docs.skills_tip1", "Setze user_invocable: true im Frontmatter, um einen Skill \u{00fc}ber /skill-name in Claude Code aufrufbar zu machen.");
        m.insert("docs.skills_tip2", "F\u{00fc}ge konkrete Beispiele in deine SKILL.md ein. Claude arbeitet deutlich besser mit Input/Output-Beispielen.");
        m.insert("docs.skills_tip3", "Nutze den Skill Browser in ClaudeAdmin, um Community-Skills zu entdecken und zu installieren.");
        m.insert("docs.skills_tip4", "Referenzdateien im Skill-Verzeichnis werden nur geladen, wenn der Skill ausgel\u{00f6}st wird \u{2013} das spart Tokens.");
        m.insert("docs.skills_ext_link", "Anthropic Docs: Skills \u{2192}");

        // ── Docs: Memory ──
        m.insert("docs.memory_heading", "Memory");
        m.insert("docs.memory_callout", "Claudes persistente Wissensbasis pro Projekt. Memory-Dateien speichern Muster, Pr\u{00e4}ferenzen und Erkenntnisse, die Claude \u{00fc}ber Sitzungen hinweg sammelt.");
        m.insert("docs.memory_how_heading", "Wie es funktioniert");
        m.insert("docs.memory_how_text", "Claude Code pflegt ein Memory-Verzeichnis f\u{00fc}r jedes Projekt, gespeichert in ~/.claude/projects/<encoded-path>/memory/. Die Hauptdatei MEMORY.md hat einen besonderen Status: Ihre ersten 200 Zeilen werden beim Sitzungsstart in den System-Prompt geladen. Zus\u{00e4}tzliche Themendateien (debugging.md, api-conventions.md usw.) werden bei Bedarf geladen, wenn Claude feststellt, dass sie f\u{00fc}r die aktuelle Aufgabe relevant sind.");
        m.insert("docs.memory_structure_heading", "Struktur");
        m.insert("docs.memory_auto_heading", "Auto-Memory");
        m.insert("docs.memory_auto_text", "Claude Code kann automatisch Eintr\u{00e4}ge zum Memory hinzuf\u{00fc}gen, wenn es Projektmuster, Debugging-L\u{00f6}sungen oder deine Pr\u{00e4}ferenzen entdeckt. Du kannst auto-generiertes Memory mit dem /memory-Befehl in Claude Code oder \u{00fc}ber ClaudeAdmins Memory-Editor \u{00fc}berpr\u{00fc}fen und bearbeiten.");
        m.insert("docs.memory_tip1", "Setze die wichtigsten Informationen in die ersten 200 Zeilen von MEMORY.md \u{2013} das wird automatisch geladen.");
        m.insert("docs.memory_tip2", "Verwende Themendateien f\u{00fc}r tiefgehendes Wissen. Sie werden nur bei Bedarf geladen und halten die Basis-Token-Nutzung niedrig.");
        m.insert("docs.memory_tip3", "\u{00dc}berpr\u{00fc}fe Auto-Memory regelm\u{00e4}\u{00df}ig. Claude speichert manchmal \u{00fc}berm\u{00e4}\u{00df}ig spezifische Einmall\u{00f6}sungen.");
        m.insert("docs.memory_tip4", "Memory ist projektbezogen. Wenn du zu einem anderen Projekt wechselst, erh\u{00e4}lt Claude einen anderen Satz Erinnerungen.");
        m.insert("docs.memory_ext_link", "Anthropic Docs: Memory \u{2192}");

        // ── Docs: Einstellungen & Hooks ──
        m.insert("docs.settings_heading", "Einstellungen & Hooks");
        m.insert("docs.settings_heading_short", "Einstellungen");
        m.insert("docs.settings_callout", "JSON-basierte Konfiguration f\u{00fc}r Verhalten, Berechtigungen und Automatisierung. Hooks erm\u{00f6}glichen es, Shell-Befehle automatisch vor oder nach Claudes Tool-Nutzung auszuf\u{00fc}hren.");
        m.insert("docs.settings_hierarchy_heading", "Einstellungshierarchie");
        m.insert("docs.settings_hierarchy_text", "Einstellungen folgen einem Schichtmodell mit zunehmender Spezifit\u{00e4}t. Spezifischere Schichten \u{00fc}berschreiben weniger spezifische:");
        m.insert("docs.settings_managed_code", "Enterprise-Richtlinien");
        m.insert("docs.settings_managed_desc", "H\u{00f6}chste Priorit\u{00e4}t, von Organisation gesetzt (nur lesen)");
        m.insert("docs.settings_global_desc", "Deine pers\u{00f6}nlichen globalen Einstellungen");
        m.insert("docs.settings_project_desc", "Team-Einstellungen, in Git committet");
        m.insert("docs.settings_local_desc", "Deine pers\u{00f6}nlichen Projekt-\u{00dc}berschreibungen (gitignored)");
        m.insert("docs.settings_hooks_heading", "Hooks");
        m.insert("docs.settings_hooks_text", "Hooks sind Shell-Befehle, die bei bestimmten Ereignissen w\u{00e4}hrend einer Claude Code Sitzung ausgel\u{00f6}st werden. Sie werden in settings.json unter dem hooks-Schl\u{00fc}ssel konfiguriert.");
        m.insert("docs.settings_hooks_events", "Ereignisse:\n\u{2022} PreToolUse  \u{2013} Bevor Claude ein Tool ausf\u{00fc}hrt (z.B. Auto-Format vor Schreiben)\n\u{2022} PostToolUse \u{2013} Nachdem Claude ein Tool ausf\u{00fc}hrt (z.B. Lint nach Datei\u{00e4}nderung)\n\u{2022} Stop        \u{2013} Wenn Claude eine Antwort beendet");
        m.insert("docs.settings_tip1", "Verwende PreToolUse-Hooks, um Code automatisch zu formatieren, bevor Claude Dateien schreibt. Das gew\u{00e4}hrleistet einheitlichen Stil.");
        m.insert("docs.settings_tip2", "PostToolUse-Hooks eignen sich hervorragend f\u{00fc}r Linting: Probleme sofort erkennen, nachdem Claude Code \u{00e4}ndert.");
        m.insert("docs.settings_tip3", "ClaudeAdmins Einstellungsseite zeigt die effektive Hook-Kette \u{00fc}ber alle Schichten.");
        m.insert("docs.settings_ext_link", "Anthropic Docs: Einstellungen \u{2192}");
        m.insert("docs.settings_hooks_ext_link", "Anthropic Docs: Hooks \u{2192}");

        // ── Docs: MCP Server ──
        m.insert("docs.mcp_heading", "MCP Server");
        m.insert("docs.mcp_callout", "Model Context Protocol Server erweitern Claude um externe Tools und Datenquellen. Sie erm\u{00f6}glichen Claude die Interaktion mit Datenbanken, APIs, Dateisystemen und anderen Diensten.");
        m.insert("docs.mcp_how_heading", "Wie es funktioniert");
        m.insert("docs.mcp_how_text", "MCP-Server sind externe Prozesse, die Claude Code startet und \u{00fc}ber das MCP-Protokoll kommuniziert. Jeder Server stellt eine Reihe von Tools bereit, die Claude aufrufen kann. Die Konfiguration befindet sich in ~/.claude.json unter dem Schl\u{00fc}ssel mcpServers.");
        m.insert("docs.mcp_config_heading", "Konfiguration");
        m.insert("docs.mcp_management_heading", "Verwaltung in ClaudeAdmin");
        m.insert("docs.mcp_management_text", "ClaudeAdmin bietet eine dedizierte MCP-Server-Seite f\u{00fc}r die vollst\u{00e4}ndige Verwaltung: Server anzeigen, hinzuf\u{00fc}gen, bearbeiten und l\u{00f6}schen ohne manuelle JSON-Bearbeitung. Die Health-Check-Funktion startet jeden Server und pr\u{00fc}ft, ob er auf JSON-RPC initialize- und tools/list-Anfragen antwortet. Nutze den MCP Browser, um beliebte Server mit einem Klick zu entdecken und zu installieren.");
        m.insert("docs.mcp_tip1", "MCP-Server k\u{00f6}nnen auch projektspezifisch in .claude/settings.json konfiguriert werden.");
        m.insert("docs.mcp_tip2", "Verwende Umgebungsvariablen f\u{00fc}r Geheimnisse \u{2013} kodiere niemals API-Schl\u{00fc}ssel direkt in Konfigurationsdateien.");
        m.insert("docs.mcp_tip3", "Nutze den MCP Browser, um beliebte Server zu entdecken und zu installieren, oder f\u{00fc}ge eigene \u{00fc}ber den Tab \u{201c}Neuer Server\u{201d} hinzu.");
        m.insert("docs.mcp_ext_link", "Anthropic Docs: MCP \u{2192}");
        m.insert("docs.mcp_spec_ext_link", "MCP Spezifikation \u{2192}");

        // ── Docs: Pl\u{00e4}ne ──
        m.insert("docs.plans_heading", "Pl\u{00e4}ne");
        m.insert("docs.plans_callout", "Markdown-Dateien, die Claude nutzt, um komplexe Aufgaben aufzuteilen. Pl\u{00e4}ne helfen Claude, den Fokus bei mehrstufiger Arbeit zu behalten und den Fortschritt zu verfolgen.");
        m.insert("docs.plans_how_heading", "Wie es funktioniert");
        m.insert("docs.plans_how_text", "Wenn Claude eine komplexe Aufgabe angeht, kann es Plandateien in ~/.claude/plans/ erstellen oder referenzieren. Pl\u{00e4}ne sind strukturierte Markdown-Dokumente mit Aufgabenlisten, Abh\u{00e4}ngigkeiten und Status-Tracking. Sie bleiben \u{00fc}ber Sitzungen hinweg bestehen, sodass Claude dort weitermachen kann, wo es aufgeh\u{00f6}rt hat.");
        m.insert("docs.plans_location_heading", "Speicherort");
        m.insert("docs.plans_loc_global", "Alle Plandateien");
        m.insert("docs.plans_tip1", "Bitte Claude, \u{201c}einen Plan zu machen\u{201d}, bevor du komplexes Refactoring durchf\u{00fc}hrst. Pl\u{00e4}ne reduzieren Fehler bei \u{00c4}nderungen \u{00fc}ber mehrere Dateien.");
        m.insert("docs.plans_tip2", "R\u{00e4}ume alte Pl\u{00e4}ne regelm\u{00e4}\u{00df}ig auf. ClaudeAdmins Pl\u{00e4}ne-Seite zeigt alle gespeicherten Pl\u{00e4}ne mit \u{00c4}nderungsdatum.");

        // ── Docs: Scopes ──
        m.insert("docs.scopes_heading", "Global vs. Projekt-Scope");
        m.insert("docs.scopes_callout", "Das Verst\u{00e4}ndnis von Scopes ist der Schl\u{00fc}ssel zu effektiver Claude Code Konfiguration. Jeder Konfigurationstyp existiert in zwei Ebenen: global (deine pers\u{00f6}nlichen Standardeinstellungen) und projektspezifisch (mit deinem Team geteilt).");
        m.insert("docs.scopes_overview_heading", "Scope-\u{00dc}bersicht");
        m.insert("docs.scopes_col_type", "Konfigurationstyp");
        m.insert("docs.scopes_col_global", "Global (Benutzer)");
        m.insert("docs.scopes_col_project", "Projekt");
        m.insert("docs.scopes_col_priority", "Priorit\u{00e4}t");
        m.insert("docs.scopes_priority_project_global", "Projekt > Global");
        m.insert("docs.scopes_priority_both", "Beide verf\u{00fc}gbar");
        m.insert("docs.scopes_memory_global", "Pro Projekt in ~/.claude/projects/");
        m.insert("docs.scopes_priority_project_keyed", "Projektbezogen");
        m.insert("docs.scopes_priority_local_project_global", "Lokal > Projekt > Global");
        m.insert("docs.scopes_priority_merged", "Zusammengef\u{00fc}hrt");
        m.insert("docs.scopes_when_heading", "Wann was verwenden?");
        m.insert("docs.scopes_use_global", "Global verwenden f\u{00fc}r");
        m.insert("docs.scopes_global_1", "Pers\u{00f6}nliche Coding-Stil-Pr\u{00e4}ferenzen");
        m.insert("docs.scopes_global_2", "Bevorzugte Sprach- und Framework-Standards");
        m.insert("docs.scopes_global_3", "Commit-Nachrichtenformat");
        m.insert("docs.scopes_global_4", "Editor-/IDE-Integrationseinstellungen");
        m.insert("docs.scopes_global_5", "MCP-Server, die du projekt\u{00fc}bergreifend nutzt");
        m.insert("docs.scopes_use_project", "Projekt verwenden f\u{00fc}r");
        m.insert("docs.scopes_project_1", "Tech-Stack-Dokumentation und -Einschr\u{00e4}nkungen");
        m.insert("docs.scopes_project_2", "Team-Coding-Konventionen");
        m.insert("docs.scopes_project_3", "Dom\u{00e4}nenspezifische Regeln (Sicherheit, Compliance)");
        m.insert("docs.scopes_project_4", "Projektspezifische Skills und Workflows");
        m.insert("docs.scopes_project_5", "CI/CD-Hooks und Automatisierung");

        // ── Docs: Tipps & Best Practices ──
        m.insert("docs.bestpractices_heading", "Tipps & Best Practices");
        m.insert("docs.bestpractices_hygiene_heading", "Konfigurationshygiene");
        m.insert("docs.bestpractices_hygiene_1", "F\u{00fc}hre ClaudeAdmins Config Health Check regelm\u{00e4}\u{00df}ig aus. Er erkennt duplizierte Regeln, aufgebl\u{00e4}hte Berechtigungslisten und fehlende CLAUDE.md-Dateien.");
        m.insert("docs.bestpractices_hygiene_2", "Wiederhole dich nicht: Wenn eine Regel global existiert, kopiere sie nicht in die Projekt-CLAUDE.md. Nutze das Scope-System.");
        m.insert("docs.bestpractices_hygiene_3", "Halte Berechtigungslisten sauber. Mit der Zeit sammelt Claude Code hunderte von Erlauben/Verweigern-Eintr\u{00e4}gen an. Nutze die Berechtigungsseite, um sie zu bereinigen.");
        m.insert("docs.bestpractices_tokens_heading", "Token-Effizienz");
        m.insert("docs.bestpractices_tokens_1", "Alles in CLAUDE.md, Regeln, Skills (wenn ausgel\u{00f6}st) und die ersten 200 Zeilen von MEMORY.md z\u{00e4}hlen gegen dein Kontextfenster. Sei pr\u{00e4}gnant.");
        m.insert("docs.bestpractices_tokens_2", "Verschiebe detailliertes Referenzmaterial in Skill-Referenzdateien oder Memory-Themendateien \u{2013} sie werden nur bei Bedarf geladen.");
        m.insert("docs.bestpractices_tokens_3", "Nutze die Analytik-Seite, um deine Token-Nutzung \u{00fc}ber Projekte und Sitzungen hinweg zu \u{00fc}berwachen.");
        m.insert("docs.bestpractices_team_heading", "Team-Zusammenarbeit");
        m.insert("docs.bestpractices_team_1", "Committe .claude/rules/ und .claude/skills/ in Git. Das teilt Konventionen im gesamten Team.");
        m.insert("docs.bestpractices_team_2", "Verwende .claude/settings.json f\u{00fc}r Team-Einstellungen und .claude/settings.local.json f\u{00fc}r pers\u{00f6}nliche \u{00dc}berschreibungen.");
        m.insert("docs.bestpractices_team_3", "CLAUDE.md im Projekt-Root ist der Vertrag deines Teams mit Claude. Behandle sie wie Dokumentation \u{2013} \u{00fc}berpr\u{00fc}fe \u{00c4}nderungen in PRs.");
        m.insert("docs.bestpractices_debug_heading", "Claude-Verhalten debuggen");
        m.insert("docs.bestpractices_debug_1", "Wenn Claude eine Regel ignoriert, pr\u{00fc}fe die Einstellungshierarchie-Seite auf widersprüchliche Einstellungen \u{00fc}ber die Schichten hinweg.");
        m.insert("docs.bestpractices_debug_2", "Memory kann unerwartetes Verhalten verursachen. \u{00dc}berpr\u{00fc}fe auto-generierte Eintr\u{00e4}ge \u{2013} Claude hat m\u{00f6}glicherweise einen Workaround statt des korrekten Ansatzes gespeichert.");
        m.insert("docs.bestpractices_debug_3", "Nutze die Sitzungsseite, um vergangene Gespr\u{00e4}che zu \u{00fc}berpr\u{00fc}fen und zu verstehen, was Claude \u{201c}gedacht\u{201d} hat.");

        // ── Docs: Links ──
        m.insert("docs.links_heading", "Offizielle Anthropic-Dokumentation");
        m.insert("docs.links_text", "Diese Links f\u{00fc}hren zur ma\u{00df}geblichen Dokumentation, die von Anthropic gepflegt wird. ClaudeAdmin baut auf diesen Spezifikationen auf.");
        m.insert("docs.link_overview_title", "Claude Code \u{00dc}bersicht");
        m.insert("docs.link_overview_desc", "Erste Schritte, Installation und grundlegende Nutzung");
        m.insert("docs.link_memory_title", "Memory & CLAUDE.md");
        m.insert("docs.link_memory_desc", "Wie Claude Projekt-Memory speichert und nutzt");
        m.insert("docs.link_skills_title", "Skills");
        m.insert("docs.link_skills_desc", "Wiederverwendbare Skills erstellen und verwalten");
        m.insert("docs.link_settings_title", "Einstellungen");
        m.insert("docs.link_settings_desc", "Konfigurationshierarchie und Optionen");
        m.insert("docs.link_hooks_title", "Hooks");
        m.insert("docs.link_hooks_desc", "Ereignisgesteuerte Automatisierung mit Shell-Befehlen");
        m.insert("docs.link_mcp_title", "MCP Server");
        m.insert("docs.link_mcp_desc", "Claude mit externen Tools erweitern");
        m.insert("docs.link_bestpractices_title", "Best Practices");
        m.insert("docs.link_bestpractices_desc", "Tipps f\u{00fc}r effektive Claude Code Nutzung");
        m.insert("docs.link_mcp_spec_title", "MCP Spezifikation");
        m.insert("docs.link_mcp_spec_desc", "Der Model Context Protocol Standard");

        // ── Licenses ──
        m.insert("sidebar.licenses", "Lizenzen");
        m.insert("licenses.title", "Lizenzen");
        m.insert("licenses.subtitle", "Open-Source-Lizenzen und Abh\u{00e4}ngigkeiten");
        m.insert("licenses.own_license", "ClaudeAdmin Lizenz");
        m.insert("licenses.third_party", "Third-Party-Abh\u{00e4}ngigkeiten");
        m.insert("licenses.col_name", "Crate");
        m.insert("licenses.col_version", "Version");
        m.insert("licenses.col_license", "Lizenz");
        m.insert("licenses.search_placeholder", "Abh\u{00e4}ngigkeiten suchen...");
        m.insert("licenses.loading", "Lizenzen laden");
        m.insert("licenses.count", "Abh\u{00e4}ngigkeiten");
        m.insert("licenses.mit_copyright", "Copyright (c) 2024-2026");
        m.insert("licenses.mit_line1", "Hiermit wird jeder Person, die eine Kopie dieser Software und der zugehörigen Dokumentationsdateien (die \u{201e}Software\u{201c}) erhält, kostenlos die Erlaubnis erteilt, uneingeschränkt mit der Software zu handeln, einschließlich und ohne Einschränkung der Rechte zur Nutzung, zum Kopieren, Ändern, Zusammenführen, Veröffentlichen, Verteilen, Unterlizenzieren und/oder zum Verkauf von Kopien der Software, und Personen, denen die Software zur Verfügung gestellt wird, dies unter den folgenden Bedingungen zu gestatten:");
        m.insert("licenses.mit_line2", "Der obige Urheberrechtshinweis und dieser Genehmigungshinweis müssen in allen Kopien oder wesentlichen Teilen der Software enthalten sein.");
        m.insert("licenses.mit_line3", "DIE SOFTWARE WIRD OHNE MÄNGELGEWÄHR UND OHNE JEGLICHE AUSDRÜCKLICHE ODER STILLSCHWEIGENDE GEWÄHRLEISTUNG, EINSCHLIEẞLICH, ABER NICHT BESCHRÄNKT AUF DIE GEWÄHRLEISTUNG DER MARKTGÄNGIGKEIT, DER EIGNUNG FÜR EINEN BESTIMMTEN ZWECK UND DER NICHTVERLETZUNG VON RECHTEN DRITTER, ZUR VERFÜGUNG GESTELLT. DIE AUTOREN ODER URHEBERRECHTSINHABER SIND IN KEINEM FALL HAFTBAR FÜR ANSPRÜCHE, SCHÄDEN ODER ANDERE VERPFLICHTUNGEN, OB IN EINER VERTRAGS- ODER HAFTUNGSKLAGE, EINER UNERLAUBTEN HANDLUNG ODER ANDERWEITIG, DIE SICH AUS, AUS ODER IN VERBINDUNG MIT DER SOFTWARE ODER DER NUTZUNG ODER ANDEREN GESCHÄFTEN MIT DER SOFTWARE ERGEBEN.");
        m.insert("licenses.direct_deps", "Direkte Abhängigkeiten");
        m.insert("licenses.transitive_deps", "Transitive Abhängigkeiten");
        m.insert("licenses.overview", "Lizenz-Übersicht");
        m.insert("licenses.direct_count", "direkte");
        m.insert("licenses.transitive_count", "transitive Abhängigkeiten");

        // ── Components ──
        m.insert("component.modal.close", "Schließen");
        m.insert("component.editor.save", "Speichern");
        m.insert("component.editor.saved", "Gespeichert!");
        m.insert("component.json_editor.valid", "Valides JSON");
        m.insert("component.json_editor.invalid", "Ungültiges JSON");
        m.insert("component.frontmatter.description", "Beschreibung");
        m.insert("component.frontmatter.user_invocable", "Benutzer-aufrufbar");
        m.insert("component.advisor.title", "Projekt-Advisor");
        m.insert("component.advisor.analyze", "Analysieren");
        m.insert("component.advisor.analyzing", "Analysiere...");
        m.insert("component.advisor.no_api_key", "Kein ANTHROPIC_API_KEY konfiguriert");
        m.insert("component.advisor.error", "Fehler beim Laden der Empfehlungen");
        m.insert("component.advisor.summary", "Zusammenfassung");
        m.insert("component.advisor.recommendations", "Empfehlungen");
        m.insert("component.advisor.apply", "Anwenden");
        m.insert("component.advisor.applied", "Erledigt!");
        m.insert("component.advisor.analyze_project", "Projekt analysieren");
        m.insert("component.advisor.hint", "Claude analysiert dein Projekt und gibt Empfehlungen");
        m.insert("component.advisor.loading", "Claude analysiert dein Projekt");
        m.insert("component.advisor.assessment", "Projekt-Einsch\u{00e4}tzung");
        m.insert("component.advisor.show_preview", "Vorschau anzeigen");
        m.insert("component.advisor.category_tip", "Tipp");
        m.insert("component.frontmatter.user_invocable_label", "Benutzer-aufrufbar (aufrufbar mit /command)");
        m.insert("component.editor.saving", "Speichere...");

        // ── Common ──
        m.insert("common.error", "Fehler");
        m.insert("common.loading", "Laden");
        m.insert("common.save", "Speichern");
        m.insert("common.delete", "Löschen");
        m.insert("common.cancel", "Abbrechen");
        m.insert("common.close", "Schließen");
        m.insert("common.yes", "Ja");
        m.insert("common.no", "Nein");
        m.insert("common.ok", "OK");
        m.insert("common.error_prefix", "Fehler: ");
        m.insert("common.invalid_json", "Ungültiges JSON: ");

        m
    })
}
