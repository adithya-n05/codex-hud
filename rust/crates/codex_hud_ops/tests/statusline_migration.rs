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
