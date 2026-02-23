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

pub fn remove_rc_block(rc_path: &Path) -> Result<(), String> {
    let begin = "# BEGIN CODEX HUD MANAGED BLOCK";
    let end = "# END CODEX HUD MANAGED BLOCK";

    let existing = match std::fs::read_to_string(rc_path) {
        Ok(v) => v,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(e) => return Err(e.to_string()),
    };
    let mut out = Vec::new();
    let mut in_block = false;

    for line in existing.lines() {
        if line.trim() == begin {
            in_block = true;
            continue;
        }
        if line.trim() == end {
            in_block = false;
            continue;
        }
        if !in_block {
            out.push(line);
        }
    }

    let rebuilt = format!("{}\n", out.join("\n"));
    std::fs::write(rc_path, rebuilt).map_err(|e| e.to_string())?;
    Ok(())
}
