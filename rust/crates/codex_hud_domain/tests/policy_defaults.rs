use codex_hud_domain::VisualOptions;

#[test]
fn visual_defaults_match_policy() {
    let v = VisualOptions::default();
    assert_eq!(v.warn_percent, 70);
    assert_eq!(v.critical_percent, 85);
    assert!(!v.show_severity_symbols);
    assert!(!v.show_confidence_markers);
    assert!(!v.colorblind_mode);
}
