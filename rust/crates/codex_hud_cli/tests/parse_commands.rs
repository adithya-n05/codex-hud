use codex_hud_cli::{parse_args, Command};

#[test]
fn parse_install_command() {
    let cmd = parse_args(["codex-hud", "install"]).unwrap();
    assert_eq!(cmd, Command::Install);
}

#[test]
fn parse_uninstall_command() {
    let cmd = parse_args(["codex-hud", "uninstall"]).unwrap();
    assert_eq!(cmd, Command::Uninstall);
}
