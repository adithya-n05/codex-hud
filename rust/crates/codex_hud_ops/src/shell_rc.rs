use std::path::Path;

pub fn ensure_rc_block(rc_path: &Path, managed_bin_path: &str) -> Result<(), String> {
    let begin = "# BEGIN CODEX HUD MANAGED BLOCK";
    let end = "# END CODEX HUD MANAGED BLOCK";

    let existing = std::fs::read_to_string(rc_path).map_err(|e| e.to_string())?;
    if existing.contains(begin) && existing.contains(end) {
        return Ok(());
    }

    let block = format!("\n{begin}\nexport PATH=\"{managed_bin_path}:$PATH\"\n{end}\n");

    let updated = format!("{existing}{block}");
    std::fs::write(rc_path, updated).map_err(|e| e.to_string())?;
    Ok(())
}
