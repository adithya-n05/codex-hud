use codex_hud_domain::parse_hud_config;

#[test]
fn parse_rejects_warn_above_critical() {
    let src = r#"
[tui.codex_hud.visual]
warn_percent = 90
critical_percent = 80
"#;

    let err = parse_hud_config(src).unwrap_err();
    assert!(err.contains("warn_percent must be <= critical_percent"));
}

#[test]
fn parse_rejects_threshold_out_of_range() {
    let src = r#"
[tui.codex_hud.visual]
warn_percent = 200
critical_percent = 250
"#;

    let err = parse_hud_config(src).unwrap_err();
    assert!(err.contains("threshold must be between 0 and 100"));
}

#[test]
fn parse_rejects_project_level_block() {
    let src = r#"
[project.codex_hud]
enabled = true
"#;

    let err = parse_hud_config(src).unwrap_err();
    assert!(err.contains("project-level codex_hud config is not supported"));
}

#[test]
fn parse_rejects_negative_threshold() {
    let src = r#"
[tui.codex_hud.visual]
warn_percent = -1
critical_percent = 85
"#;
    let err = parse_hud_config(src).unwrap_err();
    assert!(err.contains("threshold must be between 0 and 100"));
}

#[test]
fn parse_returns_syntax_error_for_malformed_toml() {
    let src = r#"[tui.codex_hud.visual\nwarn_percent = 70"#;
    let err = parse_hud_config(src).unwrap_err();
    assert!(err.starts_with("syntax error:"));
}

#[test]
fn parse_rejects_unknown_key() {
    let src = r#"
[tui.codex_hud]
mystery = true
"#;
    let err = parse_hud_config(src).unwrap_err();
    assert!(err.contains("unknown key in tui.codex_hud"));
}
