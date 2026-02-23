use codex_hud_ops::manifest_signing::{sign_manifest_for_tests, verify_manifest_signature};

#[test]
fn signed_manifest_verifies_with_public_key() {
    let manifest = "version=0.94.0\nsha256=abc123";
    let sig = sign_manifest_for_tests(manifest);
    assert!(verify_manifest_signature(manifest, &sig));
}
