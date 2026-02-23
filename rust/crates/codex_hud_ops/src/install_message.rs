pub fn build_install_next_steps(rc_file: &str) -> String {
    format!(
        "codex-hud install complete. Restart your shell, or run `source ~/{rc_file}` to activate PATH changes."
    )
}
