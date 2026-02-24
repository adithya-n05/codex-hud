use codex_hud_ops::status::{render_status_summary, StatusSnapshot};

#[test]
fn summary_contains_core_install_state() {
    let s = StatusSnapshot {
        installed: true,
        shim_present: true,
        rc_block_present: true,
        compatible: true,
        codex_version: Some("0.94.0".to_string()),
        patch_mode: Some("patched".to_string()),
        compat_key: Some("0.104.0+abc123".to_string()),
        compat_refresh_source: Some("github-release".to_string()),
        ..StatusSnapshot::default()
    };

    let text = render_status_summary(&s);
    assert!(text.contains("installed: yes"));
    assert!(text.contains("shim: present"));
    assert!(text.contains("compatibility: supported"));
    assert!(text.contains("patch_mode: patched"));
    assert!(text.contains("compat_key: 0.104.0+abc123"));
    assert!(text.contains("refresh_source: github-release"));
}
