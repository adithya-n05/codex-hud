#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Preset {
    Minimal,
    Essential,
    Full,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HudConfig {
    pub preset: Preset,
}

impl Default for HudConfig {
    fn default() -> Self {
        Self {
            preset: Preset::Essential,
        }
    }
}

pub fn domain_ready() -> bool {
    true
}
