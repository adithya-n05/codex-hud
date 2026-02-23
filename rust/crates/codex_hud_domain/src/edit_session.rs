use crate::{apply_preset, HudConfig, Preset};

#[derive(Debug, Clone)]
pub struct EditSession {
    current: HudConfig,
    previous_before_preset: Option<HudConfig>,
}

impl EditSession {
    pub fn new(current: HudConfig) -> Self {
        Self {
            current,
            previous_before_preset: None,
        }
    }

    pub fn apply_preset(&mut self, preset: Preset) {
        self.previous_before_preset = Some(self.current.clone());
        apply_preset(&mut self.current, preset);
    }

    pub fn undo_last_preset(&mut self) -> Result<(), String> {
        let previous = self
            .previous_before_preset
            .take()
            .ok_or_else(|| "no preset change to undo".to_string())?;
        self.current = previous;
        Ok(())
    }

    pub fn current(&self) -> &HudConfig {
        &self.current
    }
}
