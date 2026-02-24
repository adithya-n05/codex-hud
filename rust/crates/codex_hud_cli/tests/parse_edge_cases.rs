use codex_hud_cli::{parse_args, Command};

#[test]
fn parse_missing_command_fails() {
    let err = parse_args(["codex-hud"]).unwrap_err();
    assert_eq!(err, "missing command");
}

#[test]
fn parse_status_with_unknown_modifier_stays_in_summary_mode() {
    let cmd = parse_args(["codex-hud", "status", "verbose"]).unwrap();
    assert_eq!(cmd, Command::Status);
}

#[test]
fn parse_run_requires_stock_codex_flag() {
    let err = parse_args(["codex-hud", "run"]).unwrap_err();
    assert_eq!(err, "missing --stock-codex");
}

#[test]
fn parse_run_requires_stock_codex_path() {
    let err = parse_args(["codex-hud", "run", "--stock-codex"]).unwrap_err();
    assert_eq!(err, "missing stock codex path");
}

#[test]
fn parse_run_without_separator_collects_trailing_args() {
    let cmd = parse_args([
        "codex-hud",
        "run",
        "--stock-codex",
        "/usr/local/bin/codex",
        "--version",
        "--json",
    ])
    .unwrap();

    assert_eq!(
        cmd,
        Command::Run {
            stock_codex_path: "/usr/local/bin/codex".to_string(),
            passthrough_args: vec!["--version".to_string(), "--json".to_string()],
        }
    );
}

#[test]
fn parse_run_with_separator_and_no_trailing_args_keeps_empty_passthrough() {
    let cmd = parse_args(["codex-hud", "run", "--stock-codex", "/usr/local/bin/codex", "--"])
        .unwrap();

    assert_eq!(
        cmd,
        Command::Run {
            stock_codex_path: "/usr/local/bin/codex".to_string(),
            passthrough_args: Vec::new(),
        }
    );
}
