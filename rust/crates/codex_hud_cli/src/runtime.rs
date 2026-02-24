use crate::dispatch::CommandHandlers;
use codex_hud_ops::codex_probe::{detect_codex_path, probe_compatibility_key};
use codex_hud_ops::integration_flow::{integration_install, integration_status, integration_status_details};
use codex_hud_ops::native_install::{
    install_native_patch_auto_for_stock_path, InstallOutcome,
    run_stock_codex_passthrough_interactive, uninstall_native_patch_auto,
};
use codex_hud_ops::unsupported_notice::{
    build_unsupported_notice_message, should_show_unsupported_notice,
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

fn record_last_run_policy(home: &Path, mode: &str, reason: Option<&str>) {
    let path = home.join(".codex-hud").join("last_run_policy.txt");
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let mut lines = vec![format!("mode={mode}")];
    if let Some(value) = reason {
        lines.push(format!("reason={value}"));
    }
    lines.push(String::new());
    let _ = std::fs::write(path, lines.join("\n"));
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
        match install_native_patch_auto_for_stock_path(&home, &path_env, &stock_codex)? {
            InstallOutcome::Patched => {
                record_last_run_policy(&home, "patched", None);
                Ok("install: patched".to_string())
            }
            InstallOutcome::RanStock { reason } => {
                record_last_run_policy(&home, "stock", Some(&reason));
                Err(format!("install blocked: {reason}"))
            }
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
        let home = user_home()?;
        let path_env = std::env::var("PATH").unwrap_or_default();
        let install_outcome = install_native_patch_auto_for_stock_path(
            &home,
            &path_env,
            Path::new(stock_codex_path),
        );
        match &install_outcome {
            Ok(InstallOutcome::Patched) => record_last_run_policy(&home, "patched", None),
            Ok(InstallOutcome::RanStock { reason }) => {
                record_last_run_policy(&home, "stock", Some(reason));
            }
            Err(err) => record_last_run_policy(&home, "error", Some(err)),
        }

        if let Ok(InstallOutcome::RanStock { .. }) = &install_outcome {
            if let Ok(key) = probe_compatibility_key(Some(Path::new(stock_codex_path)), &path_env)
            {
                let state = home.join(".codex-hud/unsupported-notice-seen.txt");
                if should_show_unsupported_notice(&key, &state).unwrap_or(false) {
                    eprintln!("{}", build_unsupported_notice_message(&key));
                }
            }
        }
        let status = run_stock_codex_passthrough_interactive(
            Path::new(stock_codex_path),
            passthrough_args,
        )?;
        if status != 0 {
            let reason = format!("stock codex exited with {status}");
            record_last_run_policy(&home, "error", Some(&reason));
            return Err(format!("stock codex exited with {status}"));
        }
        Ok(String::new())
    }
}
