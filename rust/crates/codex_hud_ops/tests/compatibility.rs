use codex_hud_ops::compatibility::resolve_launch_mode;
use codex_hud_ops::compatibility::is_supported_exact;

#[test]
fn unsupported_key_runs_stock_codex() {
    let mode = resolve_launch_mode("0.95.0+zzz", &["0.94.0+abc".to_string()]);
    assert_eq!(mode, "stock_codex");
}

#[test]
fn support_requires_exact_match() {
    let supported = vec!["0.94.0+abc123".to_string()];
    assert!(is_supported_exact("0.94.0+abc123", &supported));
    assert!(!is_supported_exact("0.94.0+zzz999", &supported));
}
