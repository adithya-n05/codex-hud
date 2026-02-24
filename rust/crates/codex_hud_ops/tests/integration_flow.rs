use codex_hud_ops::integration_flow::{
    integration_exec_shim,
    integration_install,
    integration_status_details,
    integration_status,
    integration_uninstall,
};
use tempfile::tempdir;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[test]
fn integration_flow_install_status_uninstall() {
    let tmp = tempdir().unwrap();
    let home = tmp.path();
    let stock = home.join("stock-codex");
    std::fs::write(&stock, "#!/usr/bin/env sh\necho \"stock-exec:$*\"\n").unwrap();
    #[cfg(unix)]
    {
        let mut perms = std::fs::metadata(&stock).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&stock, perms).unwrap();
    }

    integration_install(home, stock.to_string_lossy().as_ref()).unwrap();
    #[cfg(unix)]
    {
        let shim = home.join(".codex-hud").join("bin").join("codex");
        let mode = std::fs::metadata(shim).unwrap().permissions().mode();
        assert_eq!(mode & 0o111, 0o111);
    }
    let shim_out = integration_exec_shim(home, &["--version", "--json"]).unwrap();
    assert!(shim_out.contains("stock-exec:--version --json"));

    let status = integration_status(home).unwrap();
    assert!(status.contains("installed: yes"));
    assert!(status.contains("shim: present"));
    assert!(status.contains("runtime: present"));

    integration_uninstall(home).unwrap();
    let status_after = integration_status(home).unwrap();
    assert!(status_after.contains("installed: no"));
}

#[test]
fn status_works_when_rc_file_and_policy_files_are_missing() {
    let tmp = tempdir().unwrap();
    let out = integration_status(tmp.path()).unwrap();
    assert!(out.contains("installed: no"));
    assert!(out.contains("compatibility: supported"));
}

#[test]
fn status_surfaces_rc_read_errors() {
    let tmp = tempdir().unwrap();
    let rc_dir = tmp.path().join(".zshrc");
    std::fs::create_dir_all(&rc_dir).unwrap();

    let err = integration_status(tmp.path()).unwrap_err();
    assert!(err.contains("rc read error:"));
}

#[test]
fn status_details_surfaces_rc_read_errors() {
    let tmp = tempdir().unwrap();
    let rc_dir = tmp.path().join(".zshrc");
    std::fs::create_dir_all(&rc_dir).unwrap();

    let err = integration_status_details(tmp.path()).unwrap_err();
    assert!(err.contains("rc read error:"));
}

#[test]
fn status_details_uses_policy_reason_to_mark_incompatible() {
    let tmp = tempdir().unwrap();
    let home = tmp.path();
    std::fs::create_dir_all(home.join(".codex-hud/bin")).unwrap();
    std::fs::create_dir_all(home.join(".codex-hud/compat")).unwrap();
    std::fs::write(home.join(".codex-hud/bin/codex"), "shim").unwrap();
    std::fs::write(
        home.join(".codex-hud/last_run_policy.txt"),
        "mode=stock\nreason=unsupported compatibility key\n",
    )
    .unwrap();
    std::fs::write(home.join(".codex-hud/compat/last_compat_key.txt"), " 0.104.0+abc \n").unwrap();
    std::fs::write(home.join(".codex-hud/compat/refresh_source.txt"), " github-release \n").unwrap();

    let out = integration_status_details(home).unwrap();
    assert!(out.contains("compatible: false"));
    assert!(out.contains("compat_key: 0.104.0+abc"));
    assert!(out.contains("compat_refresh_source: github-release"));
}

#[test]
fn install_preserves_existing_runtime_script() {
    let tmp = tempdir().unwrap();
    let home = tmp.path();
    let runtime = home.join(".codex-hud/bin/codex-hud");
    std::fs::create_dir_all(runtime.parent().unwrap()).unwrap();
    std::fs::write(&runtime, "custom-runtime").unwrap();
    let stock = home.join("stock-codex");
    std::fs::write(&stock, "#!/usr/bin/env sh\necho stock\n").unwrap();

    integration_install(home, stock.to_string_lossy().as_ref()).unwrap();
    assert_eq!(std::fs::read_to_string(runtime).unwrap(), "custom-runtime");
}

#[test]
fn uninstall_succeeds_when_rc_file_is_missing() {
    let tmp = tempdir().unwrap();
    let home = tmp.path();
    std::fs::create_dir_all(home.join(".codex-hud/bin")).unwrap();
    std::fs::write(home.join(".codex-hud/bin/codex"), "shim").unwrap();

    integration_uninstall(home).unwrap();
    assert!(!home.join(".codex-hud").exists());
}
