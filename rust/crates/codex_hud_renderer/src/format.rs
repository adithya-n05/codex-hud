fn usage_bar(percent: u8, width: usize) -> String {
    let filled = ((percent as usize) * width) / 100;
    let empty = width.saturating_sub(filled);
    format!("{}{}", "█".repeat(filled), "░".repeat(empty))
}

pub fn metric_fragment(label: &str, percent: u8, usage_mode: Option<&str>) -> String {
    if usage_mode == Some("text") {
        return format!("{label} {percent}%");
    }
    format!("{label} {percent}% [{}]", usage_bar(percent, 8))
}

pub fn compact_path(path: &str, segments: usize) -> String {
    let parts: Vec<&str> = path.split('/').filter(|v| !v.is_empty()).collect();
    if parts.len() <= segments {
        return parts.join("/");
    }
    parts[parts.len() - segments..].join("/")
}

pub fn format_usage_text(
    context_percent: Option<u8>,
    five_hour_percent: Option<u8>,
    weekly_percent: Option<u8>,
) -> String {
    let mut parts: Vec<String> = Vec::new();
    if let Some(v) = context_percent {
        parts.push(format!("CTX {v}%"));
    }
    if let Some(v) = five_hour_percent {
        parts.push(format!("5H {v}%"));
    }
    if let Some(v) = weekly_percent {
        parts.push(format!("7D {v}%"));
    }
    parts.join(" | ")
}
