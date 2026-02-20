use std::collections::HashMap;
use std::sync::OnceLock;

static TRANSLATIONS: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();

pub fn translations() -> &'static HashMap<&'static str, &'static str> {
    TRANSLATIONS.get_or_init(|| {
        let mut m = HashMap::new();

        // ── App ──
        m.insert("app.title", "ClaudeAdmin");
        m.insert("app.subtitle", "Gestor de configura\u{e7}\u{e3}o");

        // ── Sidebar ──
        m.insert("sidebar.overview", "Vis\u{e3}o geral");
        m.insert("sidebar.dashboard", "Painel");
        m.insert("sidebar.analytics", "Anal\u{ed}tica");
        m.insert("sidebar.manage", "Gerir");
        m.insert("sidebar.projects", "Projetos");
        m.insert("sidebar.global_skills", "Skills globais");
        m.insert("sidebar.skill_browser", "Explorador de Skills");
        m.insert("sidebar.global_rules", "Regras globais");
        m.insert("sidebar.plans", "Planos");
        m.insert("sidebar.mcp_servers", "Servidores MCP");
        m.insert("sidebar.mcp_browser", "Explorador MCP");
        m.insert("sidebar.security", "Seguran\u{e7}a");
        m.insert("sidebar.permissions", "Permiss\u{f5}es");
        m.insert("sidebar.config_health", "Estado da configura\u{e7}\u{e3}o");
        m.insert("sidebar.system", "Sistema");
        m.insert("sidebar.settings", "Configura\u{e7}\u{e3}o");
        m.insert("sidebar.sessions", "Sess\u{f5}es");
        m.insert("sidebar.github", "GitHub");
        m.insert("sidebar.learn", "Aprender");
        m.insert("sidebar.docs", "Documenta\u{e7}\u{e3}o");
        m.insert("sidebar.help", "Info do sistema");

        // ── Dashboard ──
        m.insert("dashboard.title", "Painel");
        m.insert("dashboard.subtitle", "Resumo da tua configura\u{e7}\u{e3}o do Claude Code");
        m.insert("dashboard.projects", "Projetos");
        m.insert("dashboard.global_skills", "Skills globais");
        m.insert("dashboard.global_rules", "Regras globais");
        m.insert("dashboard.mcp_servers", "Servidores MCP");
        m.insert("dashboard.plans", "Planos");
        m.insert("dashboard.config_health", "Estado da configura\u{e7}\u{e3}o");
        m.insert("dashboard.recent_projects", "Projetos recentes");
        m.insert("dashboard.loading", "A carregar");
        m.insert("dashboard.error_loading", "Erro ao carregar o painel");
        m.insert("dashboard.col_name", "Nome");
        m.insert("dashboard.col_claude_md", "CLAUDE.md");
        m.insert("dashboard.col_skills", "Skills");
        m.insert("dashboard.col_rules", "Regras");
        m.insert("dashboard.col_memory", "Mem\u{f3}ria");
        m.insert("dashboard.yes", "Sim");

        // ── MCP ──
        m.insert("mcp.title", "Servidores MCP");
        m.insert("mcp.subtitle", "Gerir servidores do Protocolo de Contexto de Modelo para o Claude Code");
        m.insert("mcp.tab_servers", "Servidores");
        m.insert("mcp.tab_health", "Verifica\u{e7}\u{e3}o de estado");
        m.insert("mcp.tab_add", "Novo servidor");
        m.insert("mcp.loading", "A carregar servidores MCP");
        m.insert("mcp.no_servers", "Nenhum servidor MCP configurado");
        m.insert("mcp.no_servers_hint", "Adiciona servidores usando o separador \u{2018}Novo servidor\u{2019} ou o Explorador MCP.");
        m.insert("mcp.select_server", "Seleciona um servidor da lista para ver e editar a sua configura\u{e7}\u{e3}o.");
        m.insert("mcp.no_servers_configured", "Nenhum servidor configurado.");
        m.insert("mcp.check_health", "Verificar estado");
        m.insert("mcp.save", "Guardar");
        m.insert("mcp.delete", "Eliminar");
        m.insert("mcp.saved", "Guardado!");
        m.insert("mcp.deleted", "Eliminado!");
        m.insert("mcp.read_only", "Somente leitura");
        m.insert("mcp.read_only_hint", "Este servidor \u{e9} gerido externamente e n\u{e3}o pode ser editado aqui.");
        m.insert("mcp.health.title", "Estado dos servidores MCP");
        m.insert("mcp.health.check_all", "Verificar todos os servidores");
        m.insert("mcp.health.checking", "A verificar...");
        m.insert("mcp.health.description", "Inicia cada processo de servidor MCP, envia JSON-RPC initialize + tools/list e apresenta os resultados. Tempo limite: 10 segundos por servidor.");
        m.insert("mcp.health.col_name", "Nome");
        m.insert("mcp.health.col_source", "Origem");
        m.insert("mcp.health.col_status", "Estado");
        m.insert("mcp.health.col_server_info", "Info do servidor");
        m.insert("mcp.health.col_tools", "Ferramentas");
        m.insert("mcp.health.col_duration", "Dura\u{e7}\u{e3}o");
        m.insert("mcp.health.running", "Em execu\u{e7}\u{e3}o");
        m.insert("mcp.health.error", "Erro");
        m.insert("mcp.health.timeout", "Tempo esgotado");
        m.insert("mcp.health.unknown", "Desconhecido");
        m.insert("mcp.add.title", "Adicionar servidor MCP");
        m.insert("mcp.add.description", "Adiciona um novo servidor MCP \u{e0} tua configura\u{e7}\u{e3}o global ~/.claude.json.");
        m.insert("mcp.add.name_label", "Nome do servidor");
        m.insert("mcp.add.name_placeholder", "ex. meu-servidor");
        m.insert("mcp.add.config_label", "Configura\u{e7}\u{e3}o do servidor (JSON)");
        m.insert("mcp.add.submit", "Adicionar servidor");
        m.insert("mcp.add.name_required", "Por favor, introduz um nome de servidor");
        m.insert("mcp.source.claude_code", "Claude Code");
        m.insert("mcp.source.claude_desktop", "Claude Desktop");

        // ── MCP Browser ──
        m.insert("mcp_browser.title", "Explorador MCP");
        m.insert("mcp_browser.subtitle", "Descobre e instala servidores MCP para o Claude Code");
        m.insert("mcp_browser.search_placeholder", "Pesquisar servidores MCP...");
        m.insert("mcp_browser.loading", "A carregar cat\u{e1}logo MCP");
        m.insert("mcp_browser.no_results", "Nenhum servidor MCP encontrado");
        m.insert("mcp_browser.installed", "Instalado");
        m.insert("mcp_browser.install", "Instalar");
        m.insert("mcp_browser.needs_api_key", "Precisa de chave API");
        m.insert("mcp_browser.install_success", "instalado com sucesso!");
        m.insert("mcp_browser.install_failed", "Erro ao instalar");

        // ── Projects ──
        m.insert("projects.title", "Projetos");
        m.insert("projects.subtitle", "Todos os projetos registados em ~/.claude.json");
        m.insert("projects.loading", "A carregar");
        m.insert("projects.error_loading", "Erro ao carregar projetos: ");
        m.insert("projects.col_name", "Nome");
        m.insert("projects.col_path", "Caminho");
        m.insert("projects.col_claude_md", "CLAUDE.md");
        m.insert("projects.col_skills", "Skills");
        m.insert("projects.col_rules", "Regras");
        m.insert("projects.col_memory", "Mem\u{f3}ria");
        m.insert("projects.yes", "Sim");

        // ── Project Detail ──
        m.insert("project_detail.loading", "A carregar detalhes do projeto");
        m.insert("project_detail.error_loading", "Erro ao carregar o projeto");
        m.insert("project_detail.tab_advisor", "Consultor");
        m.insert("project_detail.tab_claude_md", "CLAUDE.md");
        m.insert("project_detail.tab_skills", "Skills");
        m.insert("project_detail.tab_rules", "Regras");
        m.insert("project_detail.tab_memory", "Mem\u{f3}ria");
        m.insert("project_detail.tab_permissions", "Permiss\u{f5}es");
        m.insert("project_detail.tab_health", "Estado");
        m.insert("project_detail.no_claude_md", "CLAUDE.md n\u{e3}o encontrado");
        m.insert("project_detail.no_claude_md_hint", "Cria um CLAUDE.md no diret\u{f3}rio do teu projeto para dar instru\u{e7}\u{f5}es ao Claude Code.");
        m.insert("project_detail.no_skills", "Sem skills para este projeto");
        m.insert("project_detail.no_rules", "Sem regras para este projeto");
        m.insert("project_detail.no_memory", "Sem mem\u{f3}ria para este projeto");
        m.insert("project_detail.save", "Guardar");
        m.insert("project_detail.saved", "Guardado!");
        m.insert("project_detail.skill_scope", "\u{c2}mbito");
        m.insert("project_detail.permissions_loading", "A carregar permiss\u{f5}es...");
        m.insert("project_detail.permissions_error", "Erro ao carregar permiss\u{f5}es");
        m.insert("project_detail.permissions_entries", "Entradas");
        m.insert("project_detail.permissions_col_tool", "Ferramenta");
        m.insert("project_detail.permissions_col_command", "Comando");
        m.insert("project_detail.permissions_no_entries", "Sem entradas de permiss\u{f5}es");
        m.insert("project_detail.health_loading", "A calcular estado...");
        m.insert("project_detail.health_error", "Erro ao carregar dados de estado");
        m.insert("project_detail.health_score", "Pontua\u{e7}\u{e3}o de estado");
        m.insert("project_detail.health_claude_md", "CLAUDE.md presente");
        m.insert("project_detail.health_memory", "Mem\u{f3}ria presente");
        m.insert("project_detail.health_permissions", "Permiss\u{f5}es");
        m.insert("project_detail.health_security_issues", "Problemas de seguran\u{e7}a");
        m.insert("project_detail.health_duplicated_rules", "Regras duplicadas");
        m.insert("project_detail.health_no_security_issues", "Nenhum problema de seguran\u{e7}a encontrado");
        m.insert("project_detail.health_col_text", "Texto");
        m.insert("project_detail.health_col_found_in", "Encontrado em");
        m.insert("project_detail.health_col_also_in", "Tamb\u{e9}m em");
        m.insert("project_detail.health_permission_entries", "Entradas de permiss\u{f5}es");
        m.insert("project_detail.permissions_col_index", "#");
        m.insert("project_detail.permissions_col_status", "Estado");
        m.insert("project_detail.permissions_fragment", "Fragmento");
        m.insert("project_detail.permissions_ok", "OK");
        m.insert("project_detail.permissions_security_warnings", "aviso(s) de seguran\u{e7}a");
        m.insert("project_detail.permissions_manage", "Gerir permiss\u{f5}es");
        m.insert("project_detail.advisor_analyze", "Analisar projeto");
        m.insert("project_detail.advisor_analyzing", "A analisar...");
        m.insert("project_detail.advisor_description", "O Claude analisa o teu projeto e fornece recomenda\u{e7}\u{f5}es");
        m.insert("project_detail.advisor_loading", "O Claude est\u{e1} a analisar o teu projeto");
        m.insert("project_detail.advisor_summary", "Avalia\u{e7}\u{e3}o do projeto");
        m.insert("project_detail.advisor_done", "Pronto!");
        m.insert("project_detail.advisor_preview", "Mostrar pr\u{e9}-visualiza\u{e7}\u{e3}o");
        m.insert("project_detail.advisor_category_tip", "Dica");
        m.insert("project_detail.skills_col_name", "Nome");
        m.insert("project_detail.skills_col_description", "Descri\u{e7}\u{e3}o");
        m.insert("project_detail.skills_col_invocable", "Invoc\u{e1}vel");
        m.insert("project_detail.rules_col_name", "Nome");
        m.insert("project_detail.rules_col_path", "Caminho");
        m.insert("project_detail.memory_col_file", "Ficheiro");
        m.insert("project_detail.memory_col_size", "Tamanho");
        m.insert("project_detail.bytes", "bytes");
        m.insert("project_detail.unknown_tab", "Separador desconhecido");

        // ── Global Skills ──
        m.insert("global_skills.title", "Skills globais");
        m.insert("global_skills.subtitle", "Gerir skills em ~/.claude/skills/");
        m.insert("global_skills.loading", "A carregar skills");
        m.insert("global_skills.no_skills", "Nenhum skill global encontrado");
        m.insert("global_skills.no_skills_hint", "Cria skills em ~/.claude/skills/ ou usa o Explorador de Skills.");
        m.insert("global_skills.select_skill", "Seleciona um skill da lista.");
        m.insert("global_skills.col_skill", "Skill");
        m.insert("global_skills.col_invocable", "Invoc\u{e1}vel");
        m.insert("global_skills.invocable", "Invoc\u{e1}vel");
        m.insert("global_skills.not_invocable", "N\u{e3}o invoc\u{e1}vel");
        m.insert("global_skills.editing", "A editar:");
        m.insert("global_skills.save", "Guardar");
        m.insert("global_skills.saved", "Guardado!");
        m.insert("global_skills.delete", "Eliminar");
        m.insert("global_skills.deleted", "Eliminado!");

        // ── Global Rules ──
        m.insert("global_rules.title", "Regras globais");
        m.insert("global_rules.subtitle", "Gerir regras em ~/.claude/rules/");
        m.insert("global_rules.loading", "A carregar regras");
        m.insert("global_rules.no_rules", "Nenhuma regra global encontrada");
        m.insert("global_rules.no_rules_hint", "Cria ficheiros .md em ~/.claude/rules/");
        m.insert("global_rules.select_rule", "Seleciona uma regra da lista.");
        m.insert("global_rules.col_rule", "Regra");
        m.insert("global_rules.editing", "A editar:");
        m.insert("global_rules.save", "Guardar");
        m.insert("global_rules.saved", "Guardado!");
        m.insert("global_rules.delete", "Eliminar");
        m.insert("global_rules.deleted", "Eliminado!");

        // ── Plans ──
        m.insert("plans.title", "Planos");
        m.insert("plans.subtitle", "Gerir ficheiros de planos em ~/.claude/plans/");
        m.insert("plans.loading", "A carregar planos");
        m.insert("plans.no_plans", "Nenhum plano encontrado");
        m.insert("plans.no_plans_hint", "Os planos s\u{e3}o criados pelo Claude Code durante o planeamento.");
        m.insert("plans.select_plan", "Seleciona um plano da lista.");
        m.insert("plans.col_plan", "Plano");
        m.insert("plans.col_modified", "Modificado");
        m.insert("plans.modified", "Modificado");
        m.insert("plans.plan_label", "Plano:");
        m.insert("plans.save", "Guardar");
        m.insert("plans.saved", "Guardado!");
        m.insert("plans.delete", "Eliminar");
        m.insert("plans.deleted", "Eliminado!");

        // ── Settings ──
        m.insert("settings.title", "Configura\u{e7}\u{e3}o");
        m.insert("settings.subtitle", "Gerir configura\u{e7}\u{e3}o e hooks do Claude Code");
        m.insert("settings.tab_overview", "Vis\u{e3}o geral");
        m.insert("settings.tab_hooks", "Modelos de Hooks");
        m.insert("settings.tab_storage", "Armazenamento");
        m.insert("settings.loading", "A carregar configura\u{e7}\u{e3}o");
        m.insert("settings.hooks_title", "Hooks");
        m.insert("settings.no_hooks", "Nenhum hook configurado");
        m.insert("settings.pre_tool_use", "PreToolUse");
        m.insert("settings.post_tool_use", "PostToolUse");
        m.insert("settings.matcher", "Filtro");
        m.insert("settings.command", "Comando");
        m.insert("settings.hook_templates_title", "Modelos de Hooks");
        m.insert("settings.hook_templates_desc", "Configura\u{e7}\u{f5}es de hooks pr\u{e9}-definidas para adicionar.");
        m.insert("settings.hook_templates_loading", "A carregar modelos");
        m.insert("settings.add_hook", "Adicionar");
        m.insert("settings.storage_title", "Uso de armazenamento");
        m.insert("settings.storage_loading", "A calcular armazenamento");
        m.insert("settings.storage_total", "Total");
        m.insert("settings.storage_dir", "Diret\u{f3}rio");
        m.insert("settings.storage_size", "Tamanho");

        // ── Permissions ──
        m.insert("permissions.title", "Permiss\u{f5}es");
        m.insert("permissions.subtitle", "Rever e gerir permiss\u{f5}es de projetos");
        m.insert("permissions.loading", "A carregar permiss\u{f5}es");
        m.insert("permissions.no_permissions", "Nenhuma permiss\u{e3}o encontrada");
        m.insert("permissions.col_project", "Projeto");
        m.insert("permissions.col_entries", "Entradas");
        m.insert("permissions.col_issues", "Problemas");
        m.insert("permissions.col_fragmented", "Fragmentado");
        m.insert("permissions.detail_title", "Permiss\u{f5}es");
        m.insert("permissions.detail_loading", "A carregar permiss\u{f5}es");
        m.insert("permissions.detail_col_tool", "Ferramenta");
        m.insert("permissions.detail_col_command", "Comando");
        m.insert("permissions.detail_col_status", "Estado");
        m.insert("permissions.detail_fragmented", "Fragmentado");
        m.insert("permissions.detail_security_issue", "Problema de seguran\u{e7}a");
        m.insert("permissions.detail_delete_selected", "Eliminar selecionados");
        m.insert("permissions.detail_deleted", "Eliminado!");
        m.insert("permissions.detail_warnings_title", "Avisos de seguran\u{e7}a");
        m.insert("permissions.health_title", "Estado da configura\u{e7}\u{e3}o");
        m.insert("permissions.health_subtitle", "Estado de todos os projetos");
        m.insert("permissions.health_loading", "A calcular estado");
        m.insert("permissions.health_col_project", "Projeto");
        m.insert("permissions.health_col_score", "Pontua\u{e7}\u{e3}o");
        m.insert("permissions.health_col_issues", "Problemas");
        m.insert("permissions.health_avg", "M\u{e9}dia");
        m.insert("permissions.subtitle_manage", "Gerir listas de permiss\u{f5}es em todos os projetos");
        m.insert("permissions.col_actions", "A\u{e7}\u{f5}es");
        m.insert("permissions.col_security_issues", "Problemas de seguran\u{e7}a");
        m.insert("permissions.details", "Detalhes");
        m.insert("permissions.detail_subtitle", "Rever e limpar entradas de permiss\u{f5}es");
        m.insert("permissions.detail_deleting", "A eliminar...");
        m.insert("permissions.detail_deleted_reloading", "Eliminado! A recarregar...");
        m.insert("permissions.detail_delete_count", "Eliminar selecionados");
        m.insert("permissions.detail_col_index", "#");
        m.insert("permissions.detail_fragment", "Fragmento");
        m.insert("permissions.detail_ok", "OK");
        m.insert("permissions.detail_warnings_count", "Avisos de seguran\u{e7}a");
        m.insert("permissions.detail_entry", "entrada");
        m.insert("permissions.health_subtitle_scores", "Pontua\u{e7}\u{f5}es de estado da configura\u{e7}\u{e3}o em todos os projetos");
        m.insert("permissions.health_avg_score", "Pontua\u{e7}\u{e3}o m\u{e9}dia de estado");
        m.insert("permissions.health_projects_analyzed", "Projetos analisados");
        m.insert("permissions.health_no_issues", "Sem problemas");

        // ── Analytics ──
        m.insert("analytics.title", "Anal\u{ed}tica");
        m.insert("analytics.subtitle", "Estat\u{ed}sticas de uso do Claude Code");
        m.insert("analytics.loading", "A carregar anal\u{ed}tica");
        m.insert("analytics.error_loading", "Erro ao carregar anal\u{ed}tica");
        m.insert("analytics.total_sessions", "Total de sess\u{f5}es");
        m.insert("analytics.total_messages", "Total de mensagens");
        m.insert("analytics.git_commits", "Commits Git");
        m.insert("analytics.lines_added", "Linhas adicionadas");
        m.insert("analytics.lines_removed", "Linhas removidas");
        m.insert("analytics.since", "desde");
        m.insert("analytics.activity_heatmap", "Mapa de atividade");
        m.insert("analytics.messages", "Mensagens");
        m.insert("analytics.sessions", "Sess\u{f5}es");
        m.insert("analytics.tool_calls", "Chamadas de ferramentas");
        m.insert("analytics.hourly_distribution", "Distribui\u{e7}\u{e3}o por hora");
        m.insert("analytics.model_usage", "Uso de modelos");
        m.insert("analytics.col_model", "Modelo");
        m.insert("analytics.col_input_tokens", "Tokens de entrada");
        m.insert("analytics.col_output_tokens", "Tokens de sa\u{ed}da");
        m.insert("analytics.col_cache_tokens", "Tokens de cache");
        m.insert("analytics.tool_ranking", "Ranking de ferramentas");
        m.insert("analytics.col_cache_read", "Leitura de cache");
        m.insert("analytics.tool_usage_top10", "Uso de ferramentas (Top 10)");
        m.insert("analytics.languages", "Linguagens");
        m.insert("analytics.session_outcomes", "Resultados de sess\u{f5}es");
        m.insert("analytics.outcomes", "Resultados");

        // ── Sessions ──
        m.insert("sessions.title", "Sess\u{f5}es");
        m.insert("sessions.subtitle", "Explorar hist\u{f3}rico de sess\u{f5}es do Claude Code");
        m.insert("sessions.loading", "A carregar sess\u{f5}es");
        m.insert("sessions.search_placeholder", "Pesquisar sess\u{f5}es...");
        m.insert("sessions.no_sessions", "Nenhuma sess\u{e3}o encontrada");
        m.insert("sessions.col_project", "Projeto");
        m.insert("sessions.col_date", "Data");
        m.insert("sessions.col_duration", "Dura\u{e7}\u{e3}o");
        m.insert("sessions.col_messages", "Mensagens");
        m.insert("sessions.col_summary", "Resumo");
        m.insert("sessions.col_outcome", "Resultado");
        m.insert("sessions.minutes", "min");
        m.insert("sessions.load_more", "Carregar mais");
        m.insert("sessions.detail_title", "Detalhes da sess\u{e3}o");
        m.insert("sessions.detail_loading", "A carregar sess\u{e3}o");
        m.insert("sessions.detail_project", "Projeto");
        m.insert("sessions.detail_start", "In\u{ed}cio");
        m.insert("sessions.detail_duration", "Dura\u{e7}\u{e3}o");
        m.insert("sessions.detail_messages", "Mensagens");
        m.insert("sessions.detail_tools", "Chamadas de ferramentas");
        m.insert("sessions.detail_tokens", "Tokens");
        m.insert("sessions.detail_git", "Git");
        m.insert("sessions.detail_first_prompt", "Primeiro prompt");
        m.insert("sessions.detail_summary", "Resumo");
        m.insert("sessions.back", "Voltar");
        m.insert("sessions.searching", "A pesquisar...");
        m.insert("sessions.search", "Pesquisar");
        m.insert("sessions.clear", "Limpar");
        m.insert("sessions.search_results", "Resultados da pesquisa");
        m.insert("sessions.no_results", "Nenhum resultado encontrado");
        m.insert("sessions.col_prompt", "Prompt");
        m.insert("sessions.session_prefix", "Sess\u{e3}o: ");
        m.insert("sessions.detail_start_time", "Hora de in\u{ed}cio");
        m.insert("sessions.user_messages", " utilizador / ");
        m.insert("sessions.assistant_messages", " assistente");
        m.insert("sessions.tokens_in", " entrada / ");
        m.insert("sessions.tokens_out", " sa\u{ed}da");
        m.insert("sessions.commits_label", " commits, +");
        m.insert("sessions.lines_minus", " / -");
        m.insert("sessions.tools_used", "Ferramentas usadas");
        m.insert("sessions.outcome_prefix", "Resultado: ");
        m.insert("sessions.showing", "A mostrar");
        m.insert("sessions.of", "de");
        m.insert("sessions.previous", "Anterior");
        m.insert("sessions.next", "Seguinte");

        // ── GitHub ──
        m.insert("github.title", "GitHub");
        m.insert("github.subtitle", "Estado da integra\u{e7}\u{e3}o com GitHub");
        m.insert("github.loading", "A carregar dados do GitHub");
        m.insert("github.auth_status", "Estado de autentica\u{e7}\u{e3}o");
        m.insert("github.username", "Utilizador");
        m.insert("github.linked_repos", "Reposit\u{f3}rios vinculados");
        m.insert("github.no_repos", "Sem reposit\u{f3}rios vinculados");
        m.insert("github.col_repo", "Reposit\u{f3}rio");
        m.insert("github.col_recent_commits", "Commits recentes");
        m.insert("github.col_open_prs", "PRs abertas");

        // ── Help / System Info ──
        m.insert("help.title", "Info do sistema");
        m.insert("help.subtitle", "Informa\u{e7}\u{e3}o do sistema do Claude Code");
        m.insert("help.loading", "A carregar informa\u{e7}\u{e3}o do sistema");
        m.insert("help.account", "Conta");
        m.insert("help.account_name", "Nome");
        m.insert("help.account_email", "Email");
        m.insert("help.subscription", "Subscri\u{e7}\u{e3}o");
        m.insert("help.claude_version", "Vers\u{e3}o do Claude Code");
        m.insert("help.gh_cli", "GitHub CLI");
        m.insert("help.skill_usage", "Uso de Skills");
        m.insert("help.no_skill_usage", "Sem registo de uso de skills");
        m.insert("help.col_skill", "Skill");
        m.insert("help.col_count", "Quantidade");
        m.insert("help.what_is_title", "O que \u{e9} o ClaudeAdmin?");
        m.insert("help.what_is_desc", "O ClaudeAdmin \u{e9} a consola de administra\u{e7}\u{e3}o visual para o Claude Code. Fornece uma interface web para gerir todos os aspetos da tua configura\u{e7}\u{e3}o do Claude Code: Projetos, Skills, Regras, Mem\u{f3}ria, Configura\u{e7}\u{e3}o, Hooks, Servidores MCP e Planos.");
        m.insert("help.system_status", "Estado do sistema");
        m.insert("help.not_set", "N\u{e3}o definido");
        m.insert("help.unknown", "Desconhecido");
        m.insert("help.not_found", "N\u{e3}o encontrado");
        m.insert("help.not_installed", "N\u{e3}o instalado");
        m.insert("help.concepts_title", "Conceitos do Claude Code");
        m.insert("help.concept_skills", "Prompts reutiliz\u{e1}veis com metadados YAML. Armazenados como ficheiros SKILL.md em ~/.claude/skills/ (global) ou .claude/skills/ (projeto).");
        m.insert("help.concept_rules", "Restri\u{e7}\u{f5}es e diretrizes que moldam o comportamento do Claude. Armazenadas como ficheiros .md em ~/.claude/rules/ ou ao n\u{ed}vel de projeto.");
        m.insert("help.concept_memory", "Notas persistentes por projeto. O MEMORY.md \u{e9} carregado automaticamente nos prompts do sistema. Armazena padr\u{f5}es, prefer\u{ea}ncias e aprendizagens.");
        m.insert("help.concept_hooks", "Comandos de shell acionados por eventos (PreToolUse, PostToolUse, Stop). Configurados em settings.json para auto-formata\u{e7}\u{e3}o, linting, etc.");
        m.insert("help.concept_mcp", "Os servidores do Protocolo de Contexto de Modelo estendem o Claude com ferramentas externas. Configurados em ~/.claude.json com command, args e env.");
        m.insert("help.concept_claudemd", "Ficheiro de instru\u{e7}\u{f5}es ao n\u{ed}vel de projeto. Carregado automaticamente como contexto. Cont\u{e9}m conven\u{e7}\u{f5}es do projeto, info do stack e guias de codifica\u{e7}\u{e3}o.");
        m.insert("help.disclaimer", "ClaudeAdmin \u{e9} um projeto comunit\u{e1}rio independente. N\u{e3}o \u{e9} afiliado, endossado ou aprovado pela Anthropic. Claude e Claude Code s\u{e3}o marcas registradas da Anthropic.");

        m.insert("github.subtitle_detail", "Integra\u{e7}\u{e3}o com GitHub CLI e reposit\u{f3}rios vinculados");
        m.insert("github.linked_repositories", "Reposit\u{f3}rios vinculados");
        m.insert("github.no_linked_repos", "Nenhum reposit\u{f3}rio GitHub vinculado em ~/.claude.json");
        m.insert("github.col_name", "Nome");
        m.insert("github.col_path", "Caminho");

        // ── Skill Browser ──
        m.insert("skill_browser.title", "Explorador de Skills");
        m.insert("skill_browser.subtitle", "Descobre e instala skills oficiais e da comunidade");
        m.insert("skill_browser.loading", "A carregar skills");
        m.insert("skill_browser.search_placeholder", "Pesquisar skills...");
        m.insert("skill_browser.no_results", "Nenhum skill encontrado");
        m.insert("skill_browser.installed", "Instalado");
        m.insert("skill_browser.install", "Instalar");
        m.insert("skill_browser.official", "Oficial");
        m.insert("skill_browser.community", "Comunidade");
        m.insert("skill_browser.tab_official", "Oficiais (Anthropic)");
        m.insert("skill_browser.tab_community", "Comunidade");
        m.insert("skill_browser.install_success", "instalado com sucesso!");
        m.insert("skill_browser.install_failed", "Erro ao instalar:");

        // ── Docs ──
        m.insert("docs.title", "Documenta\u{e7}\u{e3}o");
        m.insert("docs.subtitle", "Tudo o que precisas de saber sobre a configura\u{e7}\u{e3}o do Claude Code");
        m.insert("docs.loading", "A carregar documenta\u{e7}\u{e3}o");

        // ── Docs: Table of Contents ──
        m.insert("docs.toc_contents", "Conte\u{fa}dos");
        m.insert("docs.toc_why_claudeadmin", "Porqu\u{ea} o ClaudeAdmin?");
        m.insert("docs.toc_capabilities", "O que pode e n\u{e3}o pode fazer");
        m.insert("docs.toc_group", "Conceitos");
        m.insert("docs.toc_claude_md", "CLAUDE.md");
        m.insert("docs.toc_rules", "Regras");
        m.insert("docs.toc_skills", "Skills");
        m.insert("docs.toc_memory", "Mem\u{f3}ria");
        m.insert("docs.toc_settings", "Configura\u{e7}\u{e3}o e Hooks");
        m.insert("docs.toc_mcp", "Servidores MCP");
        m.insert("docs.toc_plans", "Planos");
        m.insert("docs.toc_scopes", "Global vs. Projeto");
        m.insert("docs.toc_tips", "Dicas e boas pr\u{e1}ticas");
        m.insert("docs.toc_links", "Documenta\u{e7}\u{e3}o oficial");

        // ── Docs: Shared labels ──
        m.insert("docs.tips_heading", "Dicas e truques");
        m.insert("docs.scope_global", "Global");
        m.insert("docs.scope_project", "Projeto");
        m.insert("docs.scope_user", "Utilizador");
        m.insert("docs.scope_parent", "Pai");
        m.insert("docs.scope_managed", "Gerido");
        m.insert("docs.scope_local", "Local");

        // ── Docs: Overview ──
        m.insert("docs.overview_heading", "Porqu\u{ea} o ClaudeAdmin?");
        m.insert("docs.overview_callout", " \u{e9} a consola de administra\u{e7}\u{e3}o central para toda a tua configura\u{e7}\u{e3}o do Claude Code. Substitui a edi\u{e7}\u{e3}o manual de ficheiros em dezenas de diret\u{f3}rios ocultos por uma \u{fa}nica interface visual.");
        m.insert("docs.overview_text1", "O Claude Code armazena a sua configura\u{e7}\u{e3}o numa hierarquia complexa de ficheiros e diret\u{f3}rios: ficheiros CLAUDE.md na raiz de projetos, regras e skills dispersos em subdiret\u{f3}rios de ~/.claude/, ficheiros de mem\u{f3}ria identificados por caminhos de projeto codificados, configura\u{e7}\u{f5}es em m\u{fa}ltiplos ficheiros JSON e configura\u{e7}\u{f5}es de servidores MCP em ~/.claude.json. \u{c0} medida que os teus projetos crescem, gerir tudo isto manualmente torna-se propenso a erros e consome muito tempo.");
        m.insert("docs.overview_text2", "O ClaudeAdmin oferece-te:");
        m.insert("docs.overview_li_visibility_label", "Visibilidade");
        m.insert("docs.overview_li_visibility", " \u{2013} V\u{ea} todos os teus projetos, skills, regras e mem\u{f3}ria num s\u{f3} lugar");
        m.insert("docs.overview_li_editing_label", "Edi\u{e7}\u{e3}o");
        m.insert("docs.overview_li_editing", " \u{2013} Edita CLAUDE.md, regras, skills e mem\u{f3}ria com um editor apropriado");
        m.insert("docs.overview_li_health_label", "Verifica\u{e7}\u{f5}es de estado");
        m.insert("docs.overview_li_health", " \u{2013} Deteta problemas de seguran\u{e7}a em permiss\u{f5}es, regras duplicadas e configura\u{e7}\u{f5}es em falta");
        m.insert("docs.overview_li_analytics_label", "Anal\u{ed}tica");
        m.insert("docs.overview_li_analytics", " \u{2013} Compreende como usas o Claude Code: sess\u{f5}es, tokens, ferramentas, custos");
        m.insert("docs.overview_li_advisor_label", "Consultor");
        m.insert("docs.overview_li_advisor", " \u{2013} Recomenda\u{e7}\u{f5}es impulsionadas por IA para melhorar a configura\u{e7}\u{e3}o do teu projeto");

        // ── Docs: Capabilities ──
        m.insert("docs.cap_heading", "O que o ClaudeAdmin pode e n\u{e3}o pode fazer");
        m.insert("docs.cap_can_heading", "O que pode fazer");
        m.insert("docs.cap_can_1", "Explorar e gerir todos os projetos registados em ~/.claude.json");
        m.insert("docs.cap_can_2", "Ver e editar ficheiros CLAUDE.md de qualquer projeto");
        m.insert("docs.cap_can_3", "Criar, editar e eliminar skills globais e de projeto");
        m.insert("docs.cap_can_4", "Criar, editar e eliminar regras globais e de projeto");
        m.insert("docs.cap_can_5", "Ver e editar ficheiros de mem\u{f3}ria do projeto (MEMORY.md e t\u{f3}picos)");
        m.insert("docs.cap_can_6", "Inspecionar a hierarquia de configura\u{e7}\u{e3}o (global \u{2192} projeto \u{2192} local)");
        m.insert("docs.cap_can_7", "Auditar entradas de permiss\u{f5}es e detetar problemas de seguran\u{e7}a");
        m.insert("docs.cap_can_8", "Ver configura\u{e7}\u{f5}es de servidores MCP");
        m.insert("docs.cap_can_9", "Analisar hist\u{f3}rico de sess\u{f5}es, uso de tokens e custos");
        m.insert("docs.cap_can_10", "Executar an\u{e1}lise de projeto impulsionada por IA com recomenda\u{e7}\u{f5}es acion\u{e1}veis");
        m.insert("docs.cap_can_11", "Explorar e instalar skills de reposit\u{f3}rios comunit\u{e1}rios");
        m.insert("docs.cap_can_12", "Todas as escritas criam c\u{f3}pias de seguran\u{e7}a autom\u{e1}ticas em ~/.claude/backups/");
        m.insert("docs.cap_cannot_heading", "O que n\u{e3}o pode fazer");
        m.insert("docs.cap_cannot_1", "Executar sess\u{f5}es do Claude Code \u{2013} gere configura\u{e7}\u{e3}o, n\u{e3}o execu\u{e7}\u{e3}o");
        m.insert("docs.cap_cannot_2", "Modificar pol\u{ed}ticas geridas (configura\u{e7}\u{e3}o ao n\u{ed}vel empresarial/organizacional)");
        m.insert("docs.cap_cannot_3", "Aceder a ambientes remotos ou sess\u{f5}es SSH");
        m.insert("docs.cap_cannot_4", "Substituir a CLI do Claude Code para trabalho de codifica\u{e7}\u{e3}o real");
        m.insert("docs.cap_cannot_5", "Editar diretamente os servidores MCP de .claude.json (somente leitura por seguran\u{e7}a)");
        m.insert("docs.cap_cannot_6", "Gerir chaves de API ou credenciais de autentica\u{e7}\u{e3}o");
        m.insert("docs.cap_cannot_callout", "O ClaudeAdmin \u{e9} um gestor de configura\u{e7}\u{e3}o, n\u{e3}o um substituto do Claude Code. Pensa nele como uma ferramenta de administra\u{e7}\u{e3}o de base de dados: ajuda-te a inspecionar, configurar e manter \u{2013} mas o trabalho real \u{e9} feito no Claude Code.");

        // ── Docs: CLAUDE.md ──
        m.insert("docs.claudemd_callout", "A constitui\u{e7}\u{e3}o do projeto. O CLAUDE.md \u{e9} o ficheiro de configura\u{e7}\u{e3}o mais importante \u{2013} \u{e9} carregado automaticamente em cada sess\u{e3}o do Claude Code como contexto persistente.");
        m.insert("docs.claudemd_how_heading", "Como funciona");
        m.insert("docs.claudemd_how_text", "Quando o Claude Code inicia uma sess\u{e3}o, procura ficheiros CLAUDE.md recursivamente desde o teu diret\u{f3}rio de trabalho atual at\u{e9} \u{e0} raiz do sistema de ficheiros. Todos os ficheiros encontrados s\u{e3}o carregados e concatenados, com os ficheiros mais pr\u{f3}ximos a ter prioridade. Isto significa que podes ter um CLAUDE.md ao n\u{ed}vel de monorepo com conven\u{e7}\u{f5}es partilhadas e ficheiros CLAUDE.md ao n\u{ed}vel de pacote com substitui\u{e7}\u{f5}es espec\u{ed}ficas.");
        m.insert("docs.claudemd_locations_heading", "Localiza\u{e7}\u{f5}es");
        m.insert("docs.claudemd_loc_project_or", " ou ");
        m.insert("docs.claudemd_loc_parent", "Raiz do monorepo, carregado para todos os subpacotes");
        m.insert("docs.claudemd_loc_user", "Valores predefinidos pessoais em todos os projetos");
        m.insert("docs.claudemd_whatto_heading", "O que incluir");
        m.insert("docs.claudemd_whatto_context_label", "Contexto do projeto");
        m.insert("docs.claudemd_whatto_context", " \u{2013} Stack tecnol\u{f3}gico, decis\u{f5}es de arquitetura, depend\u{ea}ncias chave");
        m.insert("docs.claudemd_whatto_standards_label", "Padr\u{f5}es de codifica\u{e7}\u{e3}o");
        m.insert("docs.claudemd_whatto_standards", " \u{2013} Conven\u{e7}\u{f5}es de nomes, regras de formata\u{e7}\u{e3}o, padr\u{f5}es de tratamento de erros");
        m.insert("docs.claudemd_whatto_workflows_label", "Fluxos de trabalho");
        m.insert("docs.claudemd_whatto_workflows", " \u{2013} Como compilar, testar, implantar; nomes de ramos; conven\u{e7}\u{f5}es de PR");
        m.insert("docs.claudemd_whatto_dodont_label", "Regras de fazer/n\u{e3}o fazer");
        m.insert("docs.claudemd_whatto_dodont", " \u{2013} Restri\u{e7}\u{f5}es expl\u{ed}citas (ex. \u{201c}nunca usar any em TypeScript\u{201d})");
        m.insert("docs.claudemd_whatto_team_label", "Acordos de equipa");
        m.insert("docs.claudemd_whatto_team", " \u{2013} Processo de revis\u{e3}o, formato de mensagens de commit, limites de m\u{f3}dulos");
        m.insert("docs.claudemd_tip1", "Mant\u{e9}m-no abaixo de 500 linhas. O Claude carrega o ficheiro completo no contexto \u{2013} ficheiros CLAUDE.md inchados desperdi\u{e7}am tokens e diluem instru\u{e7}\u{f5}es importantes.");
        m.insert("docs.claudemd_tip2", "Usa cabe\u{e7}alhos de sec\u{e7}\u{e3}o claros (## Arquitetura, ## Conven\u{e7}\u{f5}es). O Claude analisa a estrutura para encontrar sec\u{e7}\u{f5}es relevantes.");
        m.insert("docs.claudemd_tip3", "Coloca as regras mais cr\u{ed}ticas no in\u{ed}cio. Em ficheiros longos, o conte\u{fa}do no in\u{ed}cio recebe mais aten\u{e7}\u{e3}o.");
        m.insert("docs.claudemd_tip4", "Usa CLAUDE.local.md para prefer\u{ea}ncias pessoais que n\u{e3}o devem ser submetidas ao git.");
        m.insert("docs.claudemd_ext_link", "Docs da Anthropic: CLAUDE.md \u{2192}");

        // ── Docs: Rules ──
        m.insert("docs.rules_heading", "Regras");
        m.insert("docs.rules_callout", "Restri\u{e7}\u{f5}es modulares e tem\u{e1}ticas que moldam o comportamento do Claude. Ao contr\u{e1}rio do CLAUDE.md que \u{e9} um \u{fa}nico ficheiro grande, as regras s\u{e3}o ficheiros .md separados \u{2013} cada um focado num tema espec\u{ed}fico.");
        m.insert("docs.rules_how_heading", "Como funciona");
        m.insert("docs.rules_how_text", "As regras s\u{e3}o carregadas automaticamente ao iniciar a sess\u{e3}o. As regras globais (as tuas prefer\u{ea}ncias pessoais) s\u{e3}o carregadas primeiro, depois as regras do projeto complementam-nas. Isto permite-te definir o teu estilo de codifica\u{e7}\u{e3}o globalmente enquanto os projetos adicionam restri\u{e7}\u{f5}es espec\u{ed}ficas do dom\u{ed}nio.");
        m.insert("docs.rules_locations_heading", "Localiza\u{e7}\u{f5}es");
        m.insert("docs.rules_loc_global", "As tuas regras pessoais, aplicadas a todos os projetos");
        m.insert("docs.rules_loc_project", "Espec\u{ed}ficas do projeto, submetidas ao git para partilhar com a equipa");
        m.insert("docs.rules_examples_heading", "Exemplos");
        m.insert("docs.rules_example_frontend", " \u{2013} Padr\u{f5}es de componentes React, regras de gest\u{e3}o de estado");
        m.insert("docs.rules_example_security", " \u{2013} Valida\u{e7}\u{e3}o de entrada, padr\u{f5}es de autentica\u{e7}\u{e3}o, conformidade OWASP");
        m.insert("docs.rules_example_testing", " \u{2013} Estrutura de testes, expectativas de cobertura, estrat\u{e9}gia de mocking");
        m.insert("docs.rules_example_rust", " \u{2013} Tratamento de erros com thiserror, estrutura de m\u{f3}dulos, nomes");
        m.insert("docs.rules_tip1", "Um tema por ficheiro. N\u{e3}o mistures regras de frontend e backend \u{2013} ficheiros mais pequenos e focados s\u{e3}o mais f\u{e1}ceis de manter e reutilizar.");
        m.insert("docs.rules_tip2", "As regras globais s\u{e3}o ideais para prefer\u{ea}ncias de estilo pessoal: linguagem preferida, ferramenta de formata\u{e7}\u{e3}o, formato de mensagens de commit.");
        m.insert("docs.rules_tip3", "As regras de projeto substituem as regras globais. Se houver um conflito, a regra ao n\u{ed}vel de projeto prevalece.");
        m.insert("docs.rules_tip4", "Usa a verifica\u{e7}\u{e3}o de estado do ClaudeAdmin para detetar regras duplicadas entre o n\u{ed}vel global e o de projeto.");
        m.insert("docs.rules_ext_link", "Docs da Anthropic: Regras \u{2192}");

        // ── Docs: Skills ──
        m.insert("docs.skills_heading", "Skills");
        m.insert("docs.skills_callout", "Prompts reutiliz\u{e1}veis e estruturados com metadados. Os skills s\u{e3}o como plugins para o Claude \u{2013} podem ser acionados automaticamente por contexto ou invocados manualmente atrav\u{e9}s de comandos slash.");
        m.insert("docs.skills_how_heading", "Como funciona");
        m.insert("docs.skills_how_text", "Cada skill reside no seu pr\u{f3}prio diret\u{f3}rio contendo um ficheiro SKILL.md com metadados YAML e um corpo em markdown. Os metadados definem informa\u{e7}\u{e3}o como descri\u{e7}\u{e3}o e condi\u{e7}\u{f5}es de ativa\u{e7}\u{e3}o. O corpo cont\u{e9}m as instru\u{e7}\u{f5}es reais do prompt, exemplos e material de refer\u{ea}ncia.");
        m.insert("docs.skills_structure_heading", "Estrutura");
        m.insert("docs.skills_locations_heading", "Localiza\u{e7}\u{f5}es");
        m.insert("docs.skills_loc_global", "Dispon\u{ed}vel em todos os projetos");
        m.insert("docs.skills_loc_project", "Skills espec\u{ed}ficos do projeto");
        m.insert("docs.skills_tip1", "Define user_invocable: true nos metadados para tornar um skill invoc\u{e1}vel via /nome-do-skill no Claude Code.");
        m.insert("docs.skills_tip2", "Inclui exemplos concretos no teu SKILL.md. O Claude funciona muito melhor com exemplos de entrada/sa\u{ed}da.");
        m.insert("docs.skills_tip3", "Usa o Explorador de Skills no ClaudeAdmin para descobrir e instalar skills da comunidade.");
        m.insert("docs.skills_tip4", "Os ficheiros de refer\u{ea}ncia no diret\u{f3}rio do skill s\u{f3} s\u{e3}o carregados quando o skill \u{e9} acionado, poupando tokens.");
        m.insert("docs.skills_ext_link", "Docs da Anthropic: Skills \u{2192}");

        // ── Docs: Memory ──
        m.insert("docs.memory_heading", "Mem\u{f3}ria");
        m.insert("docs.memory_callout", "A base de conhecimento persistente do Claude por projeto. Os ficheiros de mem\u{f3}ria armazenam padr\u{f5}es, prefer\u{ea}ncias e aprendizagens que o Claude acumula ao longo das sess\u{f5}es.");
        m.insert("docs.memory_how_heading", "Como funciona");
        m.insert("docs.memory_how_text", "O Claude Code mant\u{e9}m um diret\u{f3}rio de mem\u{f3}ria para cada projeto, armazenado em ~/.claude/projects/<encoded-path>/memory/. O ficheiro principal MEMORY.md tem um estatuto especial: as suas primeiras 200 linhas s\u{e3}o carregadas no prompt do sistema ao iniciar a sess\u{e3}o. Os ficheiros de t\u{f3}picos adicionais (debugging.md, api-conventions.md, etc.) s\u{e3}o carregados a pedido quando o Claude determina que s\u{e3}o relevantes para a tarefa atual.");
        m.insert("docs.memory_structure_heading", "Estrutura");
        m.insert("docs.memory_auto_heading", "Auto-Mem\u{f3}ria");
        m.insert("docs.memory_auto_text", "O Claude Code pode adicionar automaticamente entradas \u{e0} mem\u{f3}ria quando descobre padr\u{f5}es do projeto, solu\u{e7}\u{f5}es de depura\u{e7}\u{e3}o ou as tuas prefer\u{ea}ncias. Podes rever e editar a mem\u{f3}ria autogerada com o comando /memory no Claude Code ou atrav\u{e9}s do editor de mem\u{f3}ria do ClaudeAdmin.");
        m.insert("docs.memory_tip1", "Coloca a informa\u{e7}\u{e3}o mais cr\u{ed}tica nas primeiras 200 linhas do MEMORY.md \u{2013} isso \u{e9} o que \u{e9} carregado automaticamente.");
        m.insert("docs.memory_tip2", "Usa ficheiros de t\u{f3}picos para conhecimento aprofundado. S\u{f3} s\u{e3}o carregados quando necess\u{e1}rio, mantendo baixo o uso base de tokens.");
        m.insert("docs.memory_tip3", "Revisa a auto-mem\u{f3}ria regularmente. O Claude por vezes armazena solu\u{e7}\u{f5}es demasiado espec\u{ed}ficas de uma s\u{f3} vez.");
        m.insert("docs.memory_tip4", "A mem\u{f3}ria \u{e9} por projeto. Se mudares para outro projeto, o Claude obt\u{e9}m um conjunto diferente de mem\u{f3}rias.");
        m.insert("docs.memory_ext_link", "Docs da Anthropic: Mem\u{f3}ria \u{2192}");

        // ── Docs: Settings & Hooks ──
        m.insert("docs.settings_heading", "Configura\u{e7}\u{e3}o e Hooks");
        m.insert("docs.settings_heading_short", "Configura\u{e7}\u{e3}o");
        m.insert("docs.settings_callout", "Configura\u{e7}\u{e3}o baseada em JSON para comportamento, permiss\u{f5}es e automa\u{e7}\u{e3}o. Os hooks permitem-te executar comandos de shell automaticamente antes ou depois do Claude usar ferramentas.");
        m.insert("docs.settings_hierarchy_heading", "Hierarquia de configura\u{e7}\u{e3}o");
        m.insert("docs.settings_hierarchy_text", "A configura\u{e7}\u{e3}o segue um modelo por camadas com especificidade crescente. As camadas mais espec\u{ed}ficas substituem as menos espec\u{ed}ficas:");
        m.insert("docs.settings_managed_code", "Pol\u{ed}ticas empresariais");
        m.insert("docs.settings_managed_desc", "M\u{e1}xima prioridade, definida pela organiza\u{e7}\u{e3}o (somente leitura)");
        m.insert("docs.settings_global_desc", "A tua configura\u{e7}\u{e3}o global pessoal");
        m.insert("docs.settings_project_desc", "Configura\u{e7}\u{e3}o de equipa, submetida ao git");
        m.insert("docs.settings_local_desc", "As tuas substitui\u{e7}\u{f5}es pessoais do projeto (no gitignore)");
        m.insert("docs.settings_hooks_heading", "Hooks");
        m.insert("docs.settings_hooks_text", "Os hooks s\u{e3}o comandos de shell acionados em eventos espec\u{ed}ficos durante uma sess\u{e3}o do Claude Code. S\u{e3}o configurados em settings.json sob a chave hooks.");
        m.insert("docs.settings_hooks_events", "Eventos:\n\u{2022} PreToolUse  \u{2013} Antes do Claude executar uma ferramenta (ex. auto-formatar antes de escrever)\n\u{2022} PostToolUse \u{2013} Depois do Claude executar uma ferramenta (ex. lint depois de alterar c\u{f3}digo)\n\u{2022} Stop        \u{2013} Quando o Claude termina uma resposta");
        m.insert("docs.settings_tip1", "Usa hooks PreToolUse para auto-formatar c\u{f3}digo antes do Claude escrever ficheiros. Isto assegura um estilo consistente.");
        m.insert("docs.settings_tip2", "Os hooks PostToolUse s\u{e3}o ideais para linting: deteta problemas imediatamente depois do Claude modificar c\u{f3}digo.");
        m.insert("docs.settings_tip3", "A p\u{e1}gina de configura\u{e7}\u{e3}o do ClaudeAdmin mostra a cadeia efetiva de hooks atrav\u{e9}s de todas as camadas.");
        m.insert("docs.settings_ext_link", "Docs da Anthropic: Configura\u{e7}\u{e3}o \u{2192}");
        m.insert("docs.settings_hooks_ext_link", "Docs da Anthropic: Hooks \u{2192}");

        // ── Docs: MCP Servers ──
        m.insert("docs.mcp_heading", "Servidores MCP");
        m.insert("docs.mcp_callout", "Os servidores do Protocolo de Contexto de Modelo estendem o Claude com ferramentas externas e fontes de dados. Permitem que o Claude interaja com bases de dados, APIs, sistemas de ficheiros e outros servi\u{e7}os.");
        m.insert("docs.mcp_how_heading", "Como funciona");
        m.insert("docs.mcp_how_text", "Os servidores MCP s\u{e3}o processos externos que o Claude Code inicia e com os quais comunica atrav\u{e9}s do protocolo MCP. Cada servidor fornece um conjunto de ferramentas que o Claude pode invocar. A configura\u{e7}\u{e3}o reside em ~/.claude.json sob a chave mcpServers.");
        m.insert("docs.mcp_config_heading", "Configura\u{e7}\u{e3}o");
        m.insert("docs.mcp_management_heading", "Gest\u{e3}o no ClaudeAdmin");
        m.insert("docs.mcp_management_text", "O ClaudeAdmin fornece uma p\u{e1}gina dedicada de servidores MCP para gest\u{e3}o completa: ver, adicionar, editar e eliminar servidores sem edi\u{e7}\u{e3}o manual de JSON. A fun\u{e7}\u{e3}o de verifica\u{e7}\u{e3}o de estado inicia cada servidor e verifica que responde a solicita\u{e7}\u{f5}es JSON-RPC initialize e tools/list. Usa o Explorador MCP para descobrir e instalar servidores populares com um s\u{f3} clique.");
        m.insert("docs.mcp_tip1", "Os servidores MCP tamb\u{e9}m podem ser configurados por projeto em .claude/settings.json.");
        m.insert("docs.mcp_tip2", "Usa vari\u{e1}veis de ambiente para segredos \u{2013} nunca escrevas chaves de API diretamente em ficheiros de configura\u{e7}\u{e3}o.");
        m.insert("docs.mcp_tip3", "Usa o Explorador MCP para descobrir e instalar servidores populares, ou adiciona personalizados atrav\u{e9}s do separador Novo servidor.");
        m.insert("docs.mcp_ext_link", "Docs da Anthropic: MCP \u{2192}");
        m.insert("docs.mcp_spec_ext_link", "Especifica\u{e7}\u{e3}o MCP \u{2192}");

        // ── Docs: Plans ──
        m.insert("docs.plans_heading", "Planos");
        m.insert("docs.plans_callout", "Ficheiros markdown que o Claude usa para decompor tarefas complexas. Os planos ajudam o Claude a manter o foco em trabalhos de m\u{fa}ltiplos passos e acompanhar o progresso.");
        m.insert("docs.plans_how_heading", "Como funciona");
        m.insert("docs.plans_how_text", "Quando o Claude aborda uma tarefa complexa, pode criar ou consultar ficheiros de plano armazenados em ~/.claude/plans/. Os planos s\u{e3}o documentos markdown estruturados com listas de tarefas, depend\u{ea}ncias e acompanhamento de estado. Persistem entre sess\u{f5}es, para que o Claude possa retomar onde parou.");
        m.insert("docs.plans_location_heading", "Localiza\u{e7}\u{e3}o");
        m.insert("docs.plans_loc_global", "Todos os ficheiros de planos");
        m.insert("docs.plans_tip1", "Pede ao Claude para \u{201c}fazer um plano\u{201d} antes de refactoriza\u{e7}\u{f5}es complexas. Os planos reduzem erros em altera\u{e7}\u{f5}es de m\u{fa}ltiplos ficheiros.");
        m.insert("docs.plans_tip2", "Limpa planos antigos periodicamente. A p\u{e1}gina de Planos do ClaudeAdmin mostra todos os planos armazenados com datas de modifica\u{e7}\u{e3}o.");

        // ── Docs: Scopes ──
        m.insert("docs.scopes_heading", "Global vs. \u{c2}mbito de projeto");
        m.insert("docs.scopes_callout", "Compreender o \u{e2}mbito \u{e9} fundamental para uma configura\u{e7}\u{e3}o eficaz do Claude Code. Cada tipo de configura\u{e7}\u{e3}o existe em duas camadas: global (os teus valores predefinidos pessoais) e espec\u{ed}fico do projeto (partilhado com a tua equipa).");
        m.insert("docs.scopes_overview_heading", "Resumo de \u{e2}mbitos");
        m.insert("docs.scopes_col_type", "Tipo de configura\u{e7}\u{e3}o");
        m.insert("docs.scopes_col_global", "Global (Utilizador)");
        m.insert("docs.scopes_col_project", "Projeto");
        m.insert("docs.scopes_col_priority", "Prioridade");
        m.insert("docs.scopes_priority_project_global", "Projeto > Global");
        m.insert("docs.scopes_priority_both", "Ambos dispon\u{ed}veis");
        m.insert("docs.scopes_memory_global", "Por projeto em ~/.claude/projects/");
        m.insert("docs.scopes_priority_project_keyed", "Chave por projeto");
        m.insert("docs.scopes_priority_local_project_global", "Local > Projeto > Global");
        m.insert("docs.scopes_priority_merged", "Combinado");
        m.insert("docs.scopes_when_heading", "Quando usar qual?");
        m.insert("docs.scopes_use_global", "Usar Global para");
        m.insert("docs.scopes_global_1", "Prefer\u{ea}ncias pessoais de estilo de codifica\u{e7}\u{e3}o");
        m.insert("docs.scopes_global_2", "Valores predefinidos de linguagem e framework preferidos");
        m.insert("docs.scopes_global_3", "Formato de mensagens de commit");
        m.insert("docs.scopes_global_4", "Configura\u{e7}\u{e3}o de integra\u{e7}\u{e3}o com editor/IDE");
        m.insert("docs.scopes_global_5", "Servidores MCP que usas em todos os projetos");
        m.insert("docs.scopes_use_project", "Usar Projeto para");
        m.insert("docs.scopes_project_1", "Documenta\u{e7}\u{e3}o do stack tecnol\u{f3}gico e restri\u{e7}\u{f5}es");
        m.insert("docs.scopes_project_2", "Conven\u{e7}\u{f5}es de codifica\u{e7}\u{e3}o da equipa");
        m.insert("docs.scopes_project_3", "Regras espec\u{ed}ficas do dom\u{ed}nio (seguran\u{e7}a, conformidade)");
        m.insert("docs.scopes_project_4", "Skills e fluxos de trabalho espec\u{ed}ficos do projeto");
        m.insert("docs.scopes_project_5", "Hooks de CI/CD e automa\u{e7}\u{e3}o");

        // ── Docs: Tips & Best Practices ──
        m.insert("docs.bestpractices_heading", "Dicas e boas pr\u{e1}ticas");
        m.insert("docs.bestpractices_hygiene_heading", "Higiene de configura\u{e7}\u{e3}o");
        m.insert("docs.bestpractices_hygiene_1", "Executa a verifica\u{e7}\u{e3}o de estado do ClaudeAdmin regularmente. Deteta regras duplicadas, listas de permiss\u{f5}es inchadas e ficheiros CLAUDE.md em falta.");
        m.insert("docs.bestpractices_hygiene_2", "N\u{e3}o te repitas: se uma regra existe globalmente, n\u{e3}o a copies no CLAUDE.md do projeto. Usa o sistema de \u{e2}mbitos.");
        m.insert("docs.bestpractices_hygiene_3", "Mant\u{e9}m limpas as listas de permiss\u{f5}es. Com o tempo, o Claude Code acumula centenas de entradas de permitir/negar. Usa a p\u{e1}gina de Permiss\u{f5}es para limp\u{e1}-las.");
        m.insert("docs.bestpractices_tokens_heading", "Efici\u{ea}ncia de tokens");
        m.insert("docs.bestpractices_tokens_1", "Tudo em CLAUDE.md, regras, skills (quando acionados) e as primeiras 200 linhas de MEMORY.md conta contra a tua janela de contexto. S\u{ea} conciso.");
        m.insert("docs.bestpractices_tokens_2", "Move material de refer\u{ea}ncia detalhado para ficheiros de refer\u{ea}ncia de skills ou ficheiros de t\u{f3}picos de mem\u{f3}ria \u{2013} s\u{f3} s\u{e3}o carregados quando necess\u{e1}rio.");
        m.insert("docs.bestpractices_tokens_3", "Usa a p\u{e1}gina de Anal\u{ed}tica para monitorizar o teu uso de tokens em projetos e sess\u{f5}es.");
        m.insert("docs.bestpractices_team_heading", "Colabora\u{e7}\u{e3}o em equipa");
        m.insert("docs.bestpractices_team_1", "Submete .claude/rules/ e .claude/skills/ ao git. Isto partilha conven\u{e7}\u{f5}es em toda a equipa.");
        m.insert("docs.bestpractices_team_2", "Usa .claude/settings.json para configura\u{e7}\u{e3}o de equipa e .claude/settings.local.json para substitui\u{e7}\u{f5}es pessoais.");
        m.insert("docs.bestpractices_team_3", "O CLAUDE.md na raiz do projeto \u{e9} o contrato da tua equipa com o Claude. Trata-o como documenta\u{e7}\u{e3}o \u{2013} revisa as altera\u{e7}\u{f5}es em PRs.");
        m.insert("docs.bestpractices_debug_heading", "Depura\u{e7}\u{e3}o do comportamento do Claude");
        m.insert("docs.bestpractices_debug_1", "Se o Claude ignora uma regra, verifica a p\u{e1}gina de hierarquia de configura\u{e7}\u{e3}o para detetar configura\u{e7}\u{f5}es em conflito entre camadas.");
        m.insert("docs.bestpractices_debug_2", "A mem\u{f3}ria pode causar comportamento inesperado. Revisa as entradas autogeradas \u{2013} o Claude pode ter memorizado uma solu\u{e7}\u{e3}o alternativa em vez da abordagem correta.");
        m.insert("docs.bestpractices_debug_3", "Usa a p\u{e1}gina de Sess\u{f5}es para rever conversas passadas e compreender o que o Claude estava a \u{201c}pensar\u{201d}.");

        // ── Docs: Links ──
        m.insert("docs.links_heading", "Documenta\u{e7}\u{e3}o oficial da Anthropic");
        m.insert("docs.links_text", "Estas liga\u{e7}\u{f5}es apontam para a documenta\u{e7}\u{e3}o oficial mantida pela Anthropic. O ClaudeAdmin est\u{e1} constru\u{ed}do sobre estas especifica\u{e7}\u{f5}es.");
        m.insert("docs.link_overview_title", "Vis\u{e3}o geral do Claude Code");
        m.insert("docs.link_overview_desc", "Primeiros passos, instala\u{e7}\u{e3}o e uso b\u{e1}sico");
        m.insert("docs.link_memory_title", "Mem\u{f3}ria e CLAUDE.md");
        m.insert("docs.link_memory_desc", "Como o Claude armazena e utiliza a mem\u{f3}ria do projeto");
        m.insert("docs.link_skills_title", "Skills");
        m.insert("docs.link_skills_desc", "Criar e gerir skills reutiliz\u{e1}veis");
        m.insert("docs.link_settings_title", "Configura\u{e7}\u{e3}o");
        m.insert("docs.link_settings_desc", "Hierarquia de configura\u{e7}\u{e3}o e op\u{e7}\u{f5}es");
        m.insert("docs.link_hooks_title", "Hooks");
        m.insert("docs.link_hooks_desc", "Automa\u{e7}\u{e3}o baseada em eventos com comandos de shell");
        m.insert("docs.link_mcp_title", "Servidores MCP");
        m.insert("docs.link_mcp_desc", "Estender o Claude com ferramentas externas");
        m.insert("docs.link_bestpractices_title", "Boas pr\u{e1}ticas");
        m.insert("docs.link_bestpractices_desc", "Dicas para um uso eficaz do Claude Code");
        m.insert("docs.link_mcp_spec_title", "Especifica\u{e7}\u{e3}o MCP");
        m.insert("docs.link_mcp_spec_desc", "O padr\u{e3}o do Protocolo de Contexto de Modelo");

        // ── Licenses ──
        m.insert("sidebar.licenses", "Licen\u{00e7}as");
        m.insert("licenses.title", "Licen\u{00e7}as");
        m.insert("licenses.subtitle", "Licen\u{00e7}as open source e depend\u{00ea}ncias");
        m.insert("licenses.own_license", "Licen\u{00e7}a ClaudeAdmin");
        m.insert("licenses.third_party", "Depend\u{00ea}ncias de terceiros");
        m.insert("licenses.col_name", "Crate");
        m.insert("licenses.col_version", "Vers\u{00e3}o");
        m.insert("licenses.col_license", "Licen\u{00e7}a");
        m.insert("licenses.search_placeholder", "Pesquisar depend\u{00ea}ncias...");
        m.insert("licenses.loading", "Carregando licen\u{00e7}as");
        m.insert("licenses.count", "depend\u{00ea}ncias");
        m.insert("licenses.mit_copyright", "Copyright (c) 2024-2026");
        m.insert("licenses.mit_line1", "A permissão é concedida, gratuitamente, a qualquer pessoa que obtenha uma cópia deste software e dos arquivos de documentação associados (o \u{201c}Software\u{201d}), para lidar com o Software sem restrição, incluindo sem limitação os direitos de usar, copiar, modificar, mesclar, publicar, distribuir, sublicenciar e/ou vender cópias do Software, e permitir que as pessoas a quem o Software é fornecido o façam, sujeito às seguintes condições:");
        m.insert("licenses.mit_line2", "O aviso de copyright acima e este aviso de permissão devem ser incluídos em todas as cópias ou partes substanciais do Software.");
        m.insert("licenses.mit_line3", "O SOFTWARE É FORNECIDO \u{201c}COMO ESTÁ\u{201d}, SEM GARANTIA DE QUALQUER TIPO, EXPRESSA OU IMPLÍCITA, INCLUINDO, MAS NÃO SE LIMITANDO ÀS GARANTIAS DE COMERCIALIZAÇÃO, ADEQUAÇÃO A UM DETERMINADO FIM E NÃO VIOLAÇÃO. EM NENHUM CASO OS AUTORES OU TITULARES DE DIREITOS AUTORAIS SERÃO RESPONSÁVEIS POR QUALQUER RECLAMAÇÃO, DANO OU OUTRA RESPONSABILIDADE, SEJA EM UMA AÇÃO CONTRATUAL, ATO ILÍCITO OU OUTRO, DECORRENTE DE, OU EM CONEXÃO COM O SOFTWARE OU O USO OU OUTRAS TRANSAÇÕES NO SOFTWARE.");
        m.insert("licenses.direct_deps", "Dependências diretas");
        m.insert("licenses.transitive_deps", "Dependências transitivas");
        m.insert("licenses.overview", "Visão geral das licenças");
        m.insert("licenses.direct_count", "diretas");
        m.insert("licenses.transitive_count", "dependências transitivas");

        // ── Components ──
        m.insert("component.modal.close", "Fechar");
        m.insert("component.editor.save", "Guardar");
        m.insert("component.editor.saved", "Guardado!");
        m.insert("component.json_editor.valid", "JSON v\u{e1}lido");
        m.insert("component.json_editor.invalid", "JSON inv\u{e1}lido");
        m.insert("component.frontmatter.description", "Descri\u{e7}\u{e3}o");
        m.insert("component.frontmatter.user_invocable", "Invoc\u{e1}vel pelo utilizador");
        m.insert("component.advisor.title", "Consultor de projeto");
        m.insert("component.advisor.analyze", "Analisar");
        m.insert("component.advisor.analyzing", "A analisar...");
        m.insert("component.advisor.no_api_key", "Nenhuma ANTHROPIC_API_KEY configurada");
        m.insert("component.advisor.error", "Erro ao carregar recomenda\u{e7}\u{f5}es");
        m.insert("component.advisor.summary", "Resumo");
        m.insert("component.advisor.recommendations", "Recomenda\u{e7}\u{f5}es");
        m.insert("component.advisor.apply", "Aplicar");
        m.insert("component.advisor.applied", "Pronto!");
        m.insert("component.advisor.analyze_project", "Analisar projeto");
        m.insert("component.advisor.hint", "O Claude analisa o teu projeto e fornece recomenda\u{e7}\u{f5}es");
        m.insert("component.advisor.loading", "O Claude est\u{e1} a analisar o teu projeto");
        m.insert("component.advisor.assessment", "Avalia\u{e7}\u{e3}o do projeto");
        m.insert("component.advisor.show_preview", "Mostrar pr\u{e9}-visualiza\u{e7}\u{e3}o");
        m.insert("component.advisor.category_tip", "Dica");
        m.insert("component.frontmatter.user_invocable_label", "Invoc\u{e1}vel pelo utilizador (pode ser chamado com /comando)");
        m.insert("component.editor.saving", "A guardar...");

        // ── Common ──
        m.insert("common.error", "Erro");
        m.insert("common.loading", "A carregar");
        m.insert("common.save", "Guardar");
        m.insert("common.delete", "Eliminar");
        m.insert("common.cancel", "Cancelar");
        m.insert("common.close", "Fechar");
        m.insert("common.yes", "Sim");
        m.insert("common.no", "N\u{e3}o");
        m.insert("common.ok", "OK");
        m.insert("common.error_prefix", "Erro: ");
        m.insert("common.invalid_json", "JSON inv\u{e1}lido: ");

        m
    })
}
