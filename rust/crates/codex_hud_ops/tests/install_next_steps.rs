use codex_hud_ops::install_message::build_install_next_steps;

#[test]
fn install_output_includes_shell_reload_guidance() {
    let message = build_install_next_steps(".zshrc");
    assert!(message.contains("Restart your shell"));
    assert!(message.contains("source ~/.zshrc"));
}
