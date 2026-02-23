use std::fs;
use std::path::PathBuf;

fn locate_release_policy_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../release/release-gate-policy.toml")
}

#[test]
fn release_policy_file_contains_required_gate_entries() {
    let policy = locate_release_policy_path();
    assert!(
        policy.exists(),
        "missing release policy file: {}",
        policy.display()
    );
    let content = fs::read_to_string(policy).unwrap();
    assert!(content.contains("snapshot_tests = true"));
    assert!(content.contains("smoke_macos = true"));
    assert!(content.contains("smoke_ubuntu = true"));
    assert!(content.contains("smoke_windows = true"));
    assert!(content.contains("release_source = \"git_tag\""));
}
