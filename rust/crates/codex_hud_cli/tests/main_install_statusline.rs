use codex_hud_ops::codex_probe::file_sha256_hex;
use codex_hud_ops::manifest_signing::{sign_manifest_for_tests, test_public_key_hex_for_tests};
use std::process::Command;
use tempfile::tempdir;

#[test]
fn install_route_migrates_legacy_statusline_config() {
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

    let codex_config_dir = home.join(".codex");
    std::fs::create_dir_all(&codex_config_dir).unwrap();
    std::fs::write(
        codex_config_dir.join("config.toml"),
        r#"
[tui]
status_line = [
  "model-with-reasoning",
  "git-branch",
  "context-remaining",
  "context-used",
]
"#,
    )
    .unwrap();

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

    let updated = std::fs::read_to_string(codex_config_dir.join("config.toml")).unwrap();
    assert!(updated.contains("\"permission-mode\""));
    assert!(updated.contains("\"auth-chip\""));
    assert!(updated.contains("\"tool-calls\""));
    assert!(updated.contains("\"ctx-bar\""));
}
