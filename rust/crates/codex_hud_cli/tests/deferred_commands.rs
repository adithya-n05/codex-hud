use codex_hud_cli::parse_args;

#[test]
fn cleanup_command_is_deferred_in_v1() {
    let err = parse_args(["codex-hud", "cleanup"]).unwrap_err();
    assert_eq!(err, "cleanup command is deferred in v1");
}
