use crate::compatibility::is_supported_exact;
use std::path::Path;

#[derive(Debug, Clone, Default)]
pub struct PreflightInput {
    pub codex_path: Option<String>,
    pub codex_version: Option<String>,
    pub codex_sha256: Option<String>,
    pub supported_keys: Vec<String>,
}

pub fn preflight(input: &PreflightInput) -> Result<(), String> {
    if input.codex_path.is_none() {
        return Err("Codex binary not found".to_string());
    }

    let version = input
        .codex_version
        .as_deref()
        .ok_or_else(|| "Codex version unavailable".to_string())?;
    let sha = input
        .codex_sha256
        .as_deref()
        .ok_or_else(|| "Codex sha256 unavailable".to_string())?;
    let key = format!("{version}+{sha}");
    if !is_supported_exact(&key, &input.supported_keys) {
        return Err("Unsupported Codex version+sha; running stock Codex".to_string());
    }
    Ok(())
}

pub fn preflight_guarded_install_root(home: &Path, input: &PreflightInput) -> Result<(), String> {
    preflight(input)?;
    std::fs::create_dir_all(home.join(".codex-hud")).map_err(|e| e.to_string())?;
    Ok(())
}
