use codex_hud_domain::HudConfig;

#[test]
fn hud_config_default_exposes_all_runtime_sections() {
    let cfg = HudConfig::default();

    let _ = cfg.native.model_with_reasoning;
    let _ = cfg.derived.permission_chip;
    assert_eq!(cfg.visual.warn_percent, 70);
    assert!(!cfg.privacy.redact_auth_identity);
    assert_eq!(cfg.format.usage_mode, "bars");
    assert_eq!(cfg.tool_counter.scope, "session_total");
}
