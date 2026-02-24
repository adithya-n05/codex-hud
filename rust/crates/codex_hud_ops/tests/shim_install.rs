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

#[test]
fn shim_quotes_stock_codex_path_and_preserves_passthrough_args() {
    let tmp = tempdir().unwrap();
    let target = write_codex_shim(tmp.path(), "/opt/Codex App/bin/codex").unwrap();
    let text = std::fs::read_to_string(&target).unwrap();

    assert!(text.contains("exec codex-hud run --stock-codex"));
    assert!(text.contains("\"/opt/Codex App/bin/codex\""));
    assert!(text.contains("\"$@\""));
}

#[test]
fn write_codex_shim_fails_when_codex_hud_root_is_a_file() {
    let tmp = tempdir().unwrap();
    let blocked_root = tmp.path().join(".codex-hud");
    std::fs::write(&blocked_root, "not-a-directory").unwrap();

    let err = write_codex_shim(tmp.path(), "/usr/local/bin/codex").unwrap_err();
    assert!(!err.is_empty());
}

#[test]
fn write_codex_shim_fails_when_target_path_is_directory() {
    let tmp = tempdir().unwrap();
    let target_dir = tmp.path().join(".codex-hud").join("bin").join("codex");
    std::fs::create_dir_all(&target_dir).unwrap();

    let err = write_codex_shim(tmp.path(), "/usr/local/bin/codex").unwrap_err();
    assert!(!err.is_empty());
}
