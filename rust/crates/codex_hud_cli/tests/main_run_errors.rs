use std::process::Command;
use tempfile::tempdir;

#[test]
fn run_route_fails_with_exit_code_one_when_stock_binary_cannot_spawn() {
    let tmp = tempdir().unwrap();
    let home = tmp.path().join("home");
    std::fs::create_dir_all(&home).unwrap();
    let missing_stock = tmp.path().join("missing-stock-codex");

    let bin = env!("CARGO_BIN_EXE_codex_hud_cli");
    let out = Command::new(bin)
        .args([
            "run",
            "--stock-codex",
            missing_stock.to_str().unwrap(),
            "--",
            "--version",
        ])
        .env("HOME", &home)
        .output()
        .unwrap();

    assert_eq!(out.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("No such file")
            || stderr.contains("os error")
            || stderr.contains("cannot find")
    );

    let policy = std::fs::read_to_string(home.join(".codex-hud/last_run_policy.txt")).unwrap();
    assert!(policy.contains("mode="));
}
