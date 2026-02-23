pub fn build_bottom_line(parts: &[String]) -> String {
    if parts.is_empty() {
        String::new()
    } else {
        parts.join(" | ")
    }
}
