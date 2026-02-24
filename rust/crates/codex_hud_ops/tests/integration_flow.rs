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

#[test]
fn install_creates_rc_file_when_missing() {
    let tmp = tempdir().unwrap();
    let home = tmp.path();
    let stock = home.join("stock-codex");
    std::fs::write(&stock, "#!/usr/bin/env sh\necho stock\n").unwrap();

    integration_install(home, stock.to_string_lossy().as_ref()).unwrap();
    assert!(home.join(".zshrc").exists());
}

#[test]
fn status_details_reads_existing_rc_file_without_block() {
    let tmp = tempdir().unwrap();
    let home = tmp.path();
    std::fs::write(home.join(".zshrc"), "# user config\n").unwrap();

    let out = integration_status_details(home).unwrap();
    assert!(out.contains("rc_block_present: false"));
    assert!(out.contains("compatible: true"));
}

#[test]
fn integration_exec_shim_errors_when_shim_is_missing() {
    let tmp = tempdir().unwrap();
    let err = integration_exec_shim(tmp.path(), &["--version"]).unwrap_err();
    assert!(!err.is_empty());
}

#[test]
fn status_reads_policy_and_compat_files_with_trimming() {
    let tmp = tempdir().unwrap();
    let home = tmp.path();
    std::fs::create_dir_all(home.join(".codex-hud/compat")).unwrap();
    std::fs::write(
        home.join(".codex-hud/last_run_policy.txt"),
        "mode=patched\nreason=ok\n",
    )
    .unwrap();
    std::fs::write(home.join(".codex-hud/compat/last_compat_key.txt"), " key \n").unwrap();
    std::fs::write(home.join(".codex-hud/compat/refresh_source.txt"), " source \n").unwrap();

    let out = integration_status(home).unwrap();
    assert!(out.contains("patch_mode: patched"));
    assert!(out.contains("compat_key: key"));
    assert!(out.contains("refresh_source: source"));
}

#[test]
fn uninstall_removes_managed_rc_block_when_present() {
    let tmp = tempdir().unwrap();
    let home = tmp.path();
    std::fs::create_dir_all(home.join(".codex-hud/bin")).unwrap();
    std::fs::write(home.join(".codex-hud/bin/codex"), "shim").unwrap();
    std::fs::write(
        home.join(".zshrc"),
        "# before\n# BEGIN CODEX HUD MANAGED BLOCK\nexport PATH=\"/tmp\"\n# END CODEX HUD MANAGED BLOCK\n# after\n",
    )
    .unwrap();

    integration_uninstall(home).unwrap();
    let rc = std::fs::read_to_string(home.join(".zshrc")).unwrap();
    assert!(!rc.contains("BEGIN CODEX HUD MANAGED BLOCK"));
}

#[test]
fn status_reads_policy_reason_without_mode_line() {
    let tmp = tempdir().unwrap();
    let home = tmp.path();
    std::fs::create_dir_all(home.join(".codex-hud")).unwrap();
    std::fs::write(
        home.join(".codex-hud/last_run_policy.txt"),
        "reason=unsupported compatibility key\n",
    )
    .unwrap();

    let out = integration_status(home).unwrap();
    assert!(out.contains("compatibility: unsupported"));
}

#[cfg(unix)]
#[test]
fn install_surfaces_rc_write_error_when_home_is_read_only() {
    use std::os::unix::fs::symlink;

    let tmp = tempdir().unwrap();
    let home = tmp.path().join("home");
    let managed_root = tmp.path().join("managed-root");
    std::fs::create_dir_all(&home).unwrap();
    std::fs::create_dir_all(&managed_root).unwrap();
    symlink(&managed_root, home.join(".codex-hud")).unwrap();

    let runtime = managed_root.join("bin/codex-hud");
    std::fs::create_dir_all(runtime.parent().unwrap()).unwrap();
    std::fs::write(&runtime, "custom-runtime").unwrap();

    let stock = tmp.path().join("stock-codex");
    std::fs::write(&stock, "#!/usr/bin/env sh\necho stock\n").unwrap();

    let mut perms = std::fs::metadata(&home).unwrap().permissions();
    perms.set_mode(0o555);
    std::fs::set_permissions(&home, perms).unwrap();

    let err = integration_install(&home, stock.to_string_lossy().as_ref()).unwrap_err();
    assert!(!err.is_empty());

    let mut reset = std::fs::metadata(&home).unwrap().permissions();
    reset.set_mode(0o755);
    std::fs::set_permissions(&home, reset).unwrap();
}

#[test]
fn install_errors_when_home_path_is_not_a_directory() {
    let tmp = tempdir().unwrap();
    let home = tmp.path().join("home-file");
    std::fs::write(&home, "not-a-dir").unwrap();
    let stock = tmp.path().join("stock-codex");
    std::fs::write(&stock, "#!/usr/bin/env sh\necho stock\n").unwrap();

    let err = integration_install(&home, stock.to_string_lossy().as_ref()).unwrap_err();
    assert!(!err.is_empty());
}

#[cfg(unix)]
#[test]
fn install_errors_when_runtime_script_cannot_be_written() {
    let tmp = tempdir().unwrap();
    let home = tmp.path().join("home");
    let managed_bin = home.join(".codex-hud/bin");
    std::fs::create_dir_all(&managed_bin).unwrap();
    let stock = tmp.path().join("stock-codex");
    std::fs::write(&stock, "#!/usr/bin/env sh\necho stock\n").unwrap();

    let mut perms = std::fs::metadata(&managed_bin).unwrap().permissions();
    perms.set_mode(0o555);
    std::fs::set_permissions(&managed_bin, perms).unwrap();

    let err = integration_install(&home, stock.to_string_lossy().as_ref()).unwrap_err();
    assert!(!err.is_empty());

    let mut reset = std::fs::metadata(&managed_bin).unwrap().permissions();
    reset.set_mode(0o755);
    std::fs::set_permissions(&managed_bin, reset).unwrap();
}

#[test]
fn install_errors_when_stock_pointer_target_is_directory() {
    let tmp = tempdir().unwrap();
    let home = tmp.path().join("home");
    std::fs::create_dir_all(home.join(".codex-hud/bin")).unwrap();
    std::fs::write(home.join(".codex-hud/bin/codex-hud"), "runtime").unwrap();
    std::fs::create_dir_all(home.join(".codex-hud/stock_codex_path.txt")).unwrap();
    let stock = tmp.path().join("stock-codex");
    std::fs::write(&stock, "#!/usr/bin/env sh\necho stock\n").unwrap();

    let err = integration_install(&home, stock.to_string_lossy().as_ref()).unwrap_err();
    assert!(!err.is_empty());
}

#[test]
fn install_errors_when_shim_path_is_directory() {
    let tmp = tempdir().unwrap();
    let home = tmp.path().join("home");
    std::fs::create_dir_all(home.join(".codex-hud/bin/codex")).unwrap();
    std::fs::write(home.join(".codex-hud/bin/codex-hud"), "runtime").unwrap();
    let stock = tmp.path().join("stock-codex");
    std::fs::write(&stock, "#!/usr/bin/env sh\necho stock\n").unwrap();

    let err = integration_install(&home, stock.to_string_lossy().as_ref()).unwrap_err();
    assert!(!err.is_empty());
}

#[test]
fn status_details_reads_stock_pointer_when_present() {
    let tmp = tempdir().unwrap();
    let home = tmp.path();
    std::fs::create_dir_all(home.join(".codex-hud/bin")).unwrap();
    std::fs::write(home.join(".codex-hud/bin/codex"), "shim").unwrap();
    std::fs::write(
        home.join(".codex-hud/stock_codex_path.txt"),
        "/usr/local/bin/codex\n",
    )
    .unwrap();

    let out = integration_status_details(home).unwrap();
    assert!(out.contains("stock_codex_path: /usr/local/bin/codex"));
}

#[test]
fn uninstall_errors_when_rc_path_is_directory() {
    let tmp = tempdir().unwrap();
    let home = tmp.path();
    std::fs::create_dir_all(home.join(".codex-hud/bin")).unwrap();
    std::fs::write(home.join(".codex-hud/bin/codex"), "shim").unwrap();
    std::fs::create_dir_all(home.join(".zshrc")).unwrap();

    let err = integration_uninstall(home).unwrap_err();
    assert!(!err.is_empty());
}

#[cfg(unix)]
#[test]
fn install_errors_when_rc_block_cannot_be_written() {
    use std::os::unix::fs::PermissionsExt;

    let tmp = tempdir().unwrap();
    let home = tmp.path().join("home");
    std::fs::create_dir_all(home.join(".codex-hud/bin")).unwrap();
    std::fs::write(home.join(".codex-hud/bin/codex-hud"), "runtime").unwrap();
    let stock = tmp.path().join("stock-codex");
    std::fs::write(&stock, "#!/usr/bin/env sh\necho stock\n").unwrap();

    let rc_path = home.join(".zshrc");
    std::fs::write(&rc_path, "# existing\n").unwrap();
    let mut perms = std::fs::metadata(&rc_path).unwrap().permissions();
    perms.set_mode(0o444);
    std::fs::set_permissions(&rc_path, perms).unwrap();

    let err = integration_install(&home, stock.to_string_lossy().as_ref()).unwrap_err();
    assert!(!err.is_empty());

    let mut reset = std::fs::metadata(&rc_path).unwrap().permissions();
    reset.set_mode(0o644);
    std::fs::set_permissions(&rc_path, reset).unwrap();
}
