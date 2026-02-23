use codex_hud_renderer::{render_hud, RenderInput};
use codex_hud_renderer::supports_third_activity_line_mode;

#[test]
fn render_returns_two_logical_lines() {
    let out = render_hud(&RenderInput::default());
    assert_eq!(out.logical_lines.len(), 2);
}

#[test]
fn third_activity_line_mode_is_not_supported() {
    assert!(!supports_third_activity_line_mode());
}
