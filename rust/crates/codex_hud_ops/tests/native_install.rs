use codex_hud_ops::manifest_signing::{sign_manifest_for_tests, test_public_key_hex_for_tests};
use codex_hud_ops::native_install::{
    install_native_patch, install_native_patch_auto_for_stock_path, install_native_patch_auto_with,
    run_stock_codex_passthrough, uninstall_native_patch, InstallOutcome,
};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;
use tempfile::tempdir;

#[test]
fn supported_install_patches_all_targets_and_creates_backups() {
    let tmp = tempdir().unwrap();
    let root = tmp.path().join("codex-rs");

    let rels = [
        "codex-rs/tui/src/slash_command.rs",
        "codex-rs/tui/src/chatwidget.rs",
        "codex-rs/tui/src/app.rs",
        "codex-rs/tui/src/bottom_pane/status_line_setup.rs",
    ];

    for rel in rels {
        let file = root.join(rel.strip_prefix("codex-rs/").unwrap());
        std::fs::create_dir_all(file.parent().unwrap()).unwrap();
        std::fs::write(&file, "SlashCommand::Statusline").unwrap();
    }

    let manifest = root.join(".codex-hud-compat.json");
    let payload = r#"{"schema_version":1,"supported_keys":["0.104.0+abc"]}"#;
    let sig = sign_manifest_for_tests(payload);
    std::fs::write(
        &manifest,
        format!(
            r#"{{"schema_version":1,"supported_keys":["0.104.0+abc"],"signature_hex":"{}"}}"#,
            sig
        ),
    )
    .unwrap();

    let out = install_native_patch(
        &root,
        "0.104.0+abc",
        &manifest,
        &test_public_key_hex_for_tests(),
    )
    .unwrap();
    assert_eq!(out, InstallOutcome::Patched);

    for rel in rels {
        let file = root.join(rel.strip_prefix("codex-rs/").unwrap());
        let patched = std::fs::read_to_string(&file).unwrap();
        assert!(patched.contains("codex-hud-managed"));

        let backup = root
            .join(".codex-hud/backups")
            .join(rel)
            .with_extension("bak");
        assert!(backup.exists());
    }

    let tx_root = tmp.path().join("codex-rs-tx");
    let tx_rels = [
        "codex-rs/tui/src/slash_command.rs",
        "codex-rs/tui/src/chatwidget.rs",
        "codex-rs/tui/src/app.rs",
        "codex-rs/tui/src/bottom_pane/status_line_setup.rs",
    ];
    for rel in tx_rels {
        let file = tx_root.join(rel.strip_prefix("codex-rs/").unwrap());
        std::fs::create_dir_all(file.parent().unwrap()).unwrap();
        if rel.ends_with("chatwidget.rs") {
            std::fs::write(&file, "NO_MARKER_PRESENT").unwrap();
        } else {
            std::fs::write(&file, "SlashCommand::Statusline").unwrap();
        }
    }

    let tx_manifest = tx_root.join(".codex-hud-compat.json");
    std::fs::write(
        &tx_manifest,
        format!(
            r#"{{"schema_version":1,"supported_keys":["0.104.0+abc"],"signature_hex":"{}"}}"#,
            sig
        ),
    )
    .unwrap();

    let err = install_native_patch(
        &tx_root,
        "0.104.0+abc",
        &tx_manifest,
        &test_public_key_hex_for_tests(),
    )
    .unwrap_err();
    assert!(err.contains("patch marker not found"));
    let untouched = tx_root.join("tui/src/slash_command.rs");
    assert_eq!(
        std::fs::read_to_string(untouched).unwrap(),
        "SlashCommand::Statusline"
    );
    let unexpected_backup = tx_root
        .join(".codex-hud/backups")
        .join("codex-rs/tui/src/slash_command.rs")
        .with_extension("bak");
    assert!(!unexpected_backup.exists());

    let missing_root = tmp.path().join("codex-rs-missing");
    let only_one = missing_root.join("tui/src/slash_command.rs");
    std::fs::create_dir_all(only_one.parent().unwrap()).unwrap();
    std::fs::write(&only_one, "SlashCommand::Statusline").unwrap();
    let missing_manifest = missing_root.join(".codex-hud-compat.json");
    std::fs::write(
        &missing_manifest,
        format!(
            r#"{{"schema_version":1,"supported_keys":["0.104.0+abc"],"signature_hex":"{}"}}"#,
            sig
        ),
    )
    .unwrap();

    let out = install_native_patch(
        &missing_root,
        "0.104.0+abc",
        &missing_manifest,
        &test_public_key_hex_for_tests(),
    )
    .unwrap();
    assert_eq!(
        out,
        InstallOutcome::RanStock {
            reason: "native patch substrate unavailable for installed codex layout".to_string(),
        }
    );
}

#[test]
fn uninstall_restores_all_patched_targets_from_state() {
    let tmp = tempdir().unwrap();
    let root = tmp.path().join("codex-rs");

    let rels = [
        "codex-rs/tui/src/slash_command.rs",
        "codex-rs/tui/src/chatwidget.rs",
        "codex-rs/tui/src/app.rs",
        "codex-rs/tui/src/bottom_pane/status_line_setup.rs",
    ];

    for rel in rels {
        let real = root.join(rel.strip_prefix("codex-rs/").unwrap());
        std::fs::create_dir_all(real.parent().unwrap()).unwrap();
        std::fs::write(&real, "SlashCommand::Statusline").unwrap();

        let backup = root
            .join(".codex-hud/backups")
            .join(rel)
            .with_extension("bak");
        std::fs::create_dir_all(backup.parent().unwrap()).unwrap();
        std::fs::write(&backup, "original").unwrap();
    }

    std::fs::create_dir_all(root.join(".codex-hud")).unwrap();
    std::fs::write(
        root.join(".codex-hud/patch-state.json"),
        r#"{"patched_rel_paths":["codex-rs/tui/src/slash_command.rs","codex-rs/tui/src/chatwidget.rs","codex-rs/tui/src/app.rs","codex-rs/tui/src/bottom_pane/status_line_setup.rs"]}"#,
    )
    .unwrap();

    uninstall_native_patch(&root).unwrap();

    for rel in rels {
        let real = root.join(rel.strip_prefix("codex-rs/").unwrap());
        assert_eq!(std::fs::read_to_string(real).unwrap(), "original");
    }
}

#[test]
fn passthrough_reports_nonzero_status_and_stderr() {
    let tmp = tempdir().unwrap();
    let script = tmp.path().join("stock.sh");
    std::fs::write(&script, "#!/usr/bin/env sh\nprintf 'ERR' 1>&2\nexit 11\n").unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&script).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&script, perms).unwrap();
    }

    let out = run_stock_codex_passthrough(&script, &[]).unwrap();
    assert_eq!(out.status_code, 11);
    assert_eq!(out.stderr, "ERR");
}

#[test]
fn supported_install_patches_npm_codex_launcher_in_place() {
    let tmp = tempdir().unwrap();
    let root = tmp.path().join("npm-codex");
    std::fs::create_dir_all(root.join("bin")).unwrap();
    std::fs::write(
        root.join("package.json"),
        r#"{"name":"@openai/codex","version":"0.104.0"}"#,
    )
    .unwrap();
    std::fs::write(
        root.join("bin/codex.js"),
        r#"#!/usr/bin/env node
const updatedPath = "x";
const env = { ...process.env, PATH: updatedPath };
"#,
    )
    .unwrap();

    let manifest = root.join(".codex-hud-compat.json");
    let payload = r#"{"schema_version":1,"supported_keys":["0.104.0+abc"]}"#;
    let sig = sign_manifest_for_tests(payload);
    std::fs::write(
        &manifest,
        format!(
            r#"{{"schema_version":1,"supported_keys":["0.104.0+abc"],"signature_hex":"{}"}}"#,
            sig
        ),
    )
    .unwrap();

    let out = install_native_patch(
        &root,
        "0.104.0+abc",
        &manifest,
        &test_public_key_hex_for_tests(),
    )
    .unwrap();
    assert_eq!(out, InstallOutcome::Patched);

    let patched = std::fs::read_to_string(root.join("bin/codex.js")).unwrap();
    assert!(patched.contains("codex-hud-managed"));
}

#[test]
fn uninstall_removes_npm_launcher_managed_block_without_backup_files() {
    let tmp = tempdir().unwrap();
    let root = tmp.path().join("npm-codex");
    std::fs::create_dir_all(root.join("bin")).unwrap();
    std::fs::create_dir_all(root.join(".codex-hud")).unwrap();
    std::fs::write(
        root.join("bin/codex.js"),
        r#"const env = { ...process.env, PATH: updatedPath };
/* codex-hud-managed:start */
env.CODEX_HUD_NATIVE_PATCH = "1";
/* codex-hud-managed:end */
"#,
    )
    .unwrap();
    std::fs::write(
        root.join(".codex-hud/patch-state.json"),
        r#"{"patched_rel_paths":["bin/codex.js"]}"#,
    )
    .unwrap();

    uninstall_native_patch(&root).unwrap();
    let restored = std::fs::read_to_string(root.join("bin/codex.js")).unwrap();
    assert!(!restored.contains("codex-hud-managed"));
    assert!(restored.contains("const env = { ...process.env, PATH: updatedPath };"));
}

#[test]
fn install_auto_refreshes_compat_bundle_before_support_gate_resolution() {
    let tmp = tempdir().unwrap();
    let home = tmp.path().join("home");
    let bin = tmp.path().join("bin");
    let root = tmp.path().join("codex-rs");
    std::fs::create_dir_all(&home).unwrap();
    std::fs::create_dir_all(&bin).unwrap();

    let codex = if cfg!(windows) {
        bin.join("codex.cmd")
    } else {
        bin.join("codex")
    };
    #[cfg(windows)]
    std::fs::write(&codex, "@echo off\r\necho codex-cli 0.104.0\r\n").unwrap();
    #[cfg(not(windows))]
    std::fs::write(&codex, "#!/usr/bin/env sh\necho codex-cli 0.104.0\n").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&codex).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&codex, perms).unwrap();
    }

    for rel in [
        "tui/src/slash_command.rs",
        "tui/src/chatwidget.rs",
        "tui/src/app.rs",
        "tui/src/bottom_pane/status_line_setup.rs",
    ] {
        let file = root.join(rel);
        std::fs::create_dir_all(file.parent().unwrap()).unwrap();
        std::fs::write(file, "SlashCommand::Statusline").unwrap();
    }

    let key = codex_hud_ops::codex_probe::probe_compatibility_key(Some(&codex), "").unwrap();
    let payload = format!(r#"{{"schema_version":1,"supported_keys":["{}"]}}"#, key);
    let signature = sign_manifest_for_tests(&payload);
    let pubkey = test_public_key_hex_for_tests();

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    let key_for_server = key.clone();
    let server = thread::spawn(move || {
        for _ in 0..2 {
            let (mut stream, _) = listener.accept().unwrap();
            let mut buf = [0u8; 4096];
            let n = stream.read(&mut buf).unwrap();
            let req = String::from_utf8_lossy(&buf[..n]);
            let first = req.lines().next().unwrap_or("");
            let (status, body) = if first.contains("GET /compat.json") {
                (
                    "200 OK",
                    format!(
                        r#"{{"schema_version":1,"supported_keys":["{}"],"signature_hex":"{}"}}"#,
                        key_for_server, signature
                    ),
                )
            } else if first.contains("GET /public_key.hex") {
                ("200 OK", pubkey.clone())
            } else {
                ("404 Not Found", "missing".to_string())
            };
            let response = format!(
                "HTTP/1.1 {status}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            stream.write_all(response.as_bytes()).unwrap();
        }
    });

    let outcome = install_native_patch_auto_with(
        &home,
        "",
        Some(&codex),
        Some(&format!("http://{addr}")),
    )
    .unwrap();
    assert_eq!(outcome, InstallOutcome::Patched);

    let downloaded =
        std::fs::read_to_string(home.join(".codex-hud/compat/compat.json")).unwrap();
    assert!(downloaded.contains(&key));
    server.join().unwrap();
}

#[test]
fn install_auto_detects_npm_package_root_from_codex_launcher_path() {
    let tmp = tempdir().unwrap();
    let home = tmp.path().join("home");
    let npm_root = tmp.path().join("node_modules/@openai/codex");
    let launcher = npm_root.join("bin/codex.js");
    std::fs::create_dir_all(launcher.parent().unwrap()).unwrap();
    std::fs::create_dir_all(home.join(".codex-hud/compat")).unwrap();

    std::fs::write(
        npm_root.join("package.json"),
        r#"{"name":"@openai/codex","version":"0.104.0"}"#,
    )
    .unwrap();
    std::fs::write(
        &launcher,
        r#"#!/usr/bin/env node
if (process.argv.includes("--version")) {
  console.log("codex-cli 0.104.0");
  process.exit(0);
}
const updatedPath = process.env.PATH || "";
const env = { ...process.env, PATH: updatedPath };
console.log(env.PATH);
"#,
    )
    .unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&launcher).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&launcher, perms).unwrap();
    }

    let key = codex_hud_ops::codex_probe::probe_compatibility_key(Some(&launcher), "").unwrap();
    let payload = format!(r#"{{"schema_version":1,"supported_keys":["{}"]}}"#, key);
    let signature = sign_manifest_for_tests(&payload);
    std::fs::write(
        home.join(".codex-hud/compat/compat.json"),
        format!(
            r#"{{"schema_version":1,"supported_keys":["{}"],"signature_hex":"{}"}}"#,
            key, signature
        ),
    )
    .unwrap();
    std::fs::write(
        home.join(".codex-hud/compat/public_key.hex"),
        test_public_key_hex_for_tests(),
    )
    .unwrap();

    let out = install_native_patch_auto_with(&home, "", Some(&launcher), None).unwrap();
    assert_eq!(out, InstallOutcome::Patched);
    let patched = std::fs::read_to_string(&launcher).unwrap();
    assert!(patched.contains("codex-hud-managed"));
}

#[test]
fn install_auto_uses_explicit_stock_path_when_path_env_is_broken() {
    let tmp = tempdir().unwrap();
    let home = tmp.path().join("home");
    let bad_bin = tmp.path().join("bad-bin");
    let bad_codex = if cfg!(windows) {
        bad_bin.join("codex.cmd")
    } else {
        bad_bin.join("codex")
    };
    std::fs::create_dir_all(&home).unwrap();
    std::fs::create_dir_all(&bad_bin).unwrap();
    #[cfg(windows)]
    std::fs::write(&bad_codex, "@echo off\r\necho NOT_A_VERSION\r\n").unwrap();
    #[cfg(not(windows))]
    std::fs::write(&bad_codex, "#!/usr/bin/env sh\necho NOT_A_VERSION\n").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&bad_codex).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&bad_codex, perms).unwrap();
    }

    let npm_root = tmp.path().join("node_modules/@openai/codex");
    let launcher = npm_root.join("bin/codex.js");
    std::fs::create_dir_all(launcher.parent().unwrap()).unwrap();
    std::fs::create_dir_all(home.join(".codex-hud/compat")).unwrap();
    std::fs::write(
        npm_root.join("package.json"),
        r#"{"name":"@openai/codex","version":"0.104.0"}"#,
    )
    .unwrap();
    std::fs::write(
        &launcher,
        r#"#!/usr/bin/env node
if (process.argv.includes("--version")) {
  console.log("codex-cli 0.104.0");
  process.exit(0);
}
const updatedPath = process.env.PATH || "";
const env = { ...process.env, PATH: updatedPath };
console.log(env.PATH);
"#,
    )
    .unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&launcher).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&launcher, perms).unwrap();
    }

    let key = codex_hud_ops::codex_probe::probe_compatibility_key(Some(&launcher), "").unwrap();
    let payload = format!(r#"{{"schema_version":1,"supported_keys":["{}"]}}"#, key);
    let signature = sign_manifest_for_tests(&payload);
    std::fs::write(
        home.join(".codex-hud/compat/compat.json"),
        format!(
            r#"{{"schema_version":1,"supported_keys":["{}"],"signature_hex":"{}"}}"#,
            key, signature
        ),
    )
    .unwrap();
    std::fs::write(
        home.join(".codex-hud/compat/public_key.hex"),
        test_public_key_hex_for_tests(),
    )
    .unwrap();

    let path_env = bad_bin.to_string_lossy().to_string();
    let out = install_native_patch_auto_for_stock_path(&home, &path_env, &launcher).unwrap();
    assert_eq!(out, InstallOutcome::Patched);
}
