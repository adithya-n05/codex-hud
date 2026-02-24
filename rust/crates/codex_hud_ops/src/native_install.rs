use crate::compat_refresh::refresh_compat_bundle;
use crate::codex_probe::{
    detect_codex_path, detect_npm_package_root_from_codex_binary, probe_compatibility_key,
    resolve_npm_vendor_binary_path_from_package_root,
};
use crate::native_patch::{apply_marker_replace, native_patch_targets};
use crate::support_gate::{resolve_install_mode, InstallMode};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

const NPM_LAUNCHER_REL_PATH: &str = "bin/codex.js";
const NPM_PATCH_MARKER: &str = "const env = { ...process.env, PATH: updatedPath };";
const NPM_PATCH_SNIPPET: &str = "const env = { ...process.env, PATH: updatedPath };\n/* codex-hud-managed:start */\nenv.CODEX_HUD_NATIVE_PATCH = \"1\";\n/* codex-hud-managed:end */";
const NPM_PATCH_STATE_REL_PATH: &str = ".codex-hud/cache/npm_patch_state.json";

fn patched_binary_cache_path(home: &Path, key: &str) -> PathBuf {
    let binary_name = if cfg!(windows) { "codex.exe" } else { "codex" };
    home.join(".codex-hud/cache/patched")
        .join(key)
        .join(binary_name)
}

pub fn patched_binary_cache_path_for_test(home: &Path, key: &str) -> PathBuf {
    patched_binary_cache_path(home, key)
}

pub fn build_patched_native_binary_for_test(
    codex_upstream_root: &Path,
    _compatibility_key: &str,
) -> Result<PathBuf, String> {
    if !codex_upstream_root.exists() {
        return Err("codex-upstream source tree not found".to_string());
    }
    Ok(codex_upstream_root.join("codex"))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallOutcome {
    Patched,
    RanStock { reason: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PatchState {
    patched_rel_paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NpmPatchState {
    compatibility_key: String,
    vendor_sha256: String,
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

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

fn npm_patch_state_path(home: &Path) -> PathBuf {
    home.join(NPM_PATCH_STATE_REL_PATH)
}

fn read_npm_patch_state(home: &Path) -> Option<NpmPatchState> {
    let raw = std::fs::read_to_string(npm_patch_state_path(home)).ok()?;
    serde_json::from_str(&raw).ok()
}

fn write_npm_patch_state(home: &Path, key: &str, vendor_sha256: &str) -> Result<(), String> {
    let path = npm_patch_state_path(home);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let state = NpmPatchState {
        compatibility_key: key.to_string(),
        vendor_sha256: vendor_sha256.to_string(),
    };
    let json = serde_json::to_string_pretty(&state).map_err(|e| e.to_string())?;
    std::fs::write(path, json).map_err(|e| e.to_string())
}

fn npm_launcher_has_managed_patch(launcher: &Path) -> bool {
    let Ok(raw) = std::fs::read_to_string(launcher) else {
        return false;
    };
    raw.contains("/* codex-hud-managed:start */")
}

fn npm_patch_state_matches(
    home: &Path,
    key: &str,
    codex_root: &Path,
) -> Result<bool, String> {
    let launcher = codex_root.join(NPM_LAUNCHER_REL_PATH);
    if !npm_launcher_has_managed_patch(&launcher) {
        return Ok(false);
    }
    let Some(state) = read_npm_patch_state(home) else {
        return Ok(false);
    };
    if state.compatibility_key != key {
        return Ok(false);
    }
    let vendor_binary = resolve_npm_vendor_binary_path_from_package_root(codex_root)?;
    if !vendor_binary.exists() {
        return Ok(false);
    }
    let vendor = std::fs::read(vendor_binary).map_err(|e| e.to_string())?;
    Ok(state.vendor_sha256 == sha256_hex(&vendor))
}

fn is_macho_binary(bytes: &[u8]) -> bool {
    if bytes.len() < 4 {
        return false;
    }
    let magic = [bytes[0], bytes[1], bytes[2], bytes[3]];
    matches!(
        magic,
        [0xfe, 0xed, 0xfa, 0xce]
            | [0xce, 0xfa, 0xed, 0xfe]
            | [0xfe, 0xed, 0xfa, 0xcf]
            | [0xcf, 0xfa, 0xed, 0xfe]
            | [0xca, 0xfe, 0xba, 0xbe]
            | [0xbe, 0xba, 0xfe, 0xca]
            | [0xca, 0xfe, 0xba, 0xbf]
            | [0xbf, 0xba, 0xfe, 0xca]
    )
}

#[cfg(target_os = "macos")]
fn ad_hoc_codesign_if_needed(binary_path: &Path, bytes: &[u8]) -> Result<(), String> {
    if !is_macho_binary(bytes) {
        return Ok(());
    }
    let status = std::process::Command::new("codesign")
        .args(["--force", "--sign", "-"])
        .arg(binary_path)
        .status()
        .map_err(|e| format!("failed to invoke codesign: {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "codesign failed with status {}",
            status
                .code()
                .map(|c| c.to_string())
                .unwrap_or_else(|| "signal".to_string())
        ))
    }
}

#[cfg(not(target_os = "macos"))]
fn ad_hoc_codesign_if_needed(_binary_path: &Path, _bytes: &[u8]) -> Result<(), String> {
    Ok(())
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

pub fn install_native_patch_using_cached_binary(
    home: &Path,
    stock_codex_path: &Path,
    key: &str,
) -> Result<InstallOutcome, String> {
    let codex_root = detect_npm_package_root_from_codex_binary(stock_codex_path)
        .ok_or_else(|| "native patch substrate unavailable for installed codex layout".to_string())?;

    let vendor_binary = resolve_npm_vendor_binary_path_from_package_root(&codex_root)?;
    if !vendor_binary.exists() {
        return Err("npm vendor codex binary not found".to_string());
    }

    let cached = patched_binary_cache_path(home, key);
    if !cached.exists() {
        return Err(format!(
            "patched native binary cache missing for compatibility key: {key}"
        ));
    }

    let cached_bytes = std::fs::read(&cached).map_err(|e| e.to_string())?;
    let existing_bytes = std::fs::read(&vendor_binary).map_err(|e| e.to_string())?;
    if existing_bytes != cached_bytes {
        std::fs::write(&vendor_binary, &cached_bytes).map_err(|e| e.to_string())?;
        ad_hoc_codesign_if_needed(&vendor_binary, &cached_bytes)?;
    }

    let vendor_final = std::fs::read(&vendor_binary).map_err(|e| e.to_string())?;
    write_npm_patch_state(home, key, &sha256_hex(&vendor_final))?;

    Ok(InstallOutcome::Patched)
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
            if !target.exists() {
                continue;
            }
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

pub fn run_stock_codex_passthrough_interactive(
    stock_codex: &Path,
    args: &[String],
) -> Result<i32, String> {
    let status = std::process::Command::new(stock_codex)
        .args(args)
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()
        .map_err(|e| e.to_string())?;
    Ok(status.code().unwrap_or(1))
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

fn persist_compat_metadata(home: &Path, key: &str, source: &str) -> Result<(), String> {
    let compat_dir = home.join(".codex-hud/compat");
    std::fs::create_dir_all(&compat_dir).map_err(|e| e.to_string())?;
    std::fs::write(compat_dir.join("last_compat_key.txt"), key).map_err(|e| e.to_string())?;
    std::fs::write(compat_dir.join("refresh_source.txt"), source).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn install_native_patch_auto(home: &Path, path_env: &str) -> Result<InstallOutcome, String> {
    install_native_patch_auto_with(home, path_env, None, None)
}

pub fn install_native_patch_auto_for_stock_path(
    home: &Path,
    path_env: &str,
    stock_codex_path: &Path,
) -> Result<InstallOutcome, String> {
    install_native_patch_auto_with(home, path_env, Some(stock_codex_path), None)
}

pub fn install_native_patch_auto_with(
    home: &Path,
    path_env: &str,
    explicit_codex_path: Option<&Path>,
    compat_base_url: Option<&str>,
) -> Result<InstallOutcome, String> {
    let codex = detect_codex_path(explicit_codex_path, path_env)?;
    let key = probe_compatibility_key(Some(&codex), path_env)?;
    let is_npm_layout = detect_npm_package_root_from_codex_binary(&codex).is_some();
    let compat_manifest = home.join(".codex-hud/compat/compat.json");
    let pubkey_path = home.join(".codex-hud/compat/public_key.hex");
    let refresh_source = match refresh_compat_bundle(home, compat_base_url) {
        Ok(()) => "github-release",
        Err(err) => {
            if !compat_manifest.exists() || !pubkey_path.exists() {
                return Ok(InstallOutcome::RanStock {
                    reason: format!("compatibility bundle refresh failed: {err}"),
                });
            }
            "local-cache-fallback"
        }
    };

    persist_compat_metadata(home, &key, refresh_source)?;

    let codex_root = if let Some(root) = detect_npm_package_root_from_codex_binary(&codex) {
        root
    } else {
        match discover_source_root_from_codex_binary(&codex) {
            Some(root) => root,
            None => {
                return Ok(InstallOutcome::RanStock {
                    reason: "native patch substrate unavailable for installed codex layout"
                        .to_string(),
                })
            }
        }
    };

    if !compat_manifest.exists() || !pubkey_path.exists() {
        return Ok(InstallOutcome::RanStock {
            reason: "compatibility bundle unavailable".to_string(),
        });
    }
    let public_key_hex = std::fs::read_to_string(pubkey_path).map_err(|e| e.to_string())?;
    let install_mode = match resolve_install_mode(&compat_manifest, &key, public_key_hex.trim()) {
        Ok(v) => v,
        Err(err) => {
            return Ok(InstallOutcome::RanStock {
                reason: format!("compatibility gate rejected patch: {err}"),
            })
        }
    };

    if let InstallMode::RunStock { reason } = install_mode {
        return Ok(InstallOutcome::RanStock { reason });
    }

    if is_npm_layout {
        let cached = patched_binary_cache_path(home, &key);
        if !cached.exists() {
            return Ok(InstallOutcome::RanStock {
                reason: "patched native binary cache missing for compatibility key".to_string(),
            });
        }
        if npm_patch_state_matches(home, &key, &codex_root)? {
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
            return Ok(InstallOutcome::Patched);
        }
    }

    let out = match install_native_patch(&codex_root, &key, &compat_manifest, public_key_hex.trim())
    {
        Ok(v) => v,
        Err(err) => {
            return Ok(InstallOutcome::RanStock {
                reason: format!("compatibility gate rejected patch: {err}"),
            })
        }
    };

    if out == InstallOutcome::Patched && is_npm_layout {
        install_native_patch_using_cached_binary(home, &codex, &key)?;
    }

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

#[cfg(test)]
mod tests {
    use super::is_macho_binary;

    #[test]
    fn macho_detection_matches_known_magic_values() {
        assert!(is_macho_binary(&[0xfe, 0xed, 0xfa, 0xcf]));
        assert!(is_macho_binary(&[0xcf, 0xfa, 0xed, 0xfe]));
        assert!(is_macho_binary(&[0xca, 0xfe, 0xba, 0xbe]));
        assert!(is_macho_binary(&[0xbe, 0xba, 0xfe, 0xca]));
        assert!(!is_macho_binary(&[0x00, 0x00, 0x00, 0x00]));
        assert!(!is_macho_binary(&[0x7f, 0x45, 0x4c, 0x46]));
    }
}
