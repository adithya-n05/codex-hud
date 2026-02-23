use codex_hud_statusline::{parse_statusline_invocation, StatuslineAction};

#[test]
fn statusline_without_args_opens_ui() {
    let action = parse_statusline_invocation([]).unwrap();
    assert_eq!(action, StatuslineAction::OpenInteractiveUi);
}

#[test]
fn statusline_with_args_is_rejected() {
    let err = parse_statusline_invocation(["preset", "full"]).unwrap_err();
    assert!(err.contains("/statusline does not accept arguments in v1"));
}
