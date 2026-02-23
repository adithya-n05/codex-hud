pub fn sanitize_auth_label(value: &str) -> String {
    let lower = value.to_ascii_lowercase();
    if lower.contains("bearer ") || lower.contains("sk-") {
        "Bearer token".to_string()
    } else {
        value.to_string()
    }
}
