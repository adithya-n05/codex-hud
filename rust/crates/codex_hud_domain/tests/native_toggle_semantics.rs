use codex_hud_domain::parse_hud_config;

#[test]
fn model_toggles_are_independent() {
    let src = r#"
[tui.codex_hud.native]
model_name = false
model_with_reasoning = true
"#;
    let cfg = parse_hud_config(src).unwrap();
    assert!(!cfg.native.model_name);
    assert!(cfg.native.model_with_reasoning);
}
