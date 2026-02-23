use codex_hud_renderer::{color_for_percent, SeverityColor};
use codex_hud_renderer::format_percent_label;

#[test]
fn color_thresholds_match_policy() {
    assert_eq!(color_for_percent(10), SeverityColor::Green);
    assert_eq!(color_for_percent(70), SeverityColor::Yellow);
    assert_eq!(color_for_percent(85), SeverityColor::Red);
}

#[test]
fn confidence_markers_are_never_emitted() {
    let s = format_percent_label(82);
    assert!(!s.contains('!'));
    assert!(!s.contains('?'));
}
