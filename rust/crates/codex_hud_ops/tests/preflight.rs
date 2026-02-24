use codex_hud_ops::{preflight, PreflightInput};
use codex_hud_ops::preflight_guarded_install_root;
use tempfile::tempdir;

#[test]
fn preflight_fails_when_codex_missing() {
    let input = PreflightInput {
        codex_path: None,
        ..PreflightInput::default()
    };

    let err = preflight(&input).unwrap_err();
    assert!(err.contains("Codex binary not found"));
}

#[test]
fn preflight_fails_for_unsupported_version_sha() {
    let input = PreflightInput {
        codex_path: Some("/usr/local/bin/codex".to_string()),
        codex_version: Some("0.95.0".to_string()),
        codex_sha256: Some("zzz999".to_string()),
        supported_keys: vec!["0.94.0+abc123".to_string()],
    };

    let err = preflight(&input).unwrap_err();
    assert!(err.contains("Unsupported Codex version+sha"));
    assert!(err.contains("stock Codex"));
}

#[test]
fn failed_preflight_creates_no_managed_side_effects() {
    let tmp = tempdir().unwrap();
    let input = PreflightInput {
        codex_path: None,
        codex_version: Some("0.95.0".to_string()),
        codex_sha256: Some("zzz999".to_string()),
        supported_keys: vec!["0.94.0+abc123".to_string()],
    };

    let res = preflight_guarded_install_root(tmp.path(), &input);
    assert!(res.is_err());
    assert!(!tmp.path().join(".codex-hud").exists());
}

#[test]
fn preflight_rejects_missing_version() {
    let input = PreflightInput {
        codex_path: Some("/usr/local/bin/codex".to_string()),
        codex_version: None,
        codex_sha256: Some("zzz999".to_string()),
        supported_keys: vec!["0.95.0+zzz999".to_string()],
    };

    let err = preflight(&input).unwrap_err();
    assert!(err.contains("Codex version unavailable"));
}

#[test]
fn preflight_rejects_missing_sha256() {
    let input = PreflightInput {
        codex_path: Some("/usr/local/bin/codex".to_string()),
        codex_version: Some("0.95.0".to_string()),
        codex_sha256: None,
        supported_keys: vec!["0.95.0+zzz999".to_string()],
    };

    let err = preflight(&input).unwrap_err();
    assert!(err.contains("Codex sha256 unavailable"));
}

#[test]
fn preflight_guarded_install_root_creates_codex_hud_dir_when_supported() {
    let tmp = tempdir().unwrap();
    let input = PreflightInput {
        codex_path: Some("/usr/local/bin/codex".to_string()),
        codex_version: Some("0.95.0".to_string()),
        codex_sha256: Some("zzz999".to_string()),
        supported_keys: vec!["0.95.0+zzz999".to_string()],
    };

    preflight_guarded_install_root(tmp.path(), &input).unwrap();
    assert!(tmp.path().join(".codex-hud").exists());
}

#[test]
fn preflight_guarded_install_root_errors_when_codex_hud_path_is_blocked() {
    let tmp = tempdir().unwrap();
    std::fs::write(tmp.path().join(".codex-hud"), "blocked").unwrap();
    let input = PreflightInput {
        codex_path: Some("/usr/local/bin/codex".to_string()),
        codex_version: Some("0.95.0".to_string()),
        codex_sha256: Some("zzz999".to_string()),
        supported_keys: vec!["0.95.0+zzz999".to_string()],
    };

    let err = preflight_guarded_install_root(tmp.path(), &input).unwrap_err();
    assert!(!err.is_empty());
}
