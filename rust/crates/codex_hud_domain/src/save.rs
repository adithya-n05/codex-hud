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
