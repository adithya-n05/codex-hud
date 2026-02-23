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

#[test]
fn details_redact_secret_like_values_across_all_fields() {
    let s = StatusSnapshot {
        codex_sha256: Some("sk-secret-token-123".to_string()),
        managed_root: Some("/home/u/token-path".to_string()),
        stock_codex_path: Some("/usr/local/bin/bearer-path".to_string()),
        ..StatusSnapshot::default()
    };
    let text = render_status_details(&s);
    assert!(!text.contains("sk-secret-token-123"));
    assert!(!text.contains("token-path"));
    assert!(!text.contains("bearer-path"));
}
