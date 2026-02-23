use codex_hud_domain::edit_session::EditSession;
use codex_hud_domain::{HudConfig, Preset};

#[test]
fn preset_undo_restores_previous_state() {
    let original = HudConfig::default();
    let mut session = EditSession::new(original.clone());
    session.apply_preset(Preset::Full);
    session.undo_last_preset().unwrap();
    assert_eq!(session.current(), &original);
}

#[test]
fn undo_without_snapshot_fails() {
    let mut s = EditSession::new(HudConfig::default());
    let err = s.undo_last_preset().unwrap_err();
    assert!(err.contains("no preset change to undo"));
}
