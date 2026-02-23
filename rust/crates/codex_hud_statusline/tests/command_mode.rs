use codex_hud_statusline::{parse_statusline_invocation, StatuslineAction};

#[test]
fn statusline_without_args_opens_ui() {
    let action = parse_statusline_invocation([]).unwrap();
    assert_eq!(action, StatuslineAction::OpenInteractiveUi);
}
