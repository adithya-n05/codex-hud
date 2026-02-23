use codex_hud_statusline::patch_contract;

#[test]
fn patch_contract_targets_statusline_paths() {
    let c = patch_contract();
    assert_eq!(c.command_symbol, "SlashCommand::Statusline");
    assert!(
        c.source_files
            .iter()
            .any(|f| f.ends_with("codex-rs/tui/src/slash_command.rs"))
    );
    assert!(
        c.source_files
            .iter()
            .any(|f| f.ends_with("codex-rs/tui/src/chatwidget.rs"))
    );
}
