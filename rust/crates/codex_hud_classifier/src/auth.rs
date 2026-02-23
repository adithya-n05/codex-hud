pub fn normalize_auth_override(value: &str) -> String {
    let normalized = value.trim().to_ascii_lowercase().replace('_', "-");
    if normalized == "openai-auth" || normalized == "openai auth" {
        "OpenAI auth".to_string()
    } else if normalized == "no-auth" || normalized == "no auth" {
        "No auth".to_string()
    } else if normalized == "chatgpt" {
        "ChatGPT".to_string()
    } else {
        value.trim().to_string()
    }
}

pub fn provider_specific_api_key_label(provider_label: &str, env_key: Option<&str>) -> Option<String> {
    if provider_label == "AWS Bedrock" && env_key.unwrap_or_default().to_ascii_uppercase().contains("BEDROCK")
    {
        return Some("Bedrock API key".to_string());
    }
    None
}
