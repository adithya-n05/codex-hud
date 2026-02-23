use codex_hud_statusline::{ConfigUiEvent, ConfigUiState, Key};

#[test]
fn enter_on_save_row_saves_and_closes() {
    let mut ui = ConfigUiState {
        selected_index: 9,
        row_count: 10,
        ..Default::default()
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

#[test]
fn toggle_change_emits_live_apply_event() {
    let mut ui = ConfigUiState::default();
    let event = ui.on_toggle_changed("native.model-with-reasoning", false);
    assert_eq!(event, ConfigUiEvent::LiveApply);
}

#[test]
fn live_apply_updates_serialized_config_for_supported_toggle() {
    let mut ui = ConfigUiState {
        current_config: "[tui.codex_hud.native]\nmodel_with_reasoning = true\n".to_string(),
        ..Default::default()
    };
    let event = ui.on_toggle_changed("native.model_with_reasoning", false);
    assert_eq!(event, ConfigUiEvent::LiveApply);
    assert!(ui.current_config.contains("model_with_reasoning = false"));
    assert!(
        ui.last_live_apply_payload
            .as_ref()
            .is_some_and(|v| v.contains("model_with_reasoning = false"))
    );
}

#[test]
fn invalid_toggle_key_is_noop_for_live_apply() {
    let mut ui = ConfigUiState {
        current_config: "[tui.codex_hud.native]\nmodel_with_reasoning = true\n".to_string(),
        ..Default::default()
    };
    let before = ui.current_config.clone();
    let event = ui.on_toggle_changed("native.unknown_toggle", false);
    assert_eq!(event, ConfigUiEvent::None);
    assert_eq!(ui.current_config, before);
    assert!(ui.last_live_apply_payload.is_none());
}
