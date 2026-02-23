use codex_hud_classifier::{classify, ClassifierInput};

#[test]
fn classify_uses_detect_auth_pipeline_baseline() {
    let input = ClassifierInput {
        explicit_provider_override: Some("OpenAI".to_string()),
        has_api_key: true,
        ..ClassifierInput::default()
    };
    let out = classify(&input);
    assert_eq!(out.auth_label, "API key");
}
