use codex_hud_renderer::{render_hud, RenderInput};

#[test]
fn duration_metric_renders_when_present() {
    let input = RenderInput {
        duration_seconds: Some(125),
        ..RenderInput::default()
    };
    let out = render_hud(&input);
    assert!(out.logical_lines[1].contains("dur 125s"));
}
