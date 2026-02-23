use codex_hud_domain::verify_must_cover_mapping;

#[test]
fn must_cover_mapping_has_no_gaps() {
    let missing = verify_must_cover_mapping();
    assert!(missing.is_empty(), "missing keys: {missing:?}");
}
