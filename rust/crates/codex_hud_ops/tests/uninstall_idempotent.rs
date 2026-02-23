use codex_hud_ops::uninstall::run_uninstall;
use tempfile::tempdir;

#[test]
fn repeated_uninstall_is_safe() {
    let tmp = tempdir().unwrap();
    run_uninstall(tmp.path()).unwrap();
    run_uninstall(tmp.path()).unwrap();
}
