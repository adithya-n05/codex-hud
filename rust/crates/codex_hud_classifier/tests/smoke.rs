use codex_hud_classifier::classifier_ready;

#[test]
fn classifier_ready_returns_true() {
    assert!(classifier_ready());
}
