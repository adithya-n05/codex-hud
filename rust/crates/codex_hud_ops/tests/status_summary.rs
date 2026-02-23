use codex_hud_ops::status::{render_status_summary, StatusSnapshot};

#[test]
fn summary_contains_core_install_state() {
    let s = StatusSnapshot {
        installed: true,
        shim_present: true,
        rc_block_present: true,
        compatible: true,
        codex_version: Some("0.94.0".to_string()),
        ..StatusSnapshot::default()
    };

    let text = render_status_summary(&s);
    assert!(text.contains("installed: yes"));
    assert!(text.contains("shim: present"));
    assert!(text.contains("compatibility: supported"));
}
