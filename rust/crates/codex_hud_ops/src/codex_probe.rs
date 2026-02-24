use sha2::{Digest, Sha256};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;

fn codex_package_root(candidate: &Path) -> Option<PathBuf> {
    let package_json = candidate.join("package.json");
    let raw = std::fs::read_to_string(package_json).ok()?;
    let value: serde_json::Value = serde_json::from_str(&raw).ok()?;
    let name = value.get("name")?.as_str()?;
    if name == "@openai/codex" {
        Some(candidate.to_path_buf())
    } else {
        None
    }
}

pub fn detect_codex_path(explicit: Option<&Path>, path_env: &str) -> Result<PathBuf, String> {
    if let Some(path) = explicit {
        if path.exists() {
            return Ok(path.to_path_buf());
        }
    }

    for dir in std::env::split_paths(OsStr::new(path_env)) {
        #[cfg(windows)]
        {
            for candidate_name in ["codex.exe", "codex.cmd", "codex.bat", "codex"] {
                let candidate = dir.join(candidate_name);
                if candidate.exists() {
                    return Ok(candidate);
                }
            }
        }

        #[cfg(not(windows))]
        {
            let candidate = dir.join("codex");
            if candidate.exists() {
                return Ok(candidate);
            }
        }
    }

    Err("Codex binary not found".to_string())
}

pub fn parse_codex_version_line(line: &str) -> Option<String> {
    let mut parts = line.split_whitespace();
    let _name = parts.next()?;
    let raw = parts.next()?;
    Some(raw.trim_start_matches('v').to_string())
}

pub fn detect_npm_package_root_from_codex_binary(codex_path: &Path) -> Option<PathBuf> {
    let mut candidates = Vec::new();
    candidates.push(codex_path.to_path_buf());
    if let Ok(real) = std::fs::canonicalize(codex_path) {
        candidates.push(real);
    }

    for candidate in candidates {
        for ancestor in candidate.ancestors() {
            if let Some(root) = codex_package_root(ancestor) {
                return Some(root);
            }
        }
    }
    None
}

pub fn file_sha256_hex(path: &Path) -> Result<String, String> {
    let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    Ok(format!("{:x}", hasher.finalize()))
}

pub fn compatibility_key(version: &str, sha256: &str) -> String {
    format!("{version}+{sha256}")
}

pub fn probe_compatibility_key(explicit: Option<&Path>, path_env: &str) -> Result<String, String> {
    let codex = detect_codex_path(explicit, path_env)?;
    let out = Command::new(&codex)
        .arg("--version")
        .output()
        .map_err(|e| e.to_string())?;

    let version_line = String::from_utf8_lossy(&out.stdout)
        .lines()
        .next()
        .ok_or_else(|| "missing codex version output".to_string())?
        .to_string();

    let version = parse_codex_version_line(&version_line)
        .ok_or_else(|| "unable to parse codex version".to_string())?;
    let sha = file_sha256_hex(&codex)?;
    Ok(compatibility_key(&version, &sha))
}
