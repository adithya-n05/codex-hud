use codex_hud_classifier::{classify, ClassifierInput};

#[test]
fn auth_label_never_contains_secret_material() {
    let input = ClassifierInput {
        explicit_provider_override: Some("OpenAI".to_string()),
        explicit_auth_override: Some("Bearer sk-secret-123".to_string()),
        ..ClassifierInput::default()
    };

    let out = classify(&input);
    assert!(!out.auth_label.contains("sk-secret"));
}
