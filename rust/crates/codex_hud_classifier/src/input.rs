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
