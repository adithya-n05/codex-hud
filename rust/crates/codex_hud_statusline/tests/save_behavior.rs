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

#[test]
fn no_reset_actions_exist_in_v1() {
    let labels = codex_hud_statusline::config_action_labels();
    assert!(!labels.iter().any(|v| v.contains("Reset to last saved")));
    assert!(!labels.iter().any(|v| v.contains("Reset to factory")));
}
