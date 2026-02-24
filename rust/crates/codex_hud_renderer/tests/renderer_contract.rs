use codex_hud_renderer::{render_hud, renderer_ready, RenderInput};

#[test]
fn renderer_ready_is_true() {
    assert!(renderer_ready());
}

#[test]
fn colorblind_severity_is_warn_at_70_percent() {
    let out = render_hud(&RenderInput {
        context_percent: Some(70),
        colorblind_mode: true,
        ..RenderInput::default()
    });
    assert!(out.logical_lines[1].contains("warn"));
}

#[test]
fn colorblind_severity_is_normal_below_70_percent() {
    let out = render_hud(&RenderInput {
        context_percent: Some(69),
        colorblind_mode: true,
        ..RenderInput::default()
    });
    assert!(out.logical_lines[1].contains("normal"));
}
