use codex_hud_ops::paths::managed_paths;

#[test]
fn unix_path_matrix() {
    let p = managed_paths("/home/u", false);
    assert_eq!(p.root, "/home/u/.codex-hud");
    assert_eq!(p.bin_dir, "/home/u/.codex-hud/bin");
}
