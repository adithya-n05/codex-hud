use crate::manifest_signing::verify_manifest_signature_with_public_key;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CompatibilityManifest {
    pub schema_version: u32,
    pub supported_keys: Vec<String>,
    pub signature_hex: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallMode {
    PatchAndRunManaged,
    RunStock { reason: String },
}

pub fn load_manifest(path: &Path) -> Result<CompatibilityManifest, String> {
    let raw = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    serde_json::from_str(&raw).map_err(|e| e.to_string())
}

pub fn verify_manifest_signature(
    payload: &str,
    signature_hex: &str,
    public_key_hex: &str,
) -> Result<(), String> {
    if verify_manifest_signature_with_public_key(payload, signature_hex, public_key_hex) {
        Ok(())
    } else {
        Err("invalid signature".to_string())
    }
}

pub fn canonical_manifest_payload(manifest: &CompatibilityManifest) -> Result<String, String> {
    if manifest.schema_version != 1 {
        return Err("unsupported manifest schema version".to_string());
    }

    let mut keys = manifest.supported_keys.clone();
    keys.sort();
    serde_json::to_string(&serde_json::json!({
        "schema_version": manifest.schema_version,
        "supported_keys": keys,
    }))
    .map_err(|e| e.to_string())
}

pub fn resolve_install_mode(
    manifest_path: &Path,
    key: &str,
    public_key_hex: &str,
) -> Result<InstallMode, String> {
    let manifest = load_manifest(manifest_path)?;
    let payload = canonical_manifest_payload(&manifest)?;
    verify_manifest_signature(&payload, &manifest.signature_hex, public_key_hex)?;

    if manifest.supported_keys.iter().any(|k| k == key) {
        Ok(InstallMode::PatchAndRunManaged)
    } else {
        Ok(InstallMode::RunStock {
            reason: "unsupported compatibility key".to_string(),
        })
    }
}
