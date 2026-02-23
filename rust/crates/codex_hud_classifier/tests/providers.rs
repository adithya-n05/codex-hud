use codex_hud_classifier::{classify, ClassifierInput};

#[test]
fn detect_azure_openai_from_endpoint() {
    let input = ClassifierInput {
        base_url: Some("https://my-resource.openai.azure.com/openai/v1".to_string()),
        ..ClassifierInput::default()
    };
    let out = classify(&input);
    assert_eq!(out.provider_label, "Azure OpenAI");
}

#[test]
fn detect_azure_foundry_from_endpoint() {
    let input = ClassifierInput {
        base_url: Some("https://myhub.services.ai.azure.com/models".to_string()),
        ..ClassifierInput::default()
    };
    let out = classify(&input);
    assert_eq!(out.provider_label, "Azure Foundry");
}

#[test]
fn detect_vertex_from_openapi_endpoint() {
    let input = ClassifierInput {
        base_url: Some(
            "https://us-central1-aiplatform.googleapis.com/v1/projects/p/locations/l/endpoints/openapi"
                .to_string(),
        ),
        ..ClassifierInput::default()
    };
    let out = classify(&input);
    assert_eq!(out.provider_label, "GCP Vertex");
}
