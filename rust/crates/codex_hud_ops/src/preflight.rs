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
    Ok(())
}
