use codex_hud_statusline::{parse_statusline_invocation, StatuslineAction};
use codex_hud_statusline::parse_statusline_command;

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

#[test]
fn hud_alias_is_not_available_in_v1() {
    let err = parse_statusline_command("/hud", []).unwrap_err();
    assert!(err.contains("Use `/statusline`"));
}

#[test]
fn valid_statusline_command_runs_interactive_invocation() {
    let action = parse_statusline_command("/statusline", []).unwrap();
    assert_eq!(action, StatuslineAction::OpenInteractiveUi);
}

#[test]
fn unknown_statusline_command_is_rejected() {
    let err = parse_statusline_command("/not-a-command", []).unwrap_err();
    assert!(err.contains("unknown statusline command"));
}

#[test]
fn statusline_command_rejects_args_after_valid_command_name() {
    let err = parse_statusline_command("/statusline", ["preset", "full"]).unwrap_err();
    assert!(err.contains("/statusline does not accept arguments in v1"));
}
