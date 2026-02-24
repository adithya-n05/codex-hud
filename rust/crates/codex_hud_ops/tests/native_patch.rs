use codex_hud_ops::native_patch::{apply_marker_replace, native_patch_targets};

#[test]
fn patch_targets_match_statusline_contract_set() {
    let targets = native_patch_targets();
    assert!(targets.contains(&"codex-rs/tui/src/slash_command.rs".to_string()));
    assert!(targets.contains(&"codex-rs/tui/src/chatwidget.rs".to_string()));
    assert!(targets.contains(&"codex-rs/tui/src/app.rs".to_string()));
    assert!(targets.contains(&"codex-rs/tui/src/bottom_pane/status_line_setup.rs".to_string()));
    assert_eq!(targets.len(), 4);
}

#[test]
fn marker_replace_is_idempotent_once_patched() {
    let original = "SlashCommand::Statusline /* codex-hud-managed */";
    let out = apply_marker_replace(
        original,
        "SlashCommand::Statusline",
        "SlashCommand::Statusline /* codex-hud-managed */",
    )
    .unwrap();
    assert_eq!(out, original);
}
