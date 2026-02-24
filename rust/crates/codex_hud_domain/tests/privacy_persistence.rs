use codex_hud_domain::save::{load_redaction_toggle, save_redaction_toggle};
use tempfile::tempdir;

#[test]
fn redaction_toggle_persists_across_save_and_reload() {
    let dir = tempdir().unwrap();
    let target = dir.path().join("config.toml");
    std::fs::write(&target, "[tui.codex_hud.native]\nmodel_name = true\n").unwrap();

    save_redaction_toggle(&target, true).unwrap();
    let after_first_save = load_redaction_toggle(&target).unwrap();
    assert!(after_first_save);
    let content_after_first = std::fs::read_to_string(&target).unwrap();
    assert!(content_after_first.contains("model_name = true"));

    save_redaction_toggle(&target, false).unwrap();
    let after_second_save = load_redaction_toggle(&target).unwrap();
    assert!(!after_second_save);
    let content_after_second = std::fs::read_to_string(&target).unwrap();
    assert!(content_after_second.contains("model_name = true"));
}

#[test]
fn redaction_toggle_errors_when_key_is_missing() {
    let dir = tempdir().unwrap();
    let target = dir.path().join("config.toml");
    std::fs::write(&target, "[tui.codex_hud]\nmodel_name = true\n").unwrap();

    let err = load_redaction_toggle(&target).unwrap_err();
    assert!(err.contains("missing redact_auth_identity key"));
}

#[test]
fn redaction_toggle_errors_for_invalid_boolean_value() {
    let dir = tempdir().unwrap();
    let target = dir.path().join("config.toml");
    std::fs::write(&target, "redact_auth_identity = maybe\n").unwrap();

    let err = load_redaction_toggle(&target).unwrap_err();
    assert!(err.contains("invalid redact_auth_identity value"));
}
