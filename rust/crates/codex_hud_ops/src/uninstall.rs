use std::path::Path;

pub fn run_uninstall(home: &Path) -> Result<(), String> {
    let root = home.join(".codex-hud");
    if root.exists() {
        std::fs::remove_dir_all(&root).map_err(|e| e.to_string())?;
    }
    Ok(())
}
