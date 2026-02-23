use codex_hud_renderer::format::metric_fragment;

#[test]
fn usage_text_mode_disables_bars() {
    let out = metric_fragment("CTX", 82, Some("text"));
    assert_eq!(out, "CTX 82%");
    assert!(!out.contains('█'));
    assert!(!out.contains('░'));
}
