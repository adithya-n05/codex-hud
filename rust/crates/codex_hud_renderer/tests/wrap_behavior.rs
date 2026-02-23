use codex_hud_renderer::{render_hud, RenderInput};

#[test]
fn narrow_width_wraps_but_keeps_two_logical_lines() {
    let input = RenderInput {
        repo: Some("personal/codex-statusbar".to_string()),
        branch: Some("feature/ui".to_string()),
        permission: Some("on-request+workspace-write".to_string()),
        auth_label: Some("ChatGPT".to_string()),
        provider_label: Some("OpenAI".to_string()),
        model_label: Some("gpt-5.3-codex medium".to_string()),
        tool_count: Some(91),
        context_percent: Some(82),
        five_hour_percent: Some(76),
        weekly_percent: Some(44),
        width: Some(40),
        ..RenderInput::default()
    };

    let out = render_hud(&input);
    assert_eq!(out.logical_lines.len(), 2);
    assert!(out.wrapped_lines.len() > 2);
}
