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
}
