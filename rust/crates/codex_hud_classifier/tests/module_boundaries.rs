use codex_hud_classifier::input::ClassifierInput;
use codex_hud_classifier::sanitize::sanitize_auth_label;

#[test]
fn classifier_input_is_exposed_from_input_module() {
    let input = ClassifierInput::default();
    assert!(input.base_url.is_none());
}

#[test]
fn sanitize_auth_label_masks_secret_like_values() {
    assert_eq!(sanitize_auth_label("Bearer sk-secret-123"), "Bearer token");
}
