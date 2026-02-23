pub mod bottom_line;
pub mod color;
pub mod format;
pub mod top_line;
pub mod wrap;
pub use color::{color_for_percent, format_percent_label, SeverityColor};

#[derive(Debug, Clone, Default)]
pub struct RenderInput {
    pub repo: Option<String>,
    pub branch: Option<String>,
    pub permission: Option<String>,
    pub auth_label: Option<String>,
    pub provider_label: Option<String>,
    pub model_label: Option<String>,
    pub tool_count: Option<u64>,
    pub context_percent: Option<u8>,
    pub five_hour_percent: Option<u8>,
    pub weekly_percent: Option<u8>,
    pub duration_seconds: Option<u64>,
    pub tokens_per_second: Option<u64>,
    pub plan_done: Option<u64>,
    pub plan_total: Option<u64>,
    pub config_count: Option<u64>,
    pub git_dirty: Option<bool>,
    pub git_ahead: Option<u64>,
    pub git_behind: Option<u64>,
    pub git_file_stats: Option<String>,
    pub failure_count: Option<u64>,
    pub activity_summary: Option<String>,
    pub width: Option<usize>,
    pub colorblind_mode: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderOutput {
    pub logical_lines: Vec<String>,
    pub wrapped_lines: Vec<String>,
}

pub fn renderer_ready() -> bool {
    true
}

pub fn supports_third_activity_line_mode() -> bool {
    false
}

fn top_line_with_width(input: &RenderInput) -> String {
    let mut fields = vec![];
    if let Some(v) = input.repo.as_ref() {
        fields.push(format!("repo {v}"));
    }
    if let Some(v) = input.branch.as_ref() {
        fields.push(format!("branch {v}"));
    }
    if let Some(v) = input.permission.as_ref() {
        fields.push(format!("perm {v}"));
    }
    let normalized_auth = input.auth_label.as_ref().map(|v| {
        if v.trim().is_empty() {
            "Unknown".to_string()
        } else {
            v.clone()
        }
    });
    if let Some(v) = normalized_auth {
        fields.push(format!("auth {v}"));
    }

    let normalized_provider = input.provider_label.as_ref().map(|v| {
        if v.trim().is_empty() {
            "Custom".to_string()
        } else {
            v.clone()
        }
    });
    if let Some(v) = normalized_provider {
        fields.push(format!("provider {v}"));
    }
    if let Some(v) = input.model_label.as_ref() {
        fields.push(format!("model {v}"));
    }
    if let Some(v) = input.tool_count {
        fields.push(format!("tools {v}"));
    }
    if input.git_dirty == Some(true) {
        fields.push("dirty".to_string());
    }
    if let (Some(ahead), Some(behind)) = (input.git_ahead, input.git_behind) {
        fields.push(format!("↑{ahead}↓{behind}"));
    }
    if let Some(v) = input.git_file_stats.as_ref() {
        fields.push(v.clone());
    }

    if let Some(width) = input.width {
        while !fields.is_empty() && fields.join(" | ").len() > width {
            fields.pop();
        }
    }

    fields.join(" | ")
}

fn metric_bar(percent: u8, width: usize) -> String {
    let filled = ((percent as usize) * width) / 100;
    let empty = width.saturating_sub(filled);
    format!("{}{}", "█".repeat(filled), "░".repeat(empty))
}

fn severity_word(percent: u8) -> &'static str {
    if percent >= 85 {
        "critical"
    } else if percent >= 70 {
        "warn"
    } else {
        "normal"
    }
}

fn bottom_line(input: &RenderInput) -> String {
    let mut parts = Vec::new();
    if let Some(v) = input.context_percent {
        if input.colorblind_mode {
            parts.push(format!("CTX {v}% {} [{}]", severity_word(v), metric_bar(v, 20)));
        } else {
            parts.push(format!("CTX {v}% [{}]", metric_bar(v, 20)));
        }
    }
    if let Some(v) = input.five_hour_percent {
        parts.push(format!("5H {v}% [{}]", metric_bar(v, 20)));
    }
    if let Some(v) = input.weekly_percent {
        parts.push(format!("7D {v}% [{}]", metric_bar(v, 20)));
    }
    if let Some(v) = input.duration_seconds {
        parts.push(format!("dur {v}s"));
    }
    if let Some(v) = input.tokens_per_second {
        parts.push(format!("spd {v}t/s"));
    }
    if let (Some(done), Some(total)) = (input.plan_done, input.plan_total) {
        parts.push(format!("plan {done}/{total}"));
    }
    if let Some(v) = input.config_count {
        parts.push(format!("cfg {v}"));
    }
    if let Some(v) = input.failure_count {
        parts.push(format!("fail {v}"));
    }
    if let Some(v) = input.activity_summary.as_ref() {
        parts.push(format!("activity {v}"));
    }
    parts.join(" | ")
}

pub fn render_hud(input: &RenderInput) -> RenderOutput {
    let line1 = top_line_with_width(input);
    let line2 = bottom_line(input);
    let logical_lines = vec![line1, line2];
    let wrapped_lines = if let Some(width) = input.width {
        let mut out = Vec::new();
        out.extend(wrap::wrap_line_unicode_safe(&logical_lines[0], width));
        out.extend(wrap::wrap_line_unicode_safe(&logical_lines[1], width));
        out
    } else {
        logical_lines.clone()
    };
    RenderOutput {
        wrapped_lines,
        logical_lines,
    }
}
