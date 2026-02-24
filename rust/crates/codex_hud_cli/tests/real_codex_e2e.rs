use std::process::Command;

#[test]
fn real_codex_minimal_env_install_status_uninstall_and_passthrough_e2e() {
    if std::env::var("CODEX_HUD_E2E_REAL").ok().as_deref() != Some("1") {
        return;
    }

    let cli = env!("CARGO_BIN_EXE_codex_hud_cli");
    let codex_bin = std::env::var("CODEX_BIN").unwrap_or_else(|_| "codex".to_string());

    let mut install_cmd = Command::new(cli);
    install_cmd.arg("install");
    for key in [
        "CODEX_HUD_CODEX_ROOT",
        "CODEX_HUD_COMPAT_MANIFEST",
        "CODEX_HUD_MANIFEST_PUBKEY",
        "CODEX_HUD_COMPAT_KEY",
    ] {
        install_cmd.env_remove(key);
    }
    let install = install_cmd.output().unwrap();
    assert!(install.status.success());

    let mut details_cmd = Command::new(cli);
    details_cmd.args(["status", "details"]);
    for key in [
        "CODEX_HUD_CODEX_ROOT",
        "CODEX_HUD_COMPAT_MANIFEST",
        "CODEX_HUD_MANIFEST_PUBKEY",
        "CODEX_HUD_COMPAT_KEY",
    ] {
        details_cmd.env_remove(key);
    }
    let details = details_cmd.output().unwrap();
    assert!(details.status.success());

    let mut uninstall_cmd = Command::new(cli);
    uninstall_cmd.arg("uninstall");
    for key in [
        "CODEX_HUD_CODEX_ROOT",
        "CODEX_HUD_COMPAT_MANIFEST",
        "CODEX_HUD_MANIFEST_PUBKEY",
        "CODEX_HUD_COMPAT_KEY",
    ] {
        uninstall_cmd.env_remove(key);
    }
    let uninstall = uninstall_cmd.output().unwrap();
    assert!(uninstall.status.success());

    let mut run_cmd = Command::new(cli);
    run_cmd.args(["run", "--stock-codex", &codex_bin, "--", "--version"]);
    for key in [
        "CODEX_HUD_CODEX_ROOT",
        "CODEX_HUD_COMPAT_MANIFEST",
        "CODEX_HUD_MANIFEST_PUBKEY",
        "CODEX_HUD_COMPAT_KEY",
    ] {
        run_cmd.env_remove(key);
    }
    let run = run_cmd.output().unwrap();
    assert!(run.status.success());
}
