use std::process::Command;
use tempfile::tempdir;

#[test]
fn binary_status_command_uses_ops_renderer_contract() {
    let home = tempdir().unwrap();
    std::fs::create_dir_all(home.path().join(".codex-hud/bin")).unwrap();
    std::fs::write(home.path().join(".codex-hud/bin/codex"), "stub").unwrap();

    let bin = env!("CARGO_BIN_EXE_codex_hud_cli");
    let out = Command::new(bin)
        .arg("status")
        .env("HOME", home.path())
        .output()
        .unwrap();

    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("codex-hud status"));
    assert!(stdout.contains("installed:"));
}

#[test]
fn status_command_reports_last_policy_and_compat_metadata() {
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
        "github-release\n",
    )
    .unwrap();

    let bin = env!("CARGO_BIN_EXE_codex_hud_cli");
    let out = Command::new(bin)
        .arg("status")
        .env("HOME", home.path())
        .output()
        .unwrap();

    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("patch_mode: stock"));
    assert!(stdout.contains("compat_key: 0.104.0+abc123"));
    assert!(stdout.contains("refresh_source: github-release"));
}

#[test]
fn status_command_marks_compatibility_unsupported_from_last_policy_reason() {
    let home = tempdir().unwrap();
    std::fs::create_dir_all(home.path().join(".codex-hud/bin")).unwrap();
    std::fs::write(home.path().join(".codex-hud/bin/codex"), "stub").unwrap();
    std::fs::write(
        home.path().join(".codex-hud/last_run_policy.txt"),
        "mode=stock\nreason=unsupported compatibility key\n",
    )
    .unwrap();

    let bin = env!("CARGO_BIN_EXE_codex_hud_cli");
    let out = Command::new(bin)
        .arg("status")
        .env("HOME", home.path())
        .output()
        .unwrap();

    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("compatibility: unsupported"));
}

#[test]
fn status_command_uses_userprofile_when_home_is_unset() {
    let home = tempdir().unwrap();
    std::fs::create_dir_all(home.path().join(".codex-hud/bin")).unwrap();
    std::fs::write(home.path().join(".codex-hud/bin/codex"), "stub").unwrap();

    let bin = env!("CARGO_BIN_EXE_codex_hud_cli");
    let out = Command::new(bin)
        .arg("status")
        .env_remove("HOME")
        .env("USERPROFILE", home.path())
        .output()
        .unwrap();

    assert!(out.status.success());
}

#[test]
fn status_command_fails_without_home_or_userprofile() {
    let bin = env!("CARGO_BIN_EXE_codex_hud_cli");
    let out = Command::new(bin)
        .arg("status")
        .env_remove("HOME")
        .env_remove("USERPROFILE")
        .output()
        .unwrap();

    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("missing env HOME/USERPROFILE"));
}
