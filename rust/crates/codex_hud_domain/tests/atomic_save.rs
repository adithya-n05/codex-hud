use codex_hud_domain::save::atomic_write_with_backup;
use codex_hud_domain::save::restore_from_backup;

#[test]
fn atomic_save_rolls_back_on_failure() {
    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("config.toml");
    std::fs::write(&target, "old = true\n").unwrap();

    let res = atomic_write_with_backup(&target, "new = true\n", true);
    assert!(res.is_err());

    let after = std::fs::read_to_string(&target).unwrap();
    assert_eq!(after, "old = true\n");
}

#[test]
fn backup_file_restores_last_good_state() {
    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("config.toml");
    let backup = target.with_extension("toml.bak");

    std::fs::write(&target, "broken = true\n").unwrap();
    std::fs::write(&backup, "good = true\n").unwrap();

    restore_from_backup(&target).unwrap();
    let after = std::fs::read_to_string(&target).unwrap();
    assert_eq!(after, "good = true\n");
}
