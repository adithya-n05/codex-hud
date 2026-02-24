use codex_hud_statusline::{ConfigUiEvent, ConfigUiState, Key};

#[test]
fn enter_on_non_save_row_returns_none() {
    let mut ui = ConfigUiState {
        selected_index: 3,
        row_count: 10,
        ..Default::default()
    };

    let event = ui.on_key_with_event(Key::Enter);
    assert_eq!(event, ConfigUiEvent::None);
    assert_eq!(ui.selected_index, 3);
}
