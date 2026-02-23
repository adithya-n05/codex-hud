use codex_hud_renderer::{render_hud, RenderInput};

#[test]
fn empty_provider_and_auth_values_fallback_to_explicit_unknown_labels() {
    let input = RenderInput {
        auth_label: Some("   ".to_string()),
        provider_label: Some("".to_string()),
        ..RenderInput::default()
    };

    let out = render_hud(&input);
    assert!(out.logical_lines[0].contains("auth Unknown"));
    assert!(out.logical_lines[0].contains("provider Custom"));
}
