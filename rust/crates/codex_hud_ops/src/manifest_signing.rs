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

pub fn test_public_key_hex_for_tests() -> String {
    let key = test_signing_key();
    hex::encode(key.verifying_key().to_bytes())
}

pub fn verify_manifest_signature_with_public_key(
    manifest: &str,
    signature_hex: &str,
    public_key_hex: &str,
) -> bool {
    let key_bytes = match hex::decode(public_key_hex) {
        Ok(v) => v,
        Err(_) => return false,
    };

    let key_array: [u8; 32] = match key_bytes.as_slice().try_into() {
        Ok(v) => v,
        Err(_) => return false,
    };
    let verify_key = match VerifyingKey::from_bytes(&key_array) {
        Ok(v) => v,
        Err(_) => return false,
    };

    let sig_bytes = match hex::decode(signature_hex) {
        Ok(v) => v,
        Err(_) => return false,
    };
    let sig = match Signature::from_slice(&sig_bytes) {
        Ok(v) => v,
        Err(_) => return false,
    };

    verify_key.verify(manifest.as_bytes(), &sig).is_ok()
}
