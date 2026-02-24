pub fn detect_provider_from_endpoint(base_url: Option<&str>) -> Option<String> {
    let lower = base_url?.to_ascii_lowercase();
    if lower.contains("bedrock-mantle") && lower.contains(".api.aws") {
        return Some("AWS Bedrock".to_string());
    }
    if lower.contains("bedrock-runtime") && lower.contains(".amazonaws.com") {
        return Some("AWS Bedrock".to_string());
    }
    None
}

pub fn detect_provider_from_model_name(model_name: Option<&str>) -> Option<String> {
    let name = model_name?.to_ascii_lowercase();
    if name.contains("bedrock") {
        return Some("AWS Bedrock".to_string());
    }
    if name.contains("vertex") {
        return Some("GCP Vertex".to_string());
    }
    if name.contains("azure") {
        return Some("Azure OpenAI".to_string());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_provider_from_endpoint_matches_bedrock_runtime() {
        assert_eq!(
            detect_provider_from_endpoint(Some("https://bedrock-runtime.us-east-1.amazonaws.com/")),
            Some("AWS Bedrock".to_string())
        );
    }

    #[test]
    fn detect_provider_from_endpoint_returns_none_for_unmatched_url() {
        assert_eq!(detect_provider_from_endpoint(Some("https://example.com/api")), None);
        assert_eq!(detect_provider_from_endpoint(None), None);
    }

    #[test]
    fn detect_provider_from_endpoint_matches_bedrock_mantle() {
        assert_eq!(
            detect_provider_from_endpoint(Some("https://bedrock-mantle.us-west-2.api.aws/openai/v1")),
            Some("AWS Bedrock".to_string())
        );
    }

    #[test]
    fn detect_provider_from_model_name_covers_known_clouds() {
        assert_eq!(
            detect_provider_from_model_name(Some("anthropic.claude-3-sonnet-20240229-bedrock")),
            Some("AWS Bedrock".to_string())
        );
        assert_eq!(
            detect_provider_from_model_name(Some("vertex-gemini-2.0-flash")),
            Some("GCP Vertex".to_string())
        );
        assert_eq!(
            detect_provider_from_model_name(Some("gpt-4o-mini-azure-preview")),
            Some("Azure OpenAI".to_string())
        );
    }

    #[test]
    fn detect_provider_from_model_name_returns_none_for_unknown() {
        assert_eq!(detect_provider_from_model_name(Some("gpt-4.0")), None);
    }
}
