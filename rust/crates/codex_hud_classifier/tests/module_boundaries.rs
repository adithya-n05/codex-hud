use codex_hud_classifier::input::ClassifierInput;

#[test]
fn classifier_input_is_exposed_from_input_module() {
    let input = ClassifierInput::default();
    assert!(input.base_url.is_none());
}
