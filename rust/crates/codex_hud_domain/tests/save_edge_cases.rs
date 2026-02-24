use codex_hud_domain::parse_hud_config;
use codex_hud_domain::save::{
    atomic_write_with_backup, load_redaction_toggle, restore_from_backup, save_redaction_toggle,
};
use tempfile::tempdir;

#[test]
fn atomic_save_failure_without_existing_file_keeps_target_absent() {
    let dir = tempdir().unwrap();
    let target = dir.path().join("config.toml");

    let err = atomic_write_with_backup(&target, "new = true\n", true).unwrap_err();

    assert!(err.contains("simulated write failure"));
    assert!(!target.exists());
    assert!(!target.with_extension("toml.bak").exists());
    assert!(!target.with_extension("toml.tmp").exists());
}

#[test]
fn save_redaction_toggle_appends_newline_when_file_lacks_trailing_newline() {
    let dir = tempdir().unwrap();
    let target = dir.path().join("config.toml");
    std::fs::write(&target, "[tui.codex_hud.native]\nmodel_name = true").unwrap();

    save_redaction_toggle(&target, true).unwrap();

    let content = std::fs::read_to_string(&target).unwrap();
    assert!(content.contains("model_name = true\nredact_auth_identity = true\n"));
    assert!(load_redaction_toggle(&target).unwrap());
}

#[test]
fn parse_accepts_input_without_tui_codex_hud_table() {
    let cfg = parse_hud_config("[tui.other]\nenabled = true\n").unwrap();
    assert_eq!(cfg.preset, codex_hud_domain::Preset::Essential);
}

#[test]
fn restore_from_backup_restores_last_saved_content() {
    let dir = tempdir().unwrap();
    let target = dir.path().join("config.toml");
    let backup = target.with_extension("toml.bak");
    std::fs::write(&target, "current = true\n").unwrap();
    std::fs::write(&backup, "backup = true\n").unwrap();

    restore_from_backup(&target).unwrap();
    assert_eq!(std::fs::read_to_string(&target).unwrap(), "backup = true\n");
}

#[test]
fn save_redaction_toggle_rewrites_existing_key_without_duplicates() {
    let dir = tempdir().unwrap();
    let target = dir.path().join("config.toml");
    std::fs::write(
        &target,
        "[tui.codex_hud.native]\nredact_auth_identity = true\n",
    )
    .unwrap();

    save_redaction_toggle(&target, false).unwrap();
    let content = std::fs::read_to_string(&target).unwrap();
    assert_eq!(content.matches("redact_auth_identity =").count(), 1);
    assert!(content.contains("redact_auth_identity = false\n"));
}

#[test]
fn load_redaction_toggle_errors_when_file_is_missing() {
    let dir = tempdir().unwrap();
    let err = load_redaction_toggle(&dir.path().join("missing.toml")).unwrap_err();
    assert!(!err.is_empty());
}
