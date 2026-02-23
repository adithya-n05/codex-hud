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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderOutput {
    pub logical_lines: Vec<String>,
    pub wrapped_lines: Vec<String>,
}

pub fn renderer_ready() -> bool {
    true
}

fn top_line(input: &RenderInput) -> String {
    let mut parts = Vec::new();
    if let Some(v) = input.repo.as_ref() {
        parts.push(format!("repo {v}"));
    }
    if let Some(v) = input.branch.as_ref() {
        parts.push(format!("branch {v}"));
    }
    if let Some(v) = input.permission.as_ref() {
        parts.push(format!("perm {v}"));
    }
    if let Some(v) = input.auth_label.as_ref() {
        parts.push(format!("auth {v}"));
    }
    if let Some(v) = input.provider_label.as_ref() {
        parts.push(format!("provider {v}"));
    }
    if let Some(v) = input.model_label.as_ref() {
        parts.push(format!("model {v}"));
    }
    if let Some(v) = input.tool_count {
        parts.push(format!("tools {v}"));
    }
    parts.join(" | ")
}

fn metric_bar(percent: u8, width: usize) -> String {
    let filled = ((percent as usize) * width) / 100;
    let empty = width.saturating_sub(filled);
    format!("{}{}", "█".repeat(filled), "░".repeat(empty))
}

fn bottom_line(input: &RenderInput) -> String {
    let mut parts = Vec::new();
    if let Some(v) = input.context_percent {
        parts.push(format!("CTX {v}% [{}]", metric_bar(v, 20)));
    }
    if let Some(v) = input.five_hour_percent {
        parts.push(format!("5H {v}% [{}]", metric_bar(v, 20)));
    }
    if let Some(v) = input.weekly_percent {
        parts.push(format!("7D {v}% [{}]", metric_bar(v, 20)));
    }
    parts.join(" | ")
}

pub fn render_hud(input: &RenderInput) -> RenderOutput {
    let line1 = top_line(input);
    let line2 = bottom_line(input);
    let logical_lines = vec![line1, line2];
    RenderOutput {
        wrapped_lines: logical_lines.clone(),
        logical_lines,
    }
}
