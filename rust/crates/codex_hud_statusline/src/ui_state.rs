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
    }
}
