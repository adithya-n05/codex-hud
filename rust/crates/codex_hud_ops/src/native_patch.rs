use codex_hud_statusline::patch_contract;

pub fn native_patch_targets() -> Vec<String> {
    patch_contract()
        .source_files
        .into_iter()
        .map(|path| path.to_string())
        .collect()
}

pub fn apply_marker_replace(
    original: &str,
    marker: &str,
    replacement: &str,
) -> Result<String, String> {
    if original.contains("codex-hud-managed") {
        return Ok(original.to_string());
    }
    if !original.contains(marker) {
        return Err("patch marker not found".to_string());
    }
    Ok(original.replacen(marker, replacement, 1))
}
