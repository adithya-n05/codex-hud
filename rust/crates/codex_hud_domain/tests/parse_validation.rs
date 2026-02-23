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
