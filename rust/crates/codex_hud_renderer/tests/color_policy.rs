use codex_hud_renderer::format_percent_label;

#[test]
fn confidence_markers_are_never_emitted() {
    let s = format_percent_label(82);
    assert!(!s.contains('!'));
    assert!(!s.contains('?'));
}
