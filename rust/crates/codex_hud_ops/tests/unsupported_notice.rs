use codex_hud_ops::unsupported_notice::should_show_unsupported_notice;
use std::path::PathBuf;

#[test]
fn unsupported_notice_only_once_per_version() {
    let state_path = PathBuf::from(std::env::temp_dir())
        .join(format!("codex_hud_unsupported_notice_{}_state.txt", std::process::id()));
    let _ = std::fs::remove_file(&state_path);
    assert!(should_show_unsupported_notice("0.95.0+zzz", &state_path).unwrap());
    assert!(!should_show_unsupported_notice("0.95.0+zzz", &state_path).unwrap());
    let _ = std::fs::remove_file(&state_path);
}
