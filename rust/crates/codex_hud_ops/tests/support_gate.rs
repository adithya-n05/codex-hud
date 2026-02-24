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
