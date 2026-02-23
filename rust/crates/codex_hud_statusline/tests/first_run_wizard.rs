#[test]
fn first_run_requires_preset_wizard() {
    let screen = codex_hud_statusline::initial_screen(true);
    assert_eq!(screen, "preset_wizard");
}
