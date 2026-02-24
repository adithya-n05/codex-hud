use codex_hud_ops::manifest_signing::{sign_manifest_for_tests, test_public_key_hex_for_tests};
use codex_hud_ops::support_gate::{
    load_manifest, resolve_install_mode, verify_manifest_signature, InstallMode,
};
use tempfile::tempdir;

#[test]
fn loads_manifest_keys_and_signature() {
    let tmp = tempdir().unwrap();
    let file = tmp.path().join("compat.json");
    std::fs::write(
        &file,
        r#"{"schema_version":1,"supported_keys":["0.104.0+abc"],"signature_hex":"ff"}"#,
    )
    .unwrap();

    let manifest = load_manifest(&file).unwrap();
    assert_eq!(manifest.schema_version, 1);
    assert_eq!(manifest.supported_keys, vec!["0.104.0+abc".to_string()]);
    assert_eq!(manifest.signature_hex, "ff");
}

#[test]
fn tampered_keys_fail_signature_validation() {
    let payload = "0.104.0+abc";
    let sig = sign_manifest_for_tests(payload);
    let pubkey = test_public_key_hex_for_tests();
    let err = verify_manifest_signature("0.104.0+zzz", &sig, &pubkey).unwrap_err();
    assert!(err.contains("invalid signature"));
}

#[test]
fn canonical_payload_verification_is_order_independent() {
    let tmp = tempdir().unwrap();
    let file = tmp.path().join("compat.json");
    let payload = r#"{"schema_version":1,"supported_keys":["0.104.0+abc","0.104.0+def"]}"#;
    let sig = sign_manifest_for_tests(payload);
    std::fs::write(
        &file,
        format!(
            r#"{{"schema_version":1,"supported_keys":["0.104.0+def","0.104.0+abc"],"signature_hex":"{}"}}"#,
            sig
        ),
    )
    .unwrap();

    let pubkey = test_public_key_hex_for_tests();
    let mode = resolve_install_mode(&file, "0.104.0+abc", &pubkey).unwrap();
    assert_eq!(mode, InstallMode::PatchAndRunManaged);
}

#[test]
fn resolve_install_mode_returns_stock_when_key_is_not_supported() {
    let tmp = tempdir().unwrap();
    let file = tmp.path().join("compat.json");
    let payload = r#"{"schema_version":1,"supported_keys":["0.104.0+abc"]}"#;
    let sig = sign_manifest_for_tests(payload);
    std::fs::write(
        &file,
        format!(
            r#"{{"schema_version":1,"supported_keys":["0.104.0+abc"],"signature_hex":"{}"}}"#,
            sig
        ),
    )
    .unwrap();

    let pubkey = test_public_key_hex_for_tests();
    let mode = resolve_install_mode(&file, "0.104.0+zzz", &pubkey).unwrap();
    assert_eq!(
        mode,
        InstallMode::RunStock {
            reason: "unsupported compatibility key".to_string(),
        }
    );
}

#[test]
fn resolve_install_mode_rejects_unsupported_manifest_schema_version() {
    let tmp = tempdir().unwrap();
    let file = tmp.path().join("compat.json");
    std::fs::write(
        &file,
        r#"{"schema_version":2,"supported_keys":[],"signature_hex":"00"}"#,
    )
    .unwrap();

    let pubkey = test_public_key_hex_for_tests();
    let err = resolve_install_mode(&file, "0.104.0+abc", &pubkey).unwrap_err();
    assert!(err.contains("unsupported manifest schema version"));
}

#[test]
fn load_manifest_surfaces_io_errors() {
    let tmp = tempdir().unwrap();
    let missing = tmp.path().join("missing-compat.json");
    let err = load_manifest(&missing).unwrap_err();
    assert!(!err.is_empty());
}
