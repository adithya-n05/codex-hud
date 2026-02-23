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
