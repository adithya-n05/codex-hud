use codex_hud_renderer::wrap::wrap_line_unicode_safe;

#[test]
fn wrap_unicode_blocks_is_safe() {
    let line = "CTX 95% [███████████████████░]";
    let wrapped = wrap_line_unicode_safe(line, 8);
    assert!(!wrapped.is_empty());
    assert!(wrapped.iter().all(|chunk| !chunk.is_empty()));
}
