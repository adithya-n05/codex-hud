#[derive(Debug, Clone, Default)]
pub struct ClassifierInput {
    pub explicit_provider_override: Option<String>,
    pub explicit_auth_override: Option<String>,
    pub provider_id: Option<String>,
    pub provider_name: Option<String>,
    pub base_url: Option<String>,
    pub model_name: Option<String>,
    pub env_key_name: Option<String>,
    pub env_header_keys: Vec<String>,
    pub has_bearer_header: bool,
    pub has_api_key: bool,
}

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
    "Custom".to_string()
}

pub fn classify(input: &ClassifierInput) -> Classification {
    if let (Some(provider), Some(auth)) = (
        input.explicit_provider_override.as_ref(),
        input.explicit_auth_override.as_ref(),
    ) {
        return Classification {
            provider_label: provider.clone(),
            auth_label: auth.clone(),
        };
    }

    Classification {
        provider_label: detect_provider_label(input),
        auth_label: "Unknown".to_string(),
    }
}
