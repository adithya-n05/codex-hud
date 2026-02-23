use codex_hud_statusline::{ConfigUiEvent, ConfigUiState, Key};

#[test]
fn enter_on_save_row_saves_and_closes() {
    let mut ui = ConfigUiState {
        selected_index: 9,
        row_count: 10,
    };

    let event = ui.on_key_with_event(Key::Enter);
    assert_eq!(event, ConfigUiEvent::SaveAndClose);
}
