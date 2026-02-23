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

#[test]
fn speed_metric_renders_when_present() {
    let input = RenderInput {
        tokens_per_second: Some(42),
        ..RenderInput::default()
    };
    let out = render_hud(&input);
    assert!(out.logical_lines[1].contains("spd 42t/s"));
}

#[test]
fn plan_progress_renders_when_present() {
    let input = RenderInput {
        plan_done: Some(3),
        plan_total: Some(5),
        ..RenderInput::default()
    };
    let out = render_hud(&input);
    assert!(out.logical_lines[1].contains("plan 3/5"));
}

#[test]
fn config_count_renders_when_present() {
    let input = RenderInput {
        config_count: Some(12),
        ..RenderInput::default()
    };
    let out = render_hud(&input);
    assert!(out.logical_lines[1].contains("cfg 12"));
}
