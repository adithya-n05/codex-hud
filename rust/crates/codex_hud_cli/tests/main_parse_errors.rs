use std::process::Command;

#[test]
fn binary_exits_with_code_2_for_missing_command() {
    let bin = env!("CARGO_BIN_EXE_codex_hud_cli");
    let out = Command::new(bin).output().unwrap();

    assert_eq!(out.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("missing command"));
}
