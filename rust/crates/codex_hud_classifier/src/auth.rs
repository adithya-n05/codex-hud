pub fn normalize_auth_override(value: &str) -> String {
    let normalized = value.trim().to_ascii_lowercase().replace('_', "-");
    if normalized == "openai-auth" || normalized == "openai auth" {
        "OpenAI auth".to_string()
    } else {
        value.to_string()
    }
}
