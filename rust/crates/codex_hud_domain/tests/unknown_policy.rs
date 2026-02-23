use codex_hud_domain::UnknownDisplayPolicy;

#[test]
fn unknown_display_defaults_are_explicit() {
    let p = UnknownDisplayPolicy::default();
    assert_eq!(p.provider_unknown_label, "Custom");
    assert_eq!(p.auth_unknown_label, "Unknown");
    assert!(p.render_unknown_explicitly);
}
