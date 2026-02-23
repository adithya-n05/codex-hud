use codex_hud_domain::config::{parse_hud_config as parse_from_config, HudConfig};
use codex_hud_domain::Preset;

#[test]
fn hud_config_is_exposed_from_config_module() {
    let cfg = HudConfig::default();
    assert_eq!(cfg.preset, Preset::Essential);

    let err = parse_from_config("[broken\n").unwrap_err();
    assert!(!err.trim().is_empty());
}
