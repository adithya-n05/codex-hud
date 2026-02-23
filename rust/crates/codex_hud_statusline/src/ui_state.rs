#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
    Enter,
    Esc,
    Space,
}

#[derive(Debug, Clone)]
pub struct ConfigUiState {
    pub selected_index: usize,
    pub row_count: usize,
    pub current_config: String,
    pub last_live_apply_payload: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigUiEvent {
    None,
    CloseWithoutPrompt,
    SaveAndClose,
    LiveApply,
}

impl Default for ConfigUiState {
    fn default() -> Self {
        Self {
            selected_index: 0,
            row_count: 10,
            current_config: String::new(),
            last_live_apply_payload: None,
        }
    }
}

impl ConfigUiState {
    pub fn on_key(&mut self, key: Key) {
        if key == Key::Down && self.selected_index + 1 < self.row_count {
            self.selected_index += 1;
        }
        if key == Key::Up && self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn on_key_with_event(&mut self, key: Key) -> ConfigUiEvent {
        if key == Key::Esc {
            return ConfigUiEvent::CloseWithoutPrompt;
        }
        if key == Key::Enter && self.selected_index == self.row_count.saturating_sub(1) {
            return ConfigUiEvent::SaveAndClose;
        }
        self.on_key(key);
        ConfigUiEvent::None
    }

    pub fn on_toggle_changed(&mut self, key: &str, value: bool) -> ConfigUiEvent {
        let normalized_key = key.replace('-', "_");
        if normalized_key == "native.model_with_reasoning" {
            let bool_text = if value { "true" } else { "false" };
            self.current_config = self
                .current_config
                .replace("model_with_reasoning = true", "model_with_reasoning = __VALUE__")
                .replace("model_with_reasoning = false", "model_with_reasoning = __VALUE__")
                .replace("__VALUE__", bool_text);
            self.last_live_apply_payload = Some(self.current_config.clone());
        }
        ConfigUiEvent::LiveApply
    }
}

pub fn config_action_labels() -> Vec<&'static str> {
    vec!["Apply Preset", "Save", "Cancel"]
}
