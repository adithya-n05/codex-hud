use codex_hud_ops::status::{render_status_details, StatusSnapshot};

#[test]
fn details_include_paths_and_hashes() {
    let s = StatusSnapshot {
        installed: true,
        shim_present: true,
        rc_block_present: true,
        compatible: false,
        codex_version: Some("0.95.0".to_string()),
        codex_sha256: Some("abc123".to_string()),
        managed_root: Some("/home/u/.codex-hud".to_string()),
        stock_codex_path: Some("/usr/local/bin/codex".to_string()),
    };

    let text = render_status_details(&s);
    assert!(text.contains("codex_version: 0.95.0"));
    assert!(text.contains("codex_sha256: abc123"));
    assert!(text.contains("managed_root: /home/u/.codex-hud"));
    assert!(text.contains("stock_codex_path: /usr/local/bin/codex"));
    assert!(text.contains("unsupported_notice: shown_once"));
}
