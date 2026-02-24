use std::process::Command;
use tempfile::tempdir;

#[test]
fn binary_run_route_propagates_stock_failure() {
    let tmp = tempdir().unwrap();
    let stock = tmp.path().join("stock.sh");
    std::fs::write(&stock, "#!/usr/bin/env sh\nexit 13\n").unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&stock).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&stock, perms).unwrap();
    }

    let bin = env!("CARGO_BIN_EXE_codex_hud_cli");
    let out = Command::new(bin)
        .args([
            "run",
            "--stock-codex",
            stock.to_str().unwrap(),
            "--",
            "--help",
        ])
        .output()
        .unwrap();

    assert_eq!(out.status.code(), Some(13));
}
