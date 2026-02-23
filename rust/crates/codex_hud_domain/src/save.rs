use std::path::Path;

pub fn atomic_write_with_backup(path: &Path, content: &str, simulate_failure: bool) -> Result<(), String> {
    let backup = path.with_extension("toml.bak");
    let temp = path.with_extension("toml.tmp");

    if path.exists() {
        std::fs::copy(path, &backup).map_err(|e| e.to_string())?;
    }

    std::fs::write(&temp, content).map_err(|e| e.to_string())?;

    if simulate_failure {
        let _ = std::fs::remove_file(&temp);
        if backup.exists() {
            std::fs::copy(&backup, path).map_err(|e| e.to_string())?;
        }
        return Err("simulated write failure".to_string());
    }

    std::fs::rename(&temp, path).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn restore_from_backup(path: &Path) -> Result<(), String> {
    let backup = path.with_extension("toml.bak");
    std::fs::copy(&backup, path).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn save_redaction_toggle(path: &Path, redact_auth_identity: bool) -> Result<(), String> {
    let value = if redact_auth_identity { "true" } else { "false" };
    let key_line = format!("redact_auth_identity = {value}");
    let mut content = std::fs::read_to_string(path).unwrap_or_default();

    if content
        .lines()
        .any(|line| line.trim_start().starts_with("redact_auth_identity ="))
    {
        let mut updated = Vec::new();
        for line in content.lines() {
            if line.trim_start().starts_with("redact_auth_identity =") {
                updated.push(key_line.clone());
            } else {
                updated.push(line.to_string());
            }
        }
        content = updated.join("\n");
        if !content.ends_with('\n') {
            content.push('\n');
        }
    } else {
        if !content.is_empty() && !content.ends_with('\n') {
            content.push('\n');
        }
        content.push_str(&key_line);
        content.push('\n');
    }

    std::fs::write(path, content).map_err(|e| e.to_string())
}

pub fn load_redaction_toggle(path: &Path) -> Result<bool, String> {
    let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let value = content
        .lines()
        .find_map(|line| line.strip_prefix("redact_auth_identity = "))
        .ok_or_else(|| "missing redact_auth_identity key".to_string())?;

    match value.trim() {
        "true" => Ok(true),
        "false" => Ok(false),
        other => Err(format!("invalid redact_auth_identity value: {other}")),
    }
}
