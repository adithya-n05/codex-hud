use codex_hud_renderer::{render_hud, RenderInput};

#[test]
fn top_line_drops_low_priority_fields_first() {
    let input = RenderInput {
        repo: Some("personal/codex-statusbar".to_string()),
        branch: Some("main".to_string()),
        permission: Some("never+dang-full".to_string()),
        auth_label: Some("ChatGPT".to_string()),
        provider_label: Some("OpenAI".to_string()),
        model_label: Some("gpt-5.3-codex high".to_string()),
        tool_count: Some(126),
        width: Some(70),
        ..RenderInput::default()
    };

    let out = render_hud(&input);
    assert!(out.logical_lines[0].contains("repo personal/codex-statusbar"));
    assert!(out.logical_lines[0].contains("branch main"));
    assert!(!out.logical_lines[0].contains("tools 126"));
}
