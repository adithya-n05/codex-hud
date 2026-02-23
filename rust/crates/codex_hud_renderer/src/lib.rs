pub mod bottom_line;
pub mod wrap;

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
    parts.join(" | ")
}

fn wrap_line(line: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return vec![line.to_string()];
    }

    let chars: Vec<char> = line.chars().collect();
    if chars.is_empty() {
        return vec![String::new()];
    }

    let mut out: Vec<String> = Vec::new();
    let mut index = 0;
    while index < chars.len() {
        let end = usize::min(index + width, chars.len());
        out.push(chars[index..end].iter().collect());
        index = end;
    }
    out
}

pub fn render_hud(input: &RenderInput) -> RenderOutput {
    let line1 = top_line_with_width(input);
    let line2 = bottom_line(input);
    let logical_lines = vec![line1, line2];
    let wrapped_lines = if let Some(width) = input.width {
        let mut out = Vec::new();
        out.extend(wrap_line(&logical_lines[0], width));
        out.extend(wrap_line(&logical_lines[1], width));
        out
    } else {
        logical_lines.clone()
    };
    RenderOutput {
        wrapped_lines,
        logical_lines,
    }
}
