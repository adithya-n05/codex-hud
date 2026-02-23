use codex_hud_ops::unsupported_notice::should_show_unsupported_notice;
use codex_hud_ops::unsupported_notice::build_unsupported_notice_message;
use codex_hud_ops::unsupported_notice::manual_unsupported_notice_reshow_available;

#[test]
fn unsupported_notice_only_once_per_version() {
    let state_path = std::env::temp_dir()
        .join(format!("codex_hud_unsupported_notice_{}_state.txt", std::process::id()));
    let _ = std::fs::remove_file(&state_path);
    assert!(should_show_unsupported_notice("0.95.0+zzz", &state_path).unwrap());
    assert!(!should_show_unsupported_notice("0.95.0+zzz", &state_path).unwrap());
    let _ = std::fs::remove_file(&state_path);
}

#[test]
fn unsupported_notice_includes_support_check_action() {
    let msg = build_unsupported_notice_message("0.95.0+zzz");
    assert!(msg.contains("Run `codex-hud status details` to check compatibility support"));
}

#[test]
fn unsupported_notice_has_no_manual_reshow_control() {
    assert!(!manual_unsupported_notice_reshow_available());
}
