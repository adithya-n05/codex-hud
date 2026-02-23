use codex_hud_ops::write_codex_shim;
use tempfile::tempdir;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[test]
fn installer_writes_shim_file() {
    let tmp = tempdir().unwrap();
    let home = tmp.path();
    let target = write_codex_shim(home, "/usr/local/bin/codex").unwrap();

    assert!(target.exists());
    let text = std::fs::read_to_string(&target).unwrap();
    assert!(text.contains("codex-hud run"));
    assert!(text.contains("/usr/local/bin/codex"));
    #[cfg(unix)]
    {
        let mode = std::fs::metadata(&target).unwrap().permissions().mode();
        assert_eq!(mode & 0o111, 0o111);
    }
}
