use std::process::Command;
use tempfile::tempdir;

#[test]
fn status_details_uses_ops_details_contract() {
    let home = tempdir().unwrap();
    std::fs::create_dir_all(home.path().join(".codex-hud/bin")).unwrap();
    std::fs::write(home.path().join(".codex-hud/bin/codex"), "stub").unwrap();

    let bin = env!("CARGO_BIN_EXE_codex_hud_cli");
    let out = Command::new(bin)
        .args(["status", "details"])
        .env("HOME", home.path())
        .output()
        .unwrap();

    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("codex-hud status details"));
    assert!(stdout.contains("installed: true"));
}
