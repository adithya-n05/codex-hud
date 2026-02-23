pub mod defaults;
pub use defaults::VisualOptions;
pub use defaults::PrivacyOptions;
pub use defaults::FormatOptions;
pub use defaults::ToolCounterOptions;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Preset {
    Minimal,
    Essential,
    Full,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HudConfig {
    pub preset: Preset,
}

impl Default for HudConfig {
    fn default() -> Self {
        Self {
            preset: Preset::Essential,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeToggles {
    pub model_name: bool,
    pub model_with_reasoning: bool,
    pub current_dir: bool,
    pub project_root: bool,
    pub git_branch: bool,
    pub context_remaining: bool,
    pub context_used: bool,
    pub five_hour_limit: bool,
    pub weekly_limit: bool,
    pub codex_version: bool,
    pub context_window_size: bool,
    pub used_tokens: bool,
    pub total_input_tokens: bool,
    pub total_output_tokens: bool,
    pub session_id: bool,
}

impl Default for NativeToggles {
    fn default() -> Self {
        Self {
            model_name: false,
            model_with_reasoning: true,
            current_dir: false,
            project_root: false,
            git_branch: true,
            context_remaining: true,
            context_used: false,
            five_hour_limit: true,
            weekly_limit: true,
            codex_version: false,
            context_window_size: false,
            used_tokens: false,
            total_input_tokens: false,
            total_output_tokens: false,
            session_id: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivedToggles {
    pub permission_chip: bool,
    pub auth_chip: bool,
    pub provider_chip: bool,
    pub context_bar: bool,
    pub five_hour_bar: bool,
    pub weekly_bar: bool,
    pub tool_counter: bool,
    pub failure_count: bool,
    pub activity_summary: bool,
    pub git_dirty: bool,
    pub git_ahead_behind: bool,
    pub git_file_stats: bool,
    pub duration_metric: bool,
    pub speed_metric: bool,
    pub plan_progress: bool,
    pub config_count: bool,
}

impl Default for DerivedToggles {
    fn default() -> Self {
        Self {
            permission_chip: true,
            auth_chip: true,
            provider_chip: true,
            context_bar: true,
            five_hour_bar: true,
            weekly_bar: true,
            tool_counter: false,
            failure_count: false,
            activity_summary: false,
            git_dirty: false,
            git_ahead_behind: false,
            git_file_stats: false,
            duration_metric: false,
            speed_metric: false,
            plan_progress: false,
            config_count: false,
        }
    }
}

pub fn domain_ready() -> bool {
    true
}
