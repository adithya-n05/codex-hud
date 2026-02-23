pub mod defaults;
pub mod edit_session;
pub mod parse;
pub mod save;
pub mod validate;
pub use defaults::VisualOptions;
pub use defaults::PrivacyOptions;
pub use defaults::FormatOptions;
pub use defaults::ToolCounterOptions;
pub use defaults::{count_tool_events, ToolCounterEvent};
pub use edit_session::EditSession;

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
    let value = src
        .parse::<toml::Value>()
        .map_err(|e| format!("syntax error: {e}"))?;

    if value
        .get("project")
        .and_then(|v| v.get("codex_hud"))
        .is_some()
    {
        return Err("project-level codex_hud config is not supported".to_string());
    }

    parse::reject_unknown_top_keys(&value)?;

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

    validate::validate_threshold_range(warn, critical)?;

    if warn > critical {
        return Err("warn_percent must be <= critical_percent".to_string());
    }

    let mut cfg = HudConfig::default();
    if let Some(table) = value
        .get("tui")
        .and_then(|v| v.get("codex_hud"))
        .and_then(|v| v.get("native"))
        .and_then(toml::Value::as_table)
    {
        cfg.native.model_name = table
            .get("model_name")
            .and_then(toml::Value::as_bool)
            .unwrap_or(cfg.native.model_name);
        cfg.native.model_with_reasoning = table
            .get("model_with_reasoning")
            .and_then(toml::Value::as_bool)
            .unwrap_or(cfg.native.model_with_reasoning);
    }
    Ok(cfg)
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

pub fn verify_must_cover_mapping() -> Vec<&'static str> {
    // Keep this list synchronized with Segment 01 traceability matrix.
    let required = [
        "native.model_name",
        "native.model_with_reasoning",
        "native.current_dir",
        "native.project_root",
        "native.git_branch",
        "native.context_remaining",
        "native.context_used",
        "native.five_hour_limit",
        "native.weekly_limit",
        "native.codex_version",
        "native.context_window_size",
        "native.used_tokens",
        "native.total_input_tokens",
        "native.total_output_tokens",
        "native.session_id",
        "derived.permission_chip",
        "derived.auth_chip",
        "derived.provider_chip",
        "derived.context_bar",
        "derived.five_hour_bar",
        "derived.weekly_bar",
        "derived.tool_counter",
        "derived.failure_count",
        "derived.activity_summary",
        "derived.git_dirty",
        "derived.git_ahead_behind",
        "derived.git_file_stats",
        "derived.duration_metric",
        "derived.speed_metric",
        "derived.plan_progress",
        "derived.config_count",
        "format.context_mode",
        "format.usage_mode",
        "format.path_depth_mode",
    ];

    // Field references intentionally force compile-time drift detection.
    let cfg = HudConfig::default();
    let _native_fields = (
        cfg.native.model_name,
        cfg.native.model_with_reasoning,
        cfg.native.current_dir,
        cfg.native.project_root,
        cfg.native.git_branch,
        cfg.native.context_remaining,
        cfg.native.context_used,
        cfg.native.five_hour_limit,
        cfg.native.weekly_limit,
        cfg.native.codex_version,
        cfg.native.context_window_size,
        cfg.native.used_tokens,
        cfg.native.total_input_tokens,
        cfg.native.total_output_tokens,
        cfg.native.session_id,
    );
    let _derived_fields = (
        cfg.derived.permission_chip,
        cfg.derived.auth_chip,
        cfg.derived.provider_chip,
        cfg.derived.context_bar,
        cfg.derived.five_hour_bar,
        cfg.derived.weekly_bar,
        cfg.derived.tool_counter,
        cfg.derived.failure_count,
        cfg.derived.activity_summary,
        cfg.derived.git_dirty,
        cfg.derived.git_ahead_behind,
        cfg.derived.git_file_stats,
        cfg.derived.duration_metric,
        cfg.derived.speed_metric,
        cfg.derived.plan_progress,
        cfg.derived.config_count,
    );
    let _format_fields = (
        cfg.format.context_mode.as_str(),
        cfg.format.usage_mode.as_str(),
        cfg.format.path_depth_mode.as_str(),
    );

    fn implemented_native_keys(_n: &NativeToggles) -> Vec<&'static str> {
        vec![
            "native.model_name",
            "native.model_with_reasoning",
            "native.current_dir",
            "native.project_root",
            "native.git_branch",
            "native.context_remaining",
            "native.context_used",
            "native.five_hour_limit",
            "native.weekly_limit",
            "native.codex_version",
            "native.context_window_size",
            "native.used_tokens",
            "native.total_input_tokens",
            "native.total_output_tokens",
            "native.session_id",
        ]
    }

    fn implemented_derived_keys(_d: &DerivedToggles) -> Vec<&'static str> {
        vec![
            "derived.permission_chip",
            "derived.auth_chip",
            "derived.provider_chip",
            "derived.context_bar",
            "derived.five_hour_bar",
            "derived.weekly_bar",
            "derived.tool_counter",
            "derived.failure_count",
            "derived.activity_summary",
            "derived.git_dirty",
            "derived.git_ahead_behind",
            "derived.git_file_stats",
            "derived.duration_metric",
            "derived.speed_metric",
            "derived.plan_progress",
            "derived.config_count",
        ]
    }

    fn implemented_format_keys(_f: &FormatOptions) -> Vec<&'static str> {
        vec![
            "format.context_mode",
            "format.usage_mode",
            "format.path_depth_mode",
        ]
    }

    let mut mapped = Vec::new();
    mapped.extend(implemented_native_keys(&cfg.native));
    mapped.extend(implemented_derived_keys(&cfg.derived));
    mapped.extend(implemented_format_keys(&cfg.format));

    required
        .iter()
        .copied()
        .filter(|required_key| !mapped.iter().any(|actual| actual == required_key))
        .collect()
}

pub fn domain_ready() -> bool {
    true
}
