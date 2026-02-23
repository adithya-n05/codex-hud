use codex_hud_domain::{apply_preset, HudConfig, Preset};

#[test]
fn apply_preset_overwrites_previous_values() {
    let mut cfg = HudConfig::default();
    cfg.derived.provider_chip = false;
    apply_preset(&mut cfg, Preset::Essential);
    assert!(cfg.derived.provider_chip);
}
