use codex_hud_domain::parse_hud_config;
use codex_hud_domain::save::{atomic_write_with_backup, load_redaction_toggle, save_redaction_toggle};
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
