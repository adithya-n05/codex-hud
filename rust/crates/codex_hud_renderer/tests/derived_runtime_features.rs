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

#[test]
fn git_extended_indicators_render_when_present() {
    let input = RenderInput {
        git_dirty: Some(true),
        git_ahead: Some(2),
        git_behind: Some(1),
        git_file_stats: Some("+5/-2".to_string()),
        ..RenderInput::default()
    };
    let out = render_hud(&input);
    assert!(out.logical_lines[0].contains("dirty"));
    assert!(out.logical_lines[0].contains("↑2↓1"));
    assert!(out.logical_lines[0].contains("+5/-2"));
}
