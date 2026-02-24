use codex_hud_domain::{apply_preset, HudConfig, Preset};

#[test]
fn apply_preset_overwrites_previous_values() {
    let mut cfg = HudConfig::default();
    cfg.derived.provider_chip = false;
    apply_preset(&mut cfg, Preset::Essential);
    assert!(cfg.derived.provider_chip);
}

#[test]
fn apply_preset_minimal_disables_derived_fields() {
    let mut cfg = HudConfig::default();
    cfg.derived.provider_chip = true;
    cfg.derived.five_hour_bar = true;
    cfg.derived.weekly_bar = true;

    apply_preset(&mut cfg, Preset::Minimal);

    assert!(!cfg.derived.provider_chip);
    assert!(!cfg.derived.five_hour_bar);
    assert!(!cfg.derived.weekly_bar);
}

#[test]
fn apply_preset_full_enables_all_derived_fields() {
    let mut cfg = HudConfig::default();
    cfg.derived.provider_chip = false;
    cfg.derived.five_hour_bar = false;
    cfg.derived.weekly_bar = false;
    cfg.derived.tool_counter = false;
    cfg.derived.failure_count = false;
    cfg.derived.activity_summary = false;

    apply_preset(&mut cfg, Preset::Full);

    assert!(cfg.derived.provider_chip);
    assert!(cfg.derived.five_hour_bar);
    assert!(cfg.derived.weekly_bar);
    assert!(cfg.derived.tool_counter);
    assert!(cfg.derived.failure_count);
    assert!(cfg.derived.activity_summary);
}
