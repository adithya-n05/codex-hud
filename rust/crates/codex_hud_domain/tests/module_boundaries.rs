use codex_hud_domain::config::{parse_hud_config as parse_from_config, HudConfig};
use codex_hud_domain::preset::{apply_preset, Preset};

#[test]
fn hud_config_is_exposed_from_config_module() {
    let cfg = HudConfig::default();
    assert_eq!(cfg.preset, Preset::Essential);

    let err = parse_from_config("[broken\n").unwrap_err();
    assert!(!err.trim().is_empty());
}

#[test]
fn apply_preset_is_exposed_from_preset_module() {
    let mut cfg = HudConfig::default();
    apply_preset(&mut cfg, Preset::Full);
    assert_eq!(cfg.preset, Preset::Full);
}
