use codex_hud_ops::uninstall::{
    reverse_patch_if_exact_state, run_uninstall, run_uninstall_with_rc,
};
use tempfile::tempdir;

#[test]
fn run_uninstall_removes_existing_root_tree() {
    let tmp = tempdir().expect("tempdir");
    let root = tmp.path().join(".codex-hud");
    std::fs::create_dir_all(root.join("bin")).expect("create root");
    std::fs::write(root.join("bin").join("codex-hud"), "shim").expect("write fixture");

    run_uninstall(tmp.path()).expect("uninstall");

    assert!(!root.exists());
}

#[test]
fn run_uninstall_returns_error_when_root_is_not_directory() {
    let tmp = tempdir().expect("tempdir");
    let root = tmp.path().join(".codex-hud");
    std::fs::write(&root, "not-a-directory").expect("write fixture");

    let err = run_uninstall(tmp.path()).expect_err("expected uninstall failure");
    assert!(!err.is_empty());
}

#[test]
fn run_uninstall_with_rc_accepts_missing_rc_file() {
    let tmp = tempdir().expect("tempdir");
    let root = tmp.path().join(".codex-hud");
    std::fs::create_dir_all(&root).expect("create root");
    let missing_rc = tmp.path().join(".zshrc");

    run_uninstall_with_rc(tmp.path(), &missing_rc).expect("uninstall with missing rc");

    assert!(!root.exists());
}

#[test]
fn run_uninstall_with_rc_surfaces_read_errors_and_skips_uninstall() {
    let tmp = tempdir().expect("tempdir");
    let root = tmp.path().join(".codex-hud");
    std::fs::create_dir_all(&root).expect("create root");
    let rc_as_dir = tmp.path().join(".zshrc");
    std::fs::create_dir_all(&rc_as_dir).expect("create directory rc");

    let err = run_uninstall_with_rc(tmp.path(), &rc_as_dir).expect_err("expected rc read error");
    assert!(err.contains("rc read error:"));
    assert!(root.exists());
}

#[test]
fn run_uninstall_with_rc_removes_managed_block_and_root() {
    let tmp = tempdir().expect("tempdir");
    let root = tmp.path().join(".codex-hud");
    std::fs::create_dir_all(&root).expect("create root");
    let rc = tmp.path().join(".zshrc");
    let content = r#"export FOO=bar
# BEGIN CODEX HUD MANAGED BLOCK
export PATH="/x/.codex-hud/bin:$PATH"
# END CODEX HUD MANAGED BLOCK
export BAR=baz
"#;
    std::fs::write(&rc, content).expect("write rc fixture");

    run_uninstall_with_rc(tmp.path(), &rc).expect("uninstall with managed rc");

    let after = std::fs::read_to_string(&rc).expect("read rc");
    assert!(after.contains("export FOO=bar"));
    assert!(after.contains("export BAR=baz"));
    assert!(!after.contains("CODEX HUD MANAGED BLOCK"));
    assert!(!root.exists());
}

#[test]
fn reverse_patch_rewrites_when_current_content_matches_expected() {
    let tmp = tempdir().expect("tempdir");
    let target = tmp.path().join("codex");
    std::fs::write(&target, "PATCHED-EXPECTED").expect("write fixture");

    let changed = reverse_patch_if_exact_state(&target, "PATCHED-EXPECTED", "ORIGINAL-STOCK")
        .expect("reverse patch");

    assert!(changed);
    assert_eq!(
        std::fs::read_to_string(&target).expect("read rewritten file"),
        "ORIGINAL-STOCK"
    );
}

#[test]
fn reverse_patch_returns_error_when_target_cannot_be_read() {
    let tmp = tempdir().expect("tempdir");
    let missing = tmp.path().join("does-not-exist");

    let err = reverse_patch_if_exact_state(&missing, "PATCHED-EXPECTED", "ORIGINAL-STOCK")
        .expect_err("expected read error");
    assert!(!err.is_empty());
}
