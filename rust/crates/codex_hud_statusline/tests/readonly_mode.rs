use codex_hud_statusline::{UiLoadResult, UiMode};

#[test]
fn parse_error_maps_to_read_only_warning_mode() {
    let res = codex_hud_statusline::map_load_result(UiLoadResult::ParseError("bad".to_string()));
    assert_eq!(res.mode, UiMode::ReadOnlyWarning);
    assert!(res.message.contains("~/.codex/config.toml"));
    assert!(res.message.contains("Fix syntax"));
}
