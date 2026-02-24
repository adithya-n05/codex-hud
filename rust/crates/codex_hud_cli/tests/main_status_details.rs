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

#[test]
fn status_details_reports_patch_reason_and_refresh_source() {
    let home = tempdir().unwrap();
    std::fs::create_dir_all(home.path().join(".codex-hud/bin")).unwrap();
    std::fs::create_dir_all(home.path().join(".codex-hud/compat")).unwrap();
    std::fs::write(home.path().join(".codex-hud/bin/codex"), "stub").unwrap();
    std::fs::write(
        home.path().join(".codex-hud/last_run_policy.txt"),
        "mode=stock\nreason=unsupported compatibility key\n",
    )
    .unwrap();
    std::fs::write(
        home.path().join(".codex-hud/compat/last_compat_key.txt"),
        "0.104.0+abc123\n",
    )
    .unwrap();
    std::fs::write(
        home.path().join(".codex-hud/compat/refresh_source.txt"),
        "local-cache-fallback\n",
    )
    .unwrap();

    let bin = env!("CARGO_BIN_EXE_codex_hud_cli");
    let out = Command::new(bin)
        .args(["status", "details"])
        .env("HOME", home.path())
        .output()
        .unwrap();

    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("patch_mode: stock"));
    assert!(stdout.contains("patch_reason: unsupported compatibility key"));
    assert!(stdout.contains("compat_key: 0.104.0+abc123"));
    assert!(stdout.contains("compat_refresh_source: local-cache-fallback"));
}
