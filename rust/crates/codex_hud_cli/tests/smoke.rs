use codex_hud_cli::cli_ready;

#[test]
fn cli_ready_returns_true() {
    assert!(cli_ready());
}
