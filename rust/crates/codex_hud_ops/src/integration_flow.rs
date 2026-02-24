use crate::shell_rc::{ensure_rc_block, remove_rc_block};
use crate::shim::write_codex_shim;
use crate::status::{render_status_details, render_status_summary, StatusSnapshot};
use crate::uninstall::run_uninstall;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

fn read_trimmed(path: &Path) -> Option<String> {
    std::fs::read_to_string(path)
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
}

fn read_policy_fields(home: &Path) -> (Option<String>, Option<String>) {
    let path = home.join(".codex-hud/last_run_policy.txt");
    let raw = match std::fs::read_to_string(path) {
        Ok(v) => v,
        Err(_) => return (None, None),
    };
    let mut mode = None;
    let mut reason = None;
    for line in raw.lines() {
        if let Some(value) = line.strip_prefix("mode=") {
            mode = Some(value.trim().to_string());
        } else if let Some(value) = line.strip_prefix("reason=") {
            reason = Some(value.trim().to_string());
        }
    }
    (mode, reason)
}

pub fn integration_install(home: &Path, stock_codex_path: &str) -> Result<(), String> {
    let managed_bin_dir = home.join(".codex-hud").join("bin");
    std::fs::create_dir_all(&managed_bin_dir).map_err(|e| e.to_string())?;

    let runtime = managed_bin_dir.join("codex-hud");
    if !runtime.exists() {
        let runtime_script = r#"#!/usr/bin/env sh
if [ "$1" = "run" ] && [ "$2" = "--stock-codex" ]; then
  shift 2
  stock="$1"
  shift
  if [ "$1" = "--" ]; then
    shift
  fi
  exec "$stock" "$@"
fi
printf "%s\n" "$*"
"#;
        std::fs::write(&runtime, runtime_script).map_err(|e| e.to_string())?;
        #[cfg(unix)]
        {
            let mut perms = std::fs::metadata(&runtime)
                .map_err(|e| e.to_string())?
                .permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&runtime, perms).map_err(|e| e.to_string())?;
        }
    }

    std::fs::write(
        home.join(".codex-hud").join("stock_codex_path.txt"),
        stock_codex_path,
    )
    .map_err(|e| e.to_string())?;

    write_codex_shim(home, stock_codex_path)?;
    let rc_path = home.join(".zshrc");
    let managed_bin = home
        .join(".codex-hud")
        .join("bin")
        .to_string_lossy()
        .to_string();
    if !rc_path.exists() {
        std::fs::write(&rc_path, "").map_err(|e| e.to_string())?;
    }
    ensure_rc_block(&rc_path, &managed_bin)?;
    Ok(())
}

pub fn integration_exec_shim(home: &Path, args: &[&str]) -> Result<String, String> {
    let shim = home.join(".codex-hud").join("bin").join("codex");
    let managed_bin = home.join(".codex-hud").join("bin");
    let path_env = std::env::var("PATH").unwrap_or_default();
    let path_value = format!("{}:{}", managed_bin.to_string_lossy(), path_env);
    let output = std::process::Command::new(shim)
        .env("PATH", path_value)
        .args(args)
        .output()
        .map_err(|e| e.to_string())?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn integration_status(home: &Path) -> Result<String, String> {
    let shim = home.join(".codex-hud").join("bin").join("codex").exists();
    let runtime = home
        .join(".codex-hud")
        .join("bin")
        .join("codex-hud")
        .exists();
    let rc_path = home.join(".zshrc");
    let rc_text = match std::fs::read_to_string(&rc_path) {
        Ok(v) => v,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(e) => return Err(format!("rc read error: {e}")),
    };
    let rc_block_present = rc_text.contains("BEGIN CODEX HUD MANAGED BLOCK");
    let installed = shim || runtime || rc_block_present;
    let stock_codex_path =
        std::fs::read_to_string(home.join(".codex-hud").join("stock_codex_path.txt"))
            .ok()
            .map(|s| s.trim().to_string());

    let (patch_mode, patch_reason) = read_policy_fields(home);
    let compat_key = read_trimmed(&home.join(".codex-hud/compat/last_compat_key.txt"));
    let compat_refresh_source = read_trimmed(&home.join(".codex-hud/compat/refresh_source.txt"));

    let snapshot = StatusSnapshot {
        installed,
        shim_present: shim,
        rc_block_present,
        compatible: true,
        codex_version: Some("unknown".to_string()),
        codex_sha256: None,
        managed_root: Some(home.join(".codex-hud").to_string_lossy().to_string()),
        stock_codex_path,
        patch_mode,
        patch_reason,
        compat_key,
        compat_refresh_source,
    };
    let mut out = render_status_summary(&snapshot);
    out.push_str(&format!(
        "\nruntime: {}",
        if runtime { "present" } else { "missing" }
    ));
    Ok(out)
}

pub fn integration_status_details(home: &Path) -> Result<String, String> {
    let shim = home.join(".codex-hud").join("bin").join("codex").exists();
    let rc_path = home.join(".zshrc");
    let rc_text = match std::fs::read_to_string(&rc_path) {
        Ok(v) => v,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(e) => return Err(format!("rc read error: {e}")),
    };
    let rc_block_present = rc_text.contains("BEGIN CODEX HUD MANAGED BLOCK");
    let installed = shim || rc_block_present;
    let stock_codex_path =
        std::fs::read_to_string(home.join(".codex-hud").join("stock_codex_path.txt"))
            .ok()
            .map(|s| s.trim().to_string());

    let (patch_mode, patch_reason) = read_policy_fields(home);
    let compat_key = read_trimmed(&home.join(".codex-hud/compat/last_compat_key.txt"));
    let compat_refresh_source = read_trimmed(&home.join(".codex-hud/compat/refresh_source.txt"));

    let snapshot = StatusSnapshot {
        installed,
        shim_present: shim,
        rc_block_present,
        compatible: true,
        codex_version: Some("unknown".to_string()),
        codex_sha256: None,
        managed_root: Some(home.join(".codex-hud").to_string_lossy().to_string()),
        stock_codex_path,
        patch_mode,
        patch_reason,
        compat_key,
        compat_refresh_source,
    };
    Ok(render_status_details(&snapshot))
}

pub fn integration_uninstall(home: &Path) -> Result<(), String> {
    let rc_path = home.join(".zshrc");
    if rc_path.exists() {
        remove_rc_block(&rc_path)?;
    }
    run_uninstall(home)
}
