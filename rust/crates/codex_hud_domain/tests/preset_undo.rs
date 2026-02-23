use codex_hud_domain::{EditSession, HudConfig, Preset};

#[test]
fn preset_undo_restores_previous_state() {
    let original = HudConfig::default();
    let mut session = EditSession::new(original.clone());
    session.apply_preset(Preset::Full);
    session.undo_last_preset().unwrap();
    assert_eq!(session.current(), &original);
}
