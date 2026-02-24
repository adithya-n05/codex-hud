use codex_hud_ops::codex_probe::{
    detect_codex_path, detect_npm_package_root_from_codex_binary, file_sha256_hex,
    probe_compatibility_key,
};
use tempfile::tempdir;

#[test]
fn detects_codex_from_explicit_existing_path() {
    let tmp = tempdir().unwrap();
    let codex = tmp.path().join("codex");
    std::fs::write(&codex, b"#!/usr/bin/env sh\n").unwrap();
    let found = detect_codex_path(Some(&codex), "").unwrap();
    assert_eq!(found, codex);
}

#[test]
fn detects_codex_from_joined_path_list() {
    let tmp = tempdir().unwrap();
    let dir = tmp.path().join("bin");
    std::fs::create_dir_all(&dir).unwrap();
    let exe_name = if cfg!(windows) { "codex.exe" } else { "codex" };
    let codex = dir.join(exe_name);
    std::fs::write(&codex, b"stub").unwrap();

    let joined = std::env::join_paths([dir.as_path()]).unwrap();
    let found = detect_codex_path(None, &joined.to_string_lossy()).unwrap();
    assert_eq!(found, codex);
}

#[test]
fn probes_compatibility_key_from_codex_binary() {
    let tmp = tempdir().unwrap();
    let codex = if cfg!(windows) {
        tmp.path().join("codex.cmd")
    } else {
        tmp.path().join("codex")
    };

    #[cfg(windows)]
    std::fs::write(&codex, "@echo off\r\necho codex-cli 0.104.0\r\n").unwrap();
    #[cfg(not(windows))]
    std::fs::write(&codex, "#!/usr/bin/env sh\necho codex-cli 0.104.0\n").unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut p = std::fs::metadata(&codex).unwrap().permissions();
        p.set_mode(0o755);
        std::fs::set_permissions(&codex, p).unwrap();
    }

    let key = probe_compatibility_key(Some(&codex), "").unwrap();
    let sha = file_sha256_hex(&codex).unwrap();
    assert_eq!(key, format!("0.104.0+{sha}"));
}

#[test]
fn detects_npm_package_root_from_real_codex_launcher_path() {
    let tmp = tempdir().unwrap();
    let root = tmp.path().join("lib/node_modules/@openai/codex");
    let launcher = root.join("bin/codex.js");
    std::fs::create_dir_all(launcher.parent().unwrap()).unwrap();
    std::fs::write(
        root.join("package.json"),
        r#"{"name":"@openai/codex","version":"0.104.0"}"#,
    )
    .unwrap();
    std::fs::write(&launcher, "#!/usr/bin/env node\n").unwrap();

    let detected = detect_npm_package_root_from_codex_binary(&launcher).unwrap();
    assert_eq!(detected, root);
}
