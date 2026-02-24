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

fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || haystack.len() < needle.len() {
        return None;
    }
    haystack.windows(needle.len()).position(|window| window == needle)
}

fn strip_managed_patch_block(bytes: &[u8]) -> Vec<u8> {
    const START: &[u8] = b"/* codex-hud-managed:start */";
    const END: &[u8] = b"/* codex-hud-managed:end */";

    let Some(start) = find_subslice(bytes, START) else {
        return bytes.to_vec();
    };
    let Some(end_rel) = find_subslice(&bytes[start..], END) else {
        return bytes.to_vec();
    };
    let end = start + end_rel + END.len();

    let mut out = Vec::with_capacity(bytes.len());
    out.extend_from_slice(&bytes[..start]);
    let mut tail = &bytes[end..];
    if tail.starts_with(b"\r\n") {
        tail = &tail[2..];
    } else if tail.starts_with(b"\n") {
        tail = &tail[1..];
    }
    out.extend_from_slice(tail);
    out
}

fn is_codex_hud_managed_shim(candidate: &Path) -> bool {
    let Some(stem) = candidate.file_stem().and_then(|v| v.to_str()) else {
        return false;
    };
    if !stem.eq_ignore_ascii_case("codex") {
        return false;
    }
    let Some(bin_dir) = candidate.parent() else {
        return false;
    };
    if bin_dir.file_name().and_then(|v| v.to_str()) != Some("bin") {
        return false;
    }
    let Some(root_dir) = bin_dir.parent() else {
        return false;
    };
    root_dir.file_name().and_then(|v| v.to_str()) == Some(".codex-hud")
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
                if candidate.exists() && !is_codex_hud_managed_shim(&candidate) {
                    return Ok(candidate);
                }
            }
        }

        #[cfg(not(windows))]
        {
            let candidate = dir.join("codex");
            if candidate.exists() && !is_codex_hud_managed_shim(&candidate) {
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

pub fn resolve_npm_vendor_binary_path_from_package_root(codex_root: &Path) -> Result<PathBuf, String> {
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    let rel = "node_modules/@openai/codex-darwin-arm64/vendor/aarch64-apple-darwin/codex/codex";

    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    let rel = "node_modules/@openai/codex-darwin-x64/vendor/x86_64-apple-darwin/codex/codex";

    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    let rel = "node_modules/@openai/codex-linux-x64/vendor/x86_64-unknown-linux-musl/codex/codex";

    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    let rel = "node_modules/@openai/codex-linux-arm64/vendor/aarch64-unknown-linux-musl/codex/codex";

    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    let rel = "node_modules/@openai/codex-win32-x64/vendor/x86_64-pc-windows-msvc/codex/codex.exe";

    #[cfg(all(target_os = "windows", target_arch = "aarch64"))]
    let rel = "node_modules/@openai/codex-win32-arm64/vendor/aarch64-pc-windows-msvc/codex/codex.exe";

    #[cfg(not(any(
        all(target_os = "macos", target_arch = "aarch64"),
        all(target_os = "macos", target_arch = "x86_64"),
        all(target_os = "linux", target_arch = "x86_64"),
        all(target_os = "linux", target_arch = "aarch64"),
        all(target_os = "windows", target_arch = "x86_64"),
        all(target_os = "windows", target_arch = "aarch64")
    )))]
    return Err("unsupported platform for npm codex vendor binary path resolution".to_string());

    Ok(codex_root.join(rel))
}

pub fn file_sha256_hex(path: &Path) -> Result<String, String> {
    let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
    let normalized = strip_managed_patch_block(&bytes);
    let mut hasher = Sha256::new();
    hasher.update(normalized);
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

#[cfg(test)]
mod tests {
    use super::{find_subslice, is_codex_hud_managed_shim};
    use std::path::{Path, PathBuf};

    #[test]
    fn find_subslice_handles_empty_and_oversized_needles() {
        assert_eq!(find_subslice(b"abc", b""), None);
        assert_eq!(find_subslice(b"ab", b"abc"), None);
        assert_eq!(find_subslice(b"abc", b"bc"), Some(1));
    }

    #[test]
    fn managed_shim_detector_requires_expected_path_shape() {
        assert!(!is_codex_hud_managed_shim(Path::new("codex")));
        assert!(!is_codex_hud_managed_shim(Path::new("tools/codex")));
        assert!(!is_codex_hud_managed_shim(Path::new(".codex-hud/bin/not-codex")));
        assert!(is_codex_hud_managed_shim(Path::new(".codex-hud/bin/codex")));
    }

    #[cfg(unix)]
    #[test]
    fn managed_shim_detector_rejects_non_utf8_stem() {
        use std::ffi::OsString;
        use std::os::unix::ffi::OsStringExt;

        let mut path = PathBuf::from(".codex-hud/bin");
        path.push(OsString::from_vec(vec![0xff]));
        assert!(!is_codex_hud_managed_shim(&path));
    }
}
