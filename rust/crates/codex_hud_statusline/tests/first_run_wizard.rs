#[test]
fn first_run_requires_preset_wizard() {
    let screen = codex_hud_statusline::initial_screen(true);
    assert_eq!(screen, "preset_wizard");
}

#[test]
fn non_first_run_opens_main_config() {
    let screen = codex_hud_statusline::initial_screen(false);
    assert_eq!(screen, "main_config");
}
