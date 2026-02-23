pub mod defaults;
pub use defaults::VisualOptions;
pub use defaults::PrivacyOptions;
pub use defaults::FormatOptions;
pub use defaults::ToolCounterOptions;
pub use defaults::{count_tool_events, ToolCounterEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Preset {
    Minimal,
    Essential,
    Full,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HudConfig {
    pub preset: Preset,
    pub native: NativeToggles,
    pub derived: DerivedToggles,
    pub visual: VisualOptions,
    pub privacy: PrivacyOptions,
    pub format: FormatOptions,
    pub tool_counter: ToolCounterOptions,
}

impl Default for HudConfig {
    fn default() -> Self {
        Self {
            preset: Preset::Essential,
            native: NativeToggles::default(),
            derived: DerivedToggles::default(),
            visual: VisualOptions::default(),
            privacy: PrivacyOptions::default(),
            format: FormatOptions::default(),
            tool_counter: ToolCounterOptions::default(),
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

pub fn parse_hud_config(src: &str) -> Result<HudConfig, String> {
    let value = src.parse::<toml::Value>().map_err(|e| e.to_string())?;

    if value
        .get("project")
        .and_then(|v| v.get("codex_hud"))
        .is_some()
    {
        return Err("project-level codex_hud config is not supported".to_string());
    }

    let warn = value
        .get("tui")
        .and_then(|v| v.get("codex_hud"))
        .and_then(|v| v.get("visual"))
        .and_then(|v| v.get("warn_percent"))
        .and_then(toml::Value::as_integer)
        .unwrap_or(70);

    let critical = value
        .get("tui")
        .and_then(|v| v.get("codex_hud"))
        .and_then(|v| v.get("visual"))
        .and_then(|v| v.get("critical_percent"))
        .and_then(toml::Value::as_integer)
        .unwrap_or(85);

    if !(0..=100).contains(&warn) || !(0..=100).contains(&critical) {
        return Err("threshold must be between 0 and 100".to_string());
    }

    if warn > critical {
        return Err("warn_percent must be <= critical_percent".to_string());
    }

    Ok(HudConfig::default())
}

pub fn apply_preset(cfg: &mut HudConfig, preset: Preset) {
    cfg.preset = preset;
    match preset {
        Preset::Minimal => {
            cfg.derived.provider_chip = false;
            cfg.derived.five_hour_bar = false;
            cfg.derived.weekly_bar = false;
        }
        Preset::Essential => {
            cfg.derived.provider_chip = true;
            cfg.derived.five_hour_bar = true;
            cfg.derived.weekly_bar = true;
        }
        Preset::Full => {
            cfg.derived.provider_chip = true;
            cfg.derived.five_hour_bar = true;
            cfg.derived.weekly_bar = true;
            cfg.derived.tool_counter = true;
            cfg.derived.failure_count = true;
            cfg.derived.activity_summary = true;
        }
    }
}

#[derive(Debug, Clone)]
pub struct EditSession {
    current: HudConfig,
    previous_before_preset: Option<HudConfig>,
}

impl EditSession {
    pub fn new(current: HudConfig) -> Self {
        Self {
            current,
            previous_before_preset: None,
        }
    }

    pub fn apply_preset(&mut self, preset: Preset) {
        self.previous_before_preset = Some(self.current.clone());
        apply_preset(&mut self.current, preset);
    }

    pub fn undo_last_preset(&mut self) -> Result<(), String> {
        let previous = self
            .previous_before_preset
            .take()
            .ok_or_else(|| "no preset change to undo".to_string())?;
        self.current = previous;
        Ok(())
    }

    pub fn current(&self) -> &HudConfig {
        &self.current
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownDisplayPolicy {
    pub provider_unknown_label: String,
    pub auth_unknown_label: String,
    pub render_unknown_explicitly: bool,
}

impl Default for UnknownDisplayPolicy {
    fn default() -> Self {
        Self {
            provider_unknown_label: "Custom".to_string(),
            auth_unknown_label: "Unknown".to_string(),
            render_unknown_explicitly: true,
        }
    }
}

pub fn domain_ready() -> bool {
    true
}
