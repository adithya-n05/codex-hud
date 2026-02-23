use codex_hud_renderer::{top_line::render_top_line, RenderInput};

#[test]
fn top_line_renderer_is_exposed_from_top_line_module() {
    let input = RenderInput {
        repo: Some("personal/codex-statusbar".to_string()),
        branch: Some("main".to_string()),
        ..RenderInput::default()
    };
    let line = render_top_line(&input);
    assert!(line.contains("repo personal/codex-statusbar"));
}
