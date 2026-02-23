use codex_hud_ops::ensure_rc_block;
use tempfile::tempdir;

#[test]
fn rc_block_is_appended_once() {
    let tmp = tempdir().unwrap();
    let rc = tmp.path().join(".zshrc");
    std::fs::write(&rc, "export FOO=bar\n").unwrap();

    ensure_rc_block(&rc, "/home/u/.codex-hud/bin").unwrap();
    ensure_rc_block(&rc, "/home/u/.codex-hud/bin").unwrap();

    let text = std::fs::read_to_string(&rc).unwrap();
    assert_eq!(text.matches("BEGIN CODEX HUD MANAGED BLOCK").count(), 1);
    assert!(text.contains("/home/u/.codex-hud/bin"));
}

#[test]
fn rc_file_is_created_when_missing() {
    let tmp = tempdir().unwrap();
    let rc = tmp.path().join(".zshrc");
    assert!(!rc.exists());

    ensure_rc_block(&rc, "/home/u/.codex-hud/bin").unwrap();
    assert!(rc.exists());
}
