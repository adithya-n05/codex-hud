use codex_hud_ops::uninstall_mode;

#[test]
fn uninstall_mode_is_non_interactive() {
    assert_eq!(uninstall_mode(), "non_interactive_no_prompt");
}
