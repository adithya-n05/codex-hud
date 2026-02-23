use codex_hud_ops::release_gate::release_gate_requirements;

#[test]
fn release_gate_requires_snapshot_tests() {
    let req = release_gate_requirements();
    assert!(req.contains(&"snapshot_tests".to_string()));
}
