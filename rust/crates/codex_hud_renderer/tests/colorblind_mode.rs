use codex_hud_renderer::{render_hud, RenderInput};

#[test]
fn colorblind_mode_adds_textual_severity_without_symbols() {
    let input = RenderInput {
        context_percent: Some(90),
        colorblind_mode: true,
        ..RenderInput::default()
    };

    let out = render_hud(&input);
    assert!(out.logical_lines[1].contains("CTX 90%"));
    assert!(out.logical_lines[1].contains("critical"));
    assert!(!out.logical_lines[1].contains('!'));
}
