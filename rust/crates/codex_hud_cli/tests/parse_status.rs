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
