use codex_hud_ops::codex_probe::{
    detect_codex_path, detect_npm_package_root_from_codex_binary, file_sha256_hex,
    parse_codex_version_line, probe_compatibility_key,
    resolve_npm_vendor_binary_path_from_package_root,
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
fn skips_codex_hud_managed_shim_when_detecting_stock_codex() {
    let tmp = tempdir().unwrap();
    let managed_dir = tmp.path().join(".codex-hud/bin");
    let stock_dir = tmp.path().join("stock/bin");
    std::fs::create_dir_all(&managed_dir).unwrap();
    std::fs::create_dir_all(&stock_dir).unwrap();

    let exe_name = if cfg!(windows) { "codex.exe" } else { "codex" };
    let managed = managed_dir.join(exe_name);
    let stock = stock_dir.join(exe_name);
    std::fs::write(&managed, b"managed").unwrap();
    std::fs::write(&stock, b"stock").unwrap();

    let joined = std::env::join_paths([managed_dir.as_path(), stock_dir.as_path()]).unwrap();
    let found = detect_codex_path(None, &joined.to_string_lossy()).unwrap();
    assert_eq!(found, stock);
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
fn probe_key_ignores_codex_hud_managed_patch_block() {
    let tmp = tempdir().unwrap();
    let codex = if cfg!(windows) {
        tmp.path().join("codex.cmd")
    } else {
        tmp.path().join("codex")
    };

    #[cfg(windows)]
    std::fs::write(
        &codex,
        "@echo off\r\nif \"%1\"==\"--version\" echo codex-cli 0.104.0\r\n",
    )
    .unwrap();
    #[cfg(not(windows))]
    std::fs::write(
        &codex,
        "#!/usr/bin/env sh\nif [ \"$1\" = \"--version\" ]; then\necho codex-cli 0.104.0\nfi\n",
    )
    .unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut p = std::fs::metadata(&codex).unwrap().permissions();
        p.set_mode(0o755);
        std::fs::set_permissions(&codex, p).unwrap();
    }

    let key_before = probe_compatibility_key(Some(&codex), "").unwrap();

    #[cfg(windows)]
    std::fs::write(
        &codex,
        "@echo off\r\n/* codex-hud-managed:start */\r\nset CODEX_HUD_NATIVE_PATCH=1\r\n/* codex-hud-managed:end */\r\nif \"%1\"==\"--version\" echo codex-cli 0.104.0\r\n",
    )
    .unwrap();
    #[cfg(not(windows))]
    std::fs::write(
        &codex,
        "#!/usr/bin/env sh\n/* codex-hud-managed:start */\nexport CODEX_HUD_NATIVE_PATCH=1\n/* codex-hud-managed:end */\nif [ \"$1\" = \"--version\" ]; then\necho codex-cli 0.104.0\nfi\n",
    )
    .unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut p = std::fs::metadata(&codex).unwrap().permissions();
        p.set_mode(0o755);
        std::fs::set_permissions(&codex, p).unwrap();
    }

    let key_after = probe_compatibility_key(Some(&codex), "").unwrap();
    assert_eq!(key_before, key_after);
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

#[test]
fn resolves_npm_vendor_binary_path_from_package_root() {
    let root = std::path::Path::new("/opt/node_modules/@openai/codex");
    let resolved = resolve_npm_vendor_binary_path_from_package_root(root).unwrap();

    #[cfg(target_os = "macos")]
    #[cfg(target_arch = "aarch64")]
    assert!(resolved.ends_with(
        "node_modules/@openai/codex-darwin-arm64/vendor/aarch64-apple-darwin/codex/codex"
    ));

    #[cfg(target_os = "macos")]
    #[cfg(target_arch = "x86_64")]
    assert!(resolved
        .ends_with("node_modules/@openai/codex-darwin-x64/vendor/x86_64-apple-darwin/codex/codex"));

    #[cfg(target_os = "linux")]
    #[cfg(target_arch = "x86_64")]
    assert!(resolved
        .ends_with("node_modules/@openai/codex-linux-x64/vendor/x86_64-unknown-linux-musl/codex/codex"));

    #[cfg(target_os = "linux")]
    #[cfg(target_arch = "aarch64")]
    assert!(resolved.ends_with(
        "node_modules/@openai/codex-linux-arm64/vendor/aarch64-unknown-linux-musl/codex/codex"
    ));

    #[cfg(target_os = "windows")]
    #[cfg(target_arch = "x86_64")]
    assert!(resolved.ends_with(
        "node_modules\\@openai\\codex-win32-x64\\vendor\\x86_64-pc-windows-msvc\\codex\\codex.exe"
    ));

    #[cfg(target_os = "windows")]
    #[cfg(target_arch = "aarch64")]
    assert!(resolved.ends_with(
        "node_modules\\@openai\\codex-win32-arm64\\vendor\\aarch64-pc-windows-msvc\\codex\\codex.exe"
    ));
}

#[test]
fn detect_codex_path_ignores_missing_explicit_and_uses_path_search() {
    let tmp = tempdir().unwrap();
    let missing_explicit = tmp.path().join("does-not-exist-codex");
    let bin_dir = tmp.path().join("bin");
    std::fs::create_dir_all(&bin_dir).unwrap();
    let exe_name = if cfg!(windows) { "codex.exe" } else { "codex" };
    let discovered = bin_dir.join(exe_name);
    std::fs::write(&discovered, "stub").unwrap();

    let joined = std::env::join_paths([bin_dir.as_path()]).unwrap();
    let found =
        detect_codex_path(Some(&missing_explicit), &joined.to_string_lossy()).unwrap();
    assert_eq!(found, discovered);
}

#[test]
fn detect_codex_path_errors_when_no_candidate_is_available() {
    let tmp = tempdir().unwrap();
    let missing_explicit = tmp.path().join("missing-codex");
    let err = detect_codex_path(Some(&missing_explicit), "").unwrap_err();
    assert!(err.contains("Codex binary not found"));
}

#[test]
fn parse_codex_version_line_accepts_v_prefix_and_rejects_invalid_shape() {
    assert_eq!(
        parse_codex_version_line("codex-cli v0.104.0"),
        Some("0.104.0".to_string())
    );
    assert_eq!(parse_codex_version_line("codex-cli"), None);
}

#[test]
fn detect_npm_package_root_returns_none_for_non_codex_package() {
    let tmp = tempdir().unwrap();
    let root = tmp.path().join("lib/node_modules/not-codex");
    let launcher = root.join("bin/codex.js");
    std::fs::create_dir_all(launcher.parent().unwrap()).unwrap();
    std::fs::write(
        root.join("package.json"),
        r#"{"name":"example/not-codex","version":"1.0.0"}"#,
    )
    .unwrap();
    std::fs::write(&launcher, "#!/usr/bin/env node\n").unwrap();

    let detected = detect_npm_package_root_from_codex_binary(&launcher);
    assert!(detected.is_none());
}

#[test]
fn file_sha256_tolerates_unclosed_managed_patch_markers() {
    let tmp = tempdir().unwrap();
    let codex = tmp.path().join("codex");
    std::fs::write(
        &codex,
        "#!/usr/bin/env sh\n/* codex-hud-managed:start */\necho codex-cli 0.104.0\n",
    )
    .unwrap();

    let sha = file_sha256_hex(&codex).unwrap();
    assert_eq!(sha.len(), 64);
}

#[test]
fn file_sha256_ignores_managed_patch_with_crlf_tail_separator() {
    let tmp = tempdir().unwrap();
    let base = tmp.path().join("base-codex");
    let patched = tmp.path().join("patched-codex");
    let body = "#!/usr/bin/env sh\necho codex-cli 0.104.0\n";

    std::fs::write(&base, body).unwrap();
    std::fs::write(
        &patched,
        format!(
            "/* codex-hud-managed:start */\r\nset -e\r\n/* codex-hud-managed:end */\r\n{body}"
        ),
    )
    .unwrap();

    let base_sha = file_sha256_hex(&base).unwrap();
    let patched_sha = file_sha256_hex(&patched).unwrap();
    assert_eq!(base_sha, patched_sha);
}
