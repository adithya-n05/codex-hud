use crate::codex_probe::{detect_codex_path, probe_compatibility_key};
use crate::native_patch::{apply_marker_replace, native_patch_targets};
use crate::support_gate::{resolve_install_mode, InstallMode};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

const NPM_LAUNCHER_REL_PATH: &str = "bin/codex.js";
const NPM_PATCH_MARKER: &str = "const env = { ...process.env, PATH: updatedPath };";
const NPM_PATCH_SNIPPET: &str = "const env = { ...process.env, PATH: updatedPath };\n/* codex-hud-managed:start */\nenv.CODEX_HUD_NATIVE_PATCH = \"1\";\n/* codex-hud-managed:end */";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallOutcome {
    Patched,
    RanStock { reason: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PatchState {
    patched_rel_paths: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PassthroughOutput {
    pub status_code: i32,
    pub stdout: String,
    pub stderr: String,
}

fn map_target_rel_to_real(codex_root: &Path, rel: &str) -> PathBuf {
    let trimmed = rel.strip_prefix("codex-rs/").unwrap_or(rel);
    codex_root.join(trimmed)
}

fn is_npm_codex_root(codex_root: &Path) -> bool {
    let package_json = codex_root.join("package.json");
    let raw = match std::fs::read_to_string(package_json) {
        Ok(v) => v,
        Err(_) => return false,
    };
    let value: serde_json::Value = match serde_json::from_str(&raw) {
        Ok(v) => v,
        Err(_) => return false,
    };
    value
        .get("name")
        .and_then(|v| v.as_str())
        .map(|v| v == "@openai/codex")
        .unwrap_or(false)
}

fn strip_managed_patch_block(value: &str) -> String {
    let start = "/* codex-hud-managed:start */";
    let end = "/* codex-hud-managed:end */";
    if let (Some(s), Some(e)) = (value.find(start), value.find(end)) {
        let end_index = e + end.len();
        let mut out = String::new();
        out.push_str(&value[..s]);
        out.push_str(value[end_index..].trim_start_matches('\n'));
        return out;
    }
    value.to_string()
}

pub fn install_native_patch(
    codex_root: &Path,
    key: &str,
    manifest_path: &Path,
    public_key_hex: &str,
) -> Result<InstallOutcome, String> {
    match resolve_install_mode(manifest_path, key, public_key_hex)? {
        InstallMode::RunStock { reason } => Ok(InstallOutcome::RanStock { reason }),
        InstallMode::PatchAndRunManaged => {
            if is_npm_codex_root(codex_root) {
                let launcher = codex_root.join(NPM_LAUNCHER_REL_PATH);
                if !launcher.exists() {
                    return Ok(InstallOutcome::RanStock {
                        reason: "native patch substrate unavailable for installed codex layout"
                            .to_string(),
                    });
                }

                let original = std::fs::read_to_string(&launcher).map_err(|e| e.to_string())?;
                let patched = apply_marker_replace(&original, NPM_PATCH_MARKER, NPM_PATCH_SNIPPET)?;
                std::fs::write(&launcher, patched).map_err(|e| e.to_string())?;

                let state = PatchState {
                    patched_rel_paths: vec![NPM_LAUNCHER_REL_PATH.to_string()],
                };
                let state_file = codex_root.join(".codex-hud/patch-state.json");
                if let Some(parent) = state_file.parent() {
                    std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
                }
                let json = serde_json::to_string_pretty(&state).map_err(|e| e.to_string())?;
                std::fs::write(state_file, json).map_err(|e| e.to_string())?;
                return Ok(InstallOutcome::Patched);
            }

            let targets = native_patch_targets();
            let mut plan: Vec<(String, PathBuf, String, String, PathBuf)> = Vec::new();

            for rel in targets {
                let real = map_target_rel_to_real(codex_root, &rel);
                if !real.exists() {
                    return Ok(InstallOutcome::RanStock {
                        reason: "native patch substrate unavailable for installed codex layout"
                            .to_string(),
                    });
                }

                let original = std::fs::read_to_string(&real).map_err(|e| e.to_string())?;
                let patched = apply_marker_replace(
                    &original,
                    "SlashCommand::Statusline",
                    "SlashCommand::Statusline /* codex-hud-managed */",
                )?;

                let backup = codex_root
                    .join(".codex-hud/backups")
                    .join(&rel)
                    .with_extension("bak");
                plan.push((rel, real, original, patched, backup));
            }

            for (_, _, original, _, backup) in &plan {
                if let Some(parent) = backup.parent() {
                    std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
                }
                std::fs::write(backup, original.as_bytes()).map_err(|e| e.to_string())?;
            }

            let mut written: Vec<(PathBuf, String)> = Vec::new();
            for (_, real, original, patched, _) in &plan {
                if let Err(e) = std::fs::write(real, patched.as_bytes()) {
                    for (already_written, rollback_text) in written {
                        let _ = std::fs::write(already_written, rollback_text.as_bytes());
                    }
                    return Err(e.to_string());
                }
                written.push((real.clone(), original.clone()));
            }

            let patched_rel_paths = plan
                .into_iter()
                .map(|(rel, _, _, _, _)| rel)
                .collect::<Vec<_>>();

            let state = PatchState { patched_rel_paths };
            let state_file = codex_root.join(".codex-hud/patch-state.json");
            if let Some(parent) = state_file.parent() {
                std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            let json = serde_json::to_string_pretty(&state).map_err(|e| e.to_string())?;
            std::fs::write(state_file, json).map_err(|e| e.to_string())?;
            Ok(InstallOutcome::Patched)
        }
    }
}

pub fn uninstall_native_patch(codex_root: &Path) -> Result<(), String> {
    let state_path = codex_root.join(".codex-hud/patch-state.json");
    if !state_path.exists() {
        return Ok(());
    }

    let raw = std::fs::read_to_string(&state_path).map_err(|e| e.to_string())?;
    let state: PatchState = serde_json::from_str(&raw).map_err(|e| e.to_string())?;

    for rel in state.patched_rel_paths {
        let target = map_target_rel_to_real(codex_root, &rel);
        let backup = codex_root
            .join(".codex-hud/backups")
            .join(&rel)
            .with_extension("bak");
        if backup.exists() {
            let original = std::fs::read(&backup).map_err(|e| e.to_string())?;
            std::fs::write(&target, original).map_err(|e| e.to_string())?;
        } else {
            let patched = std::fs::read_to_string(&target).map_err(|e| e.to_string())?;
            let restored = strip_managed_patch_block(&patched);
            std::fs::write(&target, restored).map_err(|e| e.to_string())?;
        }
    }

    std::fs::remove_file(state_path).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn run_stock_codex_passthrough(
    stock_codex: &Path,
    args: &[String],
) -> Result<PassthroughOutput, String> {
    let out = std::process::Command::new(stock_codex)
        .args(args)
        .output()
        .map_err(|e| e.to_string())?;

    Ok(PassthroughOutput {
        status_code: out.status.code().unwrap_or(1),
        stdout: String::from_utf8_lossy(&out.stdout).to_string(),
        stderr: String::from_utf8_lossy(&out.stderr).to_string(),
    })
}

fn discover_source_root_from_codex_binary(codex_path: &Path) -> Option<PathBuf> {
    for ancestor in codex_path.ancestors() {
        if ancestor.join("tui/src/slash_command.rs").exists() {
            return Some(ancestor.to_path_buf());
        }
        let candidate = ancestor.join("codex-rs");
        if candidate.join("tui/src/slash_command.rs").exists() {
            return Some(candidate);
        }
    }
    None
}

pub fn install_native_patch_auto(home: &Path, path_env: &str) -> Result<InstallOutcome, String> {
    let codex = detect_codex_path(None, path_env)?;
    let key = probe_compatibility_key(Some(&codex), path_env)?;

    let codex_root = match discover_source_root_from_codex_binary(&codex) {
        Some(root) => root,
        None => {
            return Ok(InstallOutcome::RanStock {
                reason: "native patch substrate unavailable for installed codex layout".to_string(),
            })
        }
    };

    let compat_manifest = home.join(".codex-hud/compat/compat.json");
    let pubkey_path = home.join(".codex-hud/compat/public_key.hex");
    if !compat_manifest.exists() || !pubkey_path.exists() {
        return Ok(InstallOutcome::RanStock {
            reason: "compatibility bundle unavailable".to_string(),
        });
    }
    let public_key_hex = std::fs::read_to_string(pubkey_path).map_err(|e| e.to_string())?;

    let out = match install_native_patch(&codex_root, &key, &compat_manifest, public_key_hex.trim())
    {
        Ok(v) => v,
        Err(err) => {
            return Ok(InstallOutcome::RanStock {
                reason: format!("compatibility gate rejected patch: {err}"),
            })
        }
    };

    if out == InstallOutcome::Patched {
        let pointer = home.join(".codex-hud/last_codex_root.txt");
        if let Some(parent) = pointer.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        std::fs::write(&pointer, codex_root.to_string_lossy().to_string())
            .map_err(|e| e.to_string())?;
        std::fs::write(
            home.join(".codex-hud/stock_codex_path.txt"),
            codex.to_string_lossy().to_string(),
        )
        .map_err(|e| e.to_string())?;
    }

    Ok(out)
}

pub fn uninstall_native_patch_auto(home: &Path) -> Result<(), String> {
    let pointer = home.join(".codex-hud/last_codex_root.txt");
    if !pointer.exists() {
        return Ok(());
    }

    let root = std::fs::read_to_string(&pointer).map_err(|e| e.to_string())?;
    uninstall_native_patch(Path::new(root.trim()))?;
    std::fs::remove_file(pointer).map_err(|e| e.to_string())?;

    let stock_pointer = home.join(".codex-hud/stock_codex_path.txt");
    if stock_pointer.exists() {
        std::fs::remove_file(stock_pointer).map_err(|e| e.to_string())?;
    }

    Ok(())
}
