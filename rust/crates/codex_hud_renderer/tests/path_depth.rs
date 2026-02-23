use codex_hud_renderer::format::compact_path;

#[test]
fn compact_path_mode_shortens_repo_path() {
    let out = compact_path("/Users/u/work/personal/codex-statusbar", 2);
    assert_eq!(out, "personal/codex-statusbar");
}
