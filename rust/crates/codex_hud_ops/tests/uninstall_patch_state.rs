use codex_hud_ops::uninstall::reverse_patch_if_exact_state;
use tempfile::tempdir;

#[test]
fn reverse_patch_requires_exact_patched_state() {
    let tmp = tempdir().unwrap();
    let target = tmp.path().join("codex");
    std::fs::write(&target, "CURRENT-UNEXPECTED").unwrap();

    let before = std::fs::read_to_string(&target).unwrap();
    let changed =
        reverse_patch_if_exact_state(&target, "PATCHED-EXPECTED", "ORIGINAL-STOCK").unwrap();
    assert!(!changed);
    let after = std::fs::read_to_string(&target).unwrap();
    assert_eq!(after, before);
}
