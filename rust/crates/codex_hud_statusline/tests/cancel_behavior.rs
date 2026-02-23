use codex_hud_statusline::{ConfigUiEvent, ConfigUiState, Key};

#[test]
fn esc_closes_without_prompt() {
    let mut ui = ConfigUiState::default();
    let event = ui.on_key_with_event(Key::Esc);
    assert_eq!(event, ConfigUiEvent::CloseWithoutPrompt);
}
