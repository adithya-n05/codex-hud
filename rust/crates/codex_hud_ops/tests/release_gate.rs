use codex_hud_ops::release_gate::release_gate_requirements;

#[test]
fn release_gate_requires_snapshot_tests() {
    let req = release_gate_requirements();
    assert!(req.contains(&"snapshot_tests".to_string()));
}

#[test]
fn release_gate_includes_cross_platform_smoke_matrix() {
    let req = release_gate_requirements();
    assert!(req.contains(&"smoke:macos-latest".to_string()));
    assert!(req.contains(&"smoke:ubuntu-latest".to_string()));
    assert!(req.contains(&"smoke:windows-latest".to_string()));
}
