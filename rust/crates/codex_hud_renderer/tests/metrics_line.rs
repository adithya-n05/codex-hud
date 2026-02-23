use codex_hud_renderer::{render_hud, RenderInput};

#[test]
fn bottom_line_renders_three_primary_bars() {
    let mut input = RenderInput::default();
    input.context_percent = Some(41);
    input.five_hour_percent = Some(12);
    input.weekly_percent = Some(38);

    let out = render_hud(&input);
    assert!(out.logical_lines[1].contains("CTX 41%"));
    assert!(out.logical_lines[1].contains("5H 12%"));
    assert!(out.logical_lines[1].contains("7D 38%"));
}
