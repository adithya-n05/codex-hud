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
        patch_mode: Some("stock".to_string()),
        patch_reason: Some("unsupported compatibility key".to_string()),
        compat_key: Some("0.95.0+deadbeef".to_string()),
        compat_refresh_source: Some("local-cache-fallback".to_string()),
    };

    let text = render_status_details(&s);
    assert!(text.contains("codex_version: 0.95.0"));
    assert!(text.contains("codex_sha256: abc123"));
    assert!(text.contains("managed_root: /home/u/.codex-hud"));
    assert!(text.contains("stock_codex_path: /usr/local/bin/codex"));
    assert!(text.contains("patch_mode: stock"));
    assert!(text.contains("patch_reason: unsupported compatibility key"));
    assert!(text.contains("compat_key: 0.95.0+deadbeef"));
    assert!(text.contains("compat_refresh_source: local-cache-fallback"));
    assert!(text.contains("unsupported_notice: shown_once"));
}
