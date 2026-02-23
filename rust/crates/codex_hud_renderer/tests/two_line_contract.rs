use codex_hud_renderer::{render_hud, RenderInput};

#[test]
fn render_returns_two_logical_lines() {
    let out = render_hud(&RenderInput::default());
    assert_eq!(out.logical_lines.len(), 2);
}
