use crate::dispatch::CommandHandlers;
use codex_hud_ops::codex_probe::detect_codex_path;
use codex_hud_ops::integration_flow::{integration_install, integration_status, integration_status_details};
use codex_hud_ops::native_install::{
    install_native_patch_auto, run_stock_codex_passthrough, uninstall_native_patch_auto,
    InstallOutcome,
};
use std::path::{Path, PathBuf};

pub struct RealHandlers;

fn user_home() -> Result<PathBuf, String> {
    if let Ok(value) = std::env::var("HOME") {
        return Ok(PathBuf::from(value));
    }
    if let Ok(value) = std::env::var("USERPROFILE") {
        return Ok(PathBuf::from(value));
    }
    Err("missing env HOME/USERPROFILE".to_string())
}

impl CommandHandlers for RealHandlers {
    fn run_status(&self) -> Result<String, String> {
        integration_status(&user_home()?)
    }

    fn run_status_details(&self) -> Result<String, String> {
        integration_status_details(&user_home()?)
    }

    fn run_install(&self) -> Result<String, String> {
        let home = user_home()?;
        let path_env = std::env::var("PATH").unwrap_or_default();
        let stock_codex = detect_codex_path(None, &path_env)?;
        integration_install(&home, stock_codex.to_string_lossy().as_ref())?;
        match install_native_patch_auto(&home, &path_env)? {
            InstallOutcome::Patched => Ok("install: patched".to_string()),
            InstallOutcome::RanStock { reason } => Err(format!("install blocked: {reason}")),
        }
    }

    fn run_uninstall(&self) -> Result<String, String> {
        uninstall_native_patch_auto(&user_home()?)?;
        Ok("uninstall: restored".to_string())
    }

    fn run_shim(
        &self,
        stock_codex_path: &str,
        passthrough_args: &[String],
    ) -> Result<String, String> {
        let out = run_stock_codex_passthrough(Path::new(stock_codex_path), passthrough_args)?;
        if out.status_code != 0 {
            return Err(format!("stock codex exited with {}", out.status_code));
        }
        Ok(out.stdout)
    }
}
