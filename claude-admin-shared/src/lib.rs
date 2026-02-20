use serde::{Deserialize, Serialize};

// === Enums ===

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ConfigScope {
    Global,
    Project,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ClaudeFileType {
    ClaudeMd,
    Rule,
    Skill,
    Memory,
    Settings,
    Plan,
}

// === Dashboard ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardOverview {
    pub global_skills_count: usize,
    pub global_rules_count: usize,
    pub projects_count: usize,
    pub mcp_servers_count: usize,
    pub plans_count: usize,
    pub recent_projects: Vec<ProjectSummaryLite>,
    pub conflicts: Vec<ConflictInfo>,
    /// Health score is loaded lazily via /api/v1/dashboard/health
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_score: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictInfo {
    pub name: String,
    pub file_type: ClaudeFileType,
    pub global_path: String,
    pub project_path: String,
}

// === Projects ===

/// Lightweight project info - instant from ~/.claude.json, no filesystem checks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSummaryLite {
    pub path: String,
    pub encoded_path: String,
    pub name: String,
}

/// Filesystem status for a single project - loaded JIT on demand.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectStatus {
    pub has_claude_md: bool,
    pub has_claude_dir: bool,
    pub has_rules: bool,
    pub has_skills: bool,
    pub has_memory: bool,
}

/// Full summary (lite + status combined) - used internally and for project detail.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSummary {
    pub path: String,
    pub encoded_path: String,
    pub name: String,
    pub has_claude_md: bool,
    pub has_claude_dir: bool,
    pub has_rules: bool,
    pub has_skills: bool,
    pub has_memory: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDetail {
    pub summary: ProjectSummary,
    pub claude_md: Option<String>,
    pub rules: Vec<RuleFile>,
    pub skills: Vec<SkillFile>,
    pub memory_files: Vec<MemoryFile>,
}

// === Skills ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillFile {
    pub name: String,
    pub path: String,
    pub scope: ConfigScope,
    pub frontmatter: SkillFrontmatter,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SkillFrontmatter {
    pub description: Option<String>,
    pub user_invocable: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillCreateRequest {
    pub name: String,
    pub scope: ConfigScope,
    pub frontmatter: SkillFrontmatter,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillUpdateRequest {
    pub frontmatter: SkillFrontmatter,
    pub content: String,
}

// === Rules ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleFile {
    pub name: String,
    pub path: String,
    pub scope: ConfigScope,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleCreateRequest {
    pub name: String,
    pub scope: ConfigScope,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleUpdateRequest {
    pub content: String,
}

// === Memory ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryFile {
    pub name: String,
    pub path: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryUpdateRequest {
    pub content: String,
}

// === Settings ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsOverview {
    pub global_settings: serde_json::Value,
    pub hooks: HooksConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HooksConfig {
    pub pre_tool_use: Vec<HookEntry>,
    pub post_tool_use: Vec<HookEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookEntry {
    pub matcher: String,
    pub hooks: Vec<HookCommand>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookCommand {
    #[serde(rename = "type")]
    pub hook_type: String,
    pub command: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsUpdateRequest {
    pub settings: serde_json::Value,
}

// === MCP Servers ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerInfo {
    pub name: String,
    pub scope: ConfigScope,
    pub project_path: Option<String>,
    pub config: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerDetail {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub env: std::collections::HashMap<String, String>,
    pub raw_config: serde_json::Value,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum McpServerStatus {
    Running,
    Error,
    Timeout,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpHealthResult {
    pub name: String,
    pub status: McpServerStatus,
    pub server_info: Option<String>,
    pub tools: Vec<McpToolInfo>,
    pub duration_ms: u64,
    pub error: Option<String>,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpToolInfo {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerCreateRequest {
    pub name: String,
    pub config: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerUpdateRequest {
    pub config: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowsableMcpServer {
    pub name: String,
    pub description: String,
    pub category: String,
    pub npm_package: String,
    pub default_config: serde_json::Value,
    pub installed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpInstallRequest {
    pub name: String,
    pub config: serde_json::Value,
}

// === Plans ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanFile {
    pub name: String,
    pub path: String,
    pub content: String,
    pub modified: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanUpdateRequest {
    pub content: String,
}

// === Claude MD ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeMdContent {
    pub content: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeMdUpdateRequest {
    pub content: String,
}

// === AI ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestionRequest {
    pub file_type: ClaudeFileType,
    pub content: String,
    pub context: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestionResponse {
    pub suggestions: Vec<String>,
    pub improved_content: Option<String>,
    pub validation_issues: Vec<String>,
}

// === Project Advisor ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvisorReport {
    pub project_summary: String,
    pub recommendations: Vec<AdvisorRecommendation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvisorRecommendation {
    pub category: AdvisorCategory,
    pub title: String,
    pub description: String,
    pub action: Option<AdvisorAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AdvisorCategory {
    GlobalSkill,
    GlobalRule,
    ClaudeMd,
    Memory,
    Hooks,
    General,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvisorAction {
    pub label: String,
    pub action_type: AdvisorActionType,
    pub payload: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AdvisorActionType {
    CreateClaudeMd,
    UpdateClaudeMd,
    EnableSkill,
    CreateRule,
    InitMemory,
}

// === Health ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

/// Lazy-loaded dashboard health score.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardHealthScore {
    pub health_score: u8,
}

// === Claude JSON (for parsing ~/.claude.json) ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeJsonOverview {
    pub projects: serde_json::Value,
    pub mcp_servers: serde_json::Value,
}

// === Phase 8: Permissions ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectPermissionSummary {
    pub project_id: String,
    pub project_name: String,
    pub path: String,
    pub total_entries: usize,
    pub security_issues: usize,
    pub fragmented_entries: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectPermissions {
    pub project_id: String,
    pub entries: Vec<PermissionEntry>,
    pub security_warnings: Vec<SecurityWarning>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionEntry {
    pub index: usize,
    pub tool: String,
    pub command: String,
    pub is_fragmented: bool,
    pub security_issue: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityWarning {
    pub index: usize,
    pub severity: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionDeleteRequest {
    pub indices: Vec<usize>,
}

// === Phase 8: Config Health ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthOverview {
    pub projects: Vec<ProjectHealthSummary>,
    pub average_score: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectHealthSummary {
    pub project_id: String,
    pub name: String,
    pub score: u8,
    pub issues: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectHealth {
    pub score: u8,
    pub has_claude_md: bool,
    pub has_memory: bool,
    pub permission_count: usize,
    pub security_issues: Vec<SecurityWarning>,
    pub duplicated_rules: Vec<DuplicatedRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicatedRule {
    pub text: String,
    pub found_in_project: String,
    pub also_in_global: String,
}

// === Phase 9: Skill Browser ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowsableSkill {
    pub name: String,
    pub description: String,
    pub source: SkillSource,
    pub repo: String,
    pub path: String,
    pub installed: bool,
    pub category: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SkillSource {
    Official,
    Community,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillDetail {
    pub name: String,
    pub content: String,
    pub frontmatter: SkillFrontmatter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillInstallRequest {
    pub name: String,
    pub content: String,
}

// === Phase 10: Settings Hierarchy ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsHierarchy {
    pub layers: Vec<SettingsLayer>,
    pub effective_hooks: Vec<EffectiveHook>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsLayer {
    pub scope: String,
    pub path: String,
    pub content: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectiveHook {
    pub event: String,
    pub matcher: Option<String>,
    pub command: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookTemplate {
    pub name: String,
    pub description: String,
    pub event: String,
    pub matcher: Option<String>,
    pub command: String,
}

// === Phase 11: Analytics ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsOverview {
    pub total_sessions: u64,
    pub total_messages: u64,
    pub first_session_date: Option<String>,
    pub daily_activity: Vec<DailyActivity>,
    pub hour_distribution: Vec<(u8, u64)>,
    pub model_usage: Vec<ModelUsageEntry>,
    pub tool_ranking: Vec<(String, u64)>,
    pub language_breakdown: Vec<(String, u64)>,
    pub outcome_distribution: Vec<(String, u64)>,
    pub total_git_commits: u64,
    pub total_lines_added: u64,
    pub total_lines_removed: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyActivity {
    pub date: String,
    pub message_count: u64,
    pub session_count: u64,
    pub tool_call_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelUsageEntry {
    pub model: String,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cache_read_tokens: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectAnalytics {
    pub path: String,
    pub name: String,
    pub session_count: u64,
    pub total_input_tokens: u64,
    pub total_output_tokens: u64,
    pub estimated_cost_usd: f64,
    pub languages: Vec<(String, u64)>,
}

// === Phase 12: Sessions ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionListResponse {
    pub sessions: Vec<SessionSummary>,
    pub total: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSummary {
    pub session_id: String,
    pub project_name: String,
    pub start_time: String,
    pub duration_minutes: u64,
    pub message_count: u64,
    pub summary: Option<String>,
    pub outcome: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionDetail {
    pub session_id: String,
    pub project_path: String,
    pub start_time: String,
    pub duration_minutes: u64,
    pub user_message_count: u64,
    pub assistant_message_count: u64,
    pub tool_counts: Vec<(String, u64)>,
    pub languages: Vec<(String, u64)>,
    pub git_commits: u64,
    pub lines_added: u64,
    pub lines_removed: u64,
    pub files_modified: u64,
    pub first_prompt: Option<String>,
    pub summary: Option<String>,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub outcome: Option<String>,
    pub helpfulness: Option<String>,
    pub brief_summary: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub display: String,
    pub timestamp: u64,
    pub project: String,
    pub session_id: String,
}

// === Phase 13: System Info & GitHub ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub account_name: Option<String>,
    pub account_email: Option<String>,
    pub subscription_type: Option<String>,
    pub claude_code_version: Option<String>,
    pub gh_cli_status: Option<String>,
    pub skill_usage: Vec<(String, u64)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubOverview {
    pub auth_status: String,
    pub username: Option<String>,
    pub linked_repos: Vec<GitHubRepo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubRepo {
    pub path: String,
    pub name: String,
    pub recent_commits: Vec<GitHubCommit>,
    pub open_prs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubCommit {
    pub sha: String,
    pub message: String,
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageInfo {
    pub total_bytes: u64,
    pub directories: Vec<StorageDirectory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageDirectory {
    pub name: String,
    pub bytes: u64,
}

// === Licenses ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicensesResponse {
    pub own_license: String,
    pub direct_dependencies: Vec<DependencyInfo>,
    pub transitive_dependencies: Vec<DependencyInfo>,
    pub license_summary: Vec<LicenseSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyInfo {
    pub name: String,
    pub version: String,
    pub license: String,
    pub repository: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseSummary {
    pub license: String,
    pub count: usize,
}
