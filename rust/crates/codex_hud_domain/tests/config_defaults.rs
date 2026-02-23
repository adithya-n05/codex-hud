use codex_hud_domain::{HudConfig, Preset};

#[test]
fn default_preset_is_essential() {
    let cfg = HudConfig::default();
    assert_eq!(cfg.preset, Preset::Essential);
}
