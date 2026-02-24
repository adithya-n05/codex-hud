use codex_hud_ops::ensure_rc_block;
use codex_hud_ops::remove_rc_block;
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

#[test]
fn remove_rc_block_is_noop_when_file_is_missing() {
    let tmp = tempdir().unwrap();
    let rc = tmp.path().join(".zshrc");
    remove_rc_block(&rc).unwrap();
    assert!(!rc.exists());
}

#[test]
fn ensure_rc_block_errors_when_rc_path_is_a_directory() {
    let tmp = tempdir().unwrap();
    let rc_dir = tmp.path().join(".zshrc");
    std::fs::create_dir_all(&rc_dir).unwrap();

    let err = ensure_rc_block(&rc_dir, "/home/u/.codex-hud/bin").unwrap_err();
    assert!(!err.is_empty());
}
