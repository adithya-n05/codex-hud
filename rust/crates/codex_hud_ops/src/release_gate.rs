pub fn release_gate_requirements() -> Vec<String> {
    vec![
        "unit_tests".to_string(),
        "integration_tests".to_string(),
        "snapshot_tests".to_string(),
        "smoke:macos-latest".to_string(),
        "smoke:ubuntu-latest".to_string(),
        "smoke:windows-latest".to_string(),
    ]
}
