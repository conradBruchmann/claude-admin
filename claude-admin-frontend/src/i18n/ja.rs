use std::collections::HashMap;
use std::sync::OnceLock;

static TRANSLATIONS: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();

pub fn translations() -> &'static HashMap<&'static str, &'static str> {
    TRANSLATIONS.get_or_init(|| {
        let mut m = HashMap::new();

        // ── App ──
        m.insert("app.title", "ClaudeAdmin");
        m.insert("app.subtitle", "構成マネージャー");

        // ── Sidebar ──
        m.insert("sidebar.overview", "概要");
        m.insert("sidebar.dashboard", "ダッシュボード");
        m.insert("sidebar.analytics", "アナリティクス");
        m.insert("sidebar.manage", "管理");
        m.insert("sidebar.projects", "プロジェクト");
        m.insert("sidebar.global_skills", "グローバル Skills");
        m.insert("sidebar.skill_browser", "Skill ブラウザ");
        m.insert("sidebar.global_rules", "グローバルルール");
        m.insert("sidebar.plans", "プラン");
        m.insert("sidebar.mcp_servers", "MCP サーバー");
        m.insert("sidebar.mcp_browser", "MCP ブラウザ");
        m.insert("sidebar.security", "セキュリティ");
        m.insert("sidebar.permissions", "権限");
        m.insert("sidebar.config_health", "構成ヘルスチェック");
        m.insert("sidebar.system", "システム");
        m.insert("sidebar.settings", "設定");
        m.insert("sidebar.sessions", "セッション");
        m.insert("sidebar.github", "GitHub");
        m.insert("sidebar.learn", "学習");
        m.insert("sidebar.docs", "ドキュメント");
        m.insert("sidebar.help", "システム情報");

        // ── Dashboard ──
        m.insert("dashboard.title", "ダッシュボード");
        m.insert("dashboard.subtitle", "Claude Code 構成の概要");
        m.insert("dashboard.projects", "プロジェクト");
        m.insert("dashboard.global_skills", "グローバル Skills");
        m.insert("dashboard.global_rules", "グローバルルール");
        m.insert("dashboard.mcp_servers", "MCP サーバー");
        m.insert("dashboard.plans", "プラン");
        m.insert("dashboard.config_health", "構成ヘルスチェック");
        m.insert("dashboard.recent_projects", "最近のプロジェクト");
        m.insert("dashboard.loading", "読み込み中");
        m.insert("dashboard.error_loading", "ダッシュボードの読み込みエラー");
        m.insert("dashboard.col_name", "名前");
        m.insert("dashboard.col_claude_md", "CLAUDE.md");
        m.insert("dashboard.col_skills", "Skills");
        m.insert("dashboard.col_rules", "ルール");
        m.insert("dashboard.col_memory", "メモリ");
        m.insert("dashboard.yes", "あり");

        // ── MCP ──
        m.insert("mcp.title", "MCP サーバー");
        m.insert("mcp.subtitle", "Claude Code の Model Context Protocol サーバーを管理");
        m.insert("mcp.tab_servers", "サーバー");
        m.insert("mcp.tab_health", "ヘルスチェック");
        m.insert("mcp.tab_add", "新規サーバー");
        m.insert("mcp.loading", "MCP サーバーを読み込み中");
        m.insert("mcp.no_servers", "MCP サーバーが構成されていません");
        m.insert("mcp.no_servers_hint", "「新規サーバー」タブまたは MCP ブラウザからサーバーを追加してください。");
        m.insert("mcp.select_server", "リストからサーバーを選択して、構成を表示・編集します。");
        m.insert("mcp.no_servers_configured", "サーバーが構成されていません。");
        m.insert("mcp.check_health", "ヘルスチェック");
        m.insert("mcp.save", "保存");
        m.insert("mcp.delete", "削除");
        m.insert("mcp.saved", "保存しました！");
        m.insert("mcp.deleted", "削除しました！");
        m.insert("mcp.read_only", "読み取り専用");
        m.insert("mcp.read_only_hint", "このサーバーは外部で管理されており、ここでは編集できません。");
        m.insert("mcp.health.title", "MCP サーバーヘルス");
        m.insert("mcp.health.check_all", "全サーバーをチェック");
        m.insert("mcp.health.checking", "チェック中...");
        m.insert("mcp.health.description", "各 MCP サーバープロセスを起動し、JSON-RPC の initialize と tools/list を送信して結果を報告します。タイムアウト：サーバーごとに10秒。");
        m.insert("mcp.health.col_name", "名前");
        m.insert("mcp.health.col_source", "ソース");
        m.insert("mcp.health.col_status", "ステータス");
        m.insert("mcp.health.col_server_info", "サーバー情報");
        m.insert("mcp.health.col_tools", "ツール");
        m.insert("mcp.health.col_duration", "所要時間");
        m.insert("mcp.health.running", "実行中");
        m.insert("mcp.health.error", "エラー");
        m.insert("mcp.health.timeout", "タイムアウト");
        m.insert("mcp.health.unknown", "不明");
        m.insert("mcp.add.title", "MCP サーバーを追加");
        m.insert("mcp.add.description", "グローバルの ~/.claude.json 構成に新しい MCP サーバーを追加します。");
        m.insert("mcp.add.name_label", "サーバー名");
        m.insert("mcp.add.name_placeholder", "例: my-server");
        m.insert("mcp.add.config_label", "サーバー構成 (JSON)");
        m.insert("mcp.add.submit", "サーバーを追加");
        m.insert("mcp.add.name_required", "サーバー名を入力してください");
        m.insert("mcp.source.claude_code", "Claude Code");
        m.insert("mcp.source.claude_desktop", "Claude Desktop");

        // ── MCP Browser ──
        m.insert("mcp_browser.title", "MCP ブラウザ");
        m.insert("mcp_browser.subtitle", "Claude Code 用の MCP サーバーを発見してインストール");
        m.insert("mcp_browser.search_placeholder", "MCP サーバーを検索...");
        m.insert("mcp_browser.loading", "MCP カタログを読み込み中");
        m.insert("mcp_browser.no_results", "MCP サーバーが見つかりません");
        m.insert("mcp_browser.installed", "インストール済み");
        m.insert("mcp_browser.install", "インストール");
        m.insert("mcp_browser.needs_api_key", "API キーが必要");
        m.insert("mcp_browser.install_success", "のインストールに成功しました！");
        m.insert("mcp_browser.install_failed", "インストールに失敗しました");

        // ── Projects ──
        m.insert("projects.title", "プロジェクト");
        m.insert("projects.subtitle", "~/.claude.json に登録されている全プロジェクト");
        m.insert("projects.loading", "読み込み中");
        m.insert("projects.error_loading", "プロジェクトの読み込みエラー: ");
        m.insert("projects.col_name", "名前");
        m.insert("projects.col_path", "パス");
        m.insert("projects.col_claude_md", "CLAUDE.md");
        m.insert("projects.col_skills", "Skills");
        m.insert("projects.col_rules", "ルール");
        m.insert("projects.col_memory", "メモリ");
        m.insert("projects.yes", "あり");

        // ── Project Detail ──
        m.insert("project_detail.loading", "プロジェクト詳細を読み込み中");
        m.insert("project_detail.error_loading", "プロジェクトの読み込みエラー");
        m.insert("project_detail.tab_advisor", "アドバイザー");
        m.insert("project_detail.tab_claude_md", "CLAUDE.md");
        m.insert("project_detail.tab_skills", "Skills");
        m.insert("project_detail.tab_rules", "ルール");
        m.insert("project_detail.tab_memory", "メモリ");
        m.insert("project_detail.tab_permissions", "権限");
        m.insert("project_detail.tab_health", "ヘルス");
        m.insert("project_detail.no_claude_md", "CLAUDE.md が見つかりません");
        m.insert("project_detail.no_claude_md_hint", "プロジェクトディレクトリに CLAUDE.md を作成して Claude Code に指示を与えましょう。");
        m.insert("project_detail.no_skills", "このプロジェクトに Skills はありません");
        m.insert("project_detail.no_rules", "このプロジェクトにルールはありません");
        m.insert("project_detail.no_memory", "このプロジェクトにメモリはありません");
        m.insert("project_detail.save", "保存");
        m.insert("project_detail.saved", "保存しました！");
        m.insert("project_detail.skill_scope", "スコープ");
        m.insert("project_detail.permissions_loading", "権限を読み込み中...");
        m.insert("project_detail.permissions_error", "権限の読み込みエラー");
        m.insert("project_detail.permissions_entries", "エントリ");
        m.insert("project_detail.permissions_col_tool", "ツール");
        m.insert("project_detail.permissions_col_command", "コマンド");
        m.insert("project_detail.permissions_no_entries", "権限エントリがありません");
        m.insert("project_detail.health_loading", "ヘルスを計算中...");
        m.insert("project_detail.health_error", "ヘルスデータの読み込みエラー");
        m.insert("project_detail.health_score", "ヘルススコア");
        m.insert("project_detail.health_claude_md", "CLAUDE.md あり");
        m.insert("project_detail.health_memory", "メモリあり");
        m.insert("project_detail.health_permissions", "権限");
        m.insert("project_detail.health_security_issues", "セキュリティの問題");
        m.insert("project_detail.health_duplicated_rules", "重複ルール");
        m.insert("project_detail.health_no_security_issues", "セキュリティの問題は見つかりませんでした");
        m.insert("project_detail.health_col_text", "テキスト");
        m.insert("project_detail.health_col_found_in", "検出場所");
        m.insert("project_detail.health_col_also_in", "他の場所");
        m.insert("project_detail.health_permission_entries", "権限エントリ");
        m.insert("project_detail.permissions_col_index", "#");
        m.insert("project_detail.permissions_col_status", "ステータス");
        m.insert("project_detail.permissions_fragment", "フラグメント");
        m.insert("project_detail.permissions_ok", "OK");
        m.insert("project_detail.permissions_security_warnings", "件のセキュリティ警告");
        m.insert("project_detail.permissions_manage", "権限を管理");
        m.insert("project_detail.advisor_analyze", "プロジェクトを分析");
        m.insert("project_detail.advisor_analyzing", "分析中...");
        m.insert("project_detail.advisor_description", "Claude がプロジェクトを分析し推奨事項を提供します");
        m.insert("project_detail.advisor_loading", "Claude がプロジェクトを分析中です");
        m.insert("project_detail.advisor_summary", "プロジェクト評価");
        m.insert("project_detail.advisor_done", "完了！");
        m.insert("project_detail.advisor_preview", "プレビューを表示");
        m.insert("project_detail.advisor_category_tip", "ヒント");
        m.insert("project_detail.skills_col_name", "名前");
        m.insert("project_detail.skills_col_description", "説明");
        m.insert("project_detail.skills_col_invocable", "呼び出し可能");
        m.insert("project_detail.rules_col_name", "名前");
        m.insert("project_detail.rules_col_path", "パス");
        m.insert("project_detail.memory_col_file", "ファイル");
        m.insert("project_detail.memory_col_size", "サイズ");
        m.insert("project_detail.bytes", "バイト");
        m.insert("project_detail.unknown_tab", "不明なタブ");

        // ── Global Skills ──
        m.insert("global_skills.title", "グローバル Skills");
        m.insert("global_skills.subtitle", "~/.claude/skills/ の Skills を管理");
        m.insert("global_skills.loading", "Skills を読み込み中");
        m.insert("global_skills.no_skills", "グローバル Skills が見つかりません");
        m.insert("global_skills.no_skills_hint", "~/.claude/skills/ に Skills を作成するか、Skill ブラウザを使用してください。");
        m.insert("global_skills.select_skill", "リストから Skill を選択してください。");
        m.insert("global_skills.col_skill", "Skill");
        m.insert("global_skills.col_invocable", "呼び出し可能");
        m.insert("global_skills.invocable", "呼び出し可能");
        m.insert("global_skills.not_invocable", "呼び出し不可");
        m.insert("global_skills.editing", "編集中:");
        m.insert("global_skills.save", "保存");
        m.insert("global_skills.saved", "保存しました！");
        m.insert("global_skills.delete", "削除");
        m.insert("global_skills.deleted", "削除しました！");

        // ── Global Rules ──
        m.insert("global_rules.title", "グローバルルール");
        m.insert("global_rules.subtitle", "~/.claude/rules/ のルールを管理");
        m.insert("global_rules.loading", "ルールを読み込み中");
        m.insert("global_rules.no_rules", "グローバルルールが見つかりません");
        m.insert("global_rules.no_rules_hint", "~/.claude/rules/ に .md ファイルを作成してください");
        m.insert("global_rules.select_rule", "リストからルールを選択してください。");
        m.insert("global_rules.col_rule", "ルール");
        m.insert("global_rules.editing", "編集中:");
        m.insert("global_rules.save", "保存");
        m.insert("global_rules.saved", "保存しました！");
        m.insert("global_rules.delete", "削除");
        m.insert("global_rules.deleted", "削除しました！");

        // ── Plans ──
        m.insert("plans.title", "プラン");
        m.insert("plans.subtitle", "~/.claude/plans/ のプランファイルを管理");
        m.insert("plans.loading", "プランを読み込み中");
        m.insert("plans.no_plans", "プランが見つかりません");
        m.insert("plans.no_plans_hint", "プランは Claude Code が計画中に作成します。");
        m.insert("plans.select_plan", "リストからプランを選択してください。");
        m.insert("plans.col_plan", "プラン");
        m.insert("plans.col_modified", "更新日");
        m.insert("plans.modified", "更新日");
        m.insert("plans.plan_label", "プラン:");
        m.insert("plans.save", "保存");
        m.insert("plans.saved", "保存しました！");
        m.insert("plans.delete", "削除");
        m.insert("plans.deleted", "削除しました！");

        // ── Settings ──
        m.insert("settings.title", "設定");
        m.insert("settings.subtitle", "Claude Code の設定と Hooks を管理");
        m.insert("settings.tab_overview", "概要");
        m.insert("settings.tab_hooks", "Hook テンプレート");
        m.insert("settings.tab_storage", "ストレージ");
        m.insert("settings.loading", "設定を読み込み中");
        m.insert("settings.hooks_title", "Hooks");
        m.insert("settings.no_hooks", "Hooks が構成されていません");
        m.insert("settings.pre_tool_use", "PreToolUse");
        m.insert("settings.post_tool_use", "PostToolUse");
        m.insert("settings.matcher", "マッチャー");
        m.insert("settings.command", "コマンド");
        m.insert("settings.hook_templates_title", "Hook テンプレート");
        m.insert("settings.hook_templates_desc", "追加できるビルド済み Hook 構成。");
        m.insert("settings.hook_templates_loading", "テンプレートを読み込み中");
        m.insert("settings.add_hook", "追加");
        m.insert("settings.storage_title", "ストレージ使用量");
        m.insert("settings.storage_loading", "ストレージを計算中");
        m.insert("settings.storage_total", "合計");
        m.insert("settings.storage_dir", "ディレクトリ");
        m.insert("settings.storage_size", "サイズ");

        // ── Permissions ──
        m.insert("permissions.title", "権限");
        m.insert("permissions.subtitle", "プロジェクトの権限を確認・管理");
        m.insert("permissions.loading", "権限を読み込み中");
        m.insert("permissions.no_permissions", "権限が見つかりません");
        m.insert("permissions.col_project", "プロジェクト");
        m.insert("permissions.col_entries", "エントリ");
        m.insert("permissions.col_issues", "問題");
        m.insert("permissions.col_fragmented", "フラグメント化");
        m.insert("permissions.detail_title", "権限");
        m.insert("permissions.detail_loading", "権限を読み込み中");
        m.insert("permissions.detail_col_tool", "ツール");
        m.insert("permissions.detail_col_command", "コマンド");
        m.insert("permissions.detail_col_status", "ステータス");
        m.insert("permissions.detail_fragmented", "フラグメント化");
        m.insert("permissions.detail_security_issue", "セキュリティの問題");
        m.insert("permissions.detail_delete_selected", "選択項目を削除");
        m.insert("permissions.detail_deleted", "削除しました！");
        m.insert("permissions.detail_warnings_title", "セキュリティ警告");
        m.insert("permissions.health_title", "構成ヘルスチェック");
        m.insert("permissions.health_subtitle", "全プロジェクトのヘルスステータス");
        m.insert("permissions.health_loading", "ヘルスを計算中");
        m.insert("permissions.health_col_project", "プロジェクト");
        m.insert("permissions.health_col_score", "スコア");
        m.insert("permissions.health_col_issues", "問題");
        m.insert("permissions.health_avg", "平均");
        m.insert("permissions.subtitle_manage", "全プロジェクトの権限許可リストを管理");
        m.insert("permissions.col_actions", "アクション");
        m.insert("permissions.col_security_issues", "セキュリティの問題");
        m.insert("permissions.details", "詳細");
        m.insert("permissions.detail_subtitle", "権限エントリの確認と整理");
        m.insert("permissions.detail_deleting", "削除中...");
        m.insert("permissions.detail_deleted_reloading", "削除しました！再読み込み中...");
        m.insert("permissions.detail_delete_count", "選択項目を削除");
        m.insert("permissions.detail_col_index", "#");
        m.insert("permissions.detail_fragment", "フラグメント");
        m.insert("permissions.detail_ok", "OK");
        m.insert("permissions.detail_warnings_count", "セキュリティ警告");
        m.insert("permissions.detail_entry", "エントリ");
        m.insert("permissions.health_subtitle_scores", "全プロジェクトの構成ヘルススコア");
        m.insert("permissions.health_avg_score", "平均ヘルススコア");
        m.insert("permissions.health_projects_analyzed", "分析済みプロジェクト");
        m.insert("permissions.health_no_issues", "問題なし");

        // ── Analytics ──
        m.insert("analytics.title", "アナリティクス");
        m.insert("analytics.subtitle", "Claude Code 使用統計");
        m.insert("analytics.loading", "アナリティクスを読み込み中");
        m.insert("analytics.error_loading", "アナリティクスの読み込みエラー");
        m.insert("analytics.total_sessions", "合計セッション");
        m.insert("analytics.total_messages", "合計メッセージ");
        m.insert("analytics.git_commits", "Git コミット");
        m.insert("analytics.lines_added", "追加行数");
        m.insert("analytics.lines_removed", "削除行数");
        m.insert("analytics.since", "開始日");
        m.insert("analytics.activity_heatmap", "アクティビティヒートマップ");
        m.insert("analytics.messages", "メッセージ");
        m.insert("analytics.sessions", "セッション");
        m.insert("analytics.tool_calls", "ツール呼び出し");
        m.insert("analytics.hourly_distribution", "時間帯別分布");
        m.insert("analytics.model_usage", "モデル使用量");
        m.insert("analytics.col_model", "モデル");
        m.insert("analytics.col_input_tokens", "入力トークン");
        m.insert("analytics.col_output_tokens", "出力トークン");
        m.insert("analytics.col_cache_tokens", "キャッシュトークン");
        m.insert("analytics.tool_ranking", "ツールランキング");
        m.insert("analytics.col_cache_read", "キャッシュ読み取り");
        m.insert("analytics.tool_usage_top10", "ツール使用量 (上位10)");
        m.insert("analytics.languages", "言語");
        m.insert("analytics.session_outcomes", "セッション結果");
        m.insert("analytics.outcomes", "結果");

        // ── Sessions ──
        m.insert("sessions.title", "セッション");
        m.insert("sessions.subtitle", "Claude Code セッション履歴を閲覧");
        m.insert("sessions.loading", "セッションを読み込み中");
        m.insert("sessions.search_placeholder", "セッションを検索...");
        m.insert("sessions.no_sessions", "セッションが見つかりません");
        m.insert("sessions.col_project", "プロジェクト");
        m.insert("sessions.col_date", "日付");
        m.insert("sessions.col_duration", "所要時間");
        m.insert("sessions.col_messages", "メッセージ");
        m.insert("sessions.col_summary", "要約");
        m.insert("sessions.col_outcome", "結果");
        m.insert("sessions.minutes", "分");
        m.insert("sessions.load_more", "さらに読み込む");
        m.insert("sessions.detail_title", "セッション詳細");
        m.insert("sessions.detail_loading", "セッションを読み込み中");
        m.insert("sessions.detail_project", "プロジェクト");
        m.insert("sessions.detail_start", "開始");
        m.insert("sessions.detail_duration", "所要時間");
        m.insert("sessions.detail_messages", "メッセージ");
        m.insert("sessions.detail_tools", "ツール呼び出し");
        m.insert("sessions.detail_tokens", "トークン");
        m.insert("sessions.detail_git", "Git");
        m.insert("sessions.detail_first_prompt", "最初のプロンプト");
        m.insert("sessions.detail_summary", "要約");
        m.insert("sessions.back", "戻る");
        m.insert("sessions.searching", "検索中...");
        m.insert("sessions.search", "検索");
        m.insert("sessions.clear", "クリア");
        m.insert("sessions.search_results", "検索結果");
        m.insert("sessions.no_results", "結果が見つかりません");
        m.insert("sessions.col_prompt", "プロンプト");
        m.insert("sessions.session_prefix", "セッション: ");
        m.insert("sessions.detail_start_time", "開始時刻");
        m.insert("sessions.user_messages", " ユーザー / ");
        m.insert("sessions.assistant_messages", " アシスタント");
        m.insert("sessions.tokens_in", " 入力 / ");
        m.insert("sessions.tokens_out", " 出力");
        m.insert("sessions.commits_label", " コミット, +");
        m.insert("sessions.lines_minus", " / -");
        m.insert("sessions.tools_used", "使用ツール");
        m.insert("sessions.outcome_prefix", "結果: ");
        m.insert("sessions.showing", "表示中");
        m.insert("sessions.of", "/");
        m.insert("sessions.previous", "前へ");
        m.insert("sessions.next", "次へ");

        // ── GitHub ──
        m.insert("github.title", "GitHub");
        m.insert("github.subtitle", "GitHub 連携ステータス");
        m.insert("github.loading", "GitHub データを読み込み中");
        m.insert("github.auth_status", "認証ステータス");
        m.insert("github.username", "ユーザー名");
        m.insert("github.linked_repos", "リンク済みリポジトリ");
        m.insert("github.no_repos", "リンク済みリポジトリなし");
        m.insert("github.col_repo", "リポジトリ");
        m.insert("github.col_recent_commits", "最近のコミット");
        m.insert("github.col_open_prs", "オープン PR");

        // ── Help / System Info ──
        m.insert("help.title", "システム情報");
        m.insert("help.subtitle", "Claude Code システム情報");
        m.insert("help.loading", "システム情報を読み込み中");
        m.insert("help.account", "アカウント");
        m.insert("help.account_name", "名前");
        m.insert("help.account_email", "メール");
        m.insert("help.subscription", "サブスクリプション");
        m.insert("help.claude_version", "Claude Code バージョン");
        m.insert("help.gh_cli", "GitHub CLI");
        m.insert("help.skill_usage", "Skill 使用状況");
        m.insert("help.no_skill_usage", "Skill の使用記録がありません");
        m.insert("help.col_skill", "Skill");
        m.insert("help.col_count", "回数");
        m.insert("help.what_is_title", "ClaudeAdmin とは？");
        m.insert("help.what_is_desc", "ClaudeAdmin は Claude Code のビジュアル管理コンソールです。プロジェクト、Skills、ルール、メモリ、設定、Hooks、MCP サーバー、プランなど、Claude Code 構成のすべてを管理するための Web ベースのインターフェースを提供します。");
        m.insert("help.system_status", "システムステータス");
        m.insert("help.not_set", "未設定");
        m.insert("help.unknown", "不明");
        m.insert("help.not_found", "見つかりません");
        m.insert("help.not_installed", "未インストール");
        m.insert("help.concepts_title", "Claude Code の概念");
        m.insert("help.concept_skills", "YAML フロントマター付きの再利用可能なプロンプト。~/.claude/skills/（グローバル）または .claude/skills/（プロジェクト）に SKILL.md ファイルとして保存されます。");
        m.insert("help.concept_rules", "Claude の振る舞いを形作る制約とガイドライン。~/.claude/rules/ またはプロジェクトレベルに .md ファイルとして保存されます。");
        m.insert("help.concept_memory", "プロジェクトごとの永続的なメモ。MEMORY.md はシステムプロンプトに自動的に読み込まれます。パターン、設定、学習内容を保存します。");
        m.insert("help.concept_hooks", "イベント（PreToolUse、PostToolUse、Stop）によってトリガーされるシェルコマンド。自動フォーマットやリントなどのために settings.json で構成します。");
        m.insert("help.concept_mcp", "Model Context Protocol サーバーは Claude を外部ツールで拡張します。~/.claude.json で command、args、env を設定します。");
        m.insert("help.concept_claudemd", "プロジェクトレベルの指示ファイル。コンテキストとして自動的に読み込まれます。プロジェクトの規約、スタック情報、コーディングガイドラインを含みます。");
        m.insert("help.disclaimer", "ClaudeAdminは独立したコミュニティプロジェクトです。Anthropicとは提携、承認、認可されていません。ClaudeおよびClaude CodeはAnthropicの商標です。");

        m.insert("github.subtitle_detail", "GitHub CLI 連携とリンク済みリポジトリ");
        m.insert("github.linked_repositories", "リンク済みリポジトリ");
        m.insert("github.no_linked_repos", "~/.claude.json にリンクされた GitHub リポジトリはありません");
        m.insert("github.col_name", "名前");
        m.insert("github.col_path", "パス");

        // ── Skill Browser ──
        m.insert("skill_browser.title", "Skill ブラウザ");
        m.insert("skill_browser.subtitle", "公式およびコミュニティの Skills を発見してインストール");
        m.insert("skill_browser.loading", "Skills を読み込み中");
        m.insert("skill_browser.search_placeholder", "Skills を検索...");
        m.insert("skill_browser.no_results", "Skills が見つかりません");
        m.insert("skill_browser.installed", "インストール済み");
        m.insert("skill_browser.install", "インストール");
        m.insert("skill_browser.official", "公式");
        m.insert("skill_browser.community", "コミュニティ");
        m.insert("skill_browser.tab_official", "公式 (Anthropic)");
        m.insert("skill_browser.tab_community", "コミュニティ");
        m.insert("skill_browser.install_success", "のインストールに成功しました！");
        m.insert("skill_browser.install_failed", "インストールに失敗しました:");

        // ── Docs ──
        m.insert("docs.title", "ドキュメント");
        m.insert("docs.subtitle", "Claude Code 構成について知っておくべきすべてのこと");
        m.insert("docs.loading", "ドキュメントを読み込み中");

        // ── Docs: Table of Contents ──
        m.insert("docs.toc_contents", "目次");
        m.insert("docs.toc_why_claudeadmin", "なぜ ClaudeAdmin？");
        m.insert("docs.toc_capabilities", "できること・できないこと");
        m.insert("docs.toc_group", "コンセプト");
        m.insert("docs.toc_claude_md", "CLAUDE.md");
        m.insert("docs.toc_rules", "ルール");
        m.insert("docs.toc_skills", "Skills");
        m.insert("docs.toc_memory", "メモリ");
        m.insert("docs.toc_settings", "設定と Hooks");
        m.insert("docs.toc_mcp", "MCP サーバー");
        m.insert("docs.toc_plans", "プラン");
        m.insert("docs.toc_scopes", "グローバル vs. プロジェクト");
        m.insert("docs.toc_tips", "ヒントとベストプラクティス");
        m.insert("docs.toc_links", "公式ドキュメント");

        // ── Docs: Shared labels ──
        m.insert("docs.tips_heading", "ヒントとコツ");
        m.insert("docs.scope_global", "グローバル");
        m.insert("docs.scope_project", "プロジェクト");
        m.insert("docs.scope_user", "ユーザー");
        m.insert("docs.scope_parent", "親");
        m.insert("docs.scope_managed", "管理対象");
        m.insert("docs.scope_local", "ローカル");

        // ── Docs: Overview ──
        m.insert("docs.overview_heading", "なぜ ClaudeAdmin？");
        m.insert("docs.overview_callout", " は Claude Code 構成全体の中央管理コンソールです。多数の隠しディレクトリにわたる手動ファイル編集を、単一のビジュアルインターフェースに置き換えます。");
        m.insert("docs.overview_text1", "Claude Code はファイルとディレクトリの複雑な階層構造に構成を保存します：プロジェクトルートの CLAUDE.md ファイル、~/.claude/ サブディレクトリに散在するルールと Skills、エンコードされたプロジェクトパスをキーとするメモリファイル、複数の JSON ファイルの設定、~/.claude.json の MCP サーバー構成。プロジェクトが増えるにつれ、これらすべてを手動で管理するのはエラーが起きやすく、時間がかかります。");
        m.insert("docs.overview_text2", "ClaudeAdmin が提供するもの:");
        m.insert("docs.overview_li_visibility_label", "可視性");
        m.insert("docs.overview_li_visibility", " \u{2013} すべてのプロジェクト、Skills、ルール、メモリを一箇所で確認");
        m.insert("docs.overview_li_editing_label", "編集");
        m.insert("docs.overview_li_editing", " \u{2013} CLAUDE.md、ルール、Skills、メモリを適切なエディタで編集");
        m.insert("docs.overview_li_health_label", "ヘルスチェック");
        m.insert("docs.overview_li_health", " \u{2013} 権限のセキュリティ問題、重複ルール、欠落構成を検出");
        m.insert("docs.overview_li_analytics_label", "アナリティクス");
        m.insert("docs.overview_li_analytics", " \u{2013} Claude Code の使用状況を把握：セッション、トークン、ツール、コスト");
        m.insert("docs.overview_li_advisor_label", "アドバイザー");
        m.insert("docs.overview_li_advisor", " \u{2013} AI を活用したプロジェクト構成の改善提案");

        // ── Docs: Capabilities ──
        m.insert("docs.cap_heading", "ClaudeAdmin でできること・できないこと");
        m.insert("docs.cap_can_heading", "できること");
        m.insert("docs.cap_can_1", "~/.claude.json に登録されたすべてのプロジェクトを閲覧・管理");
        m.insert("docs.cap_can_2", "任意のプロジェクトの CLAUDE.md ファイルを表示・編集");
        m.insert("docs.cap_can_3", "グローバルおよびプロジェクトの Skills を作成、編集、削除");
        m.insert("docs.cap_can_4", "グローバルおよびプロジェクトのルールを作成、編集、削除");
        m.insert("docs.cap_can_5", "プロジェクトのメモリファイル（MEMORY.md とトピック）を表示・編集");
        m.insert("docs.cap_can_6", "設定の階層構造（グローバル \u{2192} プロジェクト \u{2192} ローカル）を検査");
        m.insert("docs.cap_can_7", "権限エントリを監査し、セキュリティの問題を検出");
        m.insert("docs.cap_can_8", "MCP サーバーの構成を表示");
        m.insert("docs.cap_can_9", "セッション履歴、トークン使用量、コストを分析");
        m.insert("docs.cap_can_10", "AI による実用的な推奨事項を含むプロジェクト分析を実行");
        m.insert("docs.cap_can_11", "コミュニティリポジトリから Skills を閲覧・インストール");
        m.insert("docs.cap_can_12", "すべての書き込みは ~/.claude/backups/ に自動バックアップを作成");
        m.insert("docs.cap_cannot_heading", "できないこと");
        m.insert("docs.cap_cannot_1", "Claude Code セッションの実行 \u{2013} 構成の管理であり、実行ではありません");
        m.insert("docs.cap_cannot_2", "管理対象ポリシー（エンタープライズ/組織レベルの設定）の変更");
        m.insert("docs.cap_cannot_3", "リモート環境や SSH セッションへのアクセス");
        m.insert("docs.cap_cannot_4", "実際のコーディング作業における Claude Code CLI の代替");
        m.insert("docs.cap_cannot_5", ".claude.json の MCP サーバーの直接編集（安全のため読み取り専用）");
        m.insert("docs.cap_cannot_6", "API キーや認証情報の管理");
        m.insert("docs.cap_cannot_callout", "ClaudeAdmin は構成マネージャーであり、Claude Code 自体の代替ではありません。データベース管理ツールのようなものと考えてください：検査、構成、保守を支援しますが、実際の作業は Claude Code で行います。");

        // ── Docs: CLAUDE.md ──
        m.insert("docs.claudemd_callout", "プロジェクトの憲法。CLAUDE.md は最も重要な構成ファイルです \u{2013} すべての Claude Code セッションに永続的なコンテキストとして自動的に読み込まれます。");
        m.insert("docs.claudemd_how_heading", "仕組み");
        m.insert("docs.claudemd_how_text", "Claude Code がセッションを開始すると、現在の作業ディレクトリからファイルシステムのルートまで再帰的に CLAUDE.md ファイルを検索します。見つかったすべてのファイルが読み込まれ連結されます。より近いファイルが優先されます。これにより、モノリポレベルの CLAUDE.md に共有規約を置き、パッケージレベルの CLAUDE.md ファイルに固有のオーバーライドを設定できます。");
        m.insert("docs.claudemd_locations_heading", "配置場所");
        m.insert("docs.claudemd_loc_project_or", " または ");
        m.insert("docs.claudemd_loc_parent", "モノリポのルート、すべてのサブパッケージに読み込まれる");
        m.insert("docs.claudemd_loc_user", "すべてのプロジェクトに適用される個人のデフォルト");
        m.insert("docs.claudemd_whatto_heading", "記載する内容");
        m.insert("docs.claudemd_whatto_context_label", "プロジェクトのコンテキスト");
        m.insert("docs.claudemd_whatto_context", " \u{2013} 技術スタック、アーキテクチャの決定、主要な依存関係");
        m.insert("docs.claudemd_whatto_standards_label", "コーディング標準");
        m.insert("docs.claudemd_whatto_standards", " \u{2013} 命名規則、フォーマットルール、エラーハンドリングパターン");
        m.insert("docs.claudemd_whatto_workflows_label", "ワークフロー");
        m.insert("docs.claudemd_whatto_workflows", " \u{2013} ビルド、テスト、デプロイの方法；ブランチ命名；PR 規約");
        m.insert("docs.claudemd_whatto_dodont_label", "すべきこと/すべきでないこと");
        m.insert("docs.claudemd_whatto_dodont", " \u{2013} 明示的な制約（例：\u{201c}TypeScript で any を使わない\u{201d}）");
        m.insert("docs.claudemd_whatto_team_label", "チーム合意");
        m.insert("docs.claudemd_whatto_team", " \u{2013} レビュープロセス、コミットメッセージ形式、モジュール境界");
        m.insert("docs.claudemd_tip1", "500行以下に保ちましょう。Claude はファイル全体をコンテキストに読み込みます \u{2013} 肥大化した CLAUDE.md はトークンを浪費し、重要な指示を薄めます。");
        m.insert("docs.claudemd_tip2", "明確なセクション見出し（## Architecture、## Conventions）を使用しましょう。Claude は関連するセクションを見つけるために構造を解析します。");
        m.insert("docs.claudemd_tip3", "最も重要なルールを先頭に置きましょう。長いファイルでは、冒頭のコンテンツがより注目されます。");
        m.insert("docs.claudemd_tip4", "CLAUDE.local.md を使用して、git にコミットすべきでない個人的な設定を記述しましょう。");
        m.insert("docs.claudemd_ext_link", "Anthropic ドキュメント: CLAUDE.md \u{2192}");

        // ── Docs: Rules ──
        m.insert("docs.rules_heading", "ルール");
        m.insert("docs.rules_callout", "Claude の振る舞いを形作るモジュール式のテーマ別制約。1つの大きなファイルである CLAUDE.md とは異なり、ルールは個別の .md ファイルで、それぞれが特定のトピックに焦点を当てています。");
        m.insert("docs.rules_how_heading", "仕組み");
        m.insert("docs.rules_how_text", "ルールはセッション開始時に自動的に読み込まれます。グローバルルール（個人の設定）が最初に読み込まれ、その上にプロジェクトルールが重ねられます。これにより、コーディングスタイルをグローバルに定義しつつ、プロジェクトがドメイン固有の制約を追加できます。");
        m.insert("docs.rules_locations_heading", "配置場所");
        m.insert("docs.rules_loc_global", "個人のルール、すべてのプロジェクトに適用");
        m.insert("docs.rules_loc_project", "プロジェクト固有、チーム共有のため git にコミット");
        m.insert("docs.rules_examples_heading", "例");
        m.insert("docs.rules_example_frontend", " \u{2013} React コンポーネントパターン、状態管理ルール");
        m.insert("docs.rules_example_security", " \u{2013} 入力検証、認証パターン、OWASP 準拠");
        m.insert("docs.rules_example_testing", " \u{2013} テスト構造、カバレッジ要件、モック戦略");
        m.insert("docs.rules_example_rust", " \u{2013} thiserror によるエラーハンドリング、モジュール構造、命名");
        m.insert("docs.rules_tip1", "1ファイルに1トピック。フロントエンドとバックエンドのルールを混ぜないでください \u{2013} 小さく焦点を絞ったファイルの方が保守・再利用が容易です。");
        m.insert("docs.rules_tip2", "グローバルルールは個人のスタイル設定に最適：優先言語、フォーマットツール、コミットメッセージ形式。");
        m.insert("docs.rules_tip3", "プロジェクトルールはグローバルルールを上書きします。競合がある場合、プロジェクトレベルのルールが優先されます。");
        m.insert("docs.rules_tip4", "ClaudeAdmin のヘルスチェックを使用して、グローバルとプロジェクトレベル間の重複ルールを検出しましょう。");
        m.insert("docs.rules_ext_link", "Anthropic ドキュメント: ルール \u{2192}");

        // ── Docs: Skills ──
        m.insert("docs.skills_heading", "Skills");
        m.insert("docs.skills_callout", "メタデータ付きの再利用可能な構造化プロンプト。Skills は Claude のプラグインのようなもので、コンテキストによって自動的にトリガーされるか、スラッシュコマンドで手動呼び出しできます。");
        m.insert("docs.skills_how_heading", "仕組み");
        m.insert("docs.skills_how_text", "各 Skill は YAML フロントマターとマークダウン本文を含む SKILL.md ファイルを持つ独自のディレクトリに存在します。フロントマターは説明やトリガー条件などのメタデータを定義します。本文には実際のプロンプト指示、例、参考資料が含まれます。");
        m.insert("docs.skills_structure_heading", "構造");
        m.insert("docs.skills_locations_heading", "配置場所");
        m.insert("docs.skills_loc_global", "すべてのプロジェクトで利用可能");
        m.insert("docs.skills_loc_project", "プロジェクト固有の Skills");
        m.insert("docs.skills_tip1", "フロントマターで user_invocable: true を設定すると、Claude Code で /skill-name として呼び出し可能になります。");
        m.insert("docs.skills_tip2", "SKILL.md に具体的な例を含めましょう。Claude は入出力の例があるとはるかに良いパフォーマンスを発揮します。");
        m.insert("docs.skills_tip3", "ClaudeAdmin の Skill ブラウザを使用して、コミュニティの Skills を発見・インストールしましょう。");
        m.insert("docs.skills_tip4", "Skill ディレクトリの参照ファイルは Skill がトリガーされたときのみ読み込まれ、トークンを節約します。");
        m.insert("docs.skills_ext_link", "Anthropic ドキュメント: Skills \u{2192}");

        // ── Docs: Memory ──
        m.insert("docs.memory_heading", "メモリ");
        m.insert("docs.memory_callout", "プロジェクトごとの Claude の永続的な知識ベース。メモリファイルは、Claude がセッションを通じて蓄積したパターン、設定、学習内容を保存します。");
        m.insert("docs.memory_how_heading", "仕組み");
        m.insert("docs.memory_how_text", "Claude Code は各プロジェクトのメモリディレクトリを ~/.claude/projects/<encoded-path>/memory/ に保持します。メインファイル MEMORY.md は特別な地位を持ち、最初の200行がセッション開始時にシステムプロンプトに読み込まれます。追加のトピックファイル（debugging.md、api-conventions.md など）は、Claude が現在のタスクに関連すると判断した場合にオンデマンドで読み込まれます。");
        m.insert("docs.memory_structure_heading", "構造");
        m.insert("docs.memory_auto_heading", "自動メモリ");
        m.insert("docs.memory_auto_text", "Claude Code はプロジェクトパターン、デバッグソリューション、ユーザーの設定を発見すると、自動的にメモリにエントリを追加できます。Claude Code の /memory コマンドまたは ClaudeAdmin のメモリエディタで、自動生成されたメモリを確認・編集できます。");
        m.insert("docs.memory_tip1", "MEMORY.md の最初の200行に最も重要な情報を置きましょう \u{2013} それが自動読み込みされる部分です。");
        m.insert("docs.memory_tip2", "詳細な知識にはトピックファイルを使用しましょう。必要なときのみ読み込まれるため、基本トークン使用量を低く抑えられます。");
        m.insert("docs.memory_tip3", "自動メモリを定期的に確認しましょう。Claude は時に過度に具体的な一回限りの解決策を保存することがあります。");
        m.insert("docs.memory_tip4", "メモリはプロジェクトごとです。別のプロジェクトに切り替えると、Claude は異なるメモリセットを取得します。");
        m.insert("docs.memory_ext_link", "Anthropic ドキュメント: メモリ \u{2192}");

        // ── Docs: Settings & Hooks ──
        m.insert("docs.settings_heading", "設定と Hooks");
        m.insert("docs.settings_heading_short", "設定");
        m.insert("docs.settings_callout", "振る舞い、権限、自動化のための JSON ベースの構成。Hooks を使うと、Claude がツールを使用する前後にシェルコマンドを自動的に実行できます。");
        m.insert("docs.settings_hierarchy_heading", "設定の階層");
        m.insert("docs.settings_hierarchy_text", "設定は特異性が増すレイヤードモデルに従います。より具体的なレイヤーが、より一般的なレイヤーを上書きします：");
        m.insert("docs.settings_managed_code", "エンタープライズポリシー");
        m.insert("docs.settings_managed_desc", "最高優先度、組織が設定（読み取り専用）");
        m.insert("docs.settings_global_desc", "個人のグローバル設定");
        m.insert("docs.settings_project_desc", "チーム設定、git にコミット");
        m.insert("docs.settings_local_desc", "個人のプロジェクトオーバーライド（gitignore 対象）");
        m.insert("docs.settings_hooks_heading", "Hooks");
        m.insert("docs.settings_hooks_text", "Hooks は Claude Code セッション中の特定のイベントでトリガーされるシェルコマンドです。settings.json の hooks キーで構成します。");
        m.insert("docs.settings_hooks_events", "イベント:\n\u{2022} PreToolUse  \u{2013} Claude がツールを実行する前（例：書き込み前の自動フォーマット）\n\u{2022} PostToolUse \u{2013} Claude がツールを実行した後（例：ファイル変更後のリント）\n\u{2022} Stop        \u{2013} Claude が応答を完了したとき");
        m.insert("docs.settings_tip1", "PreToolUse Hooks を使用して、Claude がファイルを書き込む前にコードを自動フォーマットしましょう。一貫したスタイルが確保されます。");
        m.insert("docs.settings_tip2", "PostToolUse Hooks はリントに最適：Claude がコードを変更した直後に問題をキャッチできます。");
        m.insert("docs.settings_tip3", "ClaudeAdmin の設定ページでは、すべてのレイヤーにわたる有効な Hook チェーンが表示されます。");
        m.insert("docs.settings_ext_link", "Anthropic ドキュメント: 設定 \u{2192}");
        m.insert("docs.settings_hooks_ext_link", "Anthropic ドキュメント: Hooks \u{2192}");

        // ── Docs: MCP Servers ──
        m.insert("docs.mcp_heading", "MCP サーバー");
        m.insert("docs.mcp_callout", "Model Context Protocol サーバーは Claude を外部ツールやデータソースで拡張します。データベース、API、ファイルシステム、その他のサービスと Claude がやり取りできるようにします。");
        m.insert("docs.mcp_how_heading", "仕組み");
        m.insert("docs.mcp_how_text", "MCP サーバーは Claude Code が起動し、MCP プロトコルを介して通信する外部プロセスです。各サーバーは Claude が呼び出せるツールのセットを提供します。構成は ~/.claude.json の mcpServers キーにあります。");
        m.insert("docs.mcp_config_heading", "構成");
        m.insert("docs.mcp_management_heading", "ClaudeAdmin での管理");
        m.insert("docs.mcp_management_text", "ClaudeAdmin は MCP サーバー管理専用のページを提供します：手動の JSON 編集なしにサーバーの表示、追加、編集、削除が可能です。ヘルスチェック機能は各サーバーを起動し、JSON-RPC の initialize と tools/list リクエストへの応答を検証します。MCP ブラウザを使用して、人気のサーバーをワンクリックで発見・インストールできます。");
        m.insert("docs.mcp_tip1", "MCP サーバーは .claude/settings.json でプロジェクトごとにも構成できます。");
        m.insert("docs.mcp_tip2", "シークレットには環境変数を使用しましょう \u{2013} 構成ファイルに API キーをハードコードしないでください。");
        m.insert("docs.mcp_tip3", "MCP ブラウザを使用して人気のサーバーを発見・インストールするか、「新規サーバー」タブでカスタムサーバーを追加しましょう。");
        m.insert("docs.mcp_ext_link", "Anthropic ドキュメント: MCP \u{2192}");
        m.insert("docs.mcp_spec_ext_link", "MCP 仕様 \u{2192}");

        // ── Docs: Plans ──
        m.insert("docs.plans_heading", "プラン");
        m.insert("docs.plans_callout", "Claude が複雑なタスクを分解するために使用するマークダウンファイル。プランは複数ステップの作業で Claude が集中力を維持し、進捗を追跡するのに役立ちます。");
        m.insert("docs.plans_how_heading", "仕組み");
        m.insert("docs.plans_how_text", "Claude が複雑なタスクに取り組む際、~/.claude/plans/ に保存されたプランファイルを作成または参照できます。プランはタスクリスト、依存関係、ステータス追跡を含む構造化されたマークダウンドキュメントです。セッション間で永続化されるため、Claude は中断したところから再開できます。");
        m.insert("docs.plans_location_heading", "配置場所");
        m.insert("docs.plans_loc_global", "すべてのプランファイル");
        m.insert("docs.plans_tip1", "複雑なリファクタリングの前に Claude に\u{201c}計画を立てて\u{201d}と依頼しましょう。プランは複数ファイルの変更でのミスを減らします。");
        m.insert("docs.plans_tip2", "古いプランを定期的にクリーンアップしましょう。ClaudeAdmin のプランページでは、更新日付付きですべての保存されたプランが表示されます。");

        // ── Docs: Scopes ──
        m.insert("docs.scopes_heading", "グローバル vs. プロジェクトスコープ");
        m.insert("docs.scopes_callout", "スコープの理解は効果的な Claude Code 構成の鍵です。すべての構成タイプはグローバル（個人のデフォルト）とプロジェクト固有（チームと共有）の2つのレイヤーに存在します。");
        m.insert("docs.scopes_overview_heading", "スコープの概要");
        m.insert("docs.scopes_col_type", "構成タイプ");
        m.insert("docs.scopes_col_global", "グローバル（ユーザー）");
        m.insert("docs.scopes_col_project", "プロジェクト");
        m.insert("docs.scopes_col_priority", "優先度");
        m.insert("docs.scopes_priority_project_global", "プロジェクト > グローバル");
        m.insert("docs.scopes_priority_both", "両方利用可能");
        m.insert("docs.scopes_memory_global", "~/.claude/projects/ にプロジェクトごと");
        m.insert("docs.scopes_priority_project_keyed", "プロジェクトキー");
        m.insert("docs.scopes_priority_local_project_global", "ローカル > プロジェクト > グローバル");
        m.insert("docs.scopes_priority_merged", "マージ");
        m.insert("docs.scopes_when_heading", "どちらを使うべき？");
        m.insert("docs.scopes_use_global", "グローバルの用途");
        m.insert("docs.scopes_global_1", "個人のコーディングスタイル設定");
        m.insert("docs.scopes_global_2", "優先言語とフレームワークのデフォルト");
        m.insert("docs.scopes_global_3", "コミットメッセージ形式");
        m.insert("docs.scopes_global_4", "エディタ/IDE 連携設定");
        m.insert("docs.scopes_global_5", "すべてのプロジェクトで使用する MCP サーバー");
        m.insert("docs.scopes_use_project", "プロジェクトの用途");
        m.insert("docs.scopes_project_1", "技術スタックのドキュメントと制約");
        m.insert("docs.scopes_project_2", "チームのコーディング規約");
        m.insert("docs.scopes_project_3", "ドメイン固有のルール（セキュリティ、コンプライアンス）");
        m.insert("docs.scopes_project_4", "プロジェクト固有の Skills とワークフロー");
        m.insert("docs.scopes_project_5", "CI/CD Hooks と自動化");

        // ── Docs: Tips & Best Practices ──
        m.insert("docs.bestpractices_heading", "ヒントとベストプラクティス");
        m.insert("docs.bestpractices_hygiene_heading", "構成の衛生");
        m.insert("docs.bestpractices_hygiene_1", "ClaudeAdmin の構成ヘルスチェックを定期的に実行しましょう。重複ルール、肥大化した権限リスト、欠落した CLAUDE.md ファイルを検出します。");
        m.insert("docs.bestpractices_hygiene_2", "繰り返しを避けましょう：グローバルにルールが存在する場合、プロジェクトの CLAUDE.md にコピーしないでください。スコープシステムを使いましょう。");
        m.insert("docs.bestpractices_hygiene_3", "権限リストをクリーンに保ちましょう。時間の経過とともに、Claude Code は数百の許可/拒否エントリを蓄積します。権限ページを使用して整理しましょう。");
        m.insert("docs.bestpractices_tokens_heading", "トークン効率");
        m.insert("docs.bestpractices_tokens_1", "CLAUDE.md、ルール、Skills（トリガー時）、MEMORY.md の最初の200行のすべてがコンテキストウィンドウにカウントされます。簡潔にしましょう。");
        m.insert("docs.bestpractices_tokens_2", "詳細な参考資料は Skill の参照ファイルやメモリのトピックファイルに移動しましょう \u{2013} 必要なときのみ読み込まれます。");
        m.insert("docs.bestpractices_tokens_3", "アナリティクスページを使用して、プロジェクトやセッションごとのトークン使用量を監視しましょう。");
        m.insert("docs.bestpractices_team_heading", "チームコラボレーション");
        m.insert("docs.bestpractices_team_1", ".claude/rules/ と .claude/skills/ を git にコミットしましょう。チーム全体で規約を共有できます。");
        m.insert("docs.bestpractices_team_2", "チーム設定には .claude/settings.json を、個人のオーバーライドには .claude/settings.local.json を使用しましょう。");
        m.insert("docs.bestpractices_team_3", "プロジェクトルートの CLAUDE.md はチームと Claude の契約です。ドキュメントと同様に扱い、PR で変更をレビューしましょう。");
        m.insert("docs.bestpractices_debug_heading", "Claude の振る舞いのデバッグ");
        m.insert("docs.bestpractices_debug_1", "Claude がルールを無視する場合、設定階層ページでレイヤー間の設定の競合を確認しましょう。");
        m.insert("docs.bestpractices_debug_2", "メモリが予期しない振る舞いの原因となることがあります。自動生成されたエントリを確認しましょう \u{2013} Claude が正しいアプローチの代わりに回避策を記憶している可能性があります。");
        m.insert("docs.bestpractices_debug_3", "セッションページを使用して過去の会話を確認し、Claude が何を\u{201c}考えていた\u{201d}かを理解しましょう。");

        // ── Docs: Links ──
        m.insert("docs.links_heading", "Anthropic 公式ドキュメント");
        m.insert("docs.links_text", "これらのリンクは Anthropic が管理する正式なドキュメントを指しています。ClaudeAdmin はこれらの仕様の上に構築されています。");
        m.insert("docs.link_overview_title", "Claude Code の概要");
        m.insert("docs.link_overview_desc", "はじめに、インストール、基本的な使い方");
        m.insert("docs.link_memory_title", "メモリと CLAUDE.md");
        m.insert("docs.link_memory_desc", "Claude がプロジェクトメモリをどのように保存・使用するか");
        m.insert("docs.link_skills_title", "Skills");
        m.insert("docs.link_skills_desc", "再利用可能な Skills の作成と管理");
        m.insert("docs.link_settings_title", "設定");
        m.insert("docs.link_settings_desc", "構成の階層とオプション");
        m.insert("docs.link_hooks_title", "Hooks");
        m.insert("docs.link_hooks_desc", "シェルコマンドによるイベント駆動の自動化");
        m.insert("docs.link_mcp_title", "MCP サーバー");
        m.insert("docs.link_mcp_desc", "外部ツールで Claude を拡張");
        m.insert("docs.link_bestpractices_title", "ベストプラクティス");
        m.insert("docs.link_bestpractices_desc", "効果的な Claude Code の使い方のヒント");
        m.insert("docs.link_mcp_spec_title", "MCP 仕様");
        m.insert("docs.link_mcp_spec_desc", "Model Context Protocol の標準仕様");

        // ── Licenses ──
        m.insert("sidebar.licenses", "\u{30e9}\u{30a4}\u{30bb}\u{30f3}\u{30b9}");
        m.insert("licenses.title", "\u{30e9}\u{30a4}\u{30bb}\u{30f3}\u{30b9}");
        m.insert("licenses.subtitle", "\u{30aa}\u{30fc}\u{30d7}\u{30f3}\u{30bd}\u{30fc}\u{30b9}\u{30e9}\u{30a4}\u{30bb}\u{30f3}\u{30b9}\u{3068}\u{4f9d}\u{5b58}\u{95a2}\u{4fc2}");
        m.insert("licenses.own_license", "ClaudeAdmin \u{30e9}\u{30a4}\u{30bb}\u{30f3}\u{30b9}");
        m.insert("licenses.third_party", "\u{30b5}\u{30fc}\u{30c9}\u{30d1}\u{30fc}\u{30c6}\u{30a3}\u{4f9d}\u{5b58}\u{95a2}\u{4fc2}");
        m.insert("licenses.col_name", "Crate");
        m.insert("licenses.col_version", "\u{30d0}\u{30fc}\u{30b8}\u{30e7}\u{30f3}");
        m.insert("licenses.col_license", "\u{30e9}\u{30a4}\u{30bb}\u{30f3}\u{30b9}");
        m.insert("licenses.search_placeholder", "\u{4f9d}\u{5b58}\u{95a2}\u{4fc2}\u{3092}\u{691c}\u{7d22}...");
        m.insert("licenses.loading", "\u{30e9}\u{30a4}\u{30bb}\u{30f3}\u{30b9}\u{3092}\u{8aad}\u{307f}\u{8fbc}\u{307f}\u{4e2d}");
        m.insert("licenses.count", "\u{500b}\u{306e}\u{4f9d}\u{5b58}\u{95a2}\u{4fc2}");
        m.insert("licenses.mit_copyright", "Copyright (c) 2024-2026");
        m.insert("licenses.mit_line1", "以下に定める条件に従い、本ソフトウェアおよび関連文書ファイル（以下「ソフトウェア」）の複製を取得するすべての人に対し、ソフトウェアを無制限に扱うことを無償で許可します。これには、ソフトウェアの複製を使用、複写、変更、結合、掲載、頒布、サブライセンス、および/または販売する権利、およびソフトウェアを提供する相手に同じことを許可する権利が含まれますが、これらに限定されません。ただし、以下の条件に従うものとします：");
        m.insert("licenses.mit_line2", "上記の著作権表示および本許諾表示を、ソフトウェアのすべての複製または重要な部分に記載するものとします。");
        m.insert("licenses.mit_line3", "ソフトウェアは「現状のまま」で、明示であるか暗黙であるかを問わず、何らの保証もなく提供されます。ここでいう保証とは、商品性、特定の目的への適合性、および権利非侵害についての保証も含みますが、それに限定されるものではありません。作者または著作権者は、契約行為、不法行為、またはそれ以外であろうと、ソフトウェアに起因または関連し、あるいはソフトウェアの使用またはその他の扱いによって生じるいかなる請求、損害、その他の義務について何らの責任も負わないものとします。");
        m.insert("licenses.direct_deps", "直接依存関係");
        m.insert("licenses.transitive_deps", "間接依存関係");
        m.insert("licenses.overview", "ライセンス概要");
        m.insert("licenses.direct_count", "直接");
        m.insert("licenses.transitive_count", "間接依存関係");

        // ── Components ──
        m.insert("component.modal.close", "閉じる");
        m.insert("component.editor.save", "保存");
        m.insert("component.editor.saved", "保存しました！");
        m.insert("component.json_editor.valid", "有効な JSON");
        m.insert("component.json_editor.invalid", "無効な JSON");
        m.insert("component.frontmatter.description", "説明");
        m.insert("component.frontmatter.user_invocable", "ユーザー呼び出し可能");
        m.insert("component.advisor.title", "プロジェクトアドバイザー");
        m.insert("component.advisor.analyze", "分析");
        m.insert("component.advisor.analyzing", "分析中...");
        m.insert("component.advisor.no_api_key", "ANTHROPIC_API_KEY が構成されていません");
        m.insert("component.advisor.error", "推奨事項の読み込みエラー");
        m.insert("component.advisor.summary", "要約");
        m.insert("component.advisor.recommendations", "推奨事項");
        m.insert("component.advisor.apply", "適用");
        m.insert("component.advisor.applied", "完了！");
        m.insert("component.advisor.analyze_project", "プロジェクトを分析");
        m.insert("component.advisor.hint", "Claude がプロジェクトを分析し推奨事項を提供します");
        m.insert("component.advisor.loading", "Claude がプロジェクトを分析中です");
        m.insert("component.advisor.assessment", "プロジェクト評価");
        m.insert("component.advisor.show_preview", "プレビューを表示");
        m.insert("component.advisor.category_tip", "ヒント");
        m.insert("component.frontmatter.user_invocable_label", "ユーザー呼び出し可能（/command で呼び出し可能）");
        m.insert("component.editor.saving", "保存中...");

        // ── Common ──
        m.insert("common.error", "エラー");
        m.insert("common.loading", "読み込み中");
        m.insert("common.save", "保存");
        m.insert("common.delete", "削除");
        m.insert("common.cancel", "キャンセル");
        m.insert("common.close", "閉じる");
        m.insert("common.yes", "はい");
        m.insert("common.no", "いいえ");
        m.insert("common.ok", "OK");
        m.insert("common.error_prefix", "エラー: ");
        m.insert("common.invalid_json", "無効な JSON: ");

        m
    })
}
