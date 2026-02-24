use codex_hud_ops::manifest_signing::{
    sign_manifest_for_tests, test_public_key_hex_for_tests, verify_manifest_signature,
    verify_manifest_signature_with_public_key,
};

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

#[test]
fn invalid_signature_hex_is_rejected() {
    assert!(!verify_manifest_signature("manifest", "not-hex"));
}

#[test]
fn invalid_signature_length_is_rejected() {
    assert!(!verify_manifest_signature("manifest", "00"));
}

#[test]
fn verify_with_public_key_rejects_invalid_public_key_hex() {
    let sig = sign_manifest_for_tests("manifest");
    assert!(!verify_manifest_signature_with_public_key(
        "manifest",
        &sig,
        "zz-not-hex"
    ));
}

#[test]
fn verify_with_public_key_rejects_invalid_public_key_length() {
    let sig = sign_manifest_for_tests("manifest");
    assert!(!verify_manifest_signature_with_public_key("manifest", &sig, "00"));
}

#[test]
fn verify_with_public_key_rejects_invalid_signature_hex_and_length() {
    let pubkey = test_public_key_hex_for_tests();
    assert!(!verify_manifest_signature_with_public_key(
        "manifest",
        "invalid-hex",
        &pubkey
    ));
    assert!(!verify_manifest_signature_with_public_key("manifest", "00", &pubkey));
}

#[test]
fn verify_with_public_key_rejects_invalid_32_byte_public_key() {
    let sig = sign_manifest_for_tests("manifest");
    let invalid_but_sized_key = "00".repeat(32);
    assert!(!verify_manifest_signature_with_public_key(
        "manifest",
        &sig,
        &invalid_but_sized_key
    ));
}
