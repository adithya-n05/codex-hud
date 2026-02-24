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

#[test]
fn unsupported_notice_surfaces_state_file_read_errors() {
    let dir = tempfile::tempdir().unwrap();
    let err = should_show_unsupported_notice("0.95.0+zzz", dir.path()).unwrap_err();
    assert!(!err.is_empty());
}

#[cfg(unix)]
#[test]
fn unsupported_notice_surfaces_state_file_write_errors() {
    use std::os::unix::fs::PermissionsExt;

    let dir = tempfile::tempdir().unwrap();
    let read_only_dir = dir.path().join("readonly");
    std::fs::create_dir_all(&read_only_dir).unwrap();
    let mut perms = std::fs::metadata(&read_only_dir).unwrap().permissions();
    perms.set_mode(0o555);
    std::fs::set_permissions(&read_only_dir, perms).unwrap();

    let err = should_show_unsupported_notice("0.95.0+zzz", &read_only_dir.join("state.txt"))
        .unwrap_err();
    assert!(!err.is_empty());
}
