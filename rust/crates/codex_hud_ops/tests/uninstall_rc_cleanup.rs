use codex_hud_ops::remove_rc_block;
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
