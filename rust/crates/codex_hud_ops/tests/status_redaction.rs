use codex_hud_ops::status::{render_status_details, StatusSnapshot};

#[test]
fn details_do_not_echo_secret_values() {
    let s = StatusSnapshot {
        codex_sha256: Some("sk-secret-token-123".to_string()),
        ..StatusSnapshot::default()
    };
    let text = render_status_details(&s);
    assert!(!text.contains("sk-secret-token-123"));
}
