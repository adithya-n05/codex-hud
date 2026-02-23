use codex_hud_ops::remove_rc_block;
use codex_hud_ops::uninstall::run_uninstall_with_rc;
use tempfile::tempdir;

#[test]
fn uninstall_removes_only_managed_block() {
    let tmp = tempdir().unwrap();
    let rc = tmp.path().join(".zshrc");
    let content = r#"export FOO=bar
# BEGIN CODEX HUD MANAGED BLOCK
export PATH="/x/.codex-hud/bin:$PATH"
# END CODEX HUD MANAGED BLOCK
export BAR=baz
"#;
    std::fs::write(&rc, content).unwrap();

    remove_rc_block(&rc).unwrap();

    let out = std::fs::read_to_string(&rc).unwrap();
    assert!(out.contains("export FOO=bar"));
    assert!(out.contains("export BAR=baz"));
    assert!(!out.contains("CODEX HUD MANAGED BLOCK"));
}

#[test]
fn remove_rc_block_surfaces_rc_read_error_context() {
    let dir = tempfile::tempdir().unwrap();
    let err = remove_rc_block(dir.path()).unwrap_err();
    assert!(err.contains("rc read error:"));
}

#[test]
fn uninstall_does_not_modify_unmanaged_rc_content() {
    let tmp = tempdir().unwrap();
    let rc = tmp.path().join(".zshrc");
    std::fs::write(&rc, "export FOO=bar\nexport BAR=baz\n").unwrap();

    run_uninstall_with_rc(tmp.path(), &rc).unwrap();
    let after = std::fs::read_to_string(&rc).unwrap();
    assert_eq!(after, "export FOO=bar\nexport BAR=baz\n");
}
