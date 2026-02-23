use codex_hud_renderer::{top_line::render_top_line, RenderInput};
use codex_hud_renderer::bar::metric_bar;

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

#[test]
fn bar_renderer_is_exposed_from_bar_module() {
    let bar = metric_bar(50, 4);
    assert_eq!(bar, "██░░");
}
