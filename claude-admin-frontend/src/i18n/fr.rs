use std::collections::HashMap;
use std::sync::OnceLock;

static TRANSLATIONS: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();

pub fn translations() -> &'static HashMap<&'static str, &'static str> {
    TRANSLATIONS.get_or_init(|| {
        let mut m = HashMap::new();

        // ── App ──
        m.insert("app.title", "ClaudeAdmin");
        m.insert("app.subtitle", "Gestionnaire de configuration");

        // ── Sidebar ──
        m.insert("sidebar.overview", "Vue d\u{2019}ensemble");
        m.insert("sidebar.dashboard", "Tableau de bord");
        m.insert("sidebar.analytics", "Statistiques");
        m.insert("sidebar.manage", "G\u{00e9}rer");
        m.insert("sidebar.projects", "Projets");
        m.insert("sidebar.global_skills", "Skills globaux");
        m.insert("sidebar.skill_browser", "Skill Browser");
        m.insert("sidebar.global_rules", "R\u{00e8}gles globales");
        m.insert("sidebar.plans", "Plans");
        m.insert("sidebar.mcp_servers", "Serveurs MCP");
        m.insert("sidebar.mcp_browser", "MCP Browser");
        m.insert("sidebar.security", "S\u{00e9}curit\u{00e9}");
        m.insert("sidebar.permissions", "Permissions");
        m.insert("sidebar.config_health", "Sant\u{00e9} de la configuration");
        m.insert("sidebar.system", "Syst\u{00e8}me");
        m.insert("sidebar.settings", "Param\u{00e8}tres");
        m.insert("sidebar.sessions", "Sessions");
        m.insert("sidebar.github", "GitHub");
        m.insert("sidebar.learn", "Apprendre");
        m.insert("sidebar.docs", "Documentation");
        m.insert("sidebar.help", "Informations syst\u{00e8}me");

        // ── Dashboard ──
        m.insert("dashboard.title", "Tableau de bord");
        m.insert("dashboard.subtitle", "Vue d\u{2019}ensemble de votre configuration Claude Code");
        m.insert("dashboard.projects", "Projets");
        m.insert("dashboard.global_skills", "Skills globaux");
        m.insert("dashboard.global_rules", "R\u{00e8}gles globales");
        m.insert("dashboard.mcp_servers", "Serveurs MCP");
        m.insert("dashboard.plans", "Plans");
        m.insert("dashboard.config_health", "Sant\u{00e9} de la configuration");
        m.insert("dashboard.recent_projects", "Projets r\u{00e9}cents");
        m.insert("dashboard.loading", "Chargement");
        m.insert("dashboard.error_loading", "Erreur lors du chargement du tableau de bord");
        m.insert("dashboard.col_name", "Nom");
        m.insert("dashboard.col_claude_md", "CLAUDE.md");
        m.insert("dashboard.col_skills", "Skills");
        m.insert("dashboard.col_rules", "R\u{00e8}gles");
        m.insert("dashboard.col_memory", "M\u{00e9}moire");
        m.insert("dashboard.yes", "Oui");

        // ── MCP ──
        m.insert("mcp.title", "Serveurs MCP");
        m.insert("mcp.subtitle", "G\u{00e9}rer les serveurs Model Context Protocol pour Claude Code");
        m.insert("mcp.tab_servers", "Serveurs");
        m.insert("mcp.tab_health", "V\u{00e9}rification de sant\u{00e9}");
        m.insert("mcp.tab_add", "Nouveau serveur");
        m.insert("mcp.loading", "Chargement des serveurs MCP");
        m.insert("mcp.no_servers", "Aucun serveur MCP configur\u{00e9}");
        m.insert("mcp.no_servers_hint", "Ajoutez des serveurs via l\u{2019}onglet \u{00ab} Nouveau serveur \u{00bb} ou le MCP Browser.");
        m.insert("mcp.select_server", "S\u{00e9}lectionnez un serveur dans la liste pour afficher et modifier sa configuration.");
        m.insert("mcp.no_servers_configured", "Aucun serveur configur\u{00e9}.");
        m.insert("mcp.check_health", "V\u{00e9}rifier la sant\u{00e9}");
        m.insert("mcp.save", "Enregistrer");
        m.insert("mcp.delete", "Supprimer");
        m.insert("mcp.saved", "Enregistr\u{00e9} !");
        m.insert("mcp.deleted", "Supprim\u{00e9} !");
        m.insert("mcp.read_only", "Lecture seule");
        m.insert("mcp.read_only_hint", "Ce serveur est g\u{00e9}r\u{00e9} en externe et ne peut pas \u{00ea}tre modifi\u{00e9} ici.");
        m.insert("mcp.health.title", "Sant\u{00e9} des serveurs MCP");
        m.insert("mcp.health.check_all", "V\u{00e9}rifier tous les serveurs");
        m.insert("mcp.health.checking", "V\u{00e9}rification...");
        m.insert("mcp.health.description", "D\u{00e9}marre chaque processus MCP, envoie JSON-RPC initialize + tools/list, et rapporte les r\u{00e9}sultats. D\u{00e9}lai d\u{2019}expiration : 10 secondes par serveur.");
        m.insert("mcp.health.col_name", "Nom");
        m.insert("mcp.health.col_source", "Source");
        m.insert("mcp.health.col_status", "Statut");
        m.insert("mcp.health.col_server_info", "Info serveur");
        m.insert("mcp.health.col_tools", "Outils");
        m.insert("mcp.health.col_duration", "Dur\u{00e9}e");
        m.insert("mcp.health.running", "En cours");
        m.insert("mcp.health.error", "Erreur");
        m.insert("mcp.health.timeout", "D\u{00e9}lai d\u{00e9}pass\u{00e9}");
        m.insert("mcp.health.unknown", "Inconnu");
        m.insert("mcp.add.title", "Ajouter un serveur MCP");
        m.insert("mcp.add.description", "Ajoutez un nouveau serveur MCP \u{00e0} votre configuration globale ~/.claude.json.");
        m.insert("mcp.add.name_label", "Nom du serveur");
        m.insert("mcp.add.name_placeholder", "ex. mon-serveur");
        m.insert("mcp.add.config_label", "Configuration du serveur (JSON)");
        m.insert("mcp.add.submit", "Ajouter le serveur");
        m.insert("mcp.add.name_required", "Veuillez entrer un nom de serveur");
        m.insert("mcp.source.claude_code", "Claude Code");
        m.insert("mcp.source.claude_desktop", "Claude Desktop");

        // ── MCP Browser ──
        m.insert("mcp_browser.title", "MCP Browser");
        m.insert("mcp_browser.subtitle", "D\u{00e9}couvrez et installez des serveurs MCP pour Claude Code");
        m.insert("mcp_browser.search_placeholder", "Rechercher des serveurs MCP...");
        m.insert("mcp_browser.loading", "Chargement du catalogue MCP");
        m.insert("mcp_browser.no_results", "Aucun serveur MCP trouv\u{00e9}");
        m.insert("mcp_browser.installed", "Install\u{00e9}");
        m.insert("mcp_browser.install", "Installer");
        m.insert("mcp_browser.needs_api_key", "Cl\u{00e9} API requise");
        m.insert("mcp_browser.install_success", "install\u{00e9} avec succ\u{00e8}s !");
        m.insert("mcp_browser.install_failed", "\u{00c9}chec de l\u{2019}installation");

        // ── Projects ──
        m.insert("projects.title", "Projets");
        m.insert("projects.subtitle", "Tous les projets enregistr\u{00e9}s dans ~/.claude.json");
        m.insert("projects.loading", "Chargement");
        m.insert("projects.error_loading", "Erreur lors du chargement des projets : ");
        m.insert("projects.col_name", "Nom");
        m.insert("projects.col_path", "Chemin");
        m.insert("projects.col_claude_md", "CLAUDE.md");
        m.insert("projects.col_skills", "Skills");
        m.insert("projects.col_rules", "R\u{00e8}gles");
        m.insert("projects.col_memory", "M\u{00e9}moire");
        m.insert("projects.yes", "Oui");

        // ── Project Detail ──
        m.insert("project_detail.loading", "Chargement des d\u{00e9}tails du projet");
        m.insert("project_detail.error_loading", "Erreur lors du chargement du projet");
        m.insert("project_detail.tab_advisor", "Conseiller");
        m.insert("project_detail.tab_claude_md", "CLAUDE.md");
        m.insert("project_detail.tab_skills", "Skills");
        m.insert("project_detail.tab_rules", "R\u{00e8}gles");
        m.insert("project_detail.tab_memory", "M\u{00e9}moire");
        m.insert("project_detail.tab_permissions", "Permissions");
        m.insert("project_detail.tab_health", "Sant\u{00e9}");
        m.insert("project_detail.no_claude_md", "Aucun CLAUDE.md trouv\u{00e9}");
        m.insert("project_detail.no_claude_md_hint", "Cr\u{00e9}ez un CLAUDE.md dans le r\u{00e9}pertoire de votre projet pour donner des instructions \u{00e0} Claude Code.");
        m.insert("project_detail.no_skills", "Aucun skill pour ce projet");
        m.insert("project_detail.no_rules", "Aucune r\u{00e8}gle pour ce projet");
        m.insert("project_detail.no_memory", "Aucune m\u{00e9}moire pour ce projet");
        m.insert("project_detail.save", "Enregistrer");
        m.insert("project_detail.saved", "Enregistr\u{00e9} !");
        m.insert("project_detail.skill_scope", "Port\u{00e9}e");
        m.insert("project_detail.permissions_loading", "Chargement des permissions...");
        m.insert("project_detail.permissions_error", "Erreur lors du chargement des permissions");
        m.insert("project_detail.permissions_entries", "Entr\u{00e9}es");
        m.insert("project_detail.permissions_col_tool", "Outil");
        m.insert("project_detail.permissions_col_command", "Commande");
        m.insert("project_detail.permissions_no_entries", "Aucune entr\u{00e9}e de permission");
        m.insert("project_detail.health_loading", "Calcul de la sant\u{00e9}...");
        m.insert("project_detail.health_error", "Erreur lors du chargement des donn\u{00e9}es de sant\u{00e9}");
        m.insert("project_detail.health_score", "Score de sant\u{00e9}");
        m.insert("project_detail.health_claude_md", "CLAUDE.md pr\u{00e9}sent");
        m.insert("project_detail.health_memory", "M\u{00e9}moire pr\u{00e9}sente");
        m.insert("project_detail.health_permissions", "Permissions");
        m.insert("project_detail.health_security_issues", "Probl\u{00e8}mes de s\u{00e9}curit\u{00e9}");
        m.insert("project_detail.health_duplicated_rules", "R\u{00e8}gles dupliqu\u{00e9}es");
        m.insert("project_detail.health_no_security_issues", "Aucun probl\u{00e8}me de s\u{00e9}curit\u{00e9} trouv\u{00e9}");
        m.insert("project_detail.health_col_text", "Texte");
        m.insert("project_detail.health_col_found_in", "Trouv\u{00e9} dans");
        m.insert("project_detail.health_col_also_in", "\u{00c9}galement dans");
        m.insert("project_detail.health_permission_entries", "Entr\u{00e9}es de permissions");
        m.insert("project_detail.permissions_col_index", "#");
        m.insert("project_detail.permissions_col_status", "Statut");
        m.insert("project_detail.permissions_fragment", "Fragment");
        m.insert("project_detail.permissions_ok", "OK");
        m.insert("project_detail.permissions_security_warnings", "avertissement(s) de s\u{00e9}curit\u{00e9}");
        m.insert("project_detail.permissions_manage", "G\u{00e9}rer les permissions");
        m.insert("project_detail.advisor_analyze", "Analyser le projet");
        m.insert("project_detail.advisor_analyzing", "Analyse en cours...");
        m.insert("project_detail.advisor_description", "Claude analyse votre projet et fournit des recommandations");
        m.insert("project_detail.advisor_loading", "Claude analyse votre projet");
        m.insert("project_detail.advisor_summary", "\u{00c9}valuation du projet");
        m.insert("project_detail.advisor_done", "Termin\u{00e9} !");
        m.insert("project_detail.advisor_preview", "Afficher l\u{2019}aper\u{00e7}u");
        m.insert("project_detail.advisor_category_tip", "Conseil");
        m.insert("project_detail.skills_col_name", "Nom");
        m.insert("project_detail.skills_col_description", "Description");
        m.insert("project_detail.skills_col_invocable", "Invocable");
        m.insert("project_detail.rules_col_name", "Nom");
        m.insert("project_detail.rules_col_path", "Chemin");
        m.insert("project_detail.memory_col_file", "Fichier");
        m.insert("project_detail.memory_col_size", "Taille");
        m.insert("project_detail.bytes", "octets");
        m.insert("project_detail.unknown_tab", "Onglet inconnu");

        // ── Global Skills ──
        m.insert("global_skills.title", "Skills globaux");
        m.insert("global_skills.subtitle", "G\u{00e9}rer les skills dans ~/.claude/skills/");
        m.insert("global_skills.loading", "Chargement des skills");
        m.insert("global_skills.no_skills", "Aucun skill global trouv\u{00e9}");
        m.insert("global_skills.no_skills_hint", "Cr\u{00e9}ez des skills dans ~/.claude/skills/ ou utilisez le Skill Browser.");
        m.insert("global_skills.select_skill", "S\u{00e9}lectionnez un skill dans la liste.");
        m.insert("global_skills.col_skill", "Skill");
        m.insert("global_skills.col_invocable", "Invocable");
        m.insert("global_skills.invocable", "Invocable");
        m.insert("global_skills.not_invocable", "Non invocable");
        m.insert("global_skills.editing", "Modification :");
        m.insert("global_skills.save", "Enregistrer");
        m.insert("global_skills.saved", "Enregistr\u{00e9} !");
        m.insert("global_skills.delete", "Supprimer");
        m.insert("global_skills.deleted", "Supprim\u{00e9} !");

        // ── Global Rules ──
        m.insert("global_rules.title", "R\u{00e8}gles globales");
        m.insert("global_rules.subtitle", "G\u{00e9}rer les r\u{00e8}gles dans ~/.claude/rules/");
        m.insert("global_rules.loading", "Chargement des r\u{00e8}gles");
        m.insert("global_rules.no_rules", "Aucune r\u{00e8}gle globale trouv\u{00e9}e");
        m.insert("global_rules.no_rules_hint", "Cr\u{00e9}ez des fichiers .md dans ~/.claude/rules/");
        m.insert("global_rules.select_rule", "S\u{00e9}lectionnez une r\u{00e8}gle dans la liste.");
        m.insert("global_rules.col_rule", "R\u{00e8}gle");
        m.insert("global_rules.editing", "Modification :");
        m.insert("global_rules.save", "Enregistrer");
        m.insert("global_rules.saved", "Enregistr\u{00e9} !");
        m.insert("global_rules.delete", "Supprimer");
        m.insert("global_rules.deleted", "Supprim\u{00e9} !");

        // ── Plans ──
        m.insert("plans.title", "Plans");
        m.insert("plans.subtitle", "G\u{00e9}rer les fichiers de plans dans ~/.claude/plans/");
        m.insert("plans.loading", "Chargement des plans");
        m.insert("plans.no_plans", "Aucun plan trouv\u{00e9}");
        m.insert("plans.no_plans_hint", "Les plans sont cr\u{00e9}\u{00e9}s par Claude Code lors de la planification.");
        m.insert("plans.select_plan", "S\u{00e9}lectionnez un plan dans la liste.");
        m.insert("plans.col_plan", "Plan");
        m.insert("plans.col_modified", "Modifi\u{00e9}");
        m.insert("plans.modified", "Modifi\u{00e9}");
        m.insert("plans.plan_label", "Plan :");
        m.insert("plans.save", "Enregistrer");
        m.insert("plans.saved", "Enregistr\u{00e9} !");
        m.insert("plans.delete", "Supprimer");
        m.insert("plans.deleted", "Supprim\u{00e9} !");

        // ── Settings ──
        m.insert("settings.title", "Param\u{00e8}tres");
        m.insert("settings.subtitle", "G\u{00e9}rer les param\u{00e8}tres et hooks de Claude Code");
        m.insert("settings.tab_overview", "Vue d\u{2019}ensemble");
        m.insert("settings.tab_hooks", "Mod\u{00e8}les de Hooks");
        m.insert("settings.tab_storage", "Stockage");
        m.insert("settings.loading", "Chargement des param\u{00e8}tres");
        m.insert("settings.hooks_title", "Hooks");
        m.insert("settings.no_hooks", "Aucun hook configur\u{00e9}");
        m.insert("settings.pre_tool_use", "PreToolUse");
        m.insert("settings.post_tool_use", "PostToolUse");
        m.insert("settings.matcher", "Matcher");
        m.insert("settings.command", "Commande");
        m.insert("settings.hook_templates_title", "Mod\u{00e8}les de Hooks");
        m.insert("settings.hook_templates_desc", "Configurations de hooks pr\u{00e9}d\u{00e9}finies \u{00e0} ajouter.");
        m.insert("settings.hook_templates_loading", "Chargement des mod\u{00e8}les");
        m.insert("settings.add_hook", "Ajouter");
        m.insert("settings.storage_title", "Utilisation du stockage");
        m.insert("settings.storage_loading", "Calcul du stockage");
        m.insert("settings.storage_total", "Total");
        m.insert("settings.storage_dir", "R\u{00e9}pertoire");
        m.insert("settings.storage_size", "Taille");

        // ── Permissions ──
        m.insert("permissions.title", "Permissions");
        m.insert("permissions.subtitle", "Examiner et g\u{00e9}rer les permissions des projets");
        m.insert("permissions.loading", "Chargement des permissions");
        m.insert("permissions.no_permissions", "Aucune permission trouv\u{00e9}e");
        m.insert("permissions.col_project", "Projet");
        m.insert("permissions.col_entries", "Entr\u{00e9}es");
        m.insert("permissions.col_issues", "Probl\u{00e8}mes");
        m.insert("permissions.col_fragmented", "Fragment\u{00e9}");
        m.insert("permissions.detail_title", "Permissions");
        m.insert("permissions.detail_loading", "Chargement des permissions");
        m.insert("permissions.detail_col_tool", "Outil");
        m.insert("permissions.detail_col_command", "Commande");
        m.insert("permissions.detail_col_status", "Statut");
        m.insert("permissions.detail_fragmented", "Fragment\u{00e9}");
        m.insert("permissions.detail_security_issue", "Probl\u{00e8}me de s\u{00e9}curit\u{00e9}");
        m.insert("permissions.detail_delete_selected", "Supprimer la s\u{00e9}lection");
        m.insert("permissions.detail_deleted", "Supprim\u{00e9} !");
        m.insert("permissions.detail_warnings_title", "Avertissements de s\u{00e9}curit\u{00e9}");
        m.insert("permissions.health_title", "Sant\u{00e9} de la configuration");
        m.insert("permissions.health_subtitle", "\u{00c9}tat de sant\u{00e9} de tous les projets");
        m.insert("permissions.health_loading", "Calcul de la sant\u{00e9}");
        m.insert("permissions.health_col_project", "Projet");
        m.insert("permissions.health_col_score", "Score");
        m.insert("permissions.health_col_issues", "Probl\u{00e8}mes");
        m.insert("permissions.health_avg", "Moyenne");
        m.insert("permissions.subtitle_manage", "G\u{00e9}rer les listes d\u{2019}autorisations de tous les projets");
        m.insert("permissions.col_actions", "Actions");
        m.insert("permissions.col_security_issues", "Probl\u{00e8}mes de s\u{00e9}curit\u{00e9}");
        m.insert("permissions.details", "D\u{00e9}tails");
        m.insert("permissions.detail_subtitle", "Examiner et nettoyer les entr\u{00e9}es de permissions");
        m.insert("permissions.detail_deleting", "Suppression...");
        m.insert("permissions.detail_deleted_reloading", "Supprim\u{00e9} ! Rechargement...");
        m.insert("permissions.detail_delete_count", "Supprimer la s\u{00e9}lection");
        m.insert("permissions.detail_col_index", "#");
        m.insert("permissions.detail_fragment", "Fragment");
        m.insert("permissions.detail_ok", "OK");
        m.insert("permissions.detail_warnings_count", "Avertissements de s\u{00e9}curit\u{00e9}");
        m.insert("permissions.detail_entry", "entr\u{00e9}e");
        m.insert("permissions.health_subtitle_scores", "Scores de sant\u{00e9} de configuration pour tous les projets");
        m.insert("permissions.health_avg_score", "Score de sant\u{00e9} moyen");
        m.insert("permissions.health_projects_analyzed", "Projets analys\u{00e9}s");
        m.insert("permissions.health_no_issues", "Aucun probl\u{00e8}me");

        // ── Analytics ──
        m.insert("analytics.title", "Statistiques");
        m.insert("analytics.subtitle", "Statistiques d\u{2019}utilisation de Claude Code");
        m.insert("analytics.loading", "Chargement des statistiques");
        m.insert("analytics.error_loading", "Erreur lors du chargement des statistiques");
        m.insert("analytics.total_sessions", "Total des sessions");
        m.insert("analytics.total_messages", "Total des messages");
        m.insert("analytics.git_commits", "Commits Git");
        m.insert("analytics.lines_added", "Lignes ajout\u{00e9}es");
        m.insert("analytics.lines_removed", "Lignes supprim\u{00e9}es");
        m.insert("analytics.since", "depuis");
        m.insert("analytics.activity_heatmap", "Carte d\u{2019}activit\u{00e9}");
        m.insert("analytics.messages", "Messages");
        m.insert("analytics.sessions", "Sessions");
        m.insert("analytics.tool_calls", "Appels d\u{2019}outils");
        m.insert("analytics.hourly_distribution", "Distribution horaire");
        m.insert("analytics.model_usage", "Utilisation des mod\u{00e8}les");
        m.insert("analytics.col_model", "Mod\u{00e8}le");
        m.insert("analytics.col_input_tokens", "Tokens d\u{2019}entr\u{00e9}e");
        m.insert("analytics.col_output_tokens", "Tokens de sortie");
        m.insert("analytics.col_cache_tokens", "Tokens de cache");
        m.insert("analytics.tool_ranking", "Classement des outils");
        m.insert("analytics.col_cache_read", "Cache lu");
        m.insert("analytics.tool_usage_top10", "Utilisation des outils (Top 10)");
        m.insert("analytics.languages", "Langues");
        m.insert("analytics.session_outcomes", "R\u{00e9}sultats des sessions");
        m.insert("analytics.outcomes", "R\u{00e9}sultats");

        // ── Sessions ──
        m.insert("sessions.title", "Sessions");
        m.insert("sessions.subtitle", "Parcourir l\u{2019}historique des sessions Claude Code");
        m.insert("sessions.loading", "Chargement des sessions");
        m.insert("sessions.search_placeholder", "Rechercher des sessions...");
        m.insert("sessions.no_sessions", "Aucune session trouv\u{00e9}e");
        m.insert("sessions.col_project", "Projet");
        m.insert("sessions.col_date", "Date");
        m.insert("sessions.col_duration", "Dur\u{00e9}e");
        m.insert("sessions.col_messages", "Messages");
        m.insert("sessions.col_summary", "R\u{00e9}sum\u{00e9}");
        m.insert("sessions.col_outcome", "R\u{00e9}sultat");
        m.insert("sessions.minutes", "min");
        m.insert("sessions.load_more", "Charger plus");
        m.insert("sessions.detail_title", "D\u{00e9}tails de la session");
        m.insert("sessions.detail_loading", "Chargement de la session");
        m.insert("sessions.detail_project", "Projet");
        m.insert("sessions.detail_start", "D\u{00e9}but");
        m.insert("sessions.detail_duration", "Dur\u{00e9}e");
        m.insert("sessions.detail_messages", "Messages");
        m.insert("sessions.detail_tools", "Appels d\u{2019}outils");
        m.insert("sessions.detail_tokens", "Tokens");
        m.insert("sessions.detail_git", "Git");
        m.insert("sessions.detail_first_prompt", "Premier prompt");
        m.insert("sessions.detail_summary", "R\u{00e9}sum\u{00e9}");
        m.insert("sessions.back", "Retour");
        m.insert("sessions.searching", "Recherche...");
        m.insert("sessions.search", "Rechercher");
        m.insert("sessions.clear", "Effacer");
        m.insert("sessions.search_results", "R\u{00e9}sultats de recherche");
        m.insert("sessions.no_results", "Aucun r\u{00e9}sultat trouv\u{00e9}");
        m.insert("sessions.col_prompt", "Prompt");
        m.insert("sessions.session_prefix", "Session : ");
        m.insert("sessions.detail_start_time", "Heure de d\u{00e9}but");
        m.insert("sessions.user_messages", " utilisateur / ");
        m.insert("sessions.assistant_messages", " assistant");
        m.insert("sessions.tokens_in", " entr\u{00e9}e / ");
        m.insert("sessions.tokens_out", " sortie");
        m.insert("sessions.commits_label", " commits, +");
        m.insert("sessions.lines_minus", " / -");
        m.insert("sessions.tools_used", "Outils utilis\u{00e9}s");
        m.insert("sessions.outcome_prefix", "R\u{00e9}sultat : ");
        m.insert("sessions.showing", "Affichage");
        m.insert("sessions.of", "sur");
        m.insert("sessions.previous", "Pr\u{00e9}c\u{00e9}dent");
        m.insert("sessions.next", "Suivant");

        // ── GitHub ──
        m.insert("github.title", "GitHub");
        m.insert("github.subtitle", "Statut de l\u{2019}int\u{00e9}gration GitHub");
        m.insert("github.loading", "Chargement des donn\u{00e9}es GitHub");
        m.insert("github.auth_status", "Statut d\u{2019}authentification");
        m.insert("github.username", "Nom d\u{2019}utilisateur");
        m.insert("github.linked_repos", "D\u{00e9}p\u{00f4}ts li\u{00e9}s");
        m.insert("github.no_repos", "Aucun d\u{00e9}p\u{00f4}t li\u{00e9}");
        m.insert("github.col_repo", "D\u{00e9}p\u{00f4}t");
        m.insert("github.col_recent_commits", "Commits r\u{00e9}cents");
        m.insert("github.col_open_prs", "PR ouvertes");

        // ── Help / System Info ──
        m.insert("help.title", "Informations syst\u{00e8}me");
        m.insert("help.subtitle", "Informations syst\u{00e8}me de Claude Code");
        m.insert("help.loading", "Chargement des informations syst\u{00e8}me");
        m.insert("help.account", "Compte");
        m.insert("help.account_name", "Nom");
        m.insert("help.account_email", "E-mail");
        m.insert("help.subscription", "Abonnement");
        m.insert("help.claude_version", "Version de Claude Code");
        m.insert("help.gh_cli", "GitHub CLI");
        m.insert("help.skill_usage", "Utilisation des skills");
        m.insert("help.no_skill_usage", "Aucune utilisation de skill enregistr\u{00e9}e");
        m.insert("help.col_skill", "Skill");
        m.insert("help.col_count", "Nombre");
        m.insert("help.what_is_title", "Qu\u{2019}est-ce que ClaudeAdmin ?");
        m.insert("help.what_is_desc", "ClaudeAdmin est la console d\u{2019}administration visuelle pour Claude Code. Elle fournit une interface web pour g\u{00e9}rer tous les aspects de votre configuration Claude Code : Projets, Skills, R\u{00e8}gles, M\u{00e9}moire, Param\u{00e8}tres, Hooks, Serveurs MCP et Plans.");
        m.insert("help.system_status", "Statut du syst\u{00e8}me");
        m.insert("help.not_set", "Non d\u{00e9}fini");
        m.insert("help.unknown", "Inconnu");
        m.insert("help.not_found", "Non trouv\u{00e9}");
        m.insert("help.not_installed", "Non install\u{00e9}");
        m.insert("help.concepts_title", "Concepts de Claude Code");
        m.insert("help.concept_skills", "Prompts r\u{00e9}utilisables avec frontmatter YAML. Stock\u{00e9}s sous forme de fichiers SKILL.md dans ~/.claude/skills/ (global) ou .claude/skills/ (projet).");
        m.insert("help.concept_rules", "Contraintes et directives qui fa\u{00e7}onnent le comportement de Claude. Stock\u{00e9}es sous forme de fichiers .md dans ~/.claude/rules/ ou au niveau du projet.");
        m.insert("help.concept_memory", "M\u{00e9}moire persistante par projet. MEMORY.md est automatiquement charg\u{00e9} dans les prompts syst\u{00e8}me. Stocke les mod\u{00e8}les, pr\u{00e9}f\u{00e9}rences et apprentissages.");
        m.insert("help.concept_hooks", "Commandes shell d\u{00e9}clench\u{00e9}es par des \u{00e9}v\u{00e9}nements (PreToolUse, PostToolUse, Stop). Configur\u{00e9}s dans settings.json pour le formatage automatique, le linting, etc.");
        m.insert("help.concept_mcp", "Les serveurs Model Context Protocol \u{00e9}tendent Claude avec des outils externes. Configur\u{00e9}s dans ~/.claude.json avec command, args et env.");
        m.insert("help.concept_claudemd", "Fichier d\u{2019}instructions au niveau du projet. Charg\u{00e9} automatiquement comme contexte. Contient les conventions du projet, les informations sur la stack et les directives de codage.");
        m.insert("help.disclaimer", "ClaudeAdmin est un projet communautaire ind\u{00e9}pendant. Il n'est pas affili\u{00e9}, soutenu ou approuv\u{00e9} par Anthropic. Claude et Claude Code sont des marques d'Anthropic.");

        m.insert("github.subtitle_detail", "Int\u{00e9}gration GitHub CLI et d\u{00e9}p\u{00f4}ts li\u{00e9}s");
        m.insert("github.linked_repositories", "D\u{00e9}p\u{00f4}ts li\u{00e9}s");
        m.insert("github.no_linked_repos", "Aucun d\u{00e9}p\u{00f4}t GitHub li\u{00e9} dans ~/.claude.json");
        m.insert("github.col_name", "Nom");
        m.insert("github.col_path", "Chemin");

        // ── Skill Browser ──
        m.insert("skill_browser.title", "Skill Browser");
        m.insert("skill_browser.subtitle", "D\u{00e9}couvrez et installez des skills officiels et communautaires");
        m.insert("skill_browser.loading", "Chargement des skills");
        m.insert("skill_browser.search_placeholder", "Rechercher des skills...");
        m.insert("skill_browser.no_results", "Aucun skill trouv\u{00e9}");
        m.insert("skill_browser.installed", "Install\u{00e9}");
        m.insert("skill_browser.install", "Installer");
        m.insert("skill_browser.official", "Officiel");
        m.insert("skill_browser.community", "Communaut\u{00e9}");
        m.insert("skill_browser.tab_official", "Officiel (Anthropic)");
        m.insert("skill_browser.tab_community", "Communaut\u{00e9}");
        m.insert("skill_browser.install_success", "install\u{00e9} avec succ\u{00e8}s !");
        m.insert("skill_browser.install_failed", "\u{00c9}chec de l\u{2019}installation :");

        // ── Docs ──
        m.insert("docs.title", "Documentation");
        m.insert("docs.subtitle", "Tout ce que vous devez savoir sur la configuration de Claude Code");
        m.insert("docs.loading", "Chargement de la documentation");

        // ── Docs: Table of Contents ──
        m.insert("docs.toc_contents", "Sommaire");
        m.insert("docs.toc_why_claudeadmin", "Pourquoi ClaudeAdmin ?");
        m.insert("docs.toc_capabilities", "Ce qu\u{2019}il peut et ne peut pas faire");
        m.insert("docs.toc_group", "Concepts");
        m.insert("docs.toc_claude_md", "CLAUDE.md");
        m.insert("docs.toc_rules", "R\u{00e8}gles");
        m.insert("docs.toc_skills", "Skills");
        m.insert("docs.toc_memory", "M\u{00e9}moire");
        m.insert("docs.toc_settings", "Param\u{00e8}tres & Hooks");
        m.insert("docs.toc_mcp", "Serveurs MCP");
        m.insert("docs.toc_plans", "Plans");
        m.insert("docs.toc_scopes", "Global vs. Projet");
        m.insert("docs.toc_tips", "Conseils & Bonnes pratiques");
        m.insert("docs.toc_links", "Documentation officielle");

        // ── Docs: Shared labels ──
        m.insert("docs.tips_heading", "Trucs & Astuces");
        m.insert("docs.scope_global", "Global");
        m.insert("docs.scope_project", "Projet");
        m.insert("docs.scope_user", "Utilisateur");
        m.insert("docs.scope_parent", "Parent");
        m.insert("docs.scope_managed", "G\u{00e9}r\u{00e9}");
        m.insert("docs.scope_local", "Local");

        // ── Docs: Overview ──
        m.insert("docs.overview_heading", "Pourquoi ClaudeAdmin ?");
        m.insert("docs.overview_callout", " est la console d\u{2019}administration centrale pour l\u{2019}ensemble de votre configuration Claude Code. Elle remplace la modification manuelle de fichiers dans des dizaines de r\u{00e9}pertoires cach\u{00e9}s par une interface visuelle unique.");
        m.insert("docs.overview_text1", "Claude Code stocke sa configuration dans une hi\u{00e9}rarchie complexe de fichiers et de r\u{00e9}pertoires : des fichiers CLAUDE.md dans les racines de projets, des r\u{00e8}gles et skills dispers\u{00e9}s dans les sous-r\u{00e9}pertoires ~/.claude/, des fichiers m\u{00e9}moire index\u{00e9}s par chemins de projets encod\u{00e9}s, des param\u{00e8}tres dans plusieurs fichiers JSON et des configurations de serveurs MCP dans ~/.claude.json. \u{00c0} mesure que vos projets grandissent, g\u{00e9}rer tout cela manuellement devient source d\u{2019}erreurs et chronophage.");
        m.insert("docs.overview_text2", "ClaudeAdmin vous offre :");
        m.insert("docs.overview_li_visibility_label", "Visibilit\u{00e9}");
        m.insert("docs.overview_li_visibility", " \u{2013} Visualisez tous vos projets, skills, r\u{00e8}gles et m\u{00e9}moire en un seul endroit");
        m.insert("docs.overview_li_editing_label", "\u{00c9}dition");
        m.insert("docs.overview_li_editing", " \u{2013} Modifiez CLAUDE.md, r\u{00e8}gles, skills et m\u{00e9}moire avec un \u{00e9}diteur adapt\u{00e9}");
        m.insert("docs.overview_li_health_label", "Contr\u{00f4}les de sant\u{00e9}");
        m.insert("docs.overview_li_health", " \u{2013} D\u{00e9}tectez les probl\u{00e8}mes de s\u{00e9}curit\u{00e9} dans les permissions, les r\u{00e8}gles dupliqu\u{00e9}es et les configurations manquantes");
        m.insert("docs.overview_li_analytics_label", "Statistiques");
        m.insert("docs.overview_li_analytics", " \u{2013} Comprenez comment vous utilisez Claude Code : sessions, tokens, outils, co\u{00fb}ts");
        m.insert("docs.overview_li_advisor_label", "Conseiller");
        m.insert("docs.overview_li_advisor", " \u{2013} Recommandations aliment\u{00e9}es par l\u{2019}IA pour am\u{00e9}liorer la configuration de votre projet");

        // ── Docs: Capabilities ──
        m.insert("docs.cap_heading", "Ce que ClaudeAdmin peut et ne peut pas faire");
        m.insert("docs.cap_can_heading", "Ce qu\u{2019}il peut faire");
        m.insert("docs.cap_can_1", "Parcourir et g\u{00e9}rer tous les projets enregistr\u{00e9}s dans ~/.claude.json");
        m.insert("docs.cap_can_2", "Afficher et modifier les fichiers CLAUDE.md de tout projet");
        m.insert("docs.cap_can_3", "Cr\u{00e9}er, modifier et supprimer les skills globaux et par projet");
        m.insert("docs.cap_can_4", "Cr\u{00e9}er, modifier et supprimer les r\u{00e8}gles globales et par projet");
        m.insert("docs.cap_can_5", "Afficher et modifier les fichiers m\u{00e9}moire du projet (MEMORY.md et sujets)");
        m.insert("docs.cap_can_6", "Inspecter la hi\u{00e9}rarchie des param\u{00e8}tres (global \u{2192} projet \u{2192} local)");
        m.insert("docs.cap_can_7", "Auditer les entr\u{00e9}es de permissions et d\u{00e9}tecter les probl\u{00e8}mes de s\u{00e9}curit\u{00e9}");
        m.insert("docs.cap_can_8", "Visualiser les configurations des serveurs MCP");
        m.insert("docs.cap_can_9", "Analyser l\u{2019}historique des sessions, l\u{2019}utilisation des tokens et les co\u{00fb}ts");
        m.insert("docs.cap_can_10", "Ex\u{00e9}cuter une analyse de projet pilot\u{00e9}e par l\u{2019}IA avec des recommandations actionables");
        m.insert("docs.cap_can_11", "Parcourir et installer des skills depuis les d\u{00e9}p\u{00f4}ts communautaires");
        m.insert("docs.cap_can_12", "Toutes les \u{00e9}critures cr\u{00e9}ent des sauvegardes automatiques dans ~/.claude/backups/");
        m.insert("docs.cap_cannot_heading", "Ce qu\u{2019}il ne peut pas faire");
        m.insert("docs.cap_cannot_1", "Ex\u{00e9}cuter des sessions Claude Code \u{2013} il g\u{00e8}re la configuration, pas l\u{2019}ex\u{00e9}cution");
        m.insert("docs.cap_cannot_2", "Modifier les politiques g\u{00e9}r\u{00e9}es (param\u{00e8}tres au niveau entreprise/organisation)");
        m.insert("docs.cap_cannot_3", "Acc\u{00e9}der aux environnements distants ou aux sessions SSH");
        m.insert("docs.cap_cannot_4", "Remplacer le CLI Claude Code pour le travail de codage r\u{00e9}el");
        m.insert("docs.cap_cannot_5", "Modifier directement les serveurs MCP de .claude.json (lecture seule par s\u{00e9}curit\u{00e9})");
        m.insert("docs.cap_cannot_6", "G\u{00e9}rer les cl\u{00e9}s API ou les identifiants d\u{2019}authentification");
        m.insert("docs.cap_cannot_callout", "ClaudeAdmin est un gestionnaire de configuration, pas un remplacement de Claude Code. Consid\u{00e9}rez-le comme un outil d\u{2019}administration de base de donn\u{00e9}es : il vous aide \u{00e0} inspecter, configurer et maintenir \u{2013} mais le travail r\u{00e9}el se fait dans Claude Code.");

        // ── Docs: CLAUDE.md ──
        m.insert("docs.claudemd_callout", "La constitution du projet. CLAUDE.md est le fichier de configuration le plus important \u{2013} il est automatiquement charg\u{00e9} dans chaque session Claude Code comme contexte persistant.");
        m.insert("docs.claudemd_how_heading", "Comment \u{00e7}a marche");
        m.insert("docs.claudemd_how_text", "Lorsque Claude Code d\u{00e9}marre une session, il recherche r\u{00e9}cursivement les fichiers CLAUDE.md depuis votre r\u{00e9}pertoire de travail actuel jusqu\u{2019}\u{00e0} la racine du syst\u{00e8}me de fichiers. Tous les fichiers trouv\u{00e9}s sont charg\u{00e9}s et concat\u{00e9}n\u{00e9}s, les fichiers les plus proches ayant la priorit\u{00e9}. Cela signifie que vous pouvez avoir un CLAUDE.md au niveau du monorepo avec des conventions partag\u{00e9}es et des fichiers CLAUDE.md au niveau des packages avec des surcharges sp\u{00e9}cifiques.");
        m.insert("docs.claudemd_locations_heading", "Emplacements");
        m.insert("docs.claudemd_loc_project_or", " ou ");
        m.insert("docs.claudemd_loc_parent", "Racine du monorepo, charg\u{00e9} pour tous les sous-packages");
        m.insert("docs.claudemd_loc_user", "Param\u{00e8}tres personnels par d\u{00e9}faut pour tous les projets");
        m.insert("docs.claudemd_whatto_heading", "Que mettre dedans");
        m.insert("docs.claudemd_whatto_context_label", "Contexte du projet");
        m.insert("docs.claudemd_whatto_context", " \u{2013} Stack technique, d\u{00e9}cisions d\u{2019}architecture, d\u{00e9}pendances cl\u{00e9}s");
        m.insert("docs.claudemd_whatto_standards_label", "Standards de codage");
        m.insert("docs.claudemd_whatto_standards", " \u{2013} Conventions de nommage, r\u{00e8}gles de formatage, patterns de gestion d\u{2019}erreurs");
        m.insert("docs.claudemd_whatto_workflows_label", "Workflows");
        m.insert("docs.claudemd_whatto_workflows", " \u{2013} Comment compiler, tester, d\u{00e9}ployer ; nommage des branches ; conventions PR");
        m.insert("docs.claudemd_whatto_dodont_label", "R\u{00e8}gles \u{00e0} faire/ne pas faire");
        m.insert("docs.claudemd_whatto_dodont", " \u{2013} Contraintes explicites (ex. \u{201c}ne jamais utiliser any en TypeScript\u{201d})");
        m.insert("docs.claudemd_whatto_team_label", "Accords d\u{2019}\u{00e9}quipe");
        m.insert("docs.claudemd_whatto_team", " \u{2013} Processus de revue, format des messages de commit, fronti\u{00e8}res de modules");
        m.insert("docs.claudemd_tip1", "Restez sous les 500 lignes. Claude charge le fichier entier dans le contexte \u{2013} les fichiers CLAUDE.md gonfl\u{00e9}s gaspillent des tokens et diluent les instructions importantes.");
        m.insert("docs.claudemd_tip2", "Utilisez des en-t\u{00ea}tes de section clairs (## Architecture, ## Conventions). Claude analyse la structure pour trouver les sections pertinentes.");
        m.insert("docs.claudemd_tip3", "Mettez les r\u{00e8}gles les plus critiques en haut. Dans les fichiers longs, le contenu au d\u{00e9}but re\u{00e7}oit plus d\u{2019}attention.");
        m.insert("docs.claudemd_tip4", "Utilisez CLAUDE.local.md pour les pr\u{00e9}f\u{00e9}rences personnelles qui ne doivent pas \u{00ea}tre commit\u{00e9}es dans git.");
        m.insert("docs.claudemd_ext_link", "Anthropic Docs : CLAUDE.md \u{2192}");

        // ── Docs: Rules ──
        m.insert("docs.rules_heading", "R\u{00e8}gles");
        m.insert("docs.rules_callout", "Contraintes modulaires et th\u{00e9}matiques qui fa\u{00e7}onnent le comportement de Claude. Contrairement \u{00e0} CLAUDE.md qui est un seul gros fichier, les r\u{00e8}gles sont des fichiers .md s\u{00e9}par\u{00e9}s \u{2013} chacun ax\u{00e9} sur un sujet sp\u{00e9}cifique.");
        m.insert("docs.rules_how_heading", "Comment \u{00e7}a marche");
        m.insert("docs.rules_how_text", "Les r\u{00e8}gles sont charg\u{00e9}es automatiquement au d\u{00e9}marrage de la session. Les r\u{00e8}gles globales (vos pr\u{00e9}f\u{00e9}rences personnelles) sont charg\u{00e9}es en premier, puis les r\u{00e8}gles du projet se superposent. Cela vous permet de d\u{00e9}finir votre style de codage globalement tandis que les projets ajoutent des contraintes sp\u{00e9}cifiques au domaine.");
        m.insert("docs.rules_locations_heading", "Emplacements");
        m.insert("docs.rules_loc_global", "Vos r\u{00e8}gles personnelles, appliqu\u{00e9}es \u{00e0} tous les projets");
        m.insert("docs.rules_loc_project", "Sp\u{00e9}cifique au projet, commit\u{00e9} dans git pour le partage en \u{00e9}quipe");
        m.insert("docs.rules_examples_heading", "Exemples");
        m.insert("docs.rules_example_frontend", " \u{2013} Patterns de composants React, r\u{00e8}gles de gestion d\u{2019}\u{00e9}tat");
        m.insert("docs.rules_example_security", " \u{2013} Validation des entr\u{00e9}es, patterns d\u{2019}authentification, conformit\u{00e9} OWASP");
        m.insert("docs.rules_example_testing", " \u{2013} Structure des tests, attentes de couverture, strat\u{00e9}gie de mocking");
        m.insert("docs.rules_example_rust", " \u{2013} Gestion d\u{2019}erreurs avec thiserror, structure de modules, nommage");
        m.insert("docs.rules_tip1", "Un sujet par fichier. Ne m\u{00e9}langez pas les r\u{00e8}gles frontend et backend \u{2013} les fichiers plus petits et cibl\u{00e9}s sont plus faciles \u{00e0} maintenir et r\u{00e9}utiliser.");
        m.insert("docs.rules_tip2", "Les r\u{00e8}gles globales sont id\u{00e9}ales pour les pr\u{00e9}f\u{00e9}rences de style personnelles : langue pr\u{00e9}f\u{00e9}r\u{00e9}e, outil de formatage, format de message de commit.");
        m.insert("docs.rules_tip3", "Les r\u{00e8}gles du projet surchargent les r\u{00e8}gles globales. En cas de conflit, la r\u{00e8}gle au niveau du projet l\u{2019}emporte.");
        m.insert("docs.rules_tip4", "Utilisez le contr\u{00f4}le de sant\u{00e9} de ClaudeAdmin pour d\u{00e9}tecter les r\u{00e8}gles dupliqu\u{00e9}es entre le niveau global et le niveau projet.");
        m.insert("docs.rules_ext_link", "Anthropic Docs : Rules \u{2192}");

        // ── Docs: Skills ──
        m.insert("docs.skills_heading", "Skills");
        m.insert("docs.skills_callout", "Prompts r\u{00e9}utilisables et structur\u{00e9}s avec des m\u{00e9}tadonn\u{00e9}es. Les skills sont comme des plugins pour Claude \u{2013} ils peuvent \u{00ea}tre d\u{00e9}clench\u{00e9}s automatiquement par le contexte ou invoqu\u{00e9}s manuellement via des commandes slash.");
        m.insert("docs.skills_how_heading", "Comment \u{00e7}a marche");
        m.insert("docs.skills_how_text", "Chaque skill r\u{00e9}side dans son propre r\u{00e9}pertoire contenant un fichier SKILL.md avec un frontmatter YAML et un corps markdown. Le frontmatter d\u{00e9}finit les m\u{00e9}tadonn\u{00e9}es comme la description et les conditions de d\u{00e9}clenchement. Le corps contient les instructions de prompt, les exemples et le mat\u{00e9}riel de r\u{00e9}f\u{00e9}rence.");
        m.insert("docs.skills_structure_heading", "Structure");
        m.insert("docs.skills_locations_heading", "Emplacements");
        m.insert("docs.skills_loc_global", "Disponible dans tous les projets");
        m.insert("docs.skills_loc_project", "Skills sp\u{00e9}cifiques au projet");
        m.insert("docs.skills_tip1", "D\u{00e9}finissez user_invocable: true dans le frontmatter pour rendre un skill appelable via /nom-du-skill dans Claude Code.");
        m.insert("docs.skills_tip2", "Incluez des exemples concrets dans votre SKILL.md. Claude fonctionne beaucoup mieux avec des exemples d\u{2019}entr\u{00e9}e/sortie.");
        m.insert("docs.skills_tip3", "Utilisez le Skill Browser de ClaudeAdmin pour d\u{00e9}couvrir et installer des skills communautaires.");
        m.insert("docs.skills_tip4", "Les fichiers de r\u{00e9}f\u{00e9}rence dans le r\u{00e9}pertoire du skill ne sont charg\u{00e9}s que lorsque le skill est d\u{00e9}clench\u{00e9}, ce qui \u{00e9}conomise des tokens.");
        m.insert("docs.skills_ext_link", "Anthropic Docs : Skills \u{2192}");

        // ── Docs: Memory ──
        m.insert("docs.memory_heading", "M\u{00e9}moire");
        m.insert("docs.memory_callout", "La base de connaissances persistante de Claude par projet. Les fichiers m\u{00e9}moire stockent les mod\u{00e8}les, pr\u{00e9}f\u{00e9}rences et apprentissages que Claude accumule au fil des sessions.");
        m.insert("docs.memory_how_heading", "Comment \u{00e7}a marche");
        m.insert("docs.memory_how_text", "Claude Code maintient un r\u{00e9}pertoire m\u{00e9}moire pour chaque projet, stock\u{00e9} dans ~/.claude/projects/<chemin-encod\u{00e9}>/memory/. Le fichier principal MEMORY.md a un statut sp\u{00e9}cial : ses 200 premi\u{00e8}res lignes sont charg\u{00e9}es dans le prompt syst\u{00e8}me au d\u{00e9}marrage de la session. Les fichiers de sujets suppl\u{00e9}mentaires (debugging.md, api-conventions.md, etc.) sont charg\u{00e9}s \u{00e0} la demande lorsque Claude d\u{00e9}termine qu\u{2019}ils sont pertinents pour la t\u{00e2}che en cours.");
        m.insert("docs.memory_structure_heading", "Structure");
        m.insert("docs.memory_auto_heading", "M\u{00e9}moire automatique");
        m.insert("docs.memory_auto_text", "Claude Code peut ajouter automatiquement des entr\u{00e9}es \u{00e0} la m\u{00e9}moire lorsqu\u{2019}il d\u{00e9}couvre des mod\u{00e8}les de projet, des solutions de d\u{00e9}bogage ou vos pr\u{00e9}f\u{00e9}rences. Vous pouvez consulter et modifier la m\u{00e9}moire auto-g\u{00e9}n\u{00e9}r\u{00e9}e avec la commande /memory dans Claude Code ou via l\u{2019}\u{00e9}diteur de m\u{00e9}moire de ClaudeAdmin.");
        m.insert("docs.memory_tip1", "Mettez les informations les plus importantes dans les 200 premi\u{00e8}res lignes de MEMORY.md \u{2013} c\u{2019}est ce qui est charg\u{00e9} automatiquement.");
        m.insert("docs.memory_tip2", "Utilisez des fichiers de sujets pour les connaissances approfondies. Ils ne sont charg\u{00e9}s qu\u{2019}en cas de besoin, maintenant une faible utilisation de tokens de base.");
        m.insert("docs.memory_tip3", "V\u{00e9}rifiez r\u{00e9}guli\u{00e8}rement la m\u{00e9}moire automatique. Claude stocke parfois des solutions ponctuelles trop sp\u{00e9}cifiques.");
        m.insert("docs.memory_tip4", "La m\u{00e9}moire est par projet. Si vous passez \u{00e0} un autre projet, Claude obtient un ensemble de souvenirs diff\u{00e9}rent.");
        m.insert("docs.memory_ext_link", "Anthropic Docs : Memory \u{2192}");

        // ── Docs: Settings & Hooks ──
        m.insert("docs.settings_heading", "Param\u{00e8}tres & Hooks");
        m.insert("docs.settings_heading_short", "Param\u{00e8}tres");
        m.insert("docs.settings_callout", "Configuration JSON pour le comportement, les permissions et l\u{2019}automatisation. Les hooks vous permettent d\u{2019}ex\u{00e9}cuter automatiquement des commandes shell avant ou apr\u{00e8}s que Claude utilise des outils.");
        m.insert("docs.settings_hierarchy_heading", "Hi\u{00e9}rarchie des param\u{00e8}tres");
        m.insert("docs.settings_hierarchy_text", "Les param\u{00e8}tres suivent un mod\u{00e8}le en couches avec une sp\u{00e9}cificit\u{00e9} croissante. Les couches plus sp\u{00e9}cifiques surchargent les moins sp\u{00e9}cifiques :");
        m.insert("docs.settings_managed_code", "Politiques d\u{2019}entreprise");
        m.insert("docs.settings_managed_desc", "Priorit\u{00e9} la plus \u{00e9}lev\u{00e9}e, d\u{00e9}finie par l\u{2019}organisation (lecture seule)");
        m.insert("docs.settings_global_desc", "Vos param\u{00e8}tres globaux personnels");
        m.insert("docs.settings_project_desc", "Param\u{00e8}tres d\u{2019}\u{00e9}quipe, commit\u{00e9}s dans git");
        m.insert("docs.settings_local_desc", "Vos surcharges personnelles de projet (ignor\u{00e9}es par git)");
        m.insert("docs.settings_hooks_heading", "Hooks");
        m.insert("docs.settings_hooks_text", "Les hooks sont des commandes shell d\u{00e9}clench\u{00e9}es lors d\u{2019}\u{00e9}v\u{00e9}nements sp\u{00e9}cifiques pendant une session Claude Code. Ils sont configur\u{00e9}s dans settings.json sous la cl\u{00e9} hooks.");
        m.insert("docs.settings_hooks_events", "\u{00c9}v\u{00e9}nements :\n\u{2022} PreToolUse  \u{2013} Avant que Claude ex\u{00e9}cute un outil (ex. formatage automatique avant \u{00e9}criture)\n\u{2022} PostToolUse \u{2013} Apr\u{00e8}s que Claude a ex\u{00e9}cut\u{00e9} un outil (ex. linting apr\u{00e8}s modification de fichier)\n\u{2022} Stop        \u{2013} Lorsque Claude termine une r\u{00e9}ponse");
        m.insert("docs.settings_tip1", "Utilisez les hooks PreToolUse pour formater automatiquement le code avant que Claude \u{00e9}crive des fichiers. Cela garantit un style coh\u{00e9}rent.");
        m.insert("docs.settings_tip2", "Les hooks PostToolUse sont id\u{00e9}aux pour le linting : d\u{00e9}tectez les probl\u{00e8}mes imm\u{00e9}diatement apr\u{00e8}s que Claude modifie le code.");
        m.insert("docs.settings_tip3", "La page Param\u{00e8}tres de ClaudeAdmin affiche la cha\u{00ee}ne de hooks effective \u{00e0} travers toutes les couches.");
        m.insert("docs.settings_ext_link", "Anthropic Docs : Settings \u{2192}");
        m.insert("docs.settings_hooks_ext_link", "Anthropic Docs : Hooks \u{2192}");

        // ── Docs: MCP Servers ──
        m.insert("docs.mcp_heading", "Serveurs MCP");
        m.insert("docs.mcp_callout", "Les serveurs Model Context Protocol \u{00e9}tendent Claude avec des outils et sources de donn\u{00e9}es externes. Ils permettent \u{00e0} Claude d\u{2019}interagir avec des bases de donn\u{00e9}es, des API, des syst\u{00e8}mes de fichiers et d\u{2019}autres services.");
        m.insert("docs.mcp_how_heading", "Comment \u{00e7}a marche");
        m.insert("docs.mcp_how_text", "Les serveurs MCP sont des processus externes que Claude Code lance et avec lesquels il communique via le protocole MCP. Chaque serveur fournit un ensemble d\u{2019}outils que Claude peut appeler. La configuration se trouve dans ~/.claude.json sous la cl\u{00e9} mcpServers.");
        m.insert("docs.mcp_config_heading", "Configuration");
        m.insert("docs.mcp_management_heading", "Gestion dans ClaudeAdmin");
        m.insert("docs.mcp_management_text", "ClaudeAdmin fournit une page d\u{00e9}di\u{00e9}e aux serveurs MCP pour une gestion compl\u{00e8}te : afficher, ajouter, modifier et supprimer des serveurs sans modification manuelle du JSON. Le contr\u{00f4}le de sant\u{00e9} lance chaque serveur et v\u{00e9}rifie qu\u{2019}il r\u{00e9}pond aux requ\u{00ea}tes JSON-RPC initialize et tools/list. Utilisez le MCP Browser pour d\u{00e9}couvrir et installer des serveurs populaires en un clic.");
        m.insert("docs.mcp_tip1", "Les serveurs MCP peuvent \u{00e9}galement \u{00ea}tre configur\u{00e9}s par projet dans .claude/settings.json.");
        m.insert("docs.mcp_tip2", "Utilisez des variables d\u{2019}environnement pour les secrets \u{2013} ne codez jamais les cl\u{00e9}s API en dur dans les fichiers de configuration.");
        m.insert("docs.mcp_tip3", "Utilisez le MCP Browser pour d\u{00e9}couvrir et installer des serveurs populaires, ou ajoutez des serveurs personnalis\u{00e9}s via l\u{2019}onglet Nouveau serveur.");
        m.insert("docs.mcp_ext_link", "Anthropic Docs : MCP \u{2192}");
        m.insert("docs.mcp_spec_ext_link", "Sp\u{00e9}cification MCP \u{2192}");

        // ── Docs: Plans ──
        m.insert("docs.plans_heading", "Plans");
        m.insert("docs.plans_callout", "Fichiers markdown que Claude utilise pour d\u{00e9}composer les t\u{00e2}ches complexes. Les plans aident Claude \u{00e0} garder le cap sur les travaux en plusieurs \u{00e9}tapes et \u{00e0} suivre la progression.");
        m.insert("docs.plans_how_heading", "Comment \u{00e7}a marche");
        m.insert("docs.plans_how_text", "Lorsque Claude s\u{2019}attaque \u{00e0} une t\u{00e2}che complexe, il peut cr\u{00e9}er ou r\u{00e9}f\u{00e9}rencer des fichiers de plans stock\u{00e9}s dans ~/.claude/plans/. Les plans sont des documents markdown structur\u{00e9}s avec des listes de t\u{00e2}ches, des d\u{00e9}pendances et un suivi de statut. Ils persistent entre les sessions, permettant \u{00e0} Claude de reprendre l\u{00e0} o\u{00f9} il s\u{2019}\u{00e9}tait arr\u{00ea}t\u{00e9}.");
        m.insert("docs.plans_location_heading", "Emplacement");
        m.insert("docs.plans_loc_global", "Tous les fichiers de plans");
        m.insert("docs.plans_tip1", "Demandez \u{00e0} Claude de \u{201c}faire un plan\u{201d} avant un refactoring complexe. Les plans r\u{00e9}duisent les erreurs lors de modifications multi-fichiers.");
        m.insert("docs.plans_tip2", "Nettoyez les anciens plans p\u{00e9}riodiquement. La page Plans de ClaudeAdmin affiche tous les plans stock\u{00e9}s avec leurs dates de modification.");

        // ── Docs: Scopes ──
        m.insert("docs.scopes_heading", "Port\u{00e9}e globale vs. projet");
        m.insert("docs.scopes_callout", "Comprendre la port\u{00e9}e est essentiel pour une configuration efficace de Claude Code. Chaque type de configuration existe en deux couches : globale (vos param\u{00e8}tres par d\u{00e9}faut personnels) et sp\u{00e9}cifique au projet (partag\u{00e9}e avec votre \u{00e9}quipe).");
        m.insert("docs.scopes_overview_heading", "Vue d\u{2019}ensemble des port\u{00e9}es");
        m.insert("docs.scopes_col_type", "Type de configuration");
        m.insert("docs.scopes_col_global", "Global (utilisateur)");
        m.insert("docs.scopes_col_project", "Projet");
        m.insert("docs.scopes_col_priority", "Priorit\u{00e9}");
        m.insert("docs.scopes_priority_project_global", "Projet > Global");
        m.insert("docs.scopes_priority_both", "Les deux disponibles");
        m.insert("docs.scopes_memory_global", "Par projet dans ~/.claude/projects/");
        m.insert("docs.scopes_priority_project_keyed", "Cl\u{00e9} par projet");
        m.insert("docs.scopes_priority_local_project_global", "Local > Projet > Global");
        m.insert("docs.scopes_priority_merged", "Fusionn\u{00e9}");
        m.insert("docs.scopes_when_heading", "Quand utiliser lequel ?");
        m.insert("docs.scopes_use_global", "Utiliser Global pour");
        m.insert("docs.scopes_global_1", "Pr\u{00e9}f\u{00e9}rences personnelles de style de codage");
        m.insert("docs.scopes_global_2", "Langue et frameworks par d\u{00e9}faut pr\u{00e9}f\u{00e9}r\u{00e9}s");
        m.insert("docs.scopes_global_3", "Format des messages de commit");
        m.insert("docs.scopes_global_4", "Param\u{00e8}tres d\u{2019}int\u{00e9}gration \u{00e9}diteur/IDE");
        m.insert("docs.scopes_global_5", "Serveurs MCP utilis\u{00e9}s dans tous les projets");
        m.insert("docs.scopes_use_project", "Utiliser Projet pour");
        m.insert("docs.scopes_project_1", "Documentation et contraintes de la stack technique");
        m.insert("docs.scopes_project_2", "Conventions de codage de l\u{2019}\u{00e9}quipe");
        m.insert("docs.scopes_project_3", "R\u{00e8}gles sp\u{00e9}cifiques au domaine (s\u{00e9}curit\u{00e9}, conformit\u{00e9})");
        m.insert("docs.scopes_project_4", "Skills et workflows sp\u{00e9}cifiques au projet");
        m.insert("docs.scopes_project_5", "Hooks et automatisation CI/CD");

        // ── Docs: Tips & Best Practices ──
        m.insert("docs.bestpractices_heading", "Conseils & Bonnes pratiques");
        m.insert("docs.bestpractices_hygiene_heading", "Hygi\u{00e8}ne de configuration");
        m.insert("docs.bestpractices_hygiene_1", "Ex\u{00e9}cutez r\u{00e9}guli\u{00e8}rement le contr\u{00f4}le de sant\u{00e9} de ClaudeAdmin. Il d\u{00e9}tecte les r\u{00e8}gles dupliqu\u{00e9}es, les listes de permissions gonfl\u{00e9}es et les fichiers CLAUDE.md manquants.");
        m.insert("docs.bestpractices_hygiene_2", "Ne vous r\u{00e9}p\u{00e9}tez pas : si une r\u{00e8}gle existe globalement, ne la copiez pas dans le CLAUDE.md du projet. Utilisez le syst\u{00e8}me de port\u{00e9}es.");
        m.insert("docs.bestpractices_hygiene_3", "Gardez les listes de permissions propres. Au fil du temps, Claude Code accumule des centaines d\u{2019}entr\u{00e9}es d\u{2019}autorisation/refus. Utilisez la page Permissions pour les nettoyer.");
        m.insert("docs.bestpractices_tokens_heading", "Efficacit\u{00e9} des tokens");
        m.insert("docs.bestpractices_tokens_1", "Tout ce qui est dans CLAUDE.md, les r\u{00e8}gles, les skills (lorsqu\u{2019}ils sont d\u{00e9}clench\u{00e9}s) et les 200 premi\u{00e8}res lignes de MEMORY.md compte dans votre fen\u{00ea}tre de contexte. Soyez concis.");
        m.insert("docs.bestpractices_tokens_2", "D\u{00e9}placez le mat\u{00e9}riel de r\u{00e9}f\u{00e9}rence d\u{00e9}taill\u{00e9} dans les fichiers de r\u{00e9}f\u{00e9}rence de skills ou les fichiers de sujets m\u{00e9}moire \u{2013} ils ne sont charg\u{00e9}s qu\u{2019}en cas de besoin.");
        m.insert("docs.bestpractices_tokens_3", "Utilisez la page Statistiques pour surveiller votre utilisation de tokens \u{00e0} travers les projets et les sessions.");
        m.insert("docs.bestpractices_team_heading", "Collaboration en \u{00e9}quipe");
        m.insert("docs.bestpractices_team_1", "Commitez .claude/rules/ et .claude/skills/ dans git. Cela partage les conventions avec l\u{2019}\u{00e9}quipe.");
        m.insert("docs.bestpractices_team_2", "Utilisez .claude/settings.json pour les param\u{00e8}tres d\u{2019}\u{00e9}quipe et .claude/settings.local.json pour les surcharges personnelles.");
        m.insert("docs.bestpractices_team_3", "CLAUDE.md \u{00e0} la racine du projet est le contrat de votre \u{00e9}quipe avec Claude. Traitez-le comme de la documentation \u{2013} examinez les modifications dans les PR.");
        m.insert("docs.bestpractices_debug_heading", "D\u{00e9}boguer le comportement de Claude");
        m.insert("docs.bestpractices_debug_1", "Si Claude ignore une r\u{00e8}gle, v\u{00e9}rifiez la hi\u{00e9}rarchie des param\u{00e8}tres pour d\u{00e9}tecter des param\u{00e8}tres conflictuels entre les couches.");
        m.insert("docs.bestpractices_debug_2", "La m\u{00e9}moire peut causer un comportement inattendu. V\u{00e9}rifiez les entr\u{00e9}es auto-g\u{00e9}n\u{00e9}r\u{00e9}es \u{2013} Claude a peut-\u{00ea}tre m\u{00e9}moris\u{00e9} un contournement au lieu de l\u{2019}approche correcte.");
        m.insert("docs.bestpractices_debug_3", "Utilisez la page Sessions pour revoir les conversations pass\u{00e9}es et comprendre ce que Claude \u{201c}pensait\u{201d}.");

        // ── Docs: Links ──
        m.insert("docs.links_heading", "Documentation officielle d\u{2019}Anthropic");
        m.insert("docs.links_text", "Ces liens pointent vers la documentation officielle maintenue par Anthropic. ClaudeAdmin est construit sur la base de ces sp\u{00e9}cifications.");
        m.insert("docs.link_overview_title", "Vue d\u{2019}ensemble de Claude Code");
        m.insert("docs.link_overview_desc", "Premiers pas, installation et utilisation de base");
        m.insert("docs.link_memory_title", "M\u{00e9}moire & CLAUDE.md");
        m.insert("docs.link_memory_desc", "Comment Claude stocke et utilise la m\u{00e9}moire du projet");
        m.insert("docs.link_skills_title", "Skills");
        m.insert("docs.link_skills_desc", "Cr\u{00e9}er et g\u{00e9}rer des skills r\u{00e9}utilisables");
        m.insert("docs.link_settings_title", "Param\u{00e8}tres");
        m.insert("docs.link_settings_desc", "Hi\u{00e9}rarchie et options de configuration");
        m.insert("docs.link_hooks_title", "Hooks");
        m.insert("docs.link_hooks_desc", "Automatisation \u{00e9}v\u{00e9}nementielle avec des commandes shell");
        m.insert("docs.link_mcp_title", "Serveurs MCP");
        m.insert("docs.link_mcp_desc", "\u{00c9}tendre Claude avec des outils externes");
        m.insert("docs.link_bestpractices_title", "Bonnes pratiques");
        m.insert("docs.link_bestpractices_desc", "Conseils pour une utilisation efficace de Claude Code");
        m.insert("docs.link_mcp_spec_title", "Sp\u{00e9}cification MCP");
        m.insert("docs.link_mcp_spec_desc", "Le standard Model Context Protocol");

        // ── Licenses ──
        m.insert("sidebar.licenses", "Licences");
        m.insert("licenses.title", "Licences");
        m.insert("licenses.subtitle", "Licences open source et d\u{00e9}pendances");
        m.insert("licenses.own_license", "Licence ClaudeAdmin");
        m.insert("licenses.third_party", "D\u{00e9}pendances tierces");
        m.insert("licenses.col_name", "Crate");
        m.insert("licenses.col_version", "Version");
        m.insert("licenses.col_license", "Licence");
        m.insert("licenses.search_placeholder", "Rechercher des d\u{00e9}pendances...");
        m.insert("licenses.loading", "Chargement des licences");
        m.insert("licenses.count", "d\u{00e9}pendances");
        m.insert("licenses.mit_copyright", "Copyright (c) 2024-2026");
        m.insert("licenses.mit_line1", "L'autorisation est par la présente accordée, gratuitement, à toute personne obtenant une copie de ce logiciel et des fichiers de documentation associés (le \u{201c}Logiciel\u{201d}), de traiter le Logiciel sans restriction, y compris, sans limitation, les droits d'utiliser, de copier, de modifier, de fusionner, de publier, de distribuer, de sous-licencier et/ou de vendre des copies du Logiciel, et d'autoriser les personnes auxquelles le Logiciel est fourni à le faire, sous réserve des conditions suivantes :");
        m.insert("licenses.mit_line2", "L'avis de droit d'auteur ci-dessus et cet avis d'autorisation doivent être inclus dans toutes les copies ou parties substantielles du Logiciel.");
        m.insert("licenses.mit_line3", "LE LOGICIEL EST FOURNI \u{201c}TEL QUEL\u{201d}, SANS GARANTIE D'AUCUNE SORTE, EXPRESSE OU IMPLICITE, Y COMPRIS, MAIS SANS S'Y LIMITER, LES GARANTIES DE QUALITÉ MARCHANDE, D'ADÉQUATION À UN USAGE PARTICULIER ET DE NON-CONTREFAÇON. EN AUCUN CAS, LES AUTEURS OU LES TITULAIRES DU DROIT D'AUTEUR NE SERONT RESPONSABLES DE TOUTE RÉCLAMATION, DOMMAGE OU AUTRE RESPONSABILITÉ, QUE CE SOIT DANS UNE ACTION CONTRACTUELLE, DÉLICTUELLE OU AUTRE, DÉCOULANT DE, OU EN RELATION AVEC LE LOGICIEL OU L'UTILISATION OU D'AUTRES TRANSACTIONS DANS LE LOGICIEL.");
        m.insert("licenses.direct_deps", "Dépendances directes");
        m.insert("licenses.transitive_deps", "Dépendances transitives");
        m.insert("licenses.overview", "Aperçu des licences");
        m.insert("licenses.direct_count", "directes");
        m.insert("licenses.transitive_count", "dépendances transitives");

        // ── Components ──
        m.insert("component.modal.close", "Fermer");
        m.insert("component.editor.save", "Enregistrer");
        m.insert("component.editor.saved", "Enregistr\u{00e9} !");
        m.insert("component.json_editor.valid", "JSON valide");
        m.insert("component.json_editor.invalid", "JSON invalide");
        m.insert("component.frontmatter.description", "Description");
        m.insert("component.frontmatter.user_invocable", "Invocable par l\u{2019}utilisateur");
        m.insert("component.advisor.title", "Conseiller de projet");
        m.insert("component.advisor.analyze", "Analyser");
        m.insert("component.advisor.analyzing", "Analyse en cours...");
        m.insert("component.advisor.no_api_key", "Aucune ANTHROPIC_API_KEY configur\u{00e9}e");
        m.insert("component.advisor.error", "Erreur lors du chargement des recommandations");
        m.insert("component.advisor.summary", "R\u{00e9}sum\u{00e9}");
        m.insert("component.advisor.recommendations", "Recommandations");
        m.insert("component.advisor.apply", "Appliquer");
        m.insert("component.advisor.applied", "Termin\u{00e9} !");
        m.insert("component.advisor.analyze_project", "Analyser le projet");
        m.insert("component.advisor.hint", "Claude analyse votre projet et fournit des recommandations");
        m.insert("component.advisor.loading", "Claude analyse votre projet");
        m.insert("component.advisor.assessment", "\u{00c9}valuation du projet");
        m.insert("component.advisor.show_preview", "Afficher l\u{2019}aper\u{00e7}u");
        m.insert("component.advisor.category_tip", "Conseil");
        m.insert("component.frontmatter.user_invocable_label", "Invocable par l\u{2019}utilisateur (peut \u{00ea}tre appel\u{00e9} avec /commande)");
        m.insert("component.editor.saving", "Enregistrement...");

        // ── Common ──
        m.insert("common.error", "Erreur");
        m.insert("common.loading", "Chargement");
        m.insert("common.save", "Enregistrer");
        m.insert("common.delete", "Supprimer");
        m.insert("common.cancel", "Annuler");
        m.insert("common.close", "Fermer");
        m.insert("common.yes", "Oui");
        m.insert("common.no", "Non");
        m.insert("common.ok", "OK");
        m.insert("common.error_prefix", "Erreur : ");
        m.insert("common.invalid_json", "JSON invalide : ");

        m
    })
}
