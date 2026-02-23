use codex_hud_renderer::{render_hud, RenderInput};

#[test]
fn top_line_uses_priority_order() {
    let input = RenderInput {
        repo: Some("personal/codex-statusbar".to_string()),
        branch: Some("main".to_string()),
        permission: Some("never+dang-full".to_string()),
        auth_label: Some("ChatGPT".to_string()),
        provider_label: Some("OpenAI".to_string()),
        model_label: Some("gpt-5.3-codex high".to_string()),
        tool_count: Some(47),
        context_percent: None,
        five_hour_percent: None,
        weekly_percent: None,
    };

    let out = render_hud(&input);
    assert_eq!(
        out.logical_lines[0],
        "repo personal/codex-statusbar | branch main | perm never+dang-full | auth ChatGPT | provider OpenAI | model gpt-5.3-codex high | tools 47"
    );
}
