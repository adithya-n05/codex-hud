use std::process::Command;
use tempfile::tempdir;

#[test]
fn uninstall_route_restores_original_content_without_explicit_root_env() {
    let tmp = tempdir().unwrap();
    let home = tmp.path().join("home");
    let root = tmp.path().join("codex-rs");
    let target = root.join("tui/src/slash_command.rs");
    std::fs::create_dir_all(target.parent().unwrap()).unwrap();
    std::fs::write(&target, "patched").unwrap();

    let backup = root
        .join(".codex-hud/backups")
        .join("codex-rs/tui/src/slash_command.rs")
        .with_extension("bak");
    std::fs::create_dir_all(backup.parent().unwrap()).unwrap();
    std::fs::write(&backup, "original").unwrap();
    std::fs::create_dir_all(root.join(".codex-hud")).unwrap();
    std::fs::write(
        root.join(".codex-hud/patch-state.json"),
        r#"{"patched_rel_paths":["codex-rs/tui/src/slash_command.rs"]}"#,
    )
    .unwrap();

    std::fs::create_dir_all(home.join(".codex-hud")).unwrap();
    std::fs::write(
        home.join(".codex-hud/last_codex_root.txt"),
        root.to_string_lossy().to_string(),
    )
    .unwrap();

    let bin = env!("CARGO_BIN_EXE_codex_hud_cli");
    let out = Command::new(bin)
        .arg("uninstall")
        .env("HOME", &home)
        .output()
        .unwrap();

    assert!(out.status.success());
    assert_eq!(std::fs::read_to_string(target).unwrap(), "original");
}
