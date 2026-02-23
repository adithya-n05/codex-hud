mod auth;
pub mod input;
pub mod provider;
pub mod sanitize;
pub use input::ClassifierInput;
pub use sanitize::sanitize_auth_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Classification {
    pub provider_label: String,
    pub auth_label: String,
}

pub fn classifier_ready() -> bool {
    true
}

fn detect_provider_label(input: &ClassifierInput) -> String {
    if let Some(url) = input.base_url.as_deref() {
        let lower = url.to_ascii_lowercase();
        if lower.contains(".openai.azure.com") && lower.contains("/openai/v1") {
            return "Azure OpenAI".to_string();
        }
        if lower.contains(".services.ai.azure.com") && lower.contains("/models") {
            return "Azure Foundry".to_string();
        }
        if lower.contains("aiplatform.googleapis.com")
            && lower.contains("/projects/")
            && lower.contains("/locations/")
            && lower.contains("/endpoints/openapi")
        {
            return "GCP Vertex".to_string();
        }
    }
    if input
        .provider_id
        .as_deref()
        .map(|v| v.eq_ignore_ascii_case("openai"))
        .unwrap_or(false)
        || input
            .provider_name
            .as_deref()
            .map(|v| v.eq_ignore_ascii_case("openai"))
            .unwrap_or(false)
    {
        return "OpenAI".to_string();
    }
    if let Some(v) = provider::detect_provider_from_endpoint(input.base_url.as_deref()) {
        return v;
    }
    if let Some(v) = provider::detect_provider_from_model_name(input.model_name.as_deref()) {
        return v;
    }
    "Custom".to_string()
}

fn detect_auth_label(input: &ClassifierInput, provider_label: &str) -> String {
    if let Some(v) = input.explicit_auth_override.as_ref() {
        return auth::normalize_auth_override(&sanitize_auth_label(v));
    }
    if input.has_api_key {
        if let Some(v) = auth::provider_specific_api_key_label(&provider_label, input.env_key_name.as_deref())
        {
            return v;
        }
        return "API key".to_string();
    }
    if input
        .env_key_name
        .as_deref()
        .map(|v| v.to_ascii_uppercase().contains("API_KEY"))
        .unwrap_or(false)
    {
        return "API key".to_string();
    }
    if input.env_key_name.as_deref().is_some() {
        return "Env key".to_string();
    }
    if provider_label == "AWS Bedrock"
        && input
            .env_header_keys
            .iter()
            .any(|h| h.eq_ignore_ascii_case("X-Amz-Date"))
    {
        return "AWS creds".to_string();
    }
    if provider_label == "Azure OpenAI" && input.has_bearer_header {
        return "Entra token".to_string();
    }
    if provider_label == "GCP Vertex" && input.has_bearer_header {
        return "GCP token".to_string();
    }
    if input.has_bearer_header {
        return "Bearer token".to_string();
    }
    "Unknown".to_string()
}

pub fn classify(input: &ClassifierInput) -> Classification {
    let provider_label = if let Some(provider) = input.explicit_provider_override.as_ref() {
        provider.clone()
    } else {
        detect_provider_label(input)
    };
    let auth_label = detect_auth_label(input, &provider_label);

    Classification {
        provider_label,
        auth_label,
    }
}
