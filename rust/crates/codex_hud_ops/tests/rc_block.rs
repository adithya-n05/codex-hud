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

#[test]
fn ensure_rc_block_errors_when_parent_directory_is_missing() {
    let tmp = tempdir().unwrap();
    let rc = tmp.path().join("missing/.zshrc");
    let err = ensure_rc_block(&rc, "/home/u/.codex-hud/bin").unwrap_err();
    assert!(!err.is_empty());
}

#[cfg(unix)]
#[test]
fn ensure_rc_block_errors_when_existing_rc_file_is_read_only() {
    use std::os::unix::fs::PermissionsExt;

    let tmp = tempdir().unwrap();
    let rc = tmp.path().join(".zshrc");
    std::fs::write(&rc, "export FOO=bar\n").unwrap();
    let mut perms = std::fs::metadata(&rc).unwrap().permissions();
    perms.set_mode(0o444);
    std::fs::set_permissions(&rc, perms).unwrap();

    let err = ensure_rc_block(&rc, "/home/u/.codex-hud/bin").unwrap_err();
    assert!(!err.is_empty());
}

#[cfg(unix)]
#[test]
fn remove_rc_block_errors_when_rc_file_is_read_only() {
    use std::os::unix::fs::PermissionsExt;

    let tmp = tempdir().unwrap();
    let rc = tmp.path().join(".zshrc");
    std::fs::write(
        &rc,
        "# BEGIN CODEX HUD MANAGED BLOCK\nexport PATH=\"/tmp\"\n# END CODEX HUD MANAGED BLOCK\n",
    )
    .unwrap();
    let mut perms = std::fs::metadata(&rc).unwrap().permissions();
    perms.set_mode(0o444);
    std::fs::set_permissions(&rc, perms).unwrap();

    let err = remove_rc_block(&rc).unwrap_err();
    assert!(!err.is_empty());
}
