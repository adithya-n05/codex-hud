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
    let vendor_binary =
        codex_hud_ops::codex_probe::resolve_npm_vendor_binary_path_from_package_root(&npm_root)
            .unwrap();
    std::fs::create_dir_all(vendor_binary.parent().unwrap()).unwrap();
    std::fs::write(&vendor_binary, b"stock-binary").unwrap();

    let cache_path = codex_hud_ops::native_install::patched_binary_cache_path_for_test(&home, &key);
    std::fs::create_dir_all(cache_path.parent().unwrap()).unwrap();
    std::fs::write(&cache_path, b"patched-binary").unwrap();

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

#[test]
fn run_route_emits_unsupported_notice_once_per_compat_key() {
    let tmp = tempdir().unwrap();
    let home = tmp.path().join("home");
    std::fs::create_dir_all(home.join(".codex-hud/compat")).unwrap();
    let stock = tmp.path().join("stock-codex");
    std::fs::write(&stock, "#!/usr/bin/env sh\necho \"codex-cli 0.104.0\"\n").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&stock).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&stock, perms).unwrap();
    }

    let signature = sign_manifest_for_tests(r#"{"schema_version":1,"supported_keys":[]}"#);
    std::fs::write(
        home.join(".codex-hud/compat/compat.json"),
        format!(
            r#"{{"schema_version":1,"supported_keys":[],"signature_hex":"{}"}}"#,
            signature
        ),
    )
    .unwrap();
    std::fs::write(
        home.join(".codex-hud/compat/public_key.hex"),
        test_public_key_hex_for_tests(),
    )
    .unwrap();

    let bin = env!("CARGO_BIN_EXE_codex_hud_cli");
    let first = Command::new(bin)
        .args(["run", "--stock-codex", stock.to_str().unwrap(), "--", "--version"])
        .env("HOME", &home)
        .output()
        .unwrap();
    assert!(first.status.success());
    let first_stderr = String::from_utf8_lossy(&first.stderr);
    assert!(first_stderr.contains("codex-hud is not yet compatible with"));

    let second = Command::new(bin)
        .args(["run", "--stock-codex", stock.to_str().unwrap(), "--", "--version"])
        .env("HOME", &home)
        .output()
        .unwrap();
    assert!(second.status.success());
    let second_stderr = String::from_utf8_lossy(&second.stderr);
    assert!(!second_stderr.contains("codex-hud is not yet compatible with"));
}

#[test]
fn run_route_records_last_policy_outcome_for_status_surfaces() {
    let tmp = tempdir().unwrap();
    let home = tmp.path().join("home");
    std::fs::create_dir_all(home.join(".codex-hud/compat")).unwrap();
    let npm_root = tmp.path().join("node_modules/@openai/codex");
    let stock = npm_root.join("bin/codex.js");
    std::fs::create_dir_all(stock.parent().unwrap()).unwrap();
    std::fs::write(
        npm_root.join("package.json"),
        r#"{"name":"@openai/codex","version":"0.104.0"}"#,
    )
    .unwrap();
    std::fs::write(
        &stock,
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
        let mut perms = std::fs::metadata(&stock).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&stock, perms).unwrap();
    }

    let signature = sign_manifest_for_tests(r#"{"schema_version":1,"supported_keys":[]}"#);
    std::fs::write(
        home.join(".codex-hud/compat/compat.json"),
        format!(
            r#"{{"schema_version":1,"supported_keys":[],"signature_hex":"{}"}}"#,
            signature
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
        .args(["run", "--stock-codex", stock.to_str().unwrap(), "--", "--version"])
        .env("HOME", &home)
        .output()
        .unwrap();
    assert!(out.status.success());

    let policy = std::fs::read_to_string(home.join(".codex-hud/last_run_policy.txt")).unwrap();
    assert!(policy.contains("mode=stock"));
    assert!(policy.contains("unsupported compatibility key"));
}

#[test]
fn run_route_records_error_policy_when_stock_process_exits_nonzero() {
    let tmp = tempdir().unwrap();
    let home = tmp.path().join("home");
    let npm_root = tmp.path().join("node_modules/@openai/codex");
    let stock = npm_root.join("bin/codex.js");
    std::fs::create_dir_all(home.join(".codex-hud/compat")).unwrap();
    std::fs::create_dir_all(stock.parent().unwrap()).unwrap();
    std::fs::write(
        npm_root.join("package.json"),
        r#"{"name":"@openai/codex","version":"0.104.0"}"#,
    )
    .unwrap();
    std::fs::write(
        &stock,
        r#"#!/usr/bin/env node
if (process.argv.includes("--version")) {
  console.log("codex-cli 0.104.0");
  process.exit(0);
}
if (process.argv.includes("--fail")) {
  console.error("boom");
  process.exit(17);
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
        let mut perms = std::fs::metadata(&stock).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&stock, perms).unwrap();
    }

    let key = codex_hud_ops::codex_probe::probe_compatibility_key(Some(&stock), "").unwrap();
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
        .args(["run", "--stock-codex", stock.to_str().unwrap(), "--", "--fail"])
        .env("HOME", &home)
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(17));

    let policy = std::fs::read_to_string(home.join(".codex-hud/last_run_policy.txt")).unwrap();
    assert!(policy.contains("mode=error"));
    assert!(policy.contains("reason=stock codex exited with 17"));
}
