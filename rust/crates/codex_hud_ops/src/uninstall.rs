use std::path::Path;

use crate::shell_rc::remove_rc_block;

pub fn run_uninstall(home: &Path) -> Result<(), String> {
    let root = home.join(".codex-hud");
    if root.exists() {
        std::fs::remove_dir_all(&root).map_err(|e| e.to_string())?;
    }
    Ok(())
}

pub fn run_uninstall_with_rc(home: &Path, rc_path: &Path) -> Result<(), String> {
    let rc_text = match std::fs::read_to_string(rc_path) {
        Ok(v) => v,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(e) => return Err(format!("rc read error: {e}")),
    };
    if rc_text.contains("BEGIN CODEX HUD MANAGED BLOCK")
        && rc_text.contains("END CODEX HUD MANAGED BLOCK")
    {
        remove_rc_block(rc_path)?;
    }
    run_uninstall(home)
}
