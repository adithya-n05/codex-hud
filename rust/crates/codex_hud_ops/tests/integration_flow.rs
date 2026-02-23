use codex_hud_ops::integration_flow::{
    integration_exec_shim,
    integration_install,
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
