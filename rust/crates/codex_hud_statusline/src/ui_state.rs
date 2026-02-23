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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigUiEvent {
    None,
    CloseWithoutPrompt,
    SaveAndClose,
}

impl Default for ConfigUiState {
    fn default() -> Self {
        Self {
            selected_index: 0,
            row_count: 10,
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
}

pub fn config_action_labels() -> Vec<&'static str> {
    vec!["Apply Preset", "Save", "Cancel"]
}
