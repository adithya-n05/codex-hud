use std::process::Command;
use tempfile::tempdir;
use codex_hud_ops::manifest_signing::{sign_manifest_for_tests, test_public_key_hex_for_tests};

#[test]
fn binary_run_route_propagates_stock_failure() {
    let tmp = tempdir().unwrap();
    let stock = tmp.path().join("stock.sh");
    std::fs::write(&stock, "#!/usr/bin/env sh\nexit 13\n").unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&stock).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&stock, perms).unwrap();
    }

    let bin = env!("CARGO_BIN_EXE_codex_hud_cli");
    let out = Command::new(bin)
        .args([
            "run",
            "--stock-codex",
            stock.to_str().unwrap(),
            "--",
            "--help",
        ])
        .output()
        .unwrap();

    assert_eq!(out.status.code(), Some(13));
}

#[test]
fn run_route_auto_patches_supported_stock_codex_before_passthrough() {
    let tmp = tempdir().unwrap();
    let home = tmp.path().join("home");
    let npm_root = tmp.path().join("node_modules/@openai/codex");
    let launcher = npm_root.join("bin/codex.js");
    std::fs::create_dir_all(home.join(".codex-hud/compat")).unwrap();
    std::fs::create_dir_all(launcher.parent().unwrap()).unwrap();

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

    let bin = env!("CARGO_BIN_EXE_codex_hud_cli");
    let out = Command::new(bin)
        .args([
            "run",
            "--stock-codex",
            launcher.to_str().unwrap(),
            "--",
            "--version",
        ])
        .env("HOME", &home)
        .output()
        .unwrap();

    assert!(out.status.success());
    let patched = std::fs::read_to_string(&launcher).unwrap();
    assert!(patched.contains("codex-hud-managed"));
}
