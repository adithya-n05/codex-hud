use std::path::{Path, PathBuf};

pub fn write_codex_shim(home: &Path, stock_codex_path: &str) -> Result<PathBuf, String> {
    let dir = home.join(".codex-hud").join("bin");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let path = dir.join("codex");

    let script = format!(
        "#!/usr/bin/env sh\nexec codex-hud run --stock-codex \"{}\" \"$@\"\n",
        stock_codex_path
    );

    std::fs::write(&path, script).map_err(|e| e.to_string())?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&path).map_err(|e| e.to_string())?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&path, perms).map_err(|e| e.to_string())?;
    }
    Ok(path)
}
