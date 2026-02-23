use codex_hud_classifier::{classify, ClassifierInput};

#[test]
fn detect_openai_auth_label() {
    let input = ClassifierInput {
        provider_id: Some("openai".to_string()),
        explicit_auth_override: Some("openai-auth".to_string()),
        ..ClassifierInput::default()
    };
    let out = classify(&input);
    assert_eq!(out.auth_label, "OpenAI auth");
}
