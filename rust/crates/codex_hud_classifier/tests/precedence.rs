use codex_hud_classifier::{classify, ClassifierInput};

#[test]
fn explicit_overrides_win() {
    let input = ClassifierInput {
        explicit_provider_override: Some("Azure OpenAI".to_string()),
        explicit_auth_override: Some("Entra token".to_string()),
        base_url: Some("https://api.openai.com/v1".to_string()),
        ..ClassifierInput::default()
    };

    let out = classify(&input);
    assert_eq!(out.provider_label, "Azure OpenAI");
    assert_eq!(out.auth_label, "Entra token");
}
