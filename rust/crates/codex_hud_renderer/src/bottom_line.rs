use crate::RenderInput;

pub fn build_bottom_line(parts: &[String]) -> String {
    if parts.is_empty() {
        String::new()
    } else {
        parts.join(" | ")
    }
}

pub fn render_bottom_line(input: &RenderInput) -> String {
    let mut parts: Vec<String> = Vec::new();
    if let Some(v) = input.context_percent {
        parts.push(format!("CTX {v}%"));
    }
    if let Some(v) = input.five_hour_percent {
        parts.push(format!("5H {v}%"));
    }
    if let Some(v) = input.weekly_percent {
        parts.push(format!("7D {v}%"));
    }
    build_bottom_line(&parts)
}
