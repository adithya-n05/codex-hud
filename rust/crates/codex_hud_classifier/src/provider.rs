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
