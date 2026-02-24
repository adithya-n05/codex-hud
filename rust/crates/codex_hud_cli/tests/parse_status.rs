use codex_hud_cli::{parse_args, Command};

#[test]
fn parse_status_command() {
    let cmd = parse_args(["codex-hud", "status"]).unwrap();
    assert_eq!(cmd, Command::Status);
}

#[test]
fn parse_status_details_command() {
    let cmd = parse_args(["codex-hud", "status", "details"]).unwrap();
    assert_eq!(cmd, Command::StatusDetails);
}

#[test]
fn parse_status_details_flag_command() {
    let cmd = parse_args(["codex-hud", "status", "--details"]).unwrap();
    assert_eq!(cmd, Command::StatusDetails);
}

#[test]
fn parse_run_command_with_stock_codex_and_passthrough() {
    let cmd = parse_args([
        "codex-hud",
        "run",
        "--stock-codex",
        "/usr/local/bin/codex",
        "--",
        "--version",
    ])
    .unwrap();

    assert_eq!(
        cmd,
        Command::Run {
            stock_codex_path: "/usr/local/bin/codex".to_string(),
            passthrough_args: vec!["--version".to_string()],
        }
    );
}
