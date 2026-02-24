use codex_hud_ops::statusline_migration::ensure_hud_statusline_config;
use tempfile::tempdir;

#[test]
fn migrates_legacy_status_line_to_hud_defaults() {
    let tmp = tempdir().unwrap();
    let home = tmp.path().join("home");
    let codex_dir = home.join(".codex");
    std::fs::create_dir_all(&codex_dir).unwrap();
    std::fs::write(
        codex_dir.join("config.toml"),
        r#"
model = "gpt-5.3-codex"

[tui]
status_line = [
  "model-with-reasoning",
  "git-branch",
  "context-remaining",
  "context-used",
]
"#,
    )
    .unwrap();

    let changed = ensure_hud_statusline_config(&home).unwrap();
    assert!(changed);

    let updated = std::fs::read_to_string(codex_dir.join("config.toml")).unwrap();
    assert!(updated.contains("\"permission-mode\""));
    assert!(updated.contains("\"auth-chip\""));
    assert!(updated.contains("\"tool-calls\""));
    assert!(updated.contains("\"ctx-bar\""));
    assert!(updated.contains("\"five-hour-bar\""));
    assert!(updated.contains("\"weekly-bar\""));
}

#[test]
fn keeps_existing_hud_statusline_configuration_unchanged() {
    let tmp = tempdir().unwrap();
    let home = tmp.path().join("home");
    let codex_dir = home.join(".codex");
    std::fs::create_dir_all(&codex_dir).unwrap();
    let original = r#"
[tui]
status_line = [
  "model-with-reasoning",
  "permission-mode",
  "auth-chip",
]
"#;
    std::fs::write(codex_dir.join("config.toml"), original).unwrap();

    let changed = ensure_hud_statusline_config(&home).unwrap();
    assert!(!changed);

    let after = std::fs::read_to_string(codex_dir.join("config.toml")).unwrap();
    assert_eq!(after, original);
}

#[test]
fn migrates_legacy_long_hud_default_to_compact_two_line_default() {
    let tmp = tempdir().unwrap();
    let home = tmp.path().join("home");
    let codex_dir = home.join(".codex");
    std::fs::create_dir_all(&codex_dir).unwrap();
    std::fs::write(
        codex_dir.join("config.toml"),
        r#"
[tui]
status_line = [
  "model-with-reasoning",
  "git-branch",
  "permission-mode",
  "auth-chip",
  "tool-calls",
  "ctx-bar",
  "five-hour-bar",
  "weekly-bar",
  "context-remaining",
  "context-used",
  "five-hour-limit",
  "weekly-limit",
  "current-dir",
  "project-root",
  "codex-version",
  "context-window-size",
  "used-tokens",
  "total-input-tokens",
  "total-output-tokens",
  "session-id",
  "model-name",
]
"#,
    )
    .unwrap();

    let changed = ensure_hud_statusline_config(&home).unwrap();
    assert!(changed);

    let updated = std::fs::read_to_string(codex_dir.join("config.toml")).unwrap();
    assert!(updated.contains("\"model-with-reasoning\""));
    assert!(updated.contains("\"git-branch\""));
    assert!(updated.contains("\"permission-mode\""));
    assert!(updated.contains("\"auth-chip\""));
    assert!(updated.contains("\"tool-calls\""));
    assert!(updated.contains("\"ctx-bar\""));
    assert!(updated.contains("\"five-hour-bar\""));
    assert!(updated.contains("\"weekly-bar\""));
    assert!(!updated.contains("\"context-remaining\""));
    assert!(!updated.contains("\"context-used\""));
    assert!(!updated.contains("\"session-id\""));
}

#[test]
fn creates_statusline_config_when_config_file_missing() {
    let tmp = tempdir().unwrap();
    let home = tmp.path().join("home");
    std::fs::create_dir_all(&home).unwrap();

    let changed = ensure_hud_statusline_config(&home).unwrap();
    assert!(changed);

    let created = std::fs::read_to_string(home.join(".codex/config.toml")).unwrap();
    assert!(created.contains("[tui]"));
    assert!(created.contains("\"permission-mode\""));
    assert!(created.contains("\"auth-chip\""));
    assert!(created.contains("\"tool-calls\""));
    assert!(created.contains("\"ctx-bar\""));
    assert!(created.contains("\"five-hour-bar\""));
    assert!(created.contains("\"weekly-bar\""));
    assert!(!created.contains("\"context-remaining\""));
    assert!(!created.contains("\"context-used\""));
}

#[test]
fn returns_error_when_codex_config_parent_is_a_file() {
    let tmp = tempdir().unwrap();
    let home = tmp.path().join("home");
    std::fs::create_dir_all(&home).unwrap();
    std::fs::write(home.join(".codex"), "not-a-directory").unwrap();

    let err = ensure_hud_statusline_config(&home).unwrap_err();
    assert!(!err.is_empty());
}

#[test]
fn malformed_toml_is_left_unchanged() {
    let tmp = tempdir().unwrap();
    let home = tmp.path().join("home");
    let codex_dir = home.join(".codex");
    std::fs::create_dir_all(&codex_dir).unwrap();
    std::fs::write(codex_dir.join("config.toml"), "[tui\nstatus_line = [").unwrap();

    let changed = ensure_hud_statusline_config(&home).unwrap();
    assert!(!changed);
}

#[test]
fn non_table_toml_root_is_ignored() {
    let tmp = tempdir().unwrap();
    let home = tmp.path().join("home");
    let codex_dir = home.join(".codex");
    std::fs::create_dir_all(&codex_dir).unwrap();
    std::fs::write(codex_dir.join("config.toml"), "42").unwrap();

    let changed = ensure_hud_statusline_config(&home).unwrap();
    assert!(!changed);
}

#[test]
fn non_table_tui_section_is_ignored() {
    let tmp = tempdir().unwrap();
    let home = tmp.path().join("home");
    let codex_dir = home.join(".codex");
    std::fs::create_dir_all(&codex_dir).unwrap();
    std::fs::write(codex_dir.join("config.toml"), r#"tui = "inline""#).unwrap();

    let changed = ensure_hud_statusline_config(&home).unwrap();
    assert!(!changed);
}

#[test]
fn status_line_without_hud_signals_is_replaced_with_hud_defaults() {
    let tmp = tempdir().unwrap();
    let home = tmp.path().join("home");
    let codex_dir = home.join(".codex");
    std::fs::create_dir_all(&codex_dir).unwrap();
    std::fs::write(
        codex_dir.join("config.toml"),
        r#"
[tui]
status_line = [
  "model-with-reasoning",
  "git-branch",
]
"#,
    )
    .unwrap();

    let changed = ensure_hud_statusline_config(&home).unwrap();
    assert!(changed);

    let updated = std::fs::read_to_string(codex_dir.join("config.toml")).unwrap();
    assert!(updated.contains("\"permission-mode\""));
    assert!(updated.contains("\"auth-chip\""));
    assert!(updated.contains("\"tool-calls\""));
}

#[test]
fn creates_tui_section_when_missing() {
    let tmp = tempdir().unwrap();
    let home = tmp.path().join("home");
    let codex_dir = home.join(".codex");
    std::fs::create_dir_all(&codex_dir).unwrap();
    std::fs::write(codex_dir.join("config.toml"), "model = \"gpt-5\"").unwrap();

    let changed = ensure_hud_statusline_config(&home).unwrap();
    assert!(changed);
    let updated = std::fs::read_to_string(codex_dir.join("config.toml")).unwrap();
    assert!(updated.contains("[tui]"));
    assert!(updated.contains("\"permission-mode\""));
}

#[test]
fn returns_error_when_config_path_is_directory() {
    let tmp = tempdir().unwrap();
    let home = tmp.path().join("home");
    let codex_dir = home.join(".codex");
    std::fs::create_dir_all(codex_dir.join("config.toml")).unwrap();

    let err = ensure_hud_statusline_config(&home).unwrap_err();
    assert!(!err.is_empty());
}

#[cfg(unix)]
#[test]
fn returns_error_when_missing_config_cannot_be_created() {
    use std::os::unix::fs::PermissionsExt;

    let tmp = tempdir().unwrap();
    let home = tmp.path().join("home");
    let codex_dir = home.join(".codex");
    std::fs::create_dir_all(&codex_dir).unwrap();
    let mut perms = std::fs::metadata(&codex_dir).unwrap().permissions();
    perms.set_mode(0o555);
    std::fs::set_permissions(&codex_dir, perms).unwrap();

    let err = ensure_hud_statusline_config(&home).unwrap_err();
    assert!(!err.is_empty());

    let mut reset = std::fs::metadata(&codex_dir).unwrap().permissions();
    reset.set_mode(0o755);
    std::fs::set_permissions(&codex_dir, reset).unwrap();
}

#[cfg(unix)]
#[test]
fn returns_error_when_overwriting_statusline_is_not_permitted() {
    use std::os::unix::fs::PermissionsExt;

    let tmp = tempdir().unwrap();
    let home = tmp.path().join("home");
    let codex_dir = home.join(".codex");
    std::fs::create_dir_all(&codex_dir).unwrap();
    let config_path = codex_dir.join("config.toml");
    std::fs::write(
        &config_path,
        r#"
[tui]
status_line = [
  "model-with-reasoning",
  "git-branch",
]
"#,
    )
    .unwrap();
    let mut perms = std::fs::metadata(&config_path).unwrap().permissions();
    perms.set_mode(0o444);
    std::fs::set_permissions(&config_path, perms).unwrap();

    let err = ensure_hud_statusline_config(&home).unwrap_err();
    assert!(!err.is_empty());

    let mut reset = std::fs::metadata(&config_path).unwrap().permissions();
    reset.set_mode(0o644);
    std::fs::set_permissions(&config_path, reset).unwrap();
}

#[cfg(unix)]
#[test]
fn returns_error_when_legacy_hud_migration_cannot_write() {
    use std::os::unix::fs::PermissionsExt;

    let tmp = tempdir().unwrap();
    let home = tmp.path().join("home");
    let codex_dir = home.join(".codex");
    std::fs::create_dir_all(&codex_dir).unwrap();
    let config_path = codex_dir.join("config.toml");
    std::fs::write(
        &config_path,
        r#"
[tui]
status_line = [
  "model-with-reasoning",
  "git-branch",
  "permission-mode",
  "auth-chip",
  "tool-calls",
  "ctx-bar",
  "five-hour-bar",
  "weekly-bar",
  "context-remaining",
  "context-used",
  "five-hour-limit",
  "weekly-limit",
  "current-dir",
  "project-root",
  "codex-version",
  "context-window-size",
  "used-tokens",
  "total-input-tokens",
  "total-output-tokens",
  "session-id",
  "model-name",
]
"#,
    )
    .unwrap();
    let mut perms = std::fs::metadata(&config_path).unwrap().permissions();
    perms.set_mode(0o444);
    std::fs::set_permissions(&config_path, perms).unwrap();

    let err = ensure_hud_statusline_config(&home).unwrap_err();
    assert!(!err.is_empty());

    let mut reset = std::fs::metadata(&config_path).unwrap().permissions();
    reset.set_mode(0o644);
    std::fs::set_permissions(&config_path, reset).unwrap();
}
