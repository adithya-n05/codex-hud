use codex_hud_ops::compatibility::resolve_launch_mode;

#[test]
fn unsupported_key_runs_stock_codex() {
    let mode = resolve_launch_mode("0.95.0+zzz", &["0.94.0+abc".to_string()]);
    assert_eq!(mode, "stock_codex");
}
