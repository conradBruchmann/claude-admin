use std::collections::HashMap;
use std::sync::OnceLock;

static TRANSLATIONS: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();

pub fn translations() -> &'static HashMap<&'static str, &'static str> {
    TRANSLATIONS.get_or_init(|| {
        let mut m = HashMap::new();

        // ── App ──
        m.insert("app.title", "ClaudeAdmin");
        m.insert("app.subtitle", "Gestor de configuraci\u{f3}n");

        // ── Sidebar ──
        m.insert("sidebar.overview", "Vista general");
        m.insert("sidebar.dashboard", "Panel");
        m.insert("sidebar.analytics", "Anal\u{ed}tica");
        m.insert("sidebar.manage", "Gestionar");
        m.insert("sidebar.projects", "Proyectos");
        m.insert("sidebar.global_skills", "Skills globales");
        m.insert("sidebar.skill_browser", "Explorador de Skills");
        m.insert("sidebar.global_rules", "Reglas globales");
        m.insert("sidebar.plans", "Planes");
        m.insert("sidebar.mcp_servers", "Servidores MCP");
        m.insert("sidebar.mcp_browser", "Explorador MCP");
        m.insert("sidebar.security", "Seguridad");
        m.insert("sidebar.permissions", "Permisos");
        m.insert("sidebar.config_health", "Estado de configuraci\u{f3}n");
        m.insert("sidebar.system", "Sistema");
        m.insert("sidebar.settings", "Configuraci\u{f3}n");
        m.insert("sidebar.sessions", "Sesiones");
        m.insert("sidebar.github", "GitHub");
        m.insert("sidebar.learn", "Aprender");
        m.insert("sidebar.docs", "Documentaci\u{f3}n");
        m.insert("sidebar.help", "Info del sistema");

        // ── Dashboard ──
        m.insert("dashboard.title", "Panel");
        m.insert("dashboard.subtitle", "Resumen de tu configuraci\u{f3}n de Claude Code");
        m.insert("dashboard.projects", "Proyectos");
        m.insert("dashboard.global_skills", "Skills globales");
        m.insert("dashboard.global_rules", "Reglas globales");
        m.insert("dashboard.mcp_servers", "Servidores MCP");
        m.insert("dashboard.plans", "Planes");
        m.insert("dashboard.config_health", "Estado de configuraci\u{f3}n");
        m.insert("dashboard.recent_projects", "Proyectos recientes");
        m.insert("dashboard.loading", "Cargando");
        m.insert("dashboard.error_loading", "Error al cargar el panel");
        m.insert("dashboard.col_name", "Nombre");
        m.insert("dashboard.col_claude_md", "CLAUDE.md");
        m.insert("dashboard.col_skills", "Skills");
        m.insert("dashboard.col_rules", "Reglas");
        m.insert("dashboard.col_memory", "Memoria");
        m.insert("dashboard.yes", "S\u{ed}");

        // ── MCP ──
        m.insert("mcp.title", "Servidores MCP");
        m.insert("mcp.subtitle", "Gestionar servidores del Protocolo de Contexto de Modelo para Claude Code");
        m.insert("mcp.tab_servers", "Servidores");
        m.insert("mcp.tab_health", "Verificaci\u{f3}n de estado");
        m.insert("mcp.tab_add", "Nuevo servidor");
        m.insert("mcp.loading", "Cargando servidores MCP");
        m.insert("mcp.no_servers", "No hay servidores MCP configurados");
        m.insert("mcp.no_servers_hint", "A\u{f1}ade servidores usando la pesta\u{f1}a \u{2018}Nuevo servidor\u{2019} o el Explorador MCP.");
        m.insert("mcp.select_server", "Selecciona un servidor de la lista para ver y editar su configuraci\u{f3}n.");
        m.insert("mcp.no_servers_configured", "No hay servidores configurados.");
        m.insert("mcp.check_health", "Verificar estado");
        m.insert("mcp.save", "Guardar");
        m.insert("mcp.delete", "Eliminar");
        m.insert("mcp.saved", "\u{a1}Guardado!");
        m.insert("mcp.deleted", "\u{a1}Eliminado!");
        m.insert("mcp.read_only", "Solo lectura");
        m.insert("mcp.read_only_hint", "Este servidor es gestionado externamente y no se puede editar aqu\u{ed}.");
        m.insert("mcp.health.title", "Estado de servidores MCP");
        m.insert("mcp.health.check_all", "Verificar todos los servidores");
        m.insert("mcp.health.checking", "Verificando...");
        m.insert("mcp.health.description", "Inicia cada proceso de servidor MCP, env\u{ed}a JSON-RPC initialize + tools/list y muestra los resultados. Tiempo l\u{ed}mite: 10 segundos por servidor.");
        m.insert("mcp.health.col_name", "Nombre");
        m.insert("mcp.health.col_source", "Origen");
        m.insert("mcp.health.col_status", "Estado");
        m.insert("mcp.health.col_server_info", "Info del servidor");
        m.insert("mcp.health.col_tools", "Herramientas");
        m.insert("mcp.health.col_duration", "Duraci\u{f3}n");
        m.insert("mcp.health.running", "En ejecuci\u{f3}n");
        m.insert("mcp.health.error", "Error");
        m.insert("mcp.health.timeout", "Tiempo agotado");
        m.insert("mcp.health.unknown", "Desconocido");
        m.insert("mcp.add.title", "A\u{f1}adir servidor MCP");
        m.insert("mcp.add.description", "A\u{f1}ade un nuevo servidor MCP a tu configuraci\u{f3}n global ~/.claude.json.");
        m.insert("mcp.add.name_label", "Nombre del servidor");
        m.insert("mcp.add.name_placeholder", "ej. mi-servidor");
        m.insert("mcp.add.config_label", "Configuraci\u{f3}n del servidor (JSON)");
        m.insert("mcp.add.submit", "A\u{f1}adir servidor");
        m.insert("mcp.add.name_required", "Por favor, introduce un nombre de servidor");
        m.insert("mcp.source.claude_code", "Claude Code");
        m.insert("mcp.source.claude_desktop", "Claude Desktop");

        // ── MCP Browser ──
        m.insert("mcp_browser.title", "Explorador MCP");
        m.insert("mcp_browser.subtitle", "Descubre e instala servidores MCP para Claude Code");
        m.insert("mcp_browser.search_placeholder", "Buscar servidores MCP...");
        m.insert("mcp_browser.loading", "Cargando cat\u{e1}logo MCP");
        m.insert("mcp_browser.no_results", "No se encontraron servidores MCP");
        m.insert("mcp_browser.installed", "Instalado");
        m.insert("mcp_browser.install", "Instalar");
        m.insert("mcp_browser.needs_api_key", "Necesita clave API");
        m.insert("mcp_browser.install_success", "\u{a1}instalado correctamente!");
        m.insert("mcp_browser.install_failed", "Error al instalar");

        // ── Projects ──
        m.insert("projects.title", "Proyectos");
        m.insert("projects.subtitle", "Todos los proyectos registrados en ~/.claude.json");
        m.insert("projects.loading", "Cargando");
        m.insert("projects.error_loading", "Error al cargar proyectos: ");
        m.insert("projects.col_name", "Nombre");
        m.insert("projects.col_path", "Ruta");
        m.insert("projects.col_claude_md", "CLAUDE.md");
        m.insert("projects.col_skills", "Skills");
        m.insert("projects.col_rules", "Reglas");
        m.insert("projects.col_memory", "Memoria");
        m.insert("projects.yes", "S\u{ed}");

        // ── Project Detail ──
        m.insert("project_detail.loading", "Cargando detalles del proyecto");
        m.insert("project_detail.error_loading", "Error al cargar el proyecto");
        m.insert("project_detail.tab_advisor", "Asesor");
        m.insert("project_detail.tab_claude_md", "CLAUDE.md");
        m.insert("project_detail.tab_skills", "Skills");
        m.insert("project_detail.tab_rules", "Reglas");
        m.insert("project_detail.tab_memory", "Memoria");
        m.insert("project_detail.tab_permissions", "Permisos");
        m.insert("project_detail.tab_health", "Estado");
        m.insert("project_detail.no_claude_md", "No se encontr\u{f3} CLAUDE.md");
        m.insert("project_detail.no_claude_md_hint", "Crea un CLAUDE.md en el directorio de tu proyecto para dar instrucciones a Claude Code.");
        m.insert("project_detail.no_skills", "No hay skills para este proyecto");
        m.insert("project_detail.no_rules", "No hay reglas para este proyecto");
        m.insert("project_detail.no_memory", "No hay memoria para este proyecto");
        m.insert("project_detail.save", "Guardar");
        m.insert("project_detail.saved", "\u{a1}Guardado!");
        m.insert("project_detail.skill_scope", "\u{c1}mbito");
        m.insert("project_detail.permissions_loading", "Cargando permisos...");
        m.insert("project_detail.permissions_error", "Error al cargar permisos");
        m.insert("project_detail.permissions_entries", "Entradas");
        m.insert("project_detail.permissions_col_tool", "Herramienta");
        m.insert("project_detail.permissions_col_command", "Comando");
        m.insert("project_detail.permissions_no_entries", "Sin entradas de permisos");
        m.insert("project_detail.health_loading", "Calculando estado...");
        m.insert("project_detail.health_error", "Error al cargar datos de estado");
        m.insert("project_detail.health_score", "Puntuaci\u{f3}n de estado");
        m.insert("project_detail.health_claude_md", "CLAUDE.md presente");
        m.insert("project_detail.health_memory", "Memoria presente");
        m.insert("project_detail.health_permissions", "Permisos");
        m.insert("project_detail.health_security_issues", "Problemas de seguridad");
        m.insert("project_detail.health_duplicated_rules", "Reglas duplicadas");
        m.insert("project_detail.health_no_security_issues", "No se encontraron problemas de seguridad");
        m.insert("project_detail.health_col_text", "Texto");
        m.insert("project_detail.health_col_found_in", "Encontrado en");
        m.insert("project_detail.health_col_also_in", "Tambi\u{e9}n en");
        m.insert("project_detail.health_permission_entries", "Entradas de permisos");
        m.insert("project_detail.permissions_col_index", "#");
        m.insert("project_detail.permissions_col_status", "Estado");
        m.insert("project_detail.permissions_fragment", "Fragmento");
        m.insert("project_detail.permissions_ok", "OK");
        m.insert("project_detail.permissions_security_warnings", "advertencia(s) de seguridad");
        m.insert("project_detail.permissions_manage", "Gestionar permisos");
        m.insert("project_detail.advisor_analyze", "Analizar proyecto");
        m.insert("project_detail.advisor_analyzing", "Analizando...");
        m.insert("project_detail.advisor_description", "Claude analiza tu proyecto y proporciona recomendaciones");
        m.insert("project_detail.advisor_loading", "Claude est\u{e1} analizando tu proyecto");
        m.insert("project_detail.advisor_summary", "Evaluaci\u{f3}n del proyecto");
        m.insert("project_detail.advisor_done", "\u{a1}Listo!");
        m.insert("project_detail.advisor_preview", "Mostrar vista previa");
        m.insert("project_detail.advisor_category_tip", "Consejo");
        m.insert("project_detail.skills_col_name", "Nombre");
        m.insert("project_detail.skills_col_description", "Descripci\u{f3}n");
        m.insert("project_detail.skills_col_invocable", "Invocable");
        m.insert("project_detail.rules_col_name", "Nombre");
        m.insert("project_detail.rules_col_path", "Ruta");
        m.insert("project_detail.memory_col_file", "Archivo");
        m.insert("project_detail.memory_col_size", "Tama\u{f1}o");
        m.insert("project_detail.bytes", "bytes");
        m.insert("project_detail.unknown_tab", "Pesta\u{f1}a desconocida");

        // ── Global Skills ──
        m.insert("global_skills.title", "Skills globales");
        m.insert("global_skills.subtitle", "Gestionar skills en ~/.claude/skills/");
        m.insert("global_skills.loading", "Cargando skills");
        m.insert("global_skills.no_skills", "No se encontraron skills globales");
        m.insert("global_skills.no_skills_hint", "Crea skills en ~/.claude/skills/ o usa el Explorador de Skills.");
        m.insert("global_skills.select_skill", "Selecciona un skill de la lista.");
        m.insert("global_skills.col_skill", "Skill");
        m.insert("global_skills.col_invocable", "Invocable");
        m.insert("global_skills.invocable", "Invocable");
        m.insert("global_skills.not_invocable", "No invocable");
        m.insert("global_skills.editing", "Editando:");
        m.insert("global_skills.save", "Guardar");
        m.insert("global_skills.saved", "\u{a1}Guardado!");
        m.insert("global_skills.delete", "Eliminar");
        m.insert("global_skills.deleted", "\u{a1}Eliminado!");

        // ── Global Rules ──
        m.insert("global_rules.title", "Reglas globales");
        m.insert("global_rules.subtitle", "Gestionar reglas en ~/.claude/rules/");
        m.insert("global_rules.loading", "Cargando reglas");
        m.insert("global_rules.no_rules", "No se encontraron reglas globales");
        m.insert("global_rules.no_rules_hint", "Crea archivos .md en ~/.claude/rules/");
        m.insert("global_rules.select_rule", "Selecciona una regla de la lista.");
        m.insert("global_rules.col_rule", "Regla");
        m.insert("global_rules.editing", "Editando:");
        m.insert("global_rules.save", "Guardar");
        m.insert("global_rules.saved", "\u{a1}Guardado!");
        m.insert("global_rules.delete", "Eliminar");
        m.insert("global_rules.deleted", "\u{a1}Eliminado!");

        // ── Plans ──
        m.insert("plans.title", "Planes");
        m.insert("plans.subtitle", "Gestionar archivos de planes en ~/.claude/plans/");
        m.insert("plans.loading", "Cargando planes");
        m.insert("plans.no_plans", "No se encontraron planes");
        m.insert("plans.no_plans_hint", "Los planes son creados por Claude Code durante la planificaci\u{f3}n.");
        m.insert("plans.select_plan", "Selecciona un plan de la lista.");
        m.insert("plans.col_plan", "Plan");
        m.insert("plans.col_modified", "Modificado");
        m.insert("plans.modified", "Modificado");
        m.insert("plans.plan_label", "Plan:");
        m.insert("plans.save", "Guardar");
        m.insert("plans.saved", "\u{a1}Guardado!");
        m.insert("plans.delete", "Eliminar");
        m.insert("plans.deleted", "\u{a1}Eliminado!");

        // ── Settings ──
        m.insert("settings.title", "Configuraci\u{f3}n");
        m.insert("settings.subtitle", "Gestionar configuraci\u{f3}n y hooks de Claude Code");
        m.insert("settings.tab_overview", "Vista general");
        m.insert("settings.tab_hooks", "Plantillas de Hooks");
        m.insert("settings.tab_storage", "Almacenamiento");
        m.insert("settings.loading", "Cargando configuraci\u{f3}n");
        m.insert("settings.hooks_title", "Hooks");
        m.insert("settings.no_hooks", "No hay hooks configurados");
        m.insert("settings.pre_tool_use", "PreToolUse");
        m.insert("settings.post_tool_use", "PostToolUse");
        m.insert("settings.matcher", "Filtro");
        m.insert("settings.command", "Comando");
        m.insert("settings.hook_templates_title", "Plantillas de Hooks");
        m.insert("settings.hook_templates_desc", "Configuraciones de hooks predefinidas para a\u{f1}adir.");
        m.insert("settings.hook_templates_loading", "Cargando plantillas");
        m.insert("settings.add_hook", "A\u{f1}adir");
        m.insert("settings.storage_title", "Uso de almacenamiento");
        m.insert("settings.storage_loading", "Calculando almacenamiento");
        m.insert("settings.storage_total", "Total");
        m.insert("settings.storage_dir", "Directorio");
        m.insert("settings.storage_size", "Tama\u{f1}o");

        // ── Permissions ──
        m.insert("permissions.title", "Permisos");
        m.insert("permissions.subtitle", "Revisar y gestionar permisos de proyectos");
        m.insert("permissions.loading", "Cargando permisos");
        m.insert("permissions.no_permissions", "No se encontraron permisos");
        m.insert("permissions.col_project", "Proyecto");
        m.insert("permissions.col_entries", "Entradas");
        m.insert("permissions.col_issues", "Problemas");
        m.insert("permissions.col_fragmented", "Fragmentado");
        m.insert("permissions.detail_title", "Permisos");
        m.insert("permissions.detail_loading", "Cargando permisos");
        m.insert("permissions.detail_col_tool", "Herramienta");
        m.insert("permissions.detail_col_command", "Comando");
        m.insert("permissions.detail_col_status", "Estado");
        m.insert("permissions.detail_fragmented", "Fragmentado");
        m.insert("permissions.detail_security_issue", "Problema de seguridad");
        m.insert("permissions.detail_delete_selected", "Eliminar seleccionados");
        m.insert("permissions.detail_deleted", "\u{a1}Eliminado!");
        m.insert("permissions.detail_warnings_title", "Advertencias de seguridad");
        m.insert("permissions.health_title", "Estado de configuraci\u{f3}n");
        m.insert("permissions.health_subtitle", "Estado de todos los proyectos");
        m.insert("permissions.health_loading", "Calculando estado");
        m.insert("permissions.health_col_project", "Proyecto");
        m.insert("permissions.health_col_score", "Puntuaci\u{f3}n");
        m.insert("permissions.health_col_issues", "Problemas");
        m.insert("permissions.health_avg", "Promedio");
        m.insert("permissions.subtitle_manage", "Gestionar listas de permisos en todos los proyectos");
        m.insert("permissions.col_actions", "Acciones");
        m.insert("permissions.col_security_issues", "Problemas de seguridad");
        m.insert("permissions.details", "Detalles");
        m.insert("permissions.detail_subtitle", "Revisar y depurar entradas de permisos");
        m.insert("permissions.detail_deleting", "Eliminando...");
        m.insert("permissions.detail_deleted_reloading", "\u{a1}Eliminado! Recargando...");
        m.insert("permissions.detail_delete_count", "Eliminar seleccionados");
        m.insert("permissions.detail_col_index", "#");
        m.insert("permissions.detail_fragment", "Fragmento");
        m.insert("permissions.detail_ok", "OK");
        m.insert("permissions.detail_warnings_count", "Advertencias de seguridad");
        m.insert("permissions.detail_entry", "entrada");
        m.insert("permissions.health_subtitle_scores", "Puntuaciones de estado de configuraci\u{f3}n en todos los proyectos");
        m.insert("permissions.health_avg_score", "Puntuaci\u{f3}n promedio de estado");
        m.insert("permissions.health_projects_analyzed", "Proyectos analizados");
        m.insert("permissions.health_no_issues", "Sin problemas");

        // ── Analytics ──
        m.insert("analytics.title", "Anal\u{ed}tica");
        m.insert("analytics.subtitle", "Estad\u{ed}sticas de uso de Claude Code");
        m.insert("analytics.loading", "Cargando anal\u{ed}tica");
        m.insert("analytics.error_loading", "Error al cargar anal\u{ed}tica");
        m.insert("analytics.total_sessions", "Sesiones totales");
        m.insert("analytics.total_messages", "Mensajes totales");
        m.insert("analytics.git_commits", "Commits de Git");
        m.insert("analytics.lines_added", "L\u{ed}neas a\u{f1}adidas");
        m.insert("analytics.lines_removed", "L\u{ed}neas eliminadas");
        m.insert("analytics.since", "desde");
        m.insert("analytics.activity_heatmap", "Mapa de actividad");
        m.insert("analytics.messages", "Mensajes");
        m.insert("analytics.sessions", "Sesiones");
        m.insert("analytics.tool_calls", "Llamadas a herramientas");
        m.insert("analytics.hourly_distribution", "Distribuci\u{f3}n por hora");
        m.insert("analytics.model_usage", "Uso de modelos");
        m.insert("analytics.col_model", "Modelo");
        m.insert("analytics.col_input_tokens", "Tokens de entrada");
        m.insert("analytics.col_output_tokens", "Tokens de salida");
        m.insert("analytics.col_cache_tokens", "Tokens de cach\u{e9}");
        m.insert("analytics.tool_ranking", "Ranking de herramientas");
        m.insert("analytics.col_cache_read", "Lectura de cach\u{e9}");
        m.insert("analytics.tool_usage_top10", "Uso de herramientas (Top 10)");
        m.insert("analytics.languages", "Lenguajes");
        m.insert("analytics.session_outcomes", "Resultados de sesiones");
        m.insert("analytics.outcomes", "Resultados");

        // ── Sessions ──
        m.insert("sessions.title", "Sesiones");
        m.insert("sessions.subtitle", "Explorar historial de sesiones de Claude Code");
        m.insert("sessions.loading", "Cargando sesiones");
        m.insert("sessions.search_placeholder", "Buscar sesiones...");
        m.insert("sessions.no_sessions", "No se encontraron sesiones");
        m.insert("sessions.col_project", "Proyecto");
        m.insert("sessions.col_date", "Fecha");
        m.insert("sessions.col_duration", "Duraci\u{f3}n");
        m.insert("sessions.col_messages", "Mensajes");
        m.insert("sessions.col_summary", "Resumen");
        m.insert("sessions.col_outcome", "Resultado");
        m.insert("sessions.minutes", "min");
        m.insert("sessions.load_more", "Cargar m\u{e1}s");
        m.insert("sessions.detail_title", "Detalles de la sesi\u{f3}n");
        m.insert("sessions.detail_loading", "Cargando sesi\u{f3}n");
        m.insert("sessions.detail_project", "Proyecto");
        m.insert("sessions.detail_start", "Inicio");
        m.insert("sessions.detail_duration", "Duraci\u{f3}n");
        m.insert("sessions.detail_messages", "Mensajes");
        m.insert("sessions.detail_tools", "Llamadas a herramientas");
        m.insert("sessions.detail_tokens", "Tokens");
        m.insert("sessions.detail_git", "Git");
        m.insert("sessions.detail_first_prompt", "Primer prompt");
        m.insert("sessions.detail_summary", "Resumen");
        m.insert("sessions.back", "Volver");
        m.insert("sessions.searching", "Buscando...");
        m.insert("sessions.search", "Buscar");
        m.insert("sessions.clear", "Limpiar");
        m.insert("sessions.search_results", "Resultados de b\u{fa}squeda");
        m.insert("sessions.no_results", "No se encontraron resultados");
        m.insert("sessions.col_prompt", "Prompt");
        m.insert("sessions.session_prefix", "Sesi\u{f3}n: ");
        m.insert("sessions.detail_start_time", "Hora de inicio");
        m.insert("sessions.user_messages", " usuario / ");
        m.insert("sessions.assistant_messages", " asistente");
        m.insert("sessions.tokens_in", " entrada / ");
        m.insert("sessions.tokens_out", " salida");
        m.insert("sessions.commits_label", " commits, +");
        m.insert("sessions.lines_minus", " / -");
        m.insert("sessions.tools_used", "Herramientas usadas");
        m.insert("sessions.outcome_prefix", "Resultado: ");
        m.insert("sessions.showing", "Mostrando");
        m.insert("sessions.of", "de");
        m.insert("sessions.previous", "Anterior");
        m.insert("sessions.next", "Siguiente");

        // ── GitHub ──
        m.insert("github.title", "GitHub");
        m.insert("github.subtitle", "Estado de integraci\u{f3}n con GitHub");
        m.insert("github.loading", "Cargando datos de GitHub");
        m.insert("github.auth_status", "Estado de autenticaci\u{f3}n");
        m.insert("github.username", "Usuario");
        m.insert("github.linked_repos", "Repositorios vinculados");
        m.insert("github.no_repos", "Sin repositorios vinculados");
        m.insert("github.col_repo", "Repositorio");
        m.insert("github.col_recent_commits", "Commits recientes");
        m.insert("github.col_open_prs", "PRs abiertas");

        // ── Help / System Info ──
        m.insert("help.title", "Info del sistema");
        m.insert("help.subtitle", "Informaci\u{f3}n del sistema de Claude Code");
        m.insert("help.loading", "Cargando informaci\u{f3}n del sistema");
        m.insert("help.account", "Cuenta");
        m.insert("help.account_name", "Nombre");
        m.insert("help.account_email", "Correo electr\u{f3}nico");
        m.insert("help.subscription", "Suscripci\u{f3}n");
        m.insert("help.claude_version", "Versi\u{f3}n de Claude Code");
        m.insert("help.gh_cli", "GitHub CLI");
        m.insert("help.skill_usage", "Uso de Skills");
        m.insert("help.no_skill_usage", "No hay registro de uso de skills");
        m.insert("help.col_skill", "Skill");
        m.insert("help.col_count", "Cantidad");
        m.insert("help.what_is_title", "\u{bf}Qu\u{e9} es ClaudeAdmin?");
        m.insert("help.what_is_desc", "ClaudeAdmin es la consola de administraci\u{f3}n visual para Claude Code. Proporciona una interfaz web para gestionar todos los aspectos de tu configuraci\u{f3}n de Claude Code: Proyectos, Skills, Reglas, Memoria, Configuraci\u{f3}n, Hooks, Servidores MCP y Planes.");
        m.insert("help.system_status", "Estado del sistema");
        m.insert("help.not_set", "No configurado");
        m.insert("help.unknown", "Desconocido");
        m.insert("help.not_found", "No encontrado");
        m.insert("help.not_installed", "No instalado");
        m.insert("help.concepts_title", "Conceptos de Claude Code");
        m.insert("help.concept_skills", "Prompts reutilizables con metadatos YAML. Almacenados como archivos SKILL.md en ~/.claude/skills/ (global) o .claude/skills/ (proyecto).");
        m.insert("help.concept_rules", "Restricciones y directrices que moldean el comportamiento de Claude. Almacenadas como archivos .md en ~/.claude/rules/ o a nivel de proyecto.");
        m.insert("help.concept_memory", "Notas persistentes por proyecto. MEMORY.md se carga autom\u{e1}ticamente en los prompts del sistema. Almacena patrones, preferencias y aprendizajes.");
        m.insert("help.concept_hooks", "Comandos de shell activados por eventos (PreToolUse, PostToolUse, Stop). Configurados en settings.json para auto-formateo, linting, etc.");
        m.insert("help.concept_mcp", "Los servidores del Protocolo de Contexto de Modelo extienden a Claude con herramientas externas. Configurados en ~/.claude.json con command, args y env.");
        m.insert("help.concept_claudemd", "Archivo de instrucciones a nivel de proyecto. Se carga autom\u{e1}ticamente como contexto. Contiene convenciones del proyecto, info del stack y gu\u{ed}as de codificaci\u{f3}n.");
        m.insert("help.disclaimer", "ClaudeAdmin es un proyecto comunitario independiente. No est\u{e1} afiliado, respaldado ni aprobado por Anthropic. Claude y Claude Code son marcas comerciales de Anthropic.");

        m.insert("github.subtitle_detail", "Integraci\u{f3}n con GitHub CLI y repositorios vinculados");
        m.insert("github.linked_repositories", "Repositorios vinculados");
        m.insert("github.no_linked_repos", "No hay repositorios de GitHub vinculados en ~/.claude.json");
        m.insert("github.col_name", "Nombre");
        m.insert("github.col_path", "Ruta");

        // ── Skill Browser ──
        m.insert("skill_browser.title", "Explorador de Skills");
        m.insert("skill_browser.subtitle", "Descubre e instala skills oficiales y de la comunidad");
        m.insert("skill_browser.loading", "Cargando skills");
        m.insert("skill_browser.search_placeholder", "Buscar skills...");
        m.insert("skill_browser.no_results", "No se encontraron skills");
        m.insert("skill_browser.installed", "Instalado");
        m.insert("skill_browser.install", "Instalar");
        m.insert("skill_browser.official", "Oficial");
        m.insert("skill_browser.community", "Comunidad");
        m.insert("skill_browser.tab_official", "Oficiales (Anthropic)");
        m.insert("skill_browser.tab_community", "Comunidad");
        m.insert("skill_browser.install_success", "\u{a1}instalado correctamente!");
        m.insert("skill_browser.install_failed", "Error al instalar:");

        // ── Docs ──
        m.insert("docs.title", "Documentaci\u{f3}n");
        m.insert("docs.subtitle", "Todo lo que necesitas saber sobre la configuraci\u{f3}n de Claude Code");
        m.insert("docs.loading", "Cargando documentaci\u{f3}n");

        // ── Docs: Table of Contents ──
        m.insert("docs.toc_contents", "Contenidos");
        m.insert("docs.toc_why_claudeadmin", "\u{bf}Por qu\u{e9} ClaudeAdmin?");
        m.insert("docs.toc_capabilities", "Qu\u{e9} puede y no puede hacer");
        m.insert("docs.toc_group", "Conceptos");
        m.insert("docs.toc_claude_md", "CLAUDE.md");
        m.insert("docs.toc_rules", "Reglas");
        m.insert("docs.toc_skills", "Skills");
        m.insert("docs.toc_memory", "Memoria");
        m.insert("docs.toc_settings", "Configuraci\u{f3}n y Hooks");
        m.insert("docs.toc_mcp", "Servidores MCP");
        m.insert("docs.toc_plans", "Planes");
        m.insert("docs.toc_scopes", "Global vs. Proyecto");
        m.insert("docs.toc_tips", "Consejos y buenas pr\u{e1}cticas");
        m.insert("docs.toc_links", "Documentaci\u{f3}n oficial");

        // ── Docs: Shared labels ──
        m.insert("docs.tips_heading", "Consejos y trucos");
        m.insert("docs.scope_global", "Global");
        m.insert("docs.scope_project", "Proyecto");
        m.insert("docs.scope_user", "Usuario");
        m.insert("docs.scope_parent", "Padre");
        m.insert("docs.scope_managed", "Gestionado");
        m.insert("docs.scope_local", "Local");

        // ── Docs: Overview ──
        m.insert("docs.overview_heading", "\u{bf}Por qu\u{e9} ClaudeAdmin?");
        m.insert("docs.overview_callout", " es la consola de administraci\u{f3}n central para toda tu configuraci\u{f3}n de Claude Code. Reemplaza la edici\u{f3}n manual de archivos en docenas de directorios ocultos con una \u{fa}nica interfaz visual.");
        m.insert("docs.overview_text1", "Claude Code almacena su configuraci\u{f3}n en una jerarqu\u{ed}a compleja de archivos y directorios: archivos CLAUDE.md en la ra\u{ed}z de proyectos, reglas y skills dispersos en subdirectorios de ~/.claude/, archivos de memoria identificados por rutas de proyecto codificadas, configuraciones en m\u{fa}ltiples archivos JSON y configuraciones de servidores MCP en ~/.claude.json. A medida que tus proyectos crecen, gestionar todo esto manualmente se vuelve propenso a errores y consume mucho tiempo.");
        m.insert("docs.overview_text2", "ClaudeAdmin te ofrece:");
        m.insert("docs.overview_li_visibility_label", "Visibilidad");
        m.insert("docs.overview_li_visibility", " \u{2013} Ve todos tus proyectos, skills, reglas y memoria en un solo lugar");
        m.insert("docs.overview_li_editing_label", "Edici\u{f3}n");
        m.insert("docs.overview_li_editing", " \u{2013} Edita CLAUDE.md, reglas, skills y memoria con un editor apropiado");
        m.insert("docs.overview_li_health_label", "Verificaciones de estado");
        m.insert("docs.overview_li_health", " \u{2013} Detecta problemas de seguridad en permisos, reglas duplicadas y configuraciones faltantes");
        m.insert("docs.overview_li_analytics_label", "Anal\u{ed}tica");
        m.insert("docs.overview_li_analytics", " \u{2013} Comprende c\u{f3}mo usas Claude Code: sesiones, tokens, herramientas, costos");
        m.insert("docs.overview_li_advisor_label", "Asesor");
        m.insert("docs.overview_li_advisor", " \u{2013} Recomendaciones impulsadas por IA para mejorar la configuraci\u{f3}n de tu proyecto");

        // ── Docs: Capabilities ──
        m.insert("docs.cap_heading", "Qu\u{e9} puede y no puede hacer ClaudeAdmin");
        m.insert("docs.cap_can_heading", "Qu\u{e9} puede hacer");
        m.insert("docs.cap_can_1", "Explorar y gestionar todos los proyectos registrados en ~/.claude.json");
        m.insert("docs.cap_can_2", "Ver y editar archivos CLAUDE.md de cualquier proyecto");
        m.insert("docs.cap_can_3", "Crear, editar y eliminar skills globales y de proyecto");
        m.insert("docs.cap_can_4", "Crear, editar y eliminar reglas globales y de proyecto");
        m.insert("docs.cap_can_5", "Ver y editar archivos de memoria del proyecto (MEMORY.md y temas)");
        m.insert("docs.cap_can_6", "Inspeccionar la jerarqu\u{ed}a de configuraci\u{f3}n (global \u{2192} proyecto \u{2192} local)");
        m.insert("docs.cap_can_7", "Auditar entradas de permisos y detectar problemas de seguridad");
        m.insert("docs.cap_can_8", "Ver configuraciones de servidores MCP");
        m.insert("docs.cap_can_9", "Analizar historial de sesiones, uso de tokens y costos");
        m.insert("docs.cap_can_10", "Ejecutar an\u{e1}lisis de proyecto impulsado por IA con recomendaciones accionables");
        m.insert("docs.cap_can_11", "Explorar e instalar skills de repositorios comunitarios");
        m.insert("docs.cap_can_12", "Todas las escrituras crean copias de seguridad autom\u{e1}ticas en ~/.claude/backups/");
        m.insert("docs.cap_cannot_heading", "Qu\u{e9} no puede hacer");
        m.insert("docs.cap_cannot_1", "Ejecutar sesiones de Claude Code \u{2013} gestiona configuraci\u{f3}n, no ejecuci\u{f3}n");
        m.insert("docs.cap_cannot_2", "Modificar pol\u{ed}ticas gestionadas (configuraci\u{f3}n a nivel empresarial/organizacional)");
        m.insert("docs.cap_cannot_3", "Acceder a entornos remotos o sesiones SSH");
        m.insert("docs.cap_cannot_4", "Reemplazar la CLI de Claude Code para trabajo de codificaci\u{f3}n real");
        m.insert("docs.cap_cannot_5", "Editar directamente los servidores MCP de .claude.json (solo lectura por seguridad)");
        m.insert("docs.cap_cannot_6", "Gestionar claves de API o credenciales de autenticaci\u{f3}n");
        m.insert("docs.cap_cannot_callout", "ClaudeAdmin es un gestor de configuraci\u{f3}n, no un reemplazo de Claude Code. Pi\u{e9}nsalo como una herramienta de administraci\u{f3}n de base de datos: te ayuda a inspeccionar, configurar y mantener \u{2013} pero el trabajo real se hace en Claude Code.");

        // ── Docs: CLAUDE.md ──
        m.insert("docs.claudemd_callout", "La constituci\u{f3}n del proyecto. CLAUDE.md es el archivo de configuraci\u{f3}n m\u{e1}s importante \u{2013} se carga autom\u{e1}ticamente en cada sesi\u{f3}n de Claude Code como contexto persistente.");
        m.insert("docs.claudemd_how_heading", "C\u{f3}mo funciona");
        m.insert("docs.claudemd_how_text", "Cuando Claude Code inicia una sesi\u{f3}n, busca archivos CLAUDE.md recursivamente desde tu directorio de trabajo actual hasta la ra\u{ed}z del sistema de archivos. Todos los archivos encontrados se cargan y concatenan, con los archivos m\u{e1}s cercanos teniendo prioridad. Esto significa que puedes tener un CLAUDE.md a nivel de monorepo con convenciones compartidas y archivos CLAUDE.md a nivel de paquete con anulaciones espec\u{ed}ficas.");
        m.insert("docs.claudemd_locations_heading", "Ubicaciones");
        m.insert("docs.claudemd_loc_project_or", " o ");
        m.insert("docs.claudemd_loc_parent", "Ra\u{ed}z del monorepo, cargado para todos los subpaquetes");
        m.insert("docs.claudemd_loc_user", "Valores predeterminados personales en todos los proyectos");
        m.insert("docs.claudemd_whatto_heading", "Qu\u{e9} incluir");
        m.insert("docs.claudemd_whatto_context_label", "Contexto del proyecto");
        m.insert("docs.claudemd_whatto_context", " \u{2013} Stack tecnol\u{f3}gico, decisiones de arquitectura, dependencias clave");
        m.insert("docs.claudemd_whatto_standards_label", "Est\u{e1}ndares de codificaci\u{f3}n");
        m.insert("docs.claudemd_whatto_standards", " \u{2013} Convenciones de nombres, reglas de formato, patrones de manejo de errores");
        m.insert("docs.claudemd_whatto_workflows_label", "Flujos de trabajo");
        m.insert("docs.claudemd_whatto_workflows", " \u{2013} C\u{f3}mo compilar, probar, desplegar; nombres de ramas; convenciones de PR");
        m.insert("docs.claudemd_whatto_dodont_label", "Reglas de hacer/no hacer");
        m.insert("docs.claudemd_whatto_dodont", " \u{2013} Restricciones expl\u{ed}citas (ej. \u{201c}nunca usar any en TypeScript\u{201d})");
        m.insert("docs.claudemd_whatto_team_label", "Acuerdos de equipo");
        m.insert("docs.claudemd_whatto_team", " \u{2013} Proceso de revisi\u{f3}n, formato de mensajes de commit, l\u{ed}mites de m\u{f3}dulos");
        m.insert("docs.claudemd_tip1", "Mant\u{e9}nlo por debajo de 500 l\u{ed}neas. Claude carga el archivo completo en el contexto \u{2013} archivos CLAUDE.md inflados desperdician tokens y diluyen instrucciones importantes.");
        m.insert("docs.claudemd_tip2", "Usa encabezados de secci\u{f3}n claros (## Arquitectura, ## Convenciones). Claude analiza la estructura para encontrar secciones relevantes.");
        m.insert("docs.claudemd_tip3", "Pon las reglas m\u{e1}s cr\u{ed}ticas al principio. En archivos largos, el contenido al inicio recibe m\u{e1}s atenci\u{f3}n.");
        m.insert("docs.claudemd_tip4", "Usa CLAUDE.local.md para preferencias personales que no deben subirse a git.");
        m.insert("docs.claudemd_ext_link", "Docs de Anthropic: CLAUDE.md \u{2192}");

        // ── Docs: Rules ──
        m.insert("docs.rules_heading", "Reglas");
        m.insert("docs.rules_callout", "Restricciones modulares y tem\u{e1}ticas que moldean el comportamiento de Claude. A diferencia de CLAUDE.md que es un solo archivo grande, las reglas son archivos .md separados \u{2013} cada uno enfocado en un tema espec\u{ed}fico.");
        m.insert("docs.rules_how_heading", "C\u{f3}mo funciona");
        m.insert("docs.rules_how_text", "Las reglas se cargan autom\u{e1}ticamente al iniciar la sesi\u{f3}n. Las reglas globales (tus preferencias personales) se cargan primero, luego las reglas del proyecto las complementan. Esto te permite definir tu estilo de codificaci\u{f3}n globalmente mientras los proyectos a\u{f1}aden restricciones espec\u{ed}ficas del dominio.");
        m.insert("docs.rules_locations_heading", "Ubicaciones");
        m.insert("docs.rules_loc_global", "Tus reglas personales, aplicadas a todos los proyectos");
        m.insert("docs.rules_loc_project", "Espec\u{ed}ficas del proyecto, subidas a git para compartir con el equipo");
        m.insert("docs.rules_examples_heading", "Ejemplos");
        m.insert("docs.rules_example_frontend", " \u{2013} Patrones de componentes React, reglas de gesti\u{f3}n de estado");
        m.insert("docs.rules_example_security", " \u{2013} Validaci\u{f3}n de entrada, patrones de autenticaci\u{f3}n, cumplimiento OWASP");
        m.insert("docs.rules_example_testing", " \u{2013} Estructura de pruebas, expectativas de cobertura, estrategia de mocking");
        m.insert("docs.rules_example_rust", " \u{2013} Manejo de errores con thiserror, estructura de m\u{f3}dulos, nombres");
        m.insert("docs.rules_tip1", "Un tema por archivo. No mezcles reglas de frontend y backend \u{2013} archivos m\u{e1}s peque\u{f1}os y enfocados son m\u{e1}s f\u{e1}ciles de mantener y reutilizar.");
        m.insert("docs.rules_tip2", "Las reglas globales son ideales para preferencias de estilo personal: lenguaje preferido, herramienta de formato, formato de mensajes de commit.");
        m.insert("docs.rules_tip3", "Las reglas de proyecto anulan las reglas globales. Si hay un conflicto, la regla a nivel de proyecto prevalece.");
        m.insert("docs.rules_tip4", "Usa la verificaci\u{f3}n de estado de ClaudeAdmin para detectar reglas duplicadas entre el nivel global y el de proyecto.");
        m.insert("docs.rules_ext_link", "Docs de Anthropic: Reglas \u{2192}");

        // ── Docs: Skills ──
        m.insert("docs.skills_heading", "Skills");
        m.insert("docs.skills_callout", "Prompts reutilizables y estructurados con metadatos. Los skills son como plugins para Claude \u{2013} pueden activarse autom\u{e1}ticamente por contexto o invocarse manualmente mediante comandos slash.");
        m.insert("docs.skills_how_heading", "C\u{f3}mo funciona");
        m.insert("docs.skills_how_text", "Cada skill reside en su propio directorio conteniendo un archivo SKILL.md con metadatos YAML y un cuerpo en markdown. Los metadatos definen informaci\u{f3}n como descripci\u{f3}n y condiciones de activaci\u{f3}n. El cuerpo contiene las instrucciones reales del prompt, ejemplos y material de referencia.");
        m.insert("docs.skills_structure_heading", "Estructura");
        m.insert("docs.skills_locations_heading", "Ubicaciones");
        m.insert("docs.skills_loc_global", "Disponible en todos los proyectos");
        m.insert("docs.skills_loc_project", "Skills espec\u{ed}ficos del proyecto");
        m.insert("docs.skills_tip1", "Establece user_invocable: true en los metadatos para hacer un skill invocable v\u{ed}a /nombre-del-skill en Claude Code.");
        m.insert("docs.skills_tip2", "Incluye ejemplos concretos en tu SKILL.md. Claude funciona mucho mejor con ejemplos de entrada/salida.");
        m.insert("docs.skills_tip3", "Usa el Explorador de Skills en ClaudeAdmin para descubrir e instalar skills de la comunidad.");
        m.insert("docs.skills_tip4", "Los archivos de referencia en el directorio del skill solo se cargan cuando el skill se activa, ahorrando tokens.");
        m.insert("docs.skills_ext_link", "Docs de Anthropic: Skills \u{2192}");

        // ── Docs: Memory ──
        m.insert("docs.memory_heading", "Memoria");
        m.insert("docs.memory_callout", "La base de conocimiento persistente de Claude por proyecto. Los archivos de memoria almacenan patrones, preferencias y aprendizajes que Claude acumula a trav\u{e9}s de las sesiones.");
        m.insert("docs.memory_how_heading", "C\u{f3}mo funciona");
        m.insert("docs.memory_how_text", "Claude Code mantiene un directorio de memoria para cada proyecto, almacenado en ~/.claude/projects/<encoded-path>/memory/. El archivo principal MEMORY.md tiene un estatus especial: sus primeras 200 l\u{ed}neas se cargan en el prompt del sistema al iniciar la sesi\u{f3}n. Los archivos de temas adicionales (debugging.md, api-conventions.md, etc.) se cargan bajo demanda cuando Claude determina que son relevantes para la tarea actual.");
        m.insert("docs.memory_structure_heading", "Estructura");
        m.insert("docs.memory_auto_heading", "Auto-Memoria");
        m.insert("docs.memory_auto_text", "Claude Code puede a\u{f1}adir autom\u{e1}ticamente entradas a la memoria cuando descubre patrones del proyecto, soluciones de depuraci\u{f3}n o tus preferencias. Puedes revisar y editar la memoria autogenerada con el comando /memory en Claude Code o a trav\u{e9}s del editor de memoria de ClaudeAdmin.");
        m.insert("docs.memory_tip1", "Pon la informaci\u{f3}n m\u{e1}s cr\u{ed}tica en las primeras 200 l\u{ed}neas de MEMORY.md \u{2013} eso es lo que se carga autom\u{e1}ticamente.");
        m.insert("docs.memory_tip2", "Usa archivos de temas para conocimiento profundo. Solo se cargan cuando se necesitan, manteniendo bajo el uso base de tokens.");
        m.insert("docs.memory_tip3", "Revisa la auto-memoria regularmente. Claude a veces almacena soluciones demasiado espec\u{ed}ficas de una sola vez.");
        m.insert("docs.memory_tip4", "La memoria es por proyecto. Si cambias a otro proyecto, Claude obtiene un conjunto diferente de memorias.");
        m.insert("docs.memory_ext_link", "Docs de Anthropic: Memoria \u{2192}");

        // ── Docs: Settings & Hooks ──
        m.insert("docs.settings_heading", "Configuraci\u{f3}n y Hooks");
        m.insert("docs.settings_heading_short", "Configuraci\u{f3}n");
        m.insert("docs.settings_callout", "Configuraci\u{f3}n basada en JSON para comportamiento, permisos y automatizaci\u{f3}n. Los hooks te permiten ejecutar comandos de shell autom\u{e1}ticamente antes o despu\u{e9}s de que Claude use herramientas.");
        m.insert("docs.settings_hierarchy_heading", "Jerarqu\u{ed}a de configuraci\u{f3}n");
        m.insert("docs.settings_hierarchy_text", "La configuraci\u{f3}n sigue un modelo por capas con especificidad creciente. Las capas m\u{e1}s espec\u{ed}ficas anulan las menos espec\u{ed}ficas:");
        m.insert("docs.settings_managed_code", "Pol\u{ed}ticas empresariales");
        m.insert("docs.settings_managed_desc", "M\u{e1}xima prioridad, definida por la organizaci\u{f3}n (solo lectura)");
        m.insert("docs.settings_global_desc", "Tu configuraci\u{f3}n global personal");
        m.insert("docs.settings_project_desc", "Configuraci\u{f3}n de equipo, subida a git");
        m.insert("docs.settings_local_desc", "Tus anulaciones personales del proyecto (en gitignore)");
        m.insert("docs.settings_hooks_heading", "Hooks");
        m.insert("docs.settings_hooks_text", "Los hooks son comandos de shell activados en eventos espec\u{ed}ficos durante una sesi\u{f3}n de Claude Code. Se configuran en settings.json bajo la clave hooks.");
        m.insert("docs.settings_hooks_events", "Eventos:\n\u{2022} PreToolUse  \u{2013} Antes de que Claude ejecute una herramienta (ej. auto-formatear antes de escribir)\n\u{2022} PostToolUse \u{2013} Despu\u{e9}s de que Claude ejecute una herramienta (ej. lint despu\u{e9}s de cambiar c\u{f3}digo)\n\u{2022} Stop        \u{2013} Cuando Claude termina una respuesta");
        m.insert("docs.settings_tip1", "Usa hooks PreToolUse para auto-formatear c\u{f3}digo antes de que Claude escriba archivos. Esto asegura un estilo consistente.");
        m.insert("docs.settings_tip2", "Los hooks PostToolUse son ideales para linting: detecta problemas inmediatamente despu\u{e9}s de que Claude modifique c\u{f3}digo.");
        m.insert("docs.settings_tip3", "La p\u{e1}gina de configuraci\u{f3}n de ClaudeAdmin muestra la cadena efectiva de hooks a trav\u{e9}s de todas las capas.");
        m.insert("docs.settings_ext_link", "Docs de Anthropic: Configuraci\u{f3}n \u{2192}");
        m.insert("docs.settings_hooks_ext_link", "Docs de Anthropic: Hooks \u{2192}");

        // ── Docs: MCP Servers ──
        m.insert("docs.mcp_heading", "Servidores MCP");
        m.insert("docs.mcp_callout", "Los servidores del Protocolo de Contexto de Modelo extienden a Claude con herramientas externas y fuentes de datos. Permiten que Claude interact\u{fa}e con bases de datos, APIs, sistemas de archivos y otros servicios.");
        m.insert("docs.mcp_how_heading", "C\u{f3}mo funciona");
        m.insert("docs.mcp_how_text", "Los servidores MCP son procesos externos que Claude Code inicia y con los que se comunica mediante el protocolo MCP. Cada servidor proporciona un conjunto de herramientas que Claude puede invocar. La configuraci\u{f3}n reside en ~/.claude.json bajo la clave mcpServers.");
        m.insert("docs.mcp_config_heading", "Configuraci\u{f3}n");
        m.insert("docs.mcp_management_heading", "Gesti\u{f3}n en ClaudeAdmin");
        m.insert("docs.mcp_management_text", "ClaudeAdmin proporciona una p\u{e1}gina dedicada de servidores MCP para gesti\u{f3}n completa: ver, a\u{f1}adir, editar y eliminar servidores sin edici\u{f3}n manual de JSON. La funci\u{f3}n de verificaci\u{f3}n de estado inicia cada servidor y verifica que responda a solicitudes JSON-RPC initialize y tools/list. Usa el Explorador MCP para descubrir e instalar servidores populares con un solo clic.");
        m.insert("docs.mcp_tip1", "Los servidores MCP tambi\u{e9}n se pueden configurar por proyecto en .claude/settings.json.");
        m.insert("docs.mcp_tip2", "Usa variables de entorno para secretos \u{2013} nunca escribas claves de API directamente en archivos de configuraci\u{f3}n.");
        m.insert("docs.mcp_tip3", "Usa el Explorador MCP para descubrir e instalar servidores populares, o a\u{f1}ade personalizados mediante la pesta\u{f1}a Nuevo servidor.");
        m.insert("docs.mcp_ext_link", "Docs de Anthropic: MCP \u{2192}");
        m.insert("docs.mcp_spec_ext_link", "Especificaci\u{f3}n MCP \u{2192}");

        // ── Docs: Plans ──
        m.insert("docs.plans_heading", "Planes");
        m.insert("docs.plans_callout", "Archivos markdown que Claude usa para desglosar tareas complejas. Los planes ayudan a Claude a mantener el enfoque en trabajos de m\u{fa}ltiples pasos y rastrear el progreso.");
        m.insert("docs.plans_how_heading", "C\u{f3}mo funciona");
        m.insert("docs.plans_how_text", "Cuando Claude aborda una tarea compleja, puede crear o consultar archivos de plan almacenados en ~/.claude/plans/. Los planes son documentos markdown estructurados con listas de tareas, dependencias y seguimiento de estado. Persisten entre sesiones, para que Claude pueda retomar donde lo dej\u{f3}.");
        m.insert("docs.plans_location_heading", "Ubicaci\u{f3}n");
        m.insert("docs.plans_loc_global", "Todos los archivos de planes");
        m.insert("docs.plans_tip1", "Pide a Claude que \u{201c}haga un plan\u{201d} antes de refactorizaciones complejas. Los planes reducen errores en cambios de m\u{fa}ltiples archivos.");
        m.insert("docs.plans_tip2", "Limpia planes antiguos peri\u{f3}dicamente. La p\u{e1}gina de Planes de ClaudeAdmin muestra todos los planes almacenados con fechas de modificaci\u{f3}n.");

        // ── Docs: Scopes ──
        m.insert("docs.scopes_heading", "Global vs. \u{c1}mbito de proyecto");
        m.insert("docs.scopes_callout", "Entender el \u{e1}mbito es clave para una configuraci\u{f3}n efectiva de Claude Code. Cada tipo de configuraci\u{f3}n existe en dos capas: global (tus valores predeterminados personales) y espec\u{ed}fico del proyecto (compartido con tu equipo).");
        m.insert("docs.scopes_overview_heading", "Resumen de \u{e1}mbitos");
        m.insert("docs.scopes_col_type", "Tipo de configuraci\u{f3}n");
        m.insert("docs.scopes_col_global", "Global (Usuario)");
        m.insert("docs.scopes_col_project", "Proyecto");
        m.insert("docs.scopes_col_priority", "Prioridad");
        m.insert("docs.scopes_priority_project_global", "Proyecto > Global");
        m.insert("docs.scopes_priority_both", "Ambos disponibles");
        m.insert("docs.scopes_memory_global", "Por proyecto en ~/.claude/projects/");
        m.insert("docs.scopes_priority_project_keyed", "Clave por proyecto");
        m.insert("docs.scopes_priority_local_project_global", "Local > Proyecto > Global");
        m.insert("docs.scopes_priority_merged", "Combinado");
        m.insert("docs.scopes_when_heading", "\u{bf}Cu\u{e1}ndo usar cu\u{e1}l?");
        m.insert("docs.scopes_use_global", "Usar Global para");
        m.insert("docs.scopes_global_1", "Preferencias personales de estilo de codificaci\u{f3}n");
        m.insert("docs.scopes_global_2", "Valores predeterminados de lenguaje y framework preferidos");
        m.insert("docs.scopes_global_3", "Formato de mensajes de commit");
        m.insert("docs.scopes_global_4", "Configuraci\u{f3}n de integraci\u{f3}n con editor/IDE");
        m.insert("docs.scopes_global_5", "Servidores MCP que usas en todos los proyectos");
        m.insert("docs.scopes_use_project", "Usar Proyecto para");
        m.insert("docs.scopes_project_1", "Documentaci\u{f3}n del stack tecnol\u{f3}gico y restricciones");
        m.insert("docs.scopes_project_2", "Convenciones de codificaci\u{f3}n del equipo");
        m.insert("docs.scopes_project_3", "Reglas espec\u{ed}ficas del dominio (seguridad, cumplimiento)");
        m.insert("docs.scopes_project_4", "Skills y flujos de trabajo espec\u{ed}ficos del proyecto");
        m.insert("docs.scopes_project_5", "Hooks de CI/CD y automatizaci\u{f3}n");

        // ── Docs: Tips & Best Practices ──
        m.insert("docs.bestpractices_heading", "Consejos y buenas pr\u{e1}cticas");
        m.insert("docs.bestpractices_hygiene_heading", "Higiene de configuraci\u{f3}n");
        m.insert("docs.bestpractices_hygiene_1", "Ejecuta la verificaci\u{f3}n de estado de ClaudeAdmin regularmente. Detecta reglas duplicadas, listas de permisos infladas y archivos CLAUDE.md faltantes.");
        m.insert("docs.bestpractices_hygiene_2", "No te repitas: si una regla existe globalmente, no la copies en el CLAUDE.md del proyecto. Usa el sistema de \u{e1}mbitos.");
        m.insert("docs.bestpractices_hygiene_3", "Mant\u{e9}n limpias las listas de permisos. Con el tiempo, Claude Code acumula cientos de entradas de permitir/denegar. Usa la p\u{e1}gina de Permisos para depurarlas.");
        m.insert("docs.bestpractices_tokens_heading", "Eficiencia de tokens");
        m.insert("docs.bestpractices_tokens_1", "Todo en CLAUDE.md, reglas, skills (cuando se activan) y las primeras 200 l\u{ed}neas de MEMORY.md cuenta contra tu ventana de contexto. S\u{e9} conciso.");
        m.insert("docs.bestpractices_tokens_2", "Mueve material de referencia detallado a archivos de referencia de skills o archivos de temas de memoria \u{2013} solo se cargan cuando se necesitan.");
        m.insert("docs.bestpractices_tokens_3", "Usa la p\u{e1}gina de Anal\u{ed}tica para monitorear tu uso de tokens en proyectos y sesiones.");
        m.insert("docs.bestpractices_team_heading", "Colaboraci\u{f3}n en equipo");
        m.insert("docs.bestpractices_team_1", "Sube .claude/rules/ y .claude/skills/ a git. Esto comparte convenciones en todo el equipo.");
        m.insert("docs.bestpractices_team_2", "Usa .claude/settings.json para configuraci\u{f3}n de equipo y .claude/settings.local.json para anulaciones personales.");
        m.insert("docs.bestpractices_team_3", "CLAUDE.md en la ra\u{ed}z del proyecto es el contrato de tu equipo con Claude. Tr\u{e1}talo como documentaci\u{f3}n \u{2013} revisa los cambios en PRs.");
        m.insert("docs.bestpractices_debug_heading", "Depuraci\u{f3}n del comportamiento de Claude");
        m.insert("docs.bestpractices_debug_1", "Si Claude ignora una regla, revisa la p\u{e1}gina de jerarqu\u{ed}a de configuraci\u{f3}n para detectar configuraciones en conflicto entre capas.");
        m.insert("docs.bestpractices_debug_2", "La memoria puede causar comportamiento inesperado. Revisa las entradas autogeneradas \u{2013} Claude puede haber memorizado una soluci\u{f3}n alternativa en lugar del enfoque correcto.");
        m.insert("docs.bestpractices_debug_3", "Usa la p\u{e1}gina de Sesiones para revisar conversaciones pasadas y entender qu\u{e9} estaba \u{201c}pensando\u{201d} Claude.");

        // ── Docs: Links ──
        m.insert("docs.links_heading", "Documentaci\u{f3}n oficial de Anthropic");
        m.insert("docs.links_text", "Estos enlaces apuntan a la documentaci\u{f3}n oficial mantenida por Anthropic. ClaudeAdmin est\u{e1} construido sobre estas especificaciones.");
        m.insert("docs.link_overview_title", "Visi\u{f3}n general de Claude Code");
        m.insert("docs.link_overview_desc", "Primeros pasos, instalaci\u{f3}n y uso b\u{e1}sico");
        m.insert("docs.link_memory_title", "Memoria y CLAUDE.md");
        m.insert("docs.link_memory_desc", "C\u{f3}mo Claude almacena y utiliza la memoria del proyecto");
        m.insert("docs.link_skills_title", "Skills");
        m.insert("docs.link_skills_desc", "Crear y gestionar skills reutilizables");
        m.insert("docs.link_settings_title", "Configuraci\u{f3}n");
        m.insert("docs.link_settings_desc", "Jerarqu\u{ed}a de configuraci\u{f3}n y opciones");
        m.insert("docs.link_hooks_title", "Hooks");
        m.insert("docs.link_hooks_desc", "Automatizaci\u{f3}n basada en eventos con comandos de shell");
        m.insert("docs.link_mcp_title", "Servidores MCP");
        m.insert("docs.link_mcp_desc", "Extender a Claude con herramientas externas");
        m.insert("docs.link_bestpractices_title", "Buenas pr\u{e1}cticas");
        m.insert("docs.link_bestpractices_desc", "Consejos para un uso efectivo de Claude Code");
        m.insert("docs.link_mcp_spec_title", "Especificaci\u{f3}n MCP");
        m.insert("docs.link_mcp_spec_desc", "El est\u{e1}ndar del Protocolo de Contexto de Modelo");

        // ── Licenses ──
        m.insert("sidebar.licenses", "Licencias");
        m.insert("licenses.title", "Licencias");
        m.insert("licenses.subtitle", "Licencias de c\u{00f3}digo abierto y dependencias");
        m.insert("licenses.own_license", "Licencia de ClaudeAdmin");
        m.insert("licenses.third_party", "Dependencias de terceros");
        m.insert("licenses.col_name", "Crate");
        m.insert("licenses.col_version", "Versi\u{00f3}n");
        m.insert("licenses.col_license", "Licencia");
        m.insert("licenses.search_placeholder", "Buscar dependencias...");
        m.insert("licenses.loading", "Cargando licencias");
        m.insert("licenses.count", "dependencias");
        m.insert("licenses.mit_copyright", "Copyright (c) 2024-2026");
        m.insert("licenses.mit_line1", "Se concede permiso, de forma gratuita, a cualquier persona que obtenga una copia de este software y los archivos de documentación asociados (el \u{201c}Software\u{201d}), para tratar el Software sin restricción, incluyendo sin limitación los derechos de usar, copiar, modificar, fusionar, publicar, distribuir, sublicenciar y/o vender copias del Software, y permitir a las personas a las que se les proporcione el Software hacerlo, sujeto a las siguientes condiciones:");
        m.insert("licenses.mit_line2", "El aviso de copyright anterior y este aviso de permiso se incluirán en todas las copias o porciones sustanciales del Software.");
        m.insert("licenses.mit_line3", "EL SOFTWARE SE PROPORCIONA \u{201c}TAL CUAL\u{201d}, SIN GARANTÍA DE NINGÚN TIPO, EXPRESA O IMPLÍCITA, INCLUYENDO PERO NO LIMITÁNDOSE A LAS GARANTÍAS DE COMERCIABILIDAD, IDONEIDAD PARA UN PROPÓSITO PARTICULAR Y NO INFRACCIÓN. EN NINGÚN CASO LOS AUTORES O TITULARES DEL COPYRIGHT SERÁN RESPONSABLES DE NINGUNA RECLAMACIÓN, DAÑO U OTRA RESPONSABILIDAD, YA SEA EN UNA ACCIÓN CONTRACTUAL, AGRAVIO U OTRO, QUE SURJA DE, O EN CONEXIÓN CON EL SOFTWARE O EL USO U OTRAS TRANSACCIONES EN EL SOFTWARE.");
        m.insert("licenses.direct_deps", "Dependencias directas");
        m.insert("licenses.transitive_deps", "Dependencias transitivas");
        m.insert("licenses.overview", "Resumen de licencias");
        m.insert("licenses.direct_count", "directas");
        m.insert("licenses.transitive_count", "dependencias transitivas");

        // ── Components ──
        m.insert("component.modal.close", "Cerrar");
        m.insert("component.editor.save", "Guardar");
        m.insert("component.editor.saved", "\u{a1}Guardado!");
        m.insert("component.json_editor.valid", "JSON v\u{e1}lido");
        m.insert("component.json_editor.invalid", "JSON inv\u{e1}lido");
        m.insert("component.frontmatter.description", "Descripci\u{f3}n");
        m.insert("component.frontmatter.user_invocable", "Invocable por usuario");
        m.insert("component.advisor.title", "Asesor de proyecto");
        m.insert("component.advisor.analyze", "Analizar");
        m.insert("component.advisor.analyzing", "Analizando...");
        m.insert("component.advisor.no_api_key", "No hay ANTHROPIC_API_KEY configurada");
        m.insert("component.advisor.error", "Error al cargar recomendaciones");
        m.insert("component.advisor.summary", "Resumen");
        m.insert("component.advisor.recommendations", "Recomendaciones");
        m.insert("component.advisor.apply", "Aplicar");
        m.insert("component.advisor.applied", "\u{a1}Listo!");
        m.insert("component.advisor.analyze_project", "Analizar proyecto");
        m.insert("component.advisor.hint", "Claude analiza tu proyecto y proporciona recomendaciones");
        m.insert("component.advisor.loading", "Claude est\u{e1} analizando tu proyecto");
        m.insert("component.advisor.assessment", "Evaluaci\u{f3}n del proyecto");
        m.insert("component.advisor.show_preview", "Mostrar vista previa");
        m.insert("component.advisor.category_tip", "Consejo");
        m.insert("component.frontmatter.user_invocable_label", "Invocable por usuario (se puede llamar con /comando)");
        m.insert("component.editor.saving", "Guardando...");

        // ── Common ──
        m.insert("common.error", "Error");
        m.insert("common.loading", "Cargando");
        m.insert("common.save", "Guardar");
        m.insert("common.delete", "Eliminar");
        m.insert("common.cancel", "Cancelar");
        m.insert("common.close", "Cerrar");
        m.insert("common.yes", "S\u{ed}");
        m.insert("common.no", "No");
        m.insert("common.ok", "OK");
        m.insert("common.error_prefix", "Error: ");
        m.insert("common.invalid_json", "JSON inv\u{e1}lido: ");

        m
    })
}
