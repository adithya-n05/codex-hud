use codex_hud_ops::{preflight, PreflightInput};

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
