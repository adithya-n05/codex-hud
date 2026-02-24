use codex_hud_ops::manifest_signing::{sign_manifest_for_tests, test_public_key_hex_for_tests};
use codex_hud_ops::native_install::{
    install_native_patch, run_stock_codex_passthrough, uninstall_native_patch, InstallOutcome,
};
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
