use codex_hud_statusline::integration_constraints::tmux_mode_supported;
use codex_hud_statusline::integration_constraints::{
    separate_pane_mode_supported, separate_window_mode_supported,
};

#[test]
fn tmux_mode_is_not_supported() {
    assert!(!tmux_mode_supported());
}

#[test]
fn separate_surface_modes_are_not_supported() {
    assert!(!separate_pane_mode_supported());
    assert!(!separate_window_mode_supported());
}
