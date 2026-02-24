use codex_hud_ops::codex_probe::file_sha256_hex;
use codex_hud_ops::manifest_signing::{sign_manifest_for_tests, test_public_key_hex_for_tests};
use std::process::Command;
use tempfile::tempdir;

#[test]
fn install_route_works_with_home_and_path_only() {
    let tmp = tempdir().unwrap();
    let root = tmp.path().join("codex-rs");
    let home = tmp.path().join("home");
    let bin_dir = tmp.path().join("bin");
    std::fs::create_dir_all(&home).unwrap();
    std::fs::create_dir_all(&bin_dir).unwrap();

    let codex = if cfg!(windows) {
        bin_dir.join("codex.cmd")
    } else {
        bin_dir.join("codex")
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
        std::fs::write(&file, "SlashCommand::Statusline").unwrap();
    }

    let sha = file_sha256_hex(&codex).unwrap();
    let key = format!("0.104.0+{sha}");
    let payload = format!(r#"{{"schema_version":1,"supported_keys":["{}"]}}"#, key);
    let signature = sign_manifest_for_tests(&payload);

    let compat_dir = home.join(".codex-hud/compat");
    std::fs::create_dir_all(&compat_dir).unwrap();
    std::fs::write(
        compat_dir.join("compat.json"),
        format!(
            r#"{{"schema_version":1,"supported_keys":["{}"],"signature_hex":"{}"}}"#,
            key, signature
        ),
    )
    .unwrap();
    std::fs::write(
        compat_dir.join("public_key.hex"),
        test_public_key_hex_for_tests(),
    )
    .unwrap();

    let existing = std::env::var("PATH").unwrap_or_default();
    let sep = if cfg!(windows) { ';' } else { ':' };
    let path_value = format!("{}{}{}", bin_dir.to_string_lossy(), sep, existing);

    let cli = env!("CARGO_BIN_EXE_codex_hud_cli");
    let out = Command::new(cli)
        .arg("install")
        .env("HOME", &home)
        .env("PATH", path_value)
        .output()
        .unwrap();
    assert!(out.status.success());

    let patched = std::fs::read_to_string(root.join("tui/src/slash_command.rs")).unwrap();
    assert!(patched.contains("codex-hud-managed"));

    let tmp2 = tempdir().unwrap();
    let home2 = tmp2.path().join("home");
    let bin_dir2 = tmp2.path().join("bin");
    std::fs::create_dir_all(&home2).unwrap();
    std::fs::create_dir_all(&bin_dir2).unwrap();
    let codex2 = if cfg!(windows) {
        bin_dir2.join("codex.cmd")
    } else {
        bin_dir2.join("codex")
    };
    #[cfg(windows)]
    std::fs::write(&codex2, "@echo off\r\necho codex-cli 0.104.0\r\n").unwrap();
    #[cfg(not(windows))]
    std::fs::write(&codex2, "#!/usr/bin/env sh\necho codex-cli 0.104.0\n").unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&codex2).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&codex2, perms).unwrap();
    }

    let sha2 = file_sha256_hex(&codex2).unwrap();
    let key2 = format!("0.104.0+{sha2}");
    let payload2 = format!(r#"{{"schema_version":1,"supported_keys":["{}"]}}"#, key2);
    let signature2 = sign_manifest_for_tests(&payload2);
    let compat_dir2 = home2.join(".codex-hud/compat");
    std::fs::create_dir_all(&compat_dir2).unwrap();
    std::fs::write(
        compat_dir2.join("compat.json"),
        format!(
            r#"{{"schema_version":1,"supported_keys":["{}"],"signature_hex":"{}"}}"#,
            key2, signature2
        ),
    )
    .unwrap();
    std::fs::write(
        compat_dir2.join("public_key.hex"),
        test_public_key_hex_for_tests(),
    )
    .unwrap();
    let existing2 = std::env::var("PATH").unwrap_or_default();
    let path_value2 = format!("{}{}{}", bin_dir2.to_string_lossy(), sep, existing2);

    let out2 = Command::new(cli)
        .arg("install")
        .env("HOME", &home2)
        .env("PATH", path_value2)
        .output()
        .unwrap();

    assert!(!out2.status.success());
    let stderr2 = String::from_utf8_lossy(&out2.stderr);
    assert!(stderr2.contains(
        "install blocked: native patch substrate unavailable for installed codex layout"
    ));
}

#[test]
fn install_route_fails_when_current_codex_is_not_supported() {
    let tmp = tempdir().unwrap();
    let home = tmp.path().join("home");
    let bin_dir = tmp.path().join("bin");
    std::fs::create_dir_all(&home).unwrap();
    std::fs::create_dir_all(&bin_dir).unwrap();

    let codex = if cfg!(windows) {
        bin_dir.join("codex.cmd")
    } else {
        bin_dir.join("codex")
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

    let existing = std::env::var("PATH").unwrap_or_default();
    let sep = if cfg!(windows) { ';' } else { ':' };
    let path_value = format!("{}{}{}", bin_dir.to_string_lossy(), sep, existing);

    let cli = env!("CARGO_BIN_EXE_codex_hud_cli");
    let out = Command::new(cli)
        .arg("install")
        .env("HOME", &home)
        .env("PATH", path_value)
        .output()
        .unwrap();

    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("install blocked"));
}
