use codex_hud_renderer::{render_hud, RenderInput};
use codex_hud_renderer::bottom_line::build_bottom_line;

#[test]
fn bottom_line_renders_three_primary_bars() {
    let input = RenderInput {
        context_percent: Some(41),
        five_hour_percent: Some(12),
        weekly_percent: Some(38),
        ..RenderInput::default()
    };

    let out = render_hud(&input);
    assert!(out.logical_lines[1].contains("CTX 41%"));
    assert!(out.logical_lines[1].contains("5H 12%"));
    assert!(out.logical_lines[1].contains("7D 38%"));
}

#[test]
fn bottom_line_empty_when_no_metrics_enabled() {
    let parts: Vec<String> = Vec::new();
    assert_eq!(build_bottom_line(&parts), "");
}
