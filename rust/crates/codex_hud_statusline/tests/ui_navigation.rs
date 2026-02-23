use codex_hud_statusline::{ConfigUiState, Key};

#[test]
fn down_key_moves_selection() {
    let mut ui = ConfigUiState::default();
    assert_eq!(ui.selected_index, 0);
    ui.on_key(Key::Down);
    assert_eq!(ui.selected_index, 1);
}

#[test]
fn up_key_clamps_at_zero() {
    let mut ui = ConfigUiState::default();
    ui.selected_index = 2;
    ui.on_key(Key::Up);
    assert_eq!(ui.selected_index, 1);
    ui.on_key(Key::Up);
    assert_eq!(ui.selected_index, 0);
    ui.on_key(Key::Up);
    assert_eq!(ui.selected_index, 0);
}
