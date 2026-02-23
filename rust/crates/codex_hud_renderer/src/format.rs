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
