use std::collections::HashMap;
use std::sync::OnceLock;

static TRANSLATIONS: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();

pub fn translations() -> &'static HashMap<&'static str, &'static str> {
    TRANSLATIONS.get_or_init(|| {
        let mut m = HashMap::new();

        // ── App ──
        m.insert("app.title", "ClaudeAdmin");
        m.insert("app.subtitle", "Yap\u{0131}land\u{0131}rma Y\u{00f6}neticisi");

        // ── Sidebar ──
        m.insert("sidebar.overview", "Genel Bak\u{0131}\u{015f}");
        m.insert("sidebar.dashboard", "Pano");
        m.insert("sidebar.analytics", "Analitik");
        m.insert("sidebar.manage", "Y\u{00f6}net");
        m.insert("sidebar.projects", "Projeler");
        m.insert("sidebar.global_skills", "Global Skills");
        m.insert("sidebar.skill_browser", "Skill Taray\u{0131}c\u{0131}s\u{0131}");
        m.insert("sidebar.global_rules", "Global Kurallar");
        m.insert("sidebar.plans", "Planlar");
        m.insert("sidebar.mcp_servers", "MCP Sunucular\u{0131}");
        m.insert("sidebar.mcp_browser", "MCP Taray\u{0131}c\u{0131}s\u{0131}");
        m.insert("sidebar.security", "G\u{00fc}venlik");
        m.insert("sidebar.permissions", "\u{0130}zinler");
        m.insert("sidebar.config_health", "Yap\u{0131}land\u{0131}rma Sa\u{011f}l\u{0131}\u{011f}\u{0131}");
        m.insert("sidebar.system", "Sistem");
        m.insert("sidebar.settings", "Ayarlar");
        m.insert("sidebar.sessions", "Oturumlar");
        m.insert("sidebar.github", "GitHub");
        m.insert("sidebar.learn", "\u{00d6}\u{011f}ren");
        m.insert("sidebar.docs", "Dok\u{00fc}mantasyon");
        m.insert("sidebar.help", "Sistem Bilgisi");

        // ── Dashboard ──
        m.insert("dashboard.title", "Pano");
        m.insert("dashboard.subtitle", "Claude Code yap\u{0131}land\u{0131}rman\u{0131}za genel bak\u{0131}\u{015f}");
        m.insert("dashboard.projects", "Projeler");
        m.insert("dashboard.global_skills", "Global Skills");
        m.insert("dashboard.global_rules", "Global Kurallar");
        m.insert("dashboard.mcp_servers", "MCP Sunucular\u{0131}");
        m.insert("dashboard.plans", "Planlar");
        m.insert("dashboard.config_health", "Yap\u{0131}land\u{0131}rma Sa\u{011f}l\u{0131}\u{011f}\u{0131}");
        m.insert("dashboard.recent_projects", "Son Projeler");
        m.insert("dashboard.loading", "Y\u{00fc}kleniyor");
        m.insert("dashboard.error_loading", "Pano y\u{00fc}klenirken hata olu\u{015f}tu");
        m.insert("dashboard.col_name", "Ad");
        m.insert("dashboard.col_claude_md", "CLAUDE.md");
        m.insert("dashboard.col_skills", "Skills");
        m.insert("dashboard.col_rules", "Kurallar");
        m.insert("dashboard.col_memory", "Haf\u{0131}za");
        m.insert("dashboard.yes", "Evet");

        // ── MCP ──
        m.insert("mcp.title", "MCP Sunucular\u{0131}");
        m.insert("mcp.subtitle", "Claude Code i\u{00e7}in Model Context Protocol sunucular\u{0131}n\u{0131} y\u{00f6}netin");
        m.insert("mcp.tab_servers", "Sunucular");
        m.insert("mcp.tab_health", "Sa\u{011f}l\u{0131}k Kontrol\u{00fc}");
        m.insert("mcp.tab_add", "Yeni Sunucu");
        m.insert("mcp.loading", "MCP sunucular\u{0131} y\u{00fc}kleniyor");
        m.insert("mcp.no_servers", "MCP sunucusu yap\u{0131}land\u{0131}r\u{0131}lmam\u{0131}\u{015f}");
        m.insert("mcp.no_servers_hint", "'Yeni Sunucu' sekmesini veya MCP Taray\u{0131}c\u{0131}s\u{0131}n\u{0131} kullanarak sunucu ekleyin.");
        m.insert("mcp.select_server", "Yap\u{0131}land\u{0131}rmas\u{0131}n\u{0131} g\u{00f6}r\u{00fc}nt\u{00fc}lemek ve d\u{00fc}zenlemek i\u{00e7}in listeden bir sunucu se\u{00e7}in.");
        m.insert("mcp.no_servers_configured", "Yap\u{0131}land\u{0131}r\u{0131}lm\u{0131}\u{015f} sunucu yok.");
        m.insert("mcp.check_health", "Sa\u{011f}l\u{0131}k Kontrol\u{00fc}");
        m.insert("mcp.save", "Kaydet");
        m.insert("mcp.delete", "Sil");
        m.insert("mcp.saved", "Kaydedildi!");
        m.insert("mcp.deleted", "Silindi!");
        m.insert("mcp.read_only", "Salt okunur");
        m.insert("mcp.read_only_hint", "Bu sunucu harici olarak y\u{00f6}netiliyor ve burada d\u{00fc}zenlenemez.");
        m.insert("mcp.health.title", "MCP Sunucu Sa\u{011f}l\u{0131}\u{011f}\u{0131}");
        m.insert("mcp.health.check_all", "T\u{00fc}m Sunucular\u{0131} Kontrol Et");
        m.insert("mcp.health.checking", "Kontrol ediliyor...");
        m.insert("mcp.health.description", "Her MCP sunucu s\u{00fc}recini ba\u{015f}lat\u{0131}r, JSON-RPC initialize + tools/list g\u{00f6}nderir ve sonu\u{00e7}lar\u{0131} raporlar. Zaman a\u{015f}\u{0131}m\u{0131}: sunucu ba\u{015f}\u{0131}na 10 saniye.");
        m.insert("mcp.health.col_name", "Ad");
        m.insert("mcp.health.col_source", "Kaynak");
        m.insert("mcp.health.col_status", "Durum");
        m.insert("mcp.health.col_server_info", "Sunucu Bilgisi");
        m.insert("mcp.health.col_tools", "Ara\u{00e7}lar");
        m.insert("mcp.health.col_duration", "S\u{00fc}re");
        m.insert("mcp.health.running", "\u{00c7}al\u{0131}\u{015f}\u{0131}yor");
        m.insert("mcp.health.error", "Hata");
        m.insert("mcp.health.timeout", "Zaman A\u{015f}\u{0131}m\u{0131}");
        m.insert("mcp.health.unknown", "Bilinmiyor");
        m.insert("mcp.add.title", "MCP Sunucusu Ekle");
        m.insert("mcp.add.description", "Global ~/.claude.json yap\u{0131}land\u{0131}rman\u{0131}za yeni bir MCP sunucusu ekleyin.");
        m.insert("mcp.add.name_label", "Sunucu Ad\u{0131}");
        m.insert("mcp.add.name_placeholder", "\u{00f6}rn. my-server");
        m.insert("mcp.add.config_label", "Sunucu Yap\u{0131}land\u{0131}rmas\u{0131} (JSON)");
        m.insert("mcp.add.submit", "Sunucu Ekle");
        m.insert("mcp.add.name_required", "L\u{00fc}tfen bir sunucu ad\u{0131} girin");
        m.insert("mcp.source.claude_code", "Claude Code");
        m.insert("mcp.source.claude_desktop", "Claude Desktop");

        // ── MCP Browser ──
        m.insert("mcp_browser.title", "MCP Taray\u{0131}c\u{0131}s\u{0131}");
        m.insert("mcp_browser.subtitle", "Claude Code i\u{00e7}in MCP sunucular\u{0131}n\u{0131} ke\u{015f}fedin ve kurun");
        m.insert("mcp_browser.search_placeholder", "MCP sunucular\u{0131}n\u{0131} ara...");
        m.insert("mcp_browser.loading", "MCP katalo\u{011f}u y\u{00fc}kleniyor");
        m.insert("mcp_browser.no_results", "MCP sunucusu bulunamad\u{0131}");
        m.insert("mcp_browser.installed", "Kurulu");
        m.insert("mcp_browser.install", "Kur");
        m.insert("mcp_browser.needs_api_key", "API Anahtar\u{0131} Gerekli");
        m.insert("mcp_browser.install_success", "ba\u{015f}ar\u{0131}yla kuruldu!");
        m.insert("mcp_browser.install_failed", "Kurulum ba\u{015f}ar\u{0131}s\u{0131}z");

        // ── Projects ──
        m.insert("projects.title", "Projeler");
        m.insert("projects.subtitle", "~/.claude.json dosyas\u{0131}nda kay\u{0131}tl\u{0131} t\u{00fc}m projeler");
        m.insert("projects.loading", "Y\u{00fc}kleniyor");
        m.insert("projects.error_loading", "Projeler y\u{00fc}klenirken hata: ");
        m.insert("projects.col_name", "Ad");
        m.insert("projects.col_path", "Yol");
        m.insert("projects.col_claude_md", "CLAUDE.md");
        m.insert("projects.col_skills", "Skills");
        m.insert("projects.col_rules", "Kurallar");
        m.insert("projects.col_memory", "Haf\u{0131}za");
        m.insert("projects.yes", "Evet");

        // ── Project Detail ──
        m.insert("project_detail.loading", "Proje detaylar\u{0131} y\u{00fc}kleniyor");
        m.insert("project_detail.error_loading", "Proje y\u{00fc}klenirken hata");
        m.insert("project_detail.tab_advisor", "Dan\u{0131}\u{015f}man");
        m.insert("project_detail.tab_claude_md", "CLAUDE.md");
        m.insert("project_detail.tab_skills", "Skills");
        m.insert("project_detail.tab_rules", "Kurallar");
        m.insert("project_detail.tab_memory", "Haf\u{0131}za");
        m.insert("project_detail.tab_permissions", "\u{0130}zinler");
        m.insert("project_detail.tab_health", "Sa\u{011f}l\u{0131}k");
        m.insert("project_detail.no_claude_md", "CLAUDE.md bulunamad\u{0131}");
        m.insert("project_detail.no_claude_md_hint", "Claude Code\u{2019}a talimatlar vermek i\u{00e7}in proje dizininizde bir CLAUDE.md olu\u{015f}turun.");
        m.insert("project_detail.no_skills", "Bu proje i\u{00e7}in Skill yok");
        m.insert("project_detail.no_rules", "Bu proje i\u{00e7}in kural yok");
        m.insert("project_detail.no_memory", "Bu proje i\u{00e7}in haf\u{0131}za yok");
        m.insert("project_detail.save", "Kaydet");
        m.insert("project_detail.saved", "Kaydedildi!");
        m.insert("project_detail.skill_scope", "Kapsam");
        m.insert("project_detail.permissions_loading", "\u{0130}zinler y\u{00fc}kleniyor...");
        m.insert("project_detail.permissions_error", "\u{0130}zinler y\u{00fc}klenirken hata");
        m.insert("project_detail.permissions_entries", "Kay\u{0131}tlar");
        m.insert("project_detail.permissions_col_tool", "Ara\u{00e7}");
        m.insert("project_detail.permissions_col_command", "Komut");
        m.insert("project_detail.permissions_no_entries", "\u{0130}zin kayd\u{0131} yok");
        m.insert("project_detail.health_loading", "Sa\u{011f}l\u{0131}k hesaplan\u{0131}yor...");
        m.insert("project_detail.health_error", "Sa\u{011f}l\u{0131}k verileri y\u{00fc}klenirken hata");
        m.insert("project_detail.health_score", "Sa\u{011f}l\u{0131}k Puan\u{0131}");
        m.insert("project_detail.health_claude_md", "CLAUDE.md mevcut");
        m.insert("project_detail.health_memory", "Haf\u{0131}za mevcut");
        m.insert("project_detail.health_permissions", "\u{0130}zinler");
        m.insert("project_detail.health_security_issues", "G\u{00fc}venlik sorunlar\u{0131}");
        m.insert("project_detail.health_duplicated_rules", "Yinelenen kurallar");
        m.insert("project_detail.health_no_security_issues", "G\u{00fc}venlik sorunu bulunamad\u{0131}");
        m.insert("project_detail.health_col_text", "Metin");
        m.insert("project_detail.health_col_found_in", "Bulundu\u{011f}u Yer");
        m.insert("project_detail.health_col_also_in", "Ayr\u{0131}ca Bulunan");
        m.insert("project_detail.health_permission_entries", "\u{0130}zin Kay\u{0131}tlar\u{0131}");
        m.insert("project_detail.permissions_col_index", "#");
        m.insert("project_detail.permissions_col_status", "Durum");
        m.insert("project_detail.permissions_fragment", "Par\u{00e7}a");
        m.insert("project_detail.permissions_ok", "Tamam");
        m.insert("project_detail.permissions_security_warnings", "g\u{00fc}venlik uyar\u{0131}s\u{0131}");
        m.insert("project_detail.permissions_manage", "\u{0130}zinleri Y\u{00f6}net");
        m.insert("project_detail.advisor_analyze", "Projeyi analiz et");
        m.insert("project_detail.advisor_analyzing", "Analiz ediliyor...");
        m.insert("project_detail.advisor_description", "Claude projenizi analiz eder ve \u{00f6}neriler sunar");
        m.insert("project_detail.advisor_loading", "Claude projenizi analiz ediyor");
        m.insert("project_detail.advisor_summary", "Proje De\u{011f}erlendirmesi");
        m.insert("project_detail.advisor_done", "Tamamland\u{0131}!");
        m.insert("project_detail.advisor_preview", "\u{00d6}nizlemeyi g\u{00f6}ster");
        m.insert("project_detail.advisor_category_tip", "\u{0130}pucu");
        m.insert("project_detail.skills_col_name", "Ad");
        m.insert("project_detail.skills_col_description", "A\u{00e7}\u{0131}klama");
        m.insert("project_detail.skills_col_invocable", "\u{00c7}a\u{011f}r\u{0131}labilir");
        m.insert("project_detail.rules_col_name", "Ad");
        m.insert("project_detail.rules_col_path", "Yol");
        m.insert("project_detail.memory_col_file", "Dosya");
        m.insert("project_detail.memory_col_size", "Boyut");
        m.insert("project_detail.bytes", "bayt");
        m.insert("project_detail.unknown_tab", "Bilinmeyen sekme");

        // ── Global Skills ──
        m.insert("global_skills.title", "Global Skills");
        m.insert("global_skills.subtitle", "~/.claude/skills/ i\u{00e7}indeki Skills\u{2019}leri y\u{00f6}netin");
        m.insert("global_skills.loading", "Skills y\u{00fc}kleniyor");
        m.insert("global_skills.no_skills", "Global Skill bulunamad\u{0131}");
        m.insert("global_skills.no_skills_hint", "~/.claude/skills/ i\u{00e7}inde Skills olu\u{015f}turun veya Skill Taray\u{0131}c\u{0131}s\u{0131}n\u{0131} kullan\u{0131}n.");
        m.insert("global_skills.select_skill", "Listeden bir Skill se\u{00e7}in.");
        m.insert("global_skills.col_skill", "Skill");
        m.insert("global_skills.col_invocable", "\u{00c7}a\u{011f}r\u{0131}labilir");
        m.insert("global_skills.invocable", "\u{00c7}a\u{011f}r\u{0131}labilir");
        m.insert("global_skills.not_invocable", "\u{00c7}a\u{011f}r\u{0131}lamaz");
        m.insert("global_skills.editing", "D\u{00fc}zenleniyor:");
        m.insert("global_skills.save", "Kaydet");
        m.insert("global_skills.saved", "Kaydedildi!");
        m.insert("global_skills.delete", "Sil");
        m.insert("global_skills.deleted", "Silindi!");

        // ── Global Rules ──
        m.insert("global_rules.title", "Global Kurallar");
        m.insert("global_rules.subtitle", "~/.claude/rules/ i\u{00e7}indeki kurallar\u{0131} y\u{00f6}netin");
        m.insert("global_rules.loading", "Kurallar y\u{00fc}kleniyor");
        m.insert("global_rules.no_rules", "Global kural bulunamad\u{0131}");
        m.insert("global_rules.no_rules_hint", "~/.claude/rules/ i\u{00e7}inde .md dosyalar\u{0131} olu\u{015f}turun");
        m.insert("global_rules.select_rule", "Listeden bir kural se\u{00e7}in.");
        m.insert("global_rules.col_rule", "Kural");
        m.insert("global_rules.editing", "D\u{00fc}zenleniyor:");
        m.insert("global_rules.save", "Kaydet");
        m.insert("global_rules.saved", "Kaydedildi!");
        m.insert("global_rules.delete", "Sil");
        m.insert("global_rules.deleted", "Silindi!");

        // ── Plans ──
        m.insert("plans.title", "Planlar");
        m.insert("plans.subtitle", "~/.claude/plans/ i\u{00e7}indeki plan dosyalar\u{0131}n\u{0131} y\u{00f6}netin");
        m.insert("plans.loading", "Planlar y\u{00fc}kleniyor");
        m.insert("plans.no_plans", "Plan bulunamad\u{0131}");
        m.insert("plans.no_plans_hint", "Planlar, Claude Code taraf\u{0131}ndan planlama s\u{0131}ras\u{0131}nda olu\u{015f}turulur.");
        m.insert("plans.select_plan", "Listeden bir plan se\u{00e7}in.");
        m.insert("plans.col_plan", "Plan");
        m.insert("plans.col_modified", "De\u{011f}i\u{015f}tirilme");
        m.insert("plans.modified", "De\u{011f}i\u{015f}tirilme");
        m.insert("plans.plan_label", "Plan:");
        m.insert("plans.save", "Kaydet");
        m.insert("plans.saved", "Kaydedildi!");
        m.insert("plans.delete", "Sil");
        m.insert("plans.deleted", "Silindi!");

        // ── Settings ──
        m.insert("settings.title", "Ayarlar");
        m.insert("settings.subtitle", "Claude Code ayarlar\u{0131}n\u{0131} ve Hooks\u{2019}lar\u{0131} y\u{00f6}netin");
        m.insert("settings.tab_overview", "Genel Bak\u{0131}\u{015f}");
        m.insert("settings.tab_hooks", "Hook \u{015e}ablonlar\u{0131}");
        m.insert("settings.tab_storage", "Depolama");
        m.insert("settings.loading", "Ayarlar y\u{00fc}kleniyor");
        m.insert("settings.hooks_title", "Hooks");
        m.insert("settings.no_hooks", "Hooks yap\u{0131}land\u{0131}r\u{0131}lmam\u{0131}\u{015f}");
        m.insert("settings.pre_tool_use", "PreToolUse");
        m.insert("settings.post_tool_use", "PostToolUse");
        m.insert("settings.matcher", "E\u{015f}le\u{015f}tirici");
        m.insert("settings.command", "Komut");
        m.insert("settings.hook_templates_title", "Hook \u{015e}ablonlar\u{0131}");
        m.insert("settings.hook_templates_desc", "Eklenebilir haz\u{0131}r Hook yap\u{0131}land\u{0131}rmalar\u{0131}.");
        m.insert("settings.hook_templates_loading", "\u{015e}ablonlar y\u{00fc}kleniyor");
        m.insert("settings.add_hook", "Ekle");
        m.insert("settings.storage_title", "Depolama Kullan\u{0131}m\u{0131}");
        m.insert("settings.storage_loading", "Depolama hesaplan\u{0131}yor");
        m.insert("settings.storage_total", "Toplam");
        m.insert("settings.storage_dir", "Dizin");
        m.insert("settings.storage_size", "Boyut");

        // ── Permissions ──
        m.insert("permissions.title", "\u{0130}zinler");
        m.insert("permissions.subtitle", "Proje izinlerini inceleyin ve y\u{00f6}netin");
        m.insert("permissions.loading", "\u{0130}zinler y\u{00fc}kleniyor");
        m.insert("permissions.no_permissions", "\u{0130}zin bulunamad\u{0131}");
        m.insert("permissions.col_project", "Proje");
        m.insert("permissions.col_entries", "Kay\u{0131}tlar");
        m.insert("permissions.col_issues", "Sorunlar");
        m.insert("permissions.col_fragmented", "Par\u{00e7}alanm\u{0131}\u{015f}");
        m.insert("permissions.detail_title", "\u{0130}zinler");
        m.insert("permissions.detail_loading", "\u{0130}zinler y\u{00fc}kleniyor");
        m.insert("permissions.detail_col_tool", "Ara\u{00e7}");
        m.insert("permissions.detail_col_command", "Komut");
        m.insert("permissions.detail_col_status", "Durum");
        m.insert("permissions.detail_fragmented", "Par\u{00e7}alanm\u{0131}\u{015f}");
        m.insert("permissions.detail_security_issue", "G\u{00fc}venlik Sorunu");
        m.insert("permissions.detail_delete_selected", "Se\u{00e7}ilenleri Sil");
        m.insert("permissions.detail_deleted", "Silindi!");
        m.insert("permissions.detail_warnings_title", "G\u{00fc}venlik Uyar\u{0131}lar\u{0131}");
        m.insert("permissions.health_title", "Yap\u{0131}land\u{0131}rma Sa\u{011f}l\u{0131}\u{011f}\u{0131}");
        m.insert("permissions.health_subtitle", "T\u{00fc}m projelerin sa\u{011f}l\u{0131}k durumu");
        m.insert("permissions.health_loading", "Sa\u{011f}l\u{0131}k hesaplan\u{0131}yor");
        m.insert("permissions.health_col_project", "Proje");
        m.insert("permissions.health_col_score", "Puan");
        m.insert("permissions.health_col_issues", "Sorunlar");
        m.insert("permissions.health_avg", "Ortalama");
        m.insert("permissions.subtitle_manage", "T\u{00fc}m projelerde izin beyaz listelerini y\u{00f6}netin");
        m.insert("permissions.col_actions", "\u{0130}\u{015f}lemler");
        m.insert("permissions.col_security_issues", "G\u{00fc}venlik Sorunlar\u{0131}");
        m.insert("permissions.details", "Detaylar");
        m.insert("permissions.detail_subtitle", "\u{0130}zin kay\u{0131}tlar\u{0131}n\u{0131} inceleyin ve temizleyin");
        m.insert("permissions.detail_deleting", "Siliniyor...");
        m.insert("permissions.detail_deleted_reloading", "Silindi! Yeniden y\u{00fc}kleniyor...");
        m.insert("permissions.detail_delete_count", "Se\u{00e7}ilenleri Sil");
        m.insert("permissions.detail_col_index", "#");
        m.insert("permissions.detail_fragment", "Par\u{00e7}a");
        m.insert("permissions.detail_ok", "Tamam");
        m.insert("permissions.detail_warnings_count", "G\u{00fc}venlik Uyar\u{0131}lar\u{0131}");
        m.insert("permissions.detail_entry", "kay\u{0131}t");
        m.insert("permissions.health_subtitle_scores", "T\u{00fc}m projelerde yap\u{0131}land\u{0131}rma sa\u{011f}l\u{0131}k puanlar\u{0131}");
        m.insert("permissions.health_avg_score", "Ortalama Sa\u{011f}l\u{0131}k Puan\u{0131}");
        m.insert("permissions.health_projects_analyzed", "Analiz Edilen Projeler");
        m.insert("permissions.health_no_issues", "Sorun yok");

        // ── Analytics ──
        m.insert("analytics.title", "Analitik");
        m.insert("analytics.subtitle", "Claude Code kullan\u{0131}m istatistikleri");
        m.insert("analytics.loading", "Analitik verileri y\u{00fc}kleniyor");
        m.insert("analytics.error_loading", "Analitik verileri y\u{00fc}klenirken hata");
        m.insert("analytics.total_sessions", "Toplam Oturum");
        m.insert("analytics.total_messages", "Toplam Mesaj");
        m.insert("analytics.git_commits", "Git Commit\u{2019}leri");
        m.insert("analytics.lines_added", "Eklenen Sat\u{0131}rlar");
        m.insert("analytics.lines_removed", "Silinen Sat\u{0131}rlar");
        m.insert("analytics.since", "ba\u{015f}lang\u{0131}\u{00e7}");
        m.insert("analytics.activity_heatmap", "Aktivite Is\u{0131} Haritas\u{0131}");
        m.insert("analytics.messages", "Mesajlar");
        m.insert("analytics.sessions", "Oturumlar");
        m.insert("analytics.tool_calls", "Ara\u{00e7} \u{00c7}a\u{011f}r\u{0131}lar\u{0131}");
        m.insert("analytics.hourly_distribution", "Saatlik Da\u{011f}\u{0131}l\u{0131}m");
        m.insert("analytics.model_usage", "Model Kullan\u{0131}m\u{0131}");
        m.insert("analytics.col_model", "Model");
        m.insert("analytics.col_input_tokens", "Giri\u{015f} Token");
        m.insert("analytics.col_output_tokens", "\u{00c7}\u{0131}k\u{0131}\u{015f} Token");
        m.insert("analytics.col_cache_tokens", "\u{00d6}nbellek Token");
        m.insert("analytics.tool_ranking", "Ara\u{00e7} S\u{0131}ralamas\u{0131}");
        m.insert("analytics.col_cache_read", "\u{00d6}nbellek Okuma");
        m.insert("analytics.tool_usage_top10", "Ara\u{00e7} Kullan\u{0131}m\u{0131} (\u{0130}lk 10)");
        m.insert("analytics.languages", "Programlama Dilleri");
        m.insert("analytics.session_outcomes", "Oturum Sonu\u{00e7}lar\u{0131}");
        m.insert("analytics.outcomes", "Sonu\u{00e7}lar");

        // ── Sessions ──
        m.insert("sessions.title", "Oturumlar");
        m.insert("sessions.subtitle", "Claude Code oturum ge\u{00e7}mi\u{015f}ini g\u{00f6}z at\u{0131}n");
        m.insert("sessions.loading", "Oturumlar y\u{00fc}kleniyor");
        m.insert("sessions.search_placeholder", "Oturumlar\u{0131} ara...");
        m.insert("sessions.no_sessions", "Oturum bulunamad\u{0131}");
        m.insert("sessions.col_project", "Proje");
        m.insert("sessions.col_date", "Tarih");
        m.insert("sessions.col_duration", "S\u{00fc}re");
        m.insert("sessions.col_messages", "Mesajlar");
        m.insert("sessions.col_summary", "\u{00d6}zet");
        m.insert("sessions.col_outcome", "Sonu\u{00e7}");
        m.insert("sessions.minutes", "dk");
        m.insert("sessions.load_more", "Daha Fazla Y\u{00fc}kle");
        m.insert("sessions.detail_title", "Oturum Detaylar\u{0131}");
        m.insert("sessions.detail_loading", "Oturum y\u{00fc}kleniyor");
        m.insert("sessions.detail_project", "Proje");
        m.insert("sessions.detail_start", "Ba\u{015f}lang\u{0131}\u{00e7}");
        m.insert("sessions.detail_duration", "S\u{00fc}re");
        m.insert("sessions.detail_messages", "Mesajlar");
        m.insert("sessions.detail_tools", "Ara\u{00e7} \u{00c7}a\u{011f}r\u{0131}lar\u{0131}");
        m.insert("sessions.detail_tokens", "Token");
        m.insert("sessions.detail_git", "Git");
        m.insert("sessions.detail_first_prompt", "\u{0130}lk \u{0130}stem");
        m.insert("sessions.detail_summary", "\u{00d6}zet");
        m.insert("sessions.back", "Geri");
        m.insert("sessions.searching", "Aran\u{0131}yor...");
        m.insert("sessions.search", "Ara");
        m.insert("sessions.clear", "Temizle");
        m.insert("sessions.search_results", "Arama Sonu\u{00e7}lar\u{0131}");
        m.insert("sessions.no_results", "Sonu\u{00e7} bulunamad\u{0131}");
        m.insert("sessions.col_prompt", "\u{0130}stem");
        m.insert("sessions.session_prefix", "Oturum: ");
        m.insert("sessions.detail_start_time", "Ba\u{015f}lang\u{0131}\u{00e7} Zaman\u{0131}");
        m.insert("sessions.user_messages", " kullan\u{0131}c\u{0131} / ");
        m.insert("sessions.assistant_messages", " asistan");
        m.insert("sessions.tokens_in", " giri\u{015f} / ");
        m.insert("sessions.tokens_out", " \u{00e7}\u{0131}k\u{0131}\u{015f}");
        m.insert("sessions.commits_label", " commit, +");
        m.insert("sessions.lines_minus", " / -");
        m.insert("sessions.tools_used", "Kullan\u{0131}lan Ara\u{00e7}lar");
        m.insert("sessions.outcome_prefix", "Sonu\u{00e7}: ");
        m.insert("sessions.showing", "G\u{00f6}sterilen");
        m.insert("sessions.of", "/");
        m.insert("sessions.previous", "\u{00d6}nceki");
        m.insert("sessions.next", "Sonraki");

        // ── GitHub ──
        m.insert("github.title", "GitHub");
        m.insert("github.subtitle", "GitHub Entegrasyon Durumu");
        m.insert("github.loading", "GitHub verileri y\u{00fc}kleniyor");
        m.insert("github.auth_status", "Kimlik Do\u{011f}rulama Durumu");
        m.insert("github.username", "Kullan\u{0131}c\u{0131} Ad\u{0131}");
        m.insert("github.linked_repos", "Ba\u{011f}l\u{0131} Depolar");
        m.insert("github.no_repos", "Ba\u{011f}l\u{0131} depo yok");
        m.insert("github.col_repo", "Depo");
        m.insert("github.col_recent_commits", "Son Commit\u{2019}ler");
        m.insert("github.col_open_prs", "A\u{00e7}\u{0131}k PR\u{2019}ler");

        // ── Help / System Info ──
        m.insert("help.title", "Sistem Bilgisi");
        m.insert("help.subtitle", "Claude Code sistem bilgileri");
        m.insert("help.loading", "Sistem bilgileri y\u{00fc}kleniyor");
        m.insert("help.account", "Hesap");
        m.insert("help.account_name", "Ad");
        m.insert("help.account_email", "E-posta");
        m.insert("help.subscription", "Abonelik");
        m.insert("help.claude_version", "Claude Code S\u{00fc}r\u{00fc}m\u{00fc}");
        m.insert("help.gh_cli", "GitHub CLI");
        m.insert("help.skill_usage", "Skill Kullan\u{0131}m\u{0131}");
        m.insert("help.no_skill_usage", "Skill kullan\u{0131}m\u{0131} kaydedilmemi\u{015f}");
        m.insert("help.col_skill", "Skill");
        m.insert("help.col_count", "Say\u{0131}");
        m.insert("help.what_is_title", "ClaudeAdmin nedir?");
        m.insert("help.what_is_desc", "ClaudeAdmin, Claude Code i\u{00e7}in g\u{00f6}rsel y\u{00f6}netim konsoludur. Claude Code yap\u{0131}land\u{0131}rman\u{0131}z\u{0131}n t\u{00fc}m y\u{00f6}nlerini y\u{00f6}netmek i\u{00e7}in web tabanl\u{0131} bir aray\u{00fc}z sa\u{011f}lar: Projeler, Skills, Kurallar, Haf\u{0131}za, Ayarlar, Hooks, MCP Sunucular\u{0131} ve Planlar.");
        m.insert("help.system_status", "Sistem Durumu");
        m.insert("help.not_set", "Ayarlanmam\u{0131}\u{015f}");
        m.insert("help.unknown", "Bilinmiyor");
        m.insert("help.not_found", "Bulunamad\u{0131}");
        m.insert("help.not_installed", "Kurulmam\u{0131}\u{015f}");
        m.insert("help.concepts_title", "Claude Code Kavramlar\u{0131}");
        m.insert("help.concept_skills", "YAML \u{00f6}n bilgi i\u{00e7}eren yeniden kullan\u{0131}labilir istemler. ~/.claude/skills/ (global) veya .claude/skills/ (proje) i\u{00e7}inde SKILL.md dosyalar\u{0131} olarak saklan\u{0131}r.");
        m.insert("help.concept_rules", "Claude\u{2019}un davran\u{0131}\u{015f}\u{0131}n\u{0131} \u{015f}ekillendiren k\u{0131}s\u{0131}tlamalar ve y\u{00f6}nergeler. ~/.claude/rules/ veya proje d\u{00fc}zeyinde .md dosyalar\u{0131} olarak saklan\u{0131}r.");
        m.insert("help.concept_memory", "Proje ba\u{015f}\u{0131}na kal\u{0131}c\u{0131} notlar. MEMORY.md otomatik olarak sistem istemlerine y\u{00fc}klenir. Kal\u{0131}plar\u{0131}, tercihleri ve \u{00f6}\u{011f}renmeleri saklar.");
        m.insert("help.concept_hooks", "Olaylar (PreToolUse, PostToolUse, Stop) taraf\u{0131}ndan tetiklenen kabuk komutlar\u{0131}. Otomatik bi\u{00e7}imlendirme, lint vb. i\u{00e7}in settings.json i\u{00e7}inde yap\u{0131}land\u{0131}r\u{0131}l\u{0131}r.");
        m.insert("help.concept_mcp", "Model Context Protocol sunucular\u{0131}, Claude\u{2019}u harici ara\u{00e7}larla geni\u{015f}letir. ~/.claude.json i\u{00e7}inde command, args ve env ile yap\u{0131}land\u{0131}r\u{0131}l\u{0131}r.");
        m.insert("help.concept_claudemd", "Proje d\u{00fc}zeyinde talimat dosyas\u{0131}. Otomatik olarak ba\u{011f}lam olarak y\u{00fc}klenir. Proje kurallar\u{0131}n\u{0131}, teknoloji y\u{0131}\u{011f}\u{0131}n\u{0131} bilgisini ve kodlama y\u{00f6}nergelerini i\u{00e7}erir.");
        m.insert("help.disclaimer", "ClaudeAdmin ba\u{011f}\u{0131}ms\u{0131}z bir topluluk projesidir. Anthropic ile ba\u{011f}lant\u{0131}l\u{0131} de\u{011f}ildir, Anthropic taraf\u{0131}ndan desteklenmez veya onaylanmaz. Claude ve Claude Code, Anthropic'in ticari markalar\u{0131}d\u{0131}r.");

        m.insert("github.subtitle_detail", "GitHub CLI entegrasyonu ve ba\u{011f}l\u{0131} depolar");
        m.insert("github.linked_repositories", "Ba\u{011f}l\u{0131} Depolar");
        m.insert("github.no_linked_repos", "~/.claude.json i\u{00e7}inde ba\u{011f}l\u{0131} GitHub deposu yok");
        m.insert("github.col_name", "Ad");
        m.insert("github.col_path", "Yol");

        // ── Skill Browser ──
        m.insert("skill_browser.title", "Skill Taray\u{0131}c\u{0131}s\u{0131}");
        m.insert("skill_browser.subtitle", "Resmi ve topluluk Skills\u{2019}lerini ke\u{015f}fedin ve kurun");
        m.insert("skill_browser.loading", "Skills y\u{00fc}kleniyor");
        m.insert("skill_browser.search_placeholder", "Skills ara...");
        m.insert("skill_browser.no_results", "Skill bulunamad\u{0131}");
        m.insert("skill_browser.installed", "Kurulu");
        m.insert("skill_browser.install", "Kur");
        m.insert("skill_browser.official", "Resmi");
        m.insert("skill_browser.community", "Topluluk");
        m.insert("skill_browser.tab_official", "Resmi (Anthropic)");
        m.insert("skill_browser.tab_community", "Topluluk");
        m.insert("skill_browser.install_success", "ba\u{015f}ar\u{0131}yla kuruldu!");
        m.insert("skill_browser.install_failed", "Kurulum ba\u{015f}ar\u{0131}s\u{0131}z:");

        // ── Docs ──
        m.insert("docs.title", "Dok\u{00fc}mantasyon");
        m.insert("docs.subtitle", "Claude Code yap\u{0131}land\u{0131}rmas\u{0131} hakk\u{0131}nda bilmeniz gereken her \u{015f}ey");
        m.insert("docs.loading", "Dok\u{00fc}mantasyon y\u{00fc}kleniyor");

        // ── Docs: Table of Contents ──
        m.insert("docs.toc_contents", "\u{0130}\u{00e7}indekiler");
        m.insert("docs.toc_why_claudeadmin", "Neden ClaudeAdmin?");
        m.insert("docs.toc_capabilities", "Neler yapabilir ve yapamaz");
        m.insert("docs.toc_group", "Kavramlar");
        m.insert("docs.toc_claude_md", "CLAUDE.md");
        m.insert("docs.toc_rules", "Kurallar");
        m.insert("docs.toc_skills", "Skills");
        m.insert("docs.toc_memory", "Haf\u{0131}za");
        m.insert("docs.toc_settings", "Ayarlar ve Hooks");
        m.insert("docs.toc_mcp", "MCP Sunucular\u{0131}");
        m.insert("docs.toc_plans", "Planlar");
        m.insert("docs.toc_scopes", "Global ve Proje Kapsam\u{0131}");
        m.insert("docs.toc_tips", "\u{0130}pu\u{00e7}lar\u{0131} ve En \u{0130}yi Uygulamalar");
        m.insert("docs.toc_links", "Resmi Dok\u{00fc}mantasyon");

        // ── Docs: Shared labels ──
        m.insert("docs.tips_heading", "\u{0130}pu\u{00e7}lar\u{0131} ve P\u{00fc}f Noktalar\u{0131}");
        m.insert("docs.scope_global", "Global");
        m.insert("docs.scope_project", "Proje");
        m.insert("docs.scope_user", "Kullan\u{0131}c\u{0131}");
        m.insert("docs.scope_parent", "\u{00dc}st");
        m.insert("docs.scope_managed", "Y\u{00f6}netilen");
        m.insert("docs.scope_local", "Yerel");

        // ── Docs: Overview ──
        m.insert("docs.overview_heading", "Neden ClaudeAdmin?");
        m.insert("docs.overview_callout", " t\u{00fc}m Claude Code yap\u{0131}land\u{0131}rman\u{0131}z i\u{00e7}in merkezi y\u{00f6}netim konsoludur. D\u{00fc}zinelerce gizli dizindeki manuel dosya d\u{00fc}zenlemeyi tek bir g\u{00f6}rsel aray\u{00fc}zle de\u{011f}i\u{015f}tirir.");
        m.insert("docs.overview_text1", "Claude Code yap\u{0131}land\u{0131}rmas\u{0131}n\u{0131} karma\u{015f}\u{0131}k bir dosya ve dizin hiyerar\u{015f}isinde saklar: proje k\u{00f6}k dizinlerindeki CLAUDE.md dosyalar\u{0131}, ~/.claude/ alt dizinlerine da\u{011f}\u{0131}lm\u{0131}\u{015f} kurallar ve Skills, kodlanm\u{0131}\u{015f} proje yollar\u{0131}na g\u{00f6}re anahtarlanm\u{0131}\u{015f} haf\u{0131}za dosyalar\u{0131}, birden fazla JSON dosyas\u{0131}ndaki ayarlar ve ~/.claude.json i\u{00e7}indeki MCP sunucu yap\u{0131}land\u{0131}rmalar\u{0131}. Projeleriniz b\u{00fc}y\u{00fc}d\u{00fc}k\u{00e7}e t\u{00fc}m bunlar\u{0131} elle y\u{00f6}netmek hataya a\u{00e7}\u{0131}k ve zaman al\u{0131}c\u{0131} hale gelir.");
        m.insert("docs.overview_text2", "ClaudeAdmin size \u{015f}unlar\u{0131} sa\u{011f}lar:");
        m.insert("docs.overview_li_visibility_label", "G\u{00f6}r\u{00fc}n\u{00fc}rl\u{00fc}k");
        m.insert("docs.overview_li_visibility", " \u{2013} T\u{00fc}m projelerinizi, Skills\u{2019}lerinizi, kurallar\u{0131}n\u{0131}z\u{0131} ve haf\u{0131}zan\u{0131}z\u{0131} tek bir yerde g\u{00f6}r\u{00fc}n");
        m.insert("docs.overview_li_editing_label", "D\u{00fc}zenleme");
        m.insert("docs.overview_li_editing", " \u{2013} CLAUDE.md, kurallar, Skills ve haf\u{0131}zay\u{0131} uygun bir edit\u{00f6}rle d\u{00fc}zenleyin");
        m.insert("docs.overview_li_health_label", "Sa\u{011f}l\u{0131}k Kontrolleri");
        m.insert("docs.overview_li_health", " \u{2013} \u{0130}zinlerdeki g\u{00fc}venlik sorunlar\u{0131}n\u{0131}, yinelenen kurallar\u{0131} ve eksik yap\u{0131}land\u{0131}rmalar\u{0131} tespit edin");
        m.insert("docs.overview_li_analytics_label", "Analitik");
        m.insert("docs.overview_li_analytics", " \u{2013} Claude Code\u{2019}u nas\u{0131}l kulland\u{0131}\u{011f}\u{0131}n\u{0131}z\u{0131} anlay\u{0131}n: oturumlar, token\u{2019}lar, ara\u{00e7}lar, maliyetler");
        m.insert("docs.overview_li_advisor_label", "Dan\u{0131}\u{015f}man");
        m.insert("docs.overview_li_advisor", " \u{2013} Proje yap\u{0131}land\u{0131}rman\u{0131}z\u{0131} iyile\u{015f}tirmek i\u{00e7}in yapay zeka destekli \u{00f6}neriler");

        // ── Docs: Capabilities ──
        m.insert("docs.cap_heading", "ClaudeAdmin Neler Yapabilir ve Yapamaz");
        m.insert("docs.cap_can_heading", "Neler yapabilir");
        m.insert("docs.cap_can_1", "~/.claude.json i\u{00e7}inde kay\u{0131}tl\u{0131} t\u{00fc}m projeleri g\u{00f6}z at\u{0131}n ve y\u{00f6}netin");
        m.insert("docs.cap_can_2", "Herhangi bir proje i\u{00e7}in CLAUDE.md dosyalar\u{0131}n\u{0131} g\u{00f6}r\u{00fc}nt\u{00fc}leyin ve d\u{00fc}zenleyin");
        m.insert("docs.cap_can_3", "Global ve proje Skills\u{2019}lerini olu\u{015f}turun, d\u{00fc}zenleyin ve silin");
        m.insert("docs.cap_can_4", "Global ve proje kurallar\u{0131}n\u{0131} olu\u{015f}turun, d\u{00fc}zenleyin ve silin");
        m.insert("docs.cap_can_5", "Proje haf\u{0131}za dosyalar\u{0131}n\u{0131} (MEMORY.md ve konular) g\u{00f6}r\u{00fc}nt\u{00fc}leyin ve d\u{00fc}zenleyin");
        m.insert("docs.cap_can_6", "Ayar hiyerar\u{015f}isini inceleyin (global \u{2192} proje \u{2192} yerel)");
        m.insert("docs.cap_can_7", "\u{0130}zin kay\u{0131}tlar\u{0131}n\u{0131} denetleyin ve g\u{00fc}venlik sorunlar\u{0131}n\u{0131} tespit edin");
        m.insert("docs.cap_can_8", "MCP sunucu yap\u{0131}land\u{0131}rmalar\u{0131}n\u{0131} g\u{00f6}r\u{00fc}nt\u{00fc}leyin");
        m.insert("docs.cap_can_9", "Oturum ge\u{00e7}mi\u{015f}ini, token kullan\u{0131}m\u{0131}n\u{0131} ve maliyetleri analiz edin");
        m.insert("docs.cap_can_10", "Uygulanabilir \u{00f6}nerilerle yapay zeka destekli proje analizi \u{00e7}al\u{0131}\u{015f}t\u{0131}r\u{0131}n");
        m.insert("docs.cap_can_11", "Topluluk depolar\u{0131}ndan Skills ke\u{015f}fedin ve kurun");
        m.insert("docs.cap_can_12", "T\u{00fc}m yazma i\u{015f}lemleri otomatik olarak ~/.claude/backups/ i\u{00e7}inde yedek olu\u{015f}turur");
        m.insert("docs.cap_cannot_heading", "Neler yapamaz");
        m.insert("docs.cap_cannot_1", "Claude Code oturumlar\u{0131}n\u{0131} \u{00e7}al\u{0131}\u{015f}t\u{0131}rmak \u{2013} yap\u{0131}land\u{0131}rmay\u{0131} y\u{00f6}netir, y\u{00fc}r\u{00fc}tmeyi de\u{011f}il");
        m.insert("docs.cap_cannot_2", "Y\u{00f6}netilen politikalar\u{0131} de\u{011f}i\u{015f}tirmek (kurumsal/organizasyon d\u{00fc}zeyi ayarlar\u{0131})");
        m.insert("docs.cap_cannot_3", "Uzak ortamlara veya SSH oturumlar\u{0131}na eri\u{015f}mek");
        m.insert("docs.cap_cannot_4", "Ger\u{00e7}ek kodlama \u{00e7}al\u{0131}\u{015f}malar\u{0131} i\u{00e7}in Claude Code CLI\u{2019}n\u{0131}n yerini almak");
        m.insert("docs.cap_cannot_5", ".claude.json MCP sunucular\u{0131}n\u{0131} do\u{011f}rudan d\u{00fc}zenlemek (g\u{00fc}venlik i\u{00e7}in salt okunur)");
        m.insert("docs.cap_cannot_6", "API anahtarlar\u{0131}n\u{0131} veya kimlik do\u{011f}rulama bilgilerini y\u{00f6}netmek");
        m.insert("docs.cap_cannot_callout", "ClaudeAdmin bir yap\u{0131}land\u{0131}rma y\u{00f6}neticisidir, Claude Code\u{2019}un kendisinin yerine ge\u{00e7}mez. Bunu bir veritaban\u{0131} y\u{00f6}netim arac\u{0131} gibi d\u{00fc}\u{015f}\u{00fc}n\u{00fc}n: incelemenize, yap\u{0131}land\u{0131}rman\u{0131}za ve bak\u{0131}m yapman\u{0131}za yard\u{0131}mc\u{0131} olur \u{2013} ancak as\u{0131}l i\u{015f} Claude Code i\u{00e7}inde yap\u{0131}l\u{0131}r.");

        // ── Docs: CLAUDE.md ──
        m.insert("docs.claudemd_callout", "Projenin anayasas\u{0131}. CLAUDE.md en \u{00f6}nemli yap\u{0131}land\u{0131}rma dosyas\u{0131}d\u{0131}r \u{2013} her Claude Code oturumuna kal\u{0131}c\u{0131} ba\u{011f}lam olarak otomatik y\u{00fc}klenir.");
        m.insert("docs.claudemd_how_heading", "Nas\u{0131}l \u{00e7}al\u{0131}\u{015f}\u{0131}r");
        m.insert("docs.claudemd_how_text", "Claude Code bir oturum ba\u{015f}latt\u{0131}\u{011f}\u{0131}nda, mevcut \u{00e7}al\u{0131}\u{015f}ma dizininden dosya sistemi k\u{00f6}k\u{00fc}ne kadar CLAUDE.md dosyalar\u{0131}n\u{0131} \u{00f6}zyinelemeli olarak arar. Bulunan t\u{00fc}m dosyalar y\u{00fc}klenir ve birle\u{015f}tirilir, daha yak\u{0131}n dosyalar \u{00f6}ncelik kazan\u{0131}r. Bu, monorepo d\u{00fc}zeyinde payla\u{015f}\u{0131}lan kurallarla bir CLAUDE.md\u{2019}ye ve paket d\u{00fc}zeyinde \u{00f6}zel ge\u{00e7}ersiz k\u{0131}lmalarla CLAUDE.md dosyalar\u{0131}na sahip olabilece\u{011f}iniz anlam\u{0131}na gelir.");
        m.insert("docs.claudemd_locations_heading", "Konumlar");
        m.insert("docs.claudemd_loc_project_or", " veya ");
        m.insert("docs.claudemd_loc_parent", "Monorepo k\u{00f6}k\u{00fc}, t\u{00fc}m alt paketler i\u{00e7}in y\u{00fc}klenir");
        m.insert("docs.claudemd_loc_user", "T\u{00fc}m projelerde ki\u{015f}isel varsay\u{0131}lanlar");
        m.insert("docs.claudemd_whatto_heading", "Ne yaz\u{0131}lmal\u{0131}");
        m.insert("docs.claudemd_whatto_context_label", "Proje ba\u{011f}lam\u{0131}");
        m.insert("docs.claudemd_whatto_context", " \u{2013} Teknoloji y\u{0131}\u{011f}\u{0131}n\u{0131}, mimari kararlar, temel ba\u{011f}\u{0131}ml\u{0131}l\u{0131}klar");
        m.insert("docs.claudemd_whatto_standards_label", "Kodlama standartlar\u{0131}");
        m.insert("docs.claudemd_whatto_standards", " \u{2013} Adland\u{0131}rma kurallar\u{0131}, bi\u{00e7}imlendirme kurallar\u{0131}, hata i\u{015f}leme kal\u{0131}plar\u{0131}");
        m.insert("docs.claudemd_whatto_workflows_label", "\u{0130}\u{015f} ak\u{0131}\u{015f}lar\u{0131}");
        m.insert("docs.claudemd_whatto_workflows", " \u{2013} Derleme, test, da\u{011f}\u{0131}t\u{0131}m y\u{00f6}ntemleri; dal adland\u{0131}rma; PR kurallar\u{0131}");
        m.insert("docs.claudemd_whatto_dodont_label", "Yap/Yapma kurallar\u{0131}");
        m.insert("docs.claudemd_whatto_dodont", " \u{2013} A\u{00e7}\u{0131}k k\u{0131}s\u{0131}tlamalar (\u{00f6}rn. \u{201c}TypeScript\u{2019}te asla any kullanma\u{201d})");
        m.insert("docs.claudemd_whatto_team_label", "Tak\u{0131}m anla\u{015f}malar\u{0131}");
        m.insert("docs.claudemd_whatto_team", " \u{2013} \u{0130}nceleme s\u{00fc}reci, commit mesaj bi\u{00e7}imi, mod\u{00fc}l s\u{0131}n\u{0131}rlar\u{0131}");
        m.insert("docs.claudemd_tip1", "500 sat\u{0131}r\u{0131}n alt\u{0131}nda tutun. Claude t\u{00fc}m dosyay\u{0131} ba\u{011f}lama y\u{00fc}kler \u{2013} \u{015f}i\u{015f}kin CLAUDE.md dosyalar\u{0131} token israf eder ve \u{00f6}nemli talimatlar\u{0131} seyreltir.");
        m.insert("docs.claudemd_tip2", "Net b\u{00f6}l\u{00fc}m ba\u{015f}l\u{0131}klar\u{0131} kullan\u{0131}n (## Mimari, ## Kurallar). Claude ilgili b\u{00f6}l\u{00fc}mleri bulmak i\u{00e7}in yap\u{0131}y\u{0131} ayr\u{0131}\u{015f}t\u{0131}r\u{0131}r.");
        m.insert("docs.claudemd_tip3", "En kritik kurallar\u{0131} en ba\u{015f}a koyun. Uzun dosyalarda ba\u{015f}lang\u{0131}\u{00e7}taki i\u{00e7}erik daha fazla dikkat \u{00e7}eker.");
        m.insert("docs.claudemd_tip4", "Git\u{2019}e commit edilmemesi gereken ki\u{015f}isel tercihler i\u{00e7}in CLAUDE.local.md kullan\u{0131}n.");
        m.insert("docs.claudemd_ext_link", "Anthropic Dok\u{00fc}mantasyonu: CLAUDE.md \u{2192}");

        // ── Docs: Rules ──
        m.insert("docs.rules_heading", "Kurallar");
        m.insert("docs.rules_callout", "Claude\u{2019}un davran\u{0131}\u{015f}\u{0131}n\u{0131} \u{015f}ekillendiren mod\u{00fc}ler, tematik k\u{0131}s\u{0131}tlamalar. Tek bir b\u{00fc}y\u{00fc}k dosya olan CLAUDE.md\u{2019}den farkl\u{0131} olarak, kurallar ayr\u{0131} .md dosyalar\u{0131}d\u{0131}r \u{2013} her biri belirli bir konuya odaklan\u{0131}r.");
        m.insert("docs.rules_how_heading", "Nas\u{0131}l \u{00e7}al\u{0131}\u{015f}\u{0131}r");
        m.insert("docs.rules_how_text", "Kurallar oturum ba\u{015f}lang\u{0131}c\u{0131}nda otomatik y\u{00fc}klenir. Global kurallar (ki\u{015f}isel tercihleriniz) \u{00f6}nce y\u{00fc}klenir, sonra proje kurallar\u{0131} bunlar\u{0131}n \u{00fc}zerine gelir. Bu, kodlama stilinizi global olarak tan\u{0131}mlarken projelerin alana \u{00f6}zg\u{00fc} k\u{0131}s\u{0131}tlamalar eklemesine olanak tan\u{0131}r.");
        m.insert("docs.rules_locations_heading", "Konumlar");
        m.insert("docs.rules_loc_global", "Ki\u{015f}isel kurallar\u{0131}n\u{0131}z, t\u{00fc}m projelere uygulan\u{0131}r");
        m.insert("docs.rules_loc_project", "Projeye \u{00f6}zel, tak\u{0131}m payla\u{015f}\u{0131}m\u{0131} i\u{00e7}in git\u{2019}e commit edilir");
        m.insert("docs.rules_examples_heading", "\u{00d6}rnekler");
        m.insert("docs.rules_example_frontend", " \u{2013} React bile\u{015f}en kal\u{0131}plar\u{0131}, durum y\u{00f6}netimi kurallar\u{0131}");
        m.insert("docs.rules_example_security", " \u{2013} Girdi do\u{011f}rulama, kimlik do\u{011f}rulama kal\u{0131}plar\u{0131}, OWASP uyumlulu\u{011f}u");
        m.insert("docs.rules_example_testing", " \u{2013} Test yap\u{0131}s\u{0131}, kapsam beklentileri, mock stratejisi");
        m.insert("docs.rules_example_rust", " \u{2013} thiserror ile hata i\u{015f}leme, mod\u{00fc}l yap\u{0131}s\u{0131}, adland\u{0131}rma");
        m.insert("docs.rules_tip1", "Dosya ba\u{015f}\u{0131}na tek konu. \u{00d6}ny\u{00fc}z ve arka y\u{00fc}z kurallar\u{0131}n\u{0131} kar\u{0131}\u{015f}t\u{0131}rmay\u{0131}n \u{2013} daha k\u{00fc}\u{00e7}\u{00fc}k, odakl\u{0131} dosyalar bak\u{0131}m\u{0131} ve yeniden kullan\u{0131}m\u{0131} kolayla\u{015f}t\u{0131}r\u{0131}r.");
        m.insert("docs.rules_tip2", "Global kurallar ki\u{015f}isel stil tercihleri i\u{00e7}in harikad\u{0131}r: tercih edilen dil, bi\u{00e7}imlendirme arac\u{0131}, commit mesaj bi\u{00e7}imi.");
        m.insert("docs.rules_tip3", "Proje kurallar\u{0131} global kurallar\u{0131} ge\u{00e7}ersiz k\u{0131}lar. Bir \u{00e7}ak\u{0131}\u{015f}ma varsa proje d\u{00fc}zeyi kural\u{0131} kazan\u{0131}r.");
        m.insert("docs.rules_tip4", "Global ve proje d\u{00fc}zeyi aras\u{0131}ndaki yinelenen kurallar\u{0131} tespit etmek i\u{00e7}in ClaudeAdmin\u{2019}in Sa\u{011f}l\u{0131}k Kontrol\u{00fc}n\u{00fc} kullan\u{0131}n.");
        m.insert("docs.rules_ext_link", "Anthropic Dok\u{00fc}mantasyonu: Kurallar \u{2192}");

        // ── Docs: Skills ──
        m.insert("docs.skills_heading", "Skills");
        m.insert("docs.skills_callout", "Meta verili yeniden kullan\u{0131}labilir, yap\u{0131}land\u{0131}r\u{0131}lm\u{0131}\u{015f} istemler. Skills, Claude i\u{00e7}in eklentiler gibidir \u{2013} ba\u{011f}lam taraf\u{0131}ndan otomatik tetiklenebilir veya e\u{011f}ik \u{00e7}izgi komutlar\u{0131}yla manuel olarak \u{00e7}a\u{011f}r\u{0131}labilir.");
        m.insert("docs.skills_how_heading", "Nas\u{0131}l \u{00e7}al\u{0131}\u{015f}\u{0131}r");
        m.insert("docs.skills_how_text", "Her Skill kendi dizininde bulunur ve YAML \u{00f6}n bilgi ile Markdown g\u{00f6}vdesi i\u{00e7}eren bir SKILL.md dosyas\u{0131} i\u{00e7}erir. \u{00d6}n bilgi, a\u{00e7}\u{0131}klama ve tetikleme ko\u{015f}ullar\u{0131} gibi meta verileri tan\u{0131}mlar. G\u{00f6}vde ger\u{00e7}ek istem talimatlar\u{0131}n\u{0131}, \u{00f6}rnekleri ve referans materyallerini i\u{00e7}erir.");
        m.insert("docs.skills_structure_heading", "Yap\u{0131}");
        m.insert("docs.skills_locations_heading", "Konumlar");
        m.insert("docs.skills_loc_global", "T\u{00fc}m projelerde kullan\u{0131}labilir");
        m.insert("docs.skills_loc_project", "Projeye \u{00f6}zel Skills");
        m.insert("docs.skills_tip1", "Bir Skill\u{2019}i Claude Code\u{2019}da /skill-name ile \u{00e7}a\u{011f}r\u{0131}labilir yapmak i\u{00e7}in \u{00f6}n bilgide user_invocable: true ayarlay\u{0131}n.");
        m.insert("docs.skills_tip2", "SKILL.md\u{2019}nize somut \u{00f6}rnekler ekleyin. Claude, girdi/\u{00e7}\u{0131}kt\u{0131} \u{00f6}rnekleriyle \u{00e7}ok daha iyi performans g\u{00f6}sterir.");
        m.insert("docs.skills_tip3", "Topluluk Skills\u{2019}lerini ke\u{015f}fetmek ve kurmak i\u{00e7}in ClaudeAdmin i\u{00e7}indeki Skill Taray\u{0131}c\u{0131}s\u{0131}n\u{0131} kullan\u{0131}n.");
        m.insert("docs.skills_tip4", "Skill dizinindeki referans dosyalar\u{0131} yaln\u{0131}zca Skill tetiklendi\u{011f}inde y\u{00fc}klenir, token tasarrufu sa\u{011f}lar.");
        m.insert("docs.skills_ext_link", "Anthropic Dok\u{00fc}mantasyonu: Skills \u{2192}");

        // ── Docs: Memory ──
        m.insert("docs.memory_heading", "Haf\u{0131}za");
        m.insert("docs.memory_callout", "Claude\u{2019}un proje ba\u{015f}\u{0131}na kal\u{0131}c\u{0131} bilgi bankas\u{0131}. Haf\u{0131}za dosyalar\u{0131}, Claude\u{2019}un oturumlar boyunca biriktirdi\u{011f}i kal\u{0131}plar\u{0131}, tercihleri ve \u{00f6}\u{011f}renmeleri saklar.");
        m.insert("docs.memory_how_heading", "Nas\u{0131}l \u{00e7}al\u{0131}\u{015f}\u{0131}r");
        m.insert("docs.memory_how_text", "Claude Code her proje i\u{00e7}in ~/.claude/projects/<encoded-path>/memory/ i\u{00e7}inde bir haf\u{0131}za dizini tutar. Ana dosya MEMORY.md \u{00f6}zel bir konuma sahiptir: ilk 200 sat\u{0131}r\u{0131} oturum ba\u{015f}lang\u{0131}c\u{0131}nda sistem istemine y\u{00fc}klenir. Ek konu dosyalar\u{0131} (debugging.md, api-conventions.md vb.) Claude mevcut g\u{00f6}revle ilgili oldu\u{011f}unu belirledi\u{011f}inde talep \u{00fc}zerine y\u{00fc}klenir.");
        m.insert("docs.memory_structure_heading", "Yap\u{0131}");
        m.insert("docs.memory_auto_heading", "Otomatik Haf\u{0131}za");
        m.insert("docs.memory_auto_text", "Claude Code, proje kal\u{0131}plar\u{0131}n\u{0131}, hata ay\u{0131}klama \u{00e7}\u{00f6}z\u{00fc}mlerini veya tercihlerinizi ke\u{015f}fetti\u{011f}inde otomatik olarak haf\u{0131}zaya kay\u{0131}t ekleyebilir. Otomatik olu\u{015f}turulan haf\u{0131}zay\u{0131} Claude Code\u{2019}daki /memory komutuyla veya ClaudeAdmin\u{2019}in Haf\u{0131}za edit\u{00f6}r\u{00fc}yle inceleyip d\u{00fc}zenleyebilirsiniz.");
        m.insert("docs.memory_tip1", "En kritik bilgileri MEMORY.md\u{2019}nin ilk 200 sat\u{0131}r\u{0131}na koyun \u{2013} otomatik y\u{00fc}klenen k\u{0131}s\u{0131}m budur.");
        m.insert("docs.memory_tip2", "Derin bilgi i\u{00e7}in konu dosyalar\u{0131}n\u{0131} kullan\u{0131}n. Yaln\u{0131}zca gerekti\u{011f}inde y\u{00fc}klenir, temel token kullan\u{0131}m\u{0131}n\u{0131} d\u{00fc}\u{015f}\u{00fc}k tutar.");
        m.insert("docs.memory_tip3", "Otomatik haf\u{0131}zay\u{0131} d\u{00fc}zenli olarak g\u{00f6}zden ge\u{00e7}irin. Claude bazen a\u{015f}\u{0131}r\u{0131} spesifik tek seferlik \u{00e7}\u{00f6}z\u{00fc}mler saklar.");
        m.insert("docs.memory_tip4", "Haf\u{0131}za proje ba\u{015f}\u{0131}nad\u{0131}r. Farkl\u{0131} bir projeye ge\u{00e7}ti\u{011f}inizde Claude farkl\u{0131} bir haf\u{0131}za seti al\u{0131}r.");
        m.insert("docs.memory_ext_link", "Anthropic Dok\u{00fc}mantasyonu: Haf\u{0131}za \u{2192}");

        // ── Docs: Settings & Hooks ──
        m.insert("docs.settings_heading", "Ayarlar ve Hooks");
        m.insert("docs.settings_heading_short", "Ayarlar");
        m.insert("docs.settings_callout", "Davran\u{0131}\u{015f}, izinler ve otomasyon i\u{00e7}in JSON tabanl\u{0131} yap\u{0131}land\u{0131}rma. Hooks, Claude ara\u{00e7}lar\u{0131} kullanmadan \u{00f6}nce veya sonra otomatik olarak kabuk komutlar\u{0131} \u{00e7}al\u{0131}\u{015f}t\u{0131}rman\u{0131}z\u{0131} sa\u{011f}lar.");
        m.insert("docs.settings_hierarchy_heading", "Ayar Hiyerar\u{015f}isi");
        m.insert("docs.settings_hierarchy_text", "Ayarlar artan \u{00f6}zg\u{00fc}ll\u{00fc}kle katmanl\u{0131} bir modeli izler. Daha \u{00f6}zg\u{00fc}l katmanlar daha az \u{00f6}zg\u{00fc}l olanlar\u{0131} ge\u{00e7}ersiz k\u{0131}lar:");
        m.insert("docs.settings_managed_code", "Kurumsal politikalar");
        m.insert("docs.settings_managed_desc", "En y\u{00fc}ksek \u{00f6}ncelik, organizasyon taraf\u{0131}ndan belirlenir (salt okunur)");
        m.insert("docs.settings_global_desc", "Ki\u{015f}isel global ayarlar\u{0131}n\u{0131}z");
        m.insert("docs.settings_project_desc", "Tak\u{0131}m ayarlar\u{0131}, git\u{2019}e commit edilir");
        m.insert("docs.settings_local_desc", "Ki\u{015f}isel proje ge\u{00e7}ersiz k\u{0131}lmalar\u{0131}n\u{0131}z (gitignore\u{2019}da)");
        m.insert("docs.settings_hooks_heading", "Hooks");
        m.insert("docs.settings_hooks_text", "Hooks, Claude Code oturumu s\u{0131}ras\u{0131}nda belirli olaylarda tetiklenen kabuk komutlar\u{0131}d\u{0131}r. settings.json i\u{00e7}indeki hooks anahtar\u{0131} alt\u{0131}nda yap\u{0131}land\u{0131}r\u{0131}l\u{0131}r.");
        m.insert("docs.settings_hooks_events", "Olaylar:\n\u{2022} PreToolUse  \u{2013} Claude bir arac\u{0131} \u{00e7}al\u{0131}\u{015f}t\u{0131}rmadan \u{00f6}nce (\u{00f6}rn. yazmadan \u{00f6}nce otomatik bi\u{00e7}imlendirme)\n\u{2022} PostToolUse \u{2013} Claude bir arac\u{0131} \u{00e7}al\u{0131}\u{015f}t\u{0131}rd\u{0131}ktan sonra (\u{00f6}rn. kod de\u{011f}i\u{015f}ikli\u{011f}inden sonra lint)\n\u{2022} Stop        \u{2013} Claude bir yan\u{0131}t\u{0131} bitirdi\u{011f}inde");
        m.insert("docs.settings_tip1", "Claude dosya yazmadan \u{00f6}nce kodu otomatik bi\u{00e7}imlendirmek i\u{00e7}in PreToolUse Hooks kullan\u{0131}n. Bu tutarl\u{0131} stil sa\u{011f}lar.");
        m.insert("docs.settings_tip2", "PostToolUse Hooks lint i\u{00e7}in harikad\u{0131}r: Claude kodu de\u{011f}i\u{015f}tirdikten hemen sonra sorunlar\u{0131} yakalay\u{0131}n.");
        m.insert("docs.settings_tip3", "ClaudeAdmin\u{2019}in Ayarlar sayfas\u{0131} t\u{00fc}m katmanlardaki etkin Hook zincirini g\u{00f6}sterir.");
        m.insert("docs.settings_ext_link", "Anthropic Dok\u{00fc}mantasyonu: Ayarlar \u{2192}");
        m.insert("docs.settings_hooks_ext_link", "Anthropic Dok\u{00fc}mantasyonu: Hooks \u{2192}");

        // ── Docs: MCP Servers ──
        m.insert("docs.mcp_heading", "MCP Sunucular\u{0131}");
        m.insert("docs.mcp_callout", "Model Context Protocol sunucular\u{0131}, Claude\u{2019}u harici ara\u{00e7}lar ve veri kaynaklar\u{0131}yla geni\u{015f}letir. Claude\u{2019}un veritabanlar\u{0131}, API\u{2019}ler, dosya sistemleri ve di\u{011f}er hizmetlerle etkile\u{015f}mesini sa\u{011f}lar.");
        m.insert("docs.mcp_how_heading", "Nas\u{0131}l \u{00e7}al\u{0131}\u{015f}\u{0131}r");
        m.insert("docs.mcp_how_text", "MCP sunucular\u{0131}, Claude Code\u{2019}un ba\u{015f}latt\u{0131}\u{011f}\u{0131} ve MCP protokol\u{00fc} \u{00fc}zerinden ileti\u{015f}im kurdu\u{011f}u harici s\u{00fc}re\u{00e7}lerdir. Her sunucu, Claude\u{2019}un \u{00e7}a\u{011f}\u{0131}rabilece\u{011f}i bir ara\u{00e7} seti sa\u{011f}lar. Yap\u{0131}land\u{0131}rma ~/.claude.json i\u{00e7}indeki mcpServers anahtar\u{0131} alt\u{0131}nda bulunur.");
        m.insert("docs.mcp_config_heading", "Yap\u{0131}land\u{0131}rma");
        m.insert("docs.mcp_management_heading", "ClaudeAdmin\u{2019}de Y\u{00f6}netim");
        m.insert("docs.mcp_management_text", "ClaudeAdmin, tam y\u{00f6}netim i\u{00e7}in \u{00f6}zel bir MCP Sunucular\u{0131} sayfas\u{0131} sa\u{011f}lar: manuel JSON d\u{00fc}zenlemesi olmadan sunucular\u{0131} g\u{00f6}r\u{00fc}nt\u{00fc}leyin, ekleyin, d\u{00fc}zenleyin ve silin. Sa\u{011f}l\u{0131}k Kontrol\u{00fc} \u{00f6}zelli\u{011f}i her sunucuyu ba\u{015f}lat\u{0131}r ve JSON-RPC initialize ile tools/list isteklerine yan\u{0131}t verip vermedi\u{011f}ini do\u{011f}rular. Pop\u{00fc}ler sunucular\u{0131} tek t\u{0131}klamayla ke\u{015f}fetmek ve kurmak i\u{00e7}in MCP Taray\u{0131}c\u{0131}s\u{0131}n\u{0131} kullan\u{0131}n.");
        m.insert("docs.mcp_tip1", "MCP sunucular\u{0131} .claude/settings.json i\u{00e7}inde proje ba\u{015f}\u{0131}na da yap\u{0131}land\u{0131}r\u{0131}labilir.");
        m.insert("docs.mcp_tip2", "S\u{0131}rlar i\u{00e7}in ortam de\u{011f}i\u{015f}kenleri kullan\u{0131}n \u{2013} yap\u{0131}land\u{0131}rma dosyalar\u{0131}na asla API anahtarlar\u{0131}n\u{0131} sabit kodlamay\u{0131}n.");
        m.insert("docs.mcp_tip3", "Pop\u{00fc}ler sunucular\u{0131} ke\u{015f}fetmek ve kurmak i\u{00e7}in MCP Taray\u{0131}c\u{0131}s\u{0131}n\u{0131} kullan\u{0131}n veya Yeni Sunucu sekmesinden \u{00f6}zel sunucular ekleyin.");
        m.insert("docs.mcp_ext_link", "Anthropic Dok\u{00fc}mantasyonu: MCP \u{2192}");
        m.insert("docs.mcp_spec_ext_link", "MCP Spesifikasyonu \u{2192}");

        // ── Docs: Plans ──
        m.insert("docs.plans_heading", "Planlar");
        m.insert("docs.plans_callout", "Claude\u{2019}un karma\u{015f}\u{0131}k g\u{00f6}revleri par\u{00e7}alamak i\u{00e7}in kulland\u{0131}\u{011f}\u{0131} Markdown dosyalar\u{0131}. Planlar, Claude\u{2019}un \u{00e7}ok ad\u{0131}ml\u{0131} \u{00e7}al\u{0131}\u{015f}malarda odaklanmas\u{0131}na ve ilerlemeyi takip etmesine yard\u{0131}mc\u{0131} olur.");
        m.insert("docs.plans_how_heading", "Nas\u{0131}l \u{00e7}al\u{0131}\u{015f}\u{0131}r");
        m.insert("docs.plans_how_text", "Claude karma\u{015f}\u{0131}k bir g\u{00f6}revle u\u{011f}ra\u{015f}t\u{0131}\u{011f}\u{0131}nda, ~/.claude/plans/ i\u{00e7}inde saklanan plan dosyalar\u{0131} olu\u{015f}turabilir veya ba\u{015f}vurabilir. Planlar, g\u{00f6}rev listeleri, ba\u{011f}\u{0131}ml\u{0131}l\u{0131}klar ve durum takibi i\u{00e7}eren yap\u{0131}land\u{0131}r\u{0131}lm\u{0131}\u{015f} Markdown belgelerdir. Oturumlar aras\u{0131}nda kal\u{0131}c\u{0131}d\u{0131}r, b\u{00f6}ylece Claude kald\u{0131}\u{011f}\u{0131} yerden devam edebilir.");
        m.insert("docs.plans_location_heading", "Konum");
        m.insert("docs.plans_loc_global", "T\u{00fc}m plan dosyalar\u{0131}");
        m.insert("docs.plans_tip1", "Karma\u{015f}\u{0131}k yeniden yap\u{0131}land\u{0131}rmadan \u{00f6}nce Claude\u{2019}dan \u{201c}bir plan yap\u{201d} isteyin. Planlar \u{00e7}oklu dosya de\u{011f}i\u{015f}ikliklerinde hatalar\u{0131} azalt\u{0131}r.");
        m.insert("docs.plans_tip2", "Eski planlar\u{0131} d\u{00fc}zenli olarak temizleyin. ClaudeAdmin\u{2019}in Planlar sayfas\u{0131} t\u{00fc}m saklanan planlar\u{0131} de\u{011f}i\u{015f}tirme tarihleriyle g\u{00f6}sterir.");

        // ── Docs: Scopes ──
        m.insert("docs.scopes_heading", "Global ve Proje Kapsam\u{0131}");
        m.insert("docs.scopes_callout", "Kapsam\u{0131} anlamak, etkili Claude Code yap\u{0131}land\u{0131}rmas\u{0131}n\u{0131}n anahtar\u{0131}d\u{0131}r. Her yap\u{0131}land\u{0131}rma t\u{00fc}r\u{00fc} iki katmanda bulunur: global (ki\u{015f}isel varsay\u{0131}lanlar\u{0131}n\u{0131}z) ve projeye \u{00f6}zel (tak\u{0131}m\u{0131}n\u{0131}zla payla\u{015f}\u{0131}lan).");
        m.insert("docs.scopes_overview_heading", "Kapsam Genel Bak\u{0131}\u{015f}\u{0131}");
        m.insert("docs.scopes_col_type", "Yap\u{0131}land\u{0131}rma T\u{00fc}r\u{00fc}");
        m.insert("docs.scopes_col_global", "Global (Kullan\u{0131}c\u{0131})");
        m.insert("docs.scopes_col_project", "Proje");
        m.insert("docs.scopes_col_priority", "\u{00d6}ncelik");
        m.insert("docs.scopes_priority_project_global", "Proje > Global");
        m.insert("docs.scopes_priority_both", "Her ikisi de kullan\u{0131}labilir");
        m.insert("docs.scopes_memory_global", "Proje ba\u{015f}\u{0131}na ~/.claude/projects/ i\u{00e7}inde");
        m.insert("docs.scopes_priority_project_keyed", "Proje anahtarl\u{0131}");
        m.insert("docs.scopes_priority_local_project_global", "Yerel > Proje > Global");
        m.insert("docs.scopes_priority_merged", "Birle\u{015f}tirilmi\u{015f}");
        m.insert("docs.scopes_when_heading", "Hangisi ne zaman kullan\u{0131}l\u{0131}r?");
        m.insert("docs.scopes_use_global", "Global kullan\u{0131}m i\u{00e7}in");
        m.insert("docs.scopes_global_1", "Ki\u{015f}isel kodlama stili tercihleri");
        m.insert("docs.scopes_global_2", "Tercih edilen dil ve framework varsay\u{0131}lanlar\u{0131}");
        m.insert("docs.scopes_global_3", "Commit mesaj bi\u{00e7}imi");
        m.insert("docs.scopes_global_4", "Edit\u{00f6}r/IDE entegrasyon ayarlar\u{0131}");
        m.insert("docs.scopes_global_5", "T\u{00fc}m projelerde kulland\u{0131}\u{011f}\u{0131}n\u{0131}z MCP sunucular\u{0131}");
        m.insert("docs.scopes_use_project", "Proje kullan\u{0131}m\u{0131} i\u{00e7}in");
        m.insert("docs.scopes_project_1", "Teknoloji y\u{0131}\u{011f}\u{0131}n\u{0131} dok\u{00fc}mantasyonu ve k\u{0131}s\u{0131}tlamalar");
        m.insert("docs.scopes_project_2", "Tak\u{0131}m kodlama kurallar\u{0131}");
        m.insert("docs.scopes_project_3", "Alana \u{00f6}zg\u{00fc} kurallar (g\u{00fc}venlik, uyumluluk)");
        m.insert("docs.scopes_project_4", "Projeye \u{00f6}zel Skills ve i\u{015f} ak\u{0131}\u{015f}lar\u{0131}");
        m.insert("docs.scopes_project_5", "CI/CD Hooks ve otomasyon");

        // ── Docs: Tips & Best Practices ──
        m.insert("docs.bestpractices_heading", "\u{0130}pu\u{00e7}lar\u{0131} ve En \u{0130}yi Uygulamalar");
        m.insert("docs.bestpractices_hygiene_heading", "Yap\u{0131}land\u{0131}rma Hijyeni");
        m.insert("docs.bestpractices_hygiene_1", "ClaudeAdmin\u{2019}in Yap\u{0131}land\u{0131}rma Sa\u{011f}l\u{0131}\u{011f}\u{0131} kontrollerini d\u{00fc}zenli \u{00e7}al\u{0131}\u{015f}t\u{0131}r\u{0131}n. Yinelenen kurallar\u{0131}, \u{015f}i\u{015f}kin izin listelerini ve eksik CLAUDE.md dosyalar\u{0131}n\u{0131} tespit eder.");
        m.insert("docs.bestpractices_hygiene_2", "Kendinizi tekrarlamay\u{0131}n: bir kural global olarak varsa, proje CLAUDE.md\u{2019}sine kopyalamay\u{0131}n. Kapsam sistemini kullan\u{0131}n.");
        m.insert("docs.bestpractices_hygiene_3", "\u{0130}zin listelerini temiz tutun. Zaman i\u{00e7}inde Claude Code y\u{00fc}zlerce izin ver/reddet kayd\u{0131} biriktirir. Bunlar\u{0131} budamak i\u{00e7}in \u{0130}zinler sayfas\u{0131}n\u{0131} kullan\u{0131}n.");
        m.insert("docs.bestpractices_tokens_heading", "Token Verimlili\u{011f}i");
        m.insert("docs.bestpractices_tokens_1", "CLAUDE.md, kurallar, Skills (tetiklendi\u{011f}inde) ve MEMORY.md\u{2019}nin ilk 200 sat\u{0131}r\u{0131}ndaki her \u{015f}ey ba\u{011f}lam penceresinden d\u{00fc}\u{015f}er. K\u{0131}sa ve \u{00f6}z tutun.");
        m.insert("docs.bestpractices_tokens_2", "Ayr\u{0131}nt\u{0131}l\u{0131} referans materyallerini Skill referans dosyalar\u{0131}na veya haf\u{0131}za konu dosyalar\u{0131}na ta\u{015f}\u{0131}y\u{0131}n \u{2013} yaln\u{0131}zca gerekti\u{011f}inde y\u{00fc}klenir.");
        m.insert("docs.bestpractices_tokens_3", "Projeler ve oturumlar aras\u{0131}ndaki token kullan\u{0131}m\u{0131}n\u{0131}z\u{0131} izlemek i\u{00e7}in Analitik sayfas\u{0131}n\u{0131} kullan\u{0131}n.");
        m.insert("docs.bestpractices_team_heading", "Tak\u{0131}m \u{0130}\u{015f}birli\u{011f}i");
        m.insert("docs.bestpractices_team_1", ".claude/rules/ ve .claude/skills/ dizinlerini git\u{2019}e commit edin. Bu, kurallar\u{0131} tak\u{0131}m genelinde payla\u{015f}\u{0131}r.");
        m.insert("docs.bestpractices_team_2", "Tak\u{0131}m ayarlar\u{0131} i\u{00e7}in .claude/settings.json, ki\u{015f}isel ge\u{00e7}ersiz k\u{0131}lmalar i\u{00e7}in .claude/settings.local.json kullan\u{0131}n.");
        m.insert("docs.bestpractices_team_3", "Proje k\u{00f6}k\u{00fc}ndeki CLAUDE.md, tak\u{0131}m\u{0131}n\u{0131}z\u{0131}n Claude ile s\u{00f6}zle\u{015f}mesidir. Dok\u{00fc}mantasyon gibi davran\u{0131}n \u{2013} de\u{011f}i\u{015f}iklikleri PR\u{2019}lerde inceleyin.");
        m.insert("docs.bestpractices_debug_heading", "Claude Davran\u{0131}\u{015f}\u{0131}n\u{0131} Hata Ay\u{0131}klama");
        m.insert("docs.bestpractices_debug_1", "Claude bir kural\u{0131} yoksay\u{0131}yorsa, katmanlar aras\u{0131}nda \u{00e7}ak\u{0131}\u{015f}an ayarlar i\u{00e7}in Ayar Hiyerar\u{015f}isi sayfas\u{0131}n\u{0131} kontrol edin.");
        m.insert("docs.bestpractices_debug_2", "Haf\u{0131}za beklenmedik davran\u{0131}\u{015f}lara neden olabilir. Otomatik olu\u{015f}turulan kay\u{0131}tlar\u{0131} inceleyin \u{2013} Claude do\u{011f}ru yakla\u{015f}\u{0131}m yerine bir ge\u{00e7}ici \u{00e7}\u{00f6}z\u{00fc}m\u{00fc} ezberlemi\u{015f} olabilir.");
        m.insert("docs.bestpractices_debug_3", "Ge\u{00e7}mi\u{015f} konu\u{015f}malar\u{0131} incelemek ve Claude\u{2019}un ne \u{201c}d\u{00fc}\u{015f}\u{00fc}nd\u{00fc}\u{011f}\u{00fc}n\u{00fc}\u{201d} anlamak i\u{00e7}in Oturumlar sayfas\u{0131}n\u{0131} kullan\u{0131}n.");

        // ── Docs: Links ──
        m.insert("docs.links_heading", "Resmi Anthropic Dok\u{00fc}mantasyonu");
        m.insert("docs.links_text", "Bu ba\u{011f}lant\u{0131}lar Anthropic taraf\u{0131}ndan s\u{00fc}rd\u{00fc}r\u{00fc}len yetkili dok\u{00fc}mantasyona y\u{00f6}nlendirir. ClaudeAdmin bu spesifikasyonlar \u{00fc}zerine in\u{015f}a edilmi\u{015f}tir.");
        m.insert("docs.link_overview_title", "Claude Code Genel Bak\u{0131}\u{015f}");
        m.insert("docs.link_overview_desc", "Ba\u{015f}lang\u{0131}\u{00e7}, kurulum ve temel kullan\u{0131}m");
        m.insert("docs.link_memory_title", "Haf\u{0131}za ve CLAUDE.md");
        m.insert("docs.link_memory_desc", "Claude\u{2019}un proje haf\u{0131}zas\u{0131}n\u{0131} nas\u{0131}l saklad\u{0131}\u{011f}\u{0131} ve kulland\u{0131}\u{011f}\u{0131}");
        m.insert("docs.link_skills_title", "Skills");
        m.insert("docs.link_skills_desc", "Yeniden kullan\u{0131}labilir Skills olu\u{015f}turma ve y\u{00f6}netme");
        m.insert("docs.link_settings_title", "Ayarlar");
        m.insert("docs.link_settings_desc", "Yap\u{0131}land\u{0131}rma hiyerar\u{015f}isi ve se\u{00e7}enekler");
        m.insert("docs.link_hooks_title", "Hooks");
        m.insert("docs.link_hooks_desc", "Kabuk komutlar\u{0131}yla olay g\u{00fc}d\u{00fc}ml\u{00fc} otomasyon");
        m.insert("docs.link_mcp_title", "MCP Sunucular\u{0131}");
        m.insert("docs.link_mcp_desc", "Claude\u{2019}u harici ara\u{00e7}larla geni\u{015f}letme");
        m.insert("docs.link_bestpractices_title", "En \u{0130}yi Uygulamalar");
        m.insert("docs.link_bestpractices_desc", "Etkili Claude Code kullan\u{0131}m\u{0131} i\u{00e7}in ipu\u{00e7}lar\u{0131}");
        m.insert("docs.link_mcp_spec_title", "MCP Spesifikasyonu");
        m.insert("docs.link_mcp_spec_desc", "Model Context Protocol standard\u{0131}");

        // ── Licenses ──
        m.insert("sidebar.licenses", "Lisanslar");
        m.insert("licenses.title", "Lisanslar");
        m.insert("licenses.subtitle", "A\u{00e7}\u{0131}k kaynak lisanslar\u{0131} ve ba\u{011f}\u{0131}ml\u{0131}l\u{0131}klar");
        m.insert("licenses.own_license", "ClaudeAdmin Lisans\u{0131}");
        m.insert("licenses.third_party", "\u{00dc}\u{00e7}\u{00fc}nc\u{00fc} taraf ba\u{011f}\u{0131}ml\u{0131}l\u{0131}klar");
        m.insert("licenses.col_name", "Crate");
        m.insert("licenses.col_version", "S\u{00fc}r\u{00fc}m");
        m.insert("licenses.col_license", "Lisans");
        m.insert("licenses.search_placeholder", "Ba\u{011f}\u{0131}ml\u{0131}l\u{0131}k ara...");
        m.insert("licenses.loading", "Lisanslar y\u{00fc}kleniyor");
        m.insert("licenses.count", "ba\u{011f}\u{0131}ml\u{0131}l\u{0131}k");
        m.insert("licenses.mit_copyright", "Copyright (c) 2024-2026");
        m.insert("licenses.mit_line1", "Bu yazılımın ve ilgili dokümantasyon dosyalarının (\u{201c}Yazılım\u{201d}) bir kopyasını edinen herkese, Yazılımı kısıtlama olmaksızın kullanma, kopyalama, değiştirme, birleştirme, yayımlama, dağıtma, alt lisanslama ve/veya Yazılımın kopyalarını satma hakları da dahil olmak üzere sınırlama olmaksızın Yazılımla ilgilenme ve Yazılımın sağlandığı kişilere bunu yapma izni verme hakkı ücretsiz olarak verilmektedir. Aşağıdaki koşullara tabidir:");
        m.insert("licenses.mit_line2", "Yukarıdaki telif hakkı bildirimi ve bu izin bildirimi, Yazılımın tüm kopyalarına veya önemli bölümlerine dahil edilecektir.");
        m.insert("licenses.mit_line3", "YAZILIM, SATILABİLİRLİK, BELİRLİ BİR AMACA UYGUNLUK VE İHLAL ETMEME GARANTİLERİ DAHİL ANCAK BUNLARLA SINIRLI OLMAMAK ÜZERE, AÇIK VEYA ZIMNİ HİÇBİR GARANTİ OLMAKSIZIN \u{201c}OLDUĞU GİBİ\u{201d} SUNULMAKTADIR. HİÇBİR DURUMDA YAZARLAR VEYA TELİF HAKKI SAHİPLERİ, YAZILIMDAN VEYA YAZILIMIN KULLANIMI VEYA DİĞER İŞLEMLERDEN KAYNAKLANAN HERHANGI BİR TALEP, HASAR VEYA DİĞER YÜKÜMLÜLÜKLERDEN SORUMLU TUTULAMAZ.");
        m.insert("licenses.direct_deps", "Doğrudan bağımlılıklar");
        m.insert("licenses.transitive_deps", "Geçişli bağımlılıklar");
        m.insert("licenses.overview", "Lisans genel bakışı");
        m.insert("licenses.direct_count", "doğrudan");
        m.insert("licenses.transitive_count", "geçişli bağımlılık");

        // ── Components ──
        m.insert("component.modal.close", "Kapat");
        m.insert("component.editor.save", "Kaydet");
        m.insert("component.editor.saved", "Kaydedildi!");
        m.insert("component.json_editor.valid", "Ge\u{00e7}erli JSON");
        m.insert("component.json_editor.invalid", "Ge\u{00e7}ersiz JSON");
        m.insert("component.frontmatter.description", "A\u{00e7}\u{0131}klama");
        m.insert("component.frontmatter.user_invocable", "Kullan\u{0131}c\u{0131} taraf\u{0131}ndan \u{00e7}a\u{011f}r\u{0131}labilir");
        m.insert("component.advisor.title", "Proje Dan\u{0131}\u{015f}man\u{0131}");
        m.insert("component.advisor.analyze", "Analiz Et");
        m.insert("component.advisor.analyzing", "Analiz ediliyor...");
        m.insert("component.advisor.no_api_key", "ANTHROPIC_API_KEY yap\u{0131}land\u{0131}r\u{0131}lmam\u{0131}\u{015f}");
        m.insert("component.advisor.error", "\u{00d6}neriler y\u{00fc}klenirken hata");
        m.insert("component.advisor.summary", "\u{00d6}zet");
        m.insert("component.advisor.recommendations", "\u{00d6}neriler");
        m.insert("component.advisor.apply", "Uygula");
        m.insert("component.advisor.applied", "Tamamland\u{0131}!");
        m.insert("component.advisor.analyze_project", "Projeyi Analiz Et");
        m.insert("component.advisor.hint", "Claude projenizi analiz eder ve \u{00f6}neriler sunar");
        m.insert("component.advisor.loading", "Claude projenizi analiz ediyor");
        m.insert("component.advisor.assessment", "Proje De\u{011f}erlendirmesi");
        m.insert("component.advisor.show_preview", "\u{00d6}nizlemeyi G\u{00f6}ster");
        m.insert("component.advisor.category_tip", "\u{0130}pucu");
        m.insert("component.frontmatter.user_invocable_label", "Kullan\u{0131}c\u{0131} Taraf\u{0131}ndan \u{00c7}a\u{011f}r\u{0131}labilir (/command ile \u{00e7}a\u{011f}r\u{0131}labilir)");
        m.insert("component.editor.saving", "Kaydediliyor...");

        // ── Common ──
        m.insert("common.error", "Hata");
        m.insert("common.loading", "Y\u{00fc}kleniyor");
        m.insert("common.save", "Kaydet");
        m.insert("common.delete", "Sil");
        m.insert("common.cancel", "\u{0130}ptal");
        m.insert("common.close", "Kapat");
        m.insert("common.yes", "Evet");
        m.insert("common.no", "Hay\u{0131}r");
        m.insert("common.ok", "Tamam");
        m.insert("common.error_prefix", "Hata: ");
        m.insert("common.invalid_json", "Ge\u{00e7}ersiz JSON: ");

        m
    })
}
