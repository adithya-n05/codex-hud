pub fn resolve_launch_mode(current: &str, supported: &[String]) -> &'static str {
    if supported.iter().any(|k| k == current) {
        "patched_codex"
    } else {
        "stock_codex"
    }
}
