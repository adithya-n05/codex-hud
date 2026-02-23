use ed25519_dalek::{Signature, SigningKey, Verifier, VerifyingKey, Signer};

fn test_signing_key() -> SigningKey {
    SigningKey::from_bytes(&[7u8; 32])
}

pub fn sign_manifest_for_tests(manifest: &str) -> String {
    let key = test_signing_key();
    let sig: Signature = key.sign(manifest.as_bytes());
    hex::encode(sig.to_bytes())
}

pub fn verify_manifest_signature(manifest: &str, signature_hex: &str) -> bool {
    let key = test_signing_key();
    let verify_key: VerifyingKey = key.verifying_key();

    let bytes = match hex::decode(signature_hex) {
        Ok(v) => v,
        Err(_) => return false,
    };

    let sig = match Signature::from_slice(&bytes) {
        Ok(v) => v,
        Err(_) => return false,
    };

    verify_key.verify(manifest.as_bytes(), &sig).is_ok()
}
