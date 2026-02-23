use codex_hud_renderer::{top_line::render_top_line, RenderInput};
use codex_hud_renderer::bar::metric_bar;
use codex_hud_renderer::bottom_line::render_bottom_line;
use codex_hud_renderer::wrap::wrap_text;

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

#[test]
fn bottom_line_renderer_is_exposed_from_bottom_line_module() {
    let input = RenderInput {
        context_percent: Some(41),
        five_hour_percent: Some(12),
        weekly_percent: Some(38),
        ..RenderInput::default()
    };
    let line = render_bottom_line(&input);
    assert!(line.contains("CTX 41%"));
}

#[test]
fn wrap_helper_is_exposed_from_wrap_module() {
    let wrapped = wrap_text("CTX ████░░░░", 5);
    assert!(!wrapped.is_empty());
}
