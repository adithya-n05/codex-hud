use codex_hud_classifier::{classify, ClassifierInput};

#[test]
fn unknown_provider_and_auth_fallback() {
    let input = ClassifierInput::default();
    let out = classify(&input);
    assert_eq!(out.provider_label, "Custom");
    assert_eq!(out.auth_label, "Unknown");
}
