pub fn release_gate_requirements() -> Vec<String> {
    vec![
        "unit_tests".to_string(),
        "integration_tests".to_string(),
        "snapshot_tests".to_string(),
    ]
}
