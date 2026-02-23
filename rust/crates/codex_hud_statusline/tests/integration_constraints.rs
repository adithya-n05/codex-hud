use codex_hud_statusline::integration_constraints::tmux_mode_supported;

#[test]
fn tmux_mode_is_not_supported() {
    assert!(!tmux_mode_supported());
}
