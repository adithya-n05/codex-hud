use codex_hud_ops::manifest_signing::{sign_manifest_for_tests, verify_manifest_signature};

#[test]
fn signed_manifest_verifies_with_public_key() {
    let manifest = "version=0.94.0\nsha256=abc123";
    let sig = sign_manifest_for_tests(manifest);
    assert!(verify_manifest_signature(manifest, &sig));
}

#[test]
fn tampered_manifest_fails_verification() {
    let manifest = "version=0.94.0\nsha256=abc123";
    let sig = codex_hud_ops::manifest_signing::sign_manifest_for_tests(manifest);
    let pubkey_hex = codex_hud_ops::manifest_signing::test_public_key_hex_for_tests();
    assert!(!codex_hud_ops::manifest_signing::verify_manifest_signature_with_public_key(
        "version=0.94.0\nsha256=zzz999",
        &sig,
        &pubkey_hex
    ));
}
