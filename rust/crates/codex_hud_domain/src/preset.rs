use crate::HudConfig;
pub use crate::Preset;

pub fn apply_preset(cfg: &mut HudConfig, preset: Preset) {
    cfg.preset = preset;
    match preset {
        Preset::Minimal => {
            cfg.derived.provider_chip = false;
            cfg.derived.five_hour_bar = false;
            cfg.derived.weekly_bar = false;
            cfg.derived.tool_counter = false;
            cfg.derived.failure_count = false;
            cfg.derived.activity_summary = false;
        }
        Preset::Essential => {
            cfg.derived.provider_chip = true;
            cfg.derived.five_hour_bar = true;
            cfg.derived.weekly_bar = true;
            cfg.derived.tool_counter = false;
            cfg.derived.failure_count = false;
            cfg.derived.activity_summary = false;
        }
        Preset::Full => {
            cfg.derived.provider_chip = true;
            cfg.derived.five_hour_bar = true;
            cfg.derived.weekly_bar = true;
            cfg.derived.tool_counter = true;
            cfg.derived.failure_count = true;
            cfg.derived.activity_summary = true;
        }
    }
}
