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
