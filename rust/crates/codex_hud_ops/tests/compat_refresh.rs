use codex_hud_ops::compat_refresh::refresh_compat_bundle;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;
use tempfile::tempdir;

#[test]
fn refresh_downloads_manifest_and_public_key_into_compat_dir() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    let server = thread::spawn(move || {
        for _ in 0..2 {
            let (mut stream, _) = listener.accept().unwrap();
            let mut buf = [0u8; 4096];
            let n = stream.read(&mut buf).unwrap();
            let req = String::from_utf8_lossy(&buf[..n]);
            let first = req.lines().next().unwrap_or("");
            let (status, body) = if first.contains("GET /compat.json") {
                ("200 OK", "{\"schema_version\":1,\"supported_keys\":[],\"signature_hex\":\"00\"}\n")
            } else if first.contains("GET /public_key.hex") {
                ("200 OK", "00\n")
            } else {
                ("404 Not Found", "missing\n")
            };
            let resp = format!(
                "HTTP/1.1 {status}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            stream.write_all(resp.as_bytes()).unwrap();
        }
    });

    let tmp = tempdir().unwrap();
    let home = tmp.path();
    let base = format!("http://{addr}");
    refresh_compat_bundle(home, Some(&base)).unwrap();

    let compat = std::fs::read_to_string(home.join(".codex-hud/compat/compat.json")).unwrap();
    let pubkey = std::fs::read_to_string(home.join(".codex-hud/compat/public_key.hex")).unwrap();
    assert!(compat.contains("\"schema_version\":1"));
    assert_eq!(pubkey, "00\n");

    server.join().unwrap();
}

#[test]
fn refresh_trims_trailing_slash_in_release_base_url() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    let server = thread::spawn(move || {
        for _ in 0..2 {
            let (mut stream, _) = listener.accept().unwrap();
            let mut buf = [0u8; 4096];
            let n = stream.read(&mut buf).unwrap();
            let req = String::from_utf8_lossy(&buf[..n]);
            let first = req.lines().next().unwrap_or("");
            let (status, body) = if first.contains("GET /compat.json") {
                ("200 OK", "{\"schema_version\":1,\"supported_keys\":[],\"signature_hex\":\"00\"}\n")
            } else if first.contains("GET /public_key.hex") {
                ("200 OK", "00\n")
            } else {
                ("404 Not Found", "missing\n")
            };
            let resp = format!(
                "HTTP/1.1 {status}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            stream.write_all(resp.as_bytes()).unwrap();
        }
    });

    let tmp = tempdir().unwrap();
    let base_with_trailing_slash = format!("http://{addr}/");
    refresh_compat_bundle(tmp.path(), Some(&base_with_trailing_slash)).unwrap();

    server.join().unwrap();
}

#[test]
fn refresh_fails_when_manifest_download_returns_non_success_status() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let body = "missing\n";
        let resp = format!(
            "HTTP/1.1 404 Not Found\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        );
        stream.write_all(resp.as_bytes()).unwrap();
    });

    let tmp = tempdir().unwrap();
    let base = format!("http://{addr}");
    let err = refresh_compat_bundle(tmp.path(), Some(&base)).unwrap_err();

    assert!(err.contains("download failed:"));
    assert!(err.contains("/compat.json"));
    assert!(!tmp.path().join(".codex-hud/compat/compat.json").exists());
    assert!(!tmp.path().join(".codex-hud/compat/public_key.hex").exists());

    server.join().unwrap();
}

#[test]
fn refresh_fails_when_public_key_download_returns_non_success_status() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    let server = thread::spawn(move || {
        for i in 0..2 {
            let (mut stream, _) = listener.accept().unwrap();
            let body = if i == 0 {
                "{\"schema_version\":1,\"supported_keys\":[],\"signature_hex\":\"00\"}\n"
            } else {
                "missing\n"
            };
            let status = if i == 0 { "200 OK" } else { "404 Not Found" };
            let resp = format!(
                "HTTP/1.1 {status}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            stream.write_all(resp.as_bytes()).unwrap();
        }
    });

    let tmp = tempdir().unwrap();
    let base = format!("http://{addr}");
    let err = refresh_compat_bundle(tmp.path(), Some(&base)).unwrap_err();

    assert!(err.contains("/public_key.hex"));
    server.join().unwrap();
}

#[test]
fn refresh_fails_when_manifest_body_is_truncated() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut buf = [0u8; 4096];
        let _ = stream.read(&mut buf).unwrap();
        let body = "{\"schema_version\":1";
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len() + 20
        );
        stream.write_all(resp.as_bytes()).unwrap();
    });

    let tmp = tempdir().unwrap();
    let base = format!("http://{addr}");
    let err = refresh_compat_bundle(tmp.path(), Some(&base)).unwrap_err();

    assert!(!err.is_empty());
    server.join().unwrap();
}

fn spawn_success_server() -> (String, thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    let server = thread::spawn(move || {
        for _ in 0..2 {
            let (mut stream, _) = listener.accept().unwrap();
            let mut buf = [0u8; 4096];
            let n = stream.read(&mut buf).unwrap();
            let req = String::from_utf8_lossy(&buf[..n]);
            let first = req.lines().next().unwrap_or("");
            let (status, body) = if first.contains("GET /compat.json") {
                ("200 OK", "{\"schema_version\":1,\"supported_keys\":[],\"signature_hex\":\"00\"}\n")
            } else if first.contains("GET /public_key.hex") {
                ("200 OK", "00\n")
            } else {
                ("404 Not Found", "missing\n")
            };
            let resp = format!(
                "HTTP/1.1 {status}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            stream.write_all(resp.as_bytes()).unwrap();
        }
    });
    (format!("http://{addr}"), server)
}

#[test]
fn refresh_fails_when_compat_parent_path_is_blocked() {
    let tmp = tempdir().unwrap();
    let home = tmp.path();
    std::fs::create_dir_all(home).unwrap();
    std::fs::write(home.join(".codex-hud"), "blocked").unwrap();

    let err = refresh_compat_bundle(home, Some("http://127.0.0.1:1")).unwrap_err();
    assert!(!err.is_empty());
}

#[test]
fn refresh_fails_when_writing_manifest_if_target_is_directory() {
    let (base, server) = spawn_success_server();
    let tmp = tempdir().unwrap();
    let compat_dir = tmp.path().join(".codex-hud/compat");
    std::fs::create_dir_all(compat_dir.join("compat.json")).unwrap();

    let err = refresh_compat_bundle(tmp.path(), Some(&base)).unwrap_err();
    assert!(!err.is_empty());
    server.join().unwrap();
}

#[test]
fn refresh_fails_when_writing_public_key_if_target_is_directory() {
    let (base, server) = spawn_success_server();
    let tmp = tempdir().unwrap();
    let compat_dir = tmp.path().join(".codex-hud/compat");
    std::fs::create_dir_all(&compat_dir).unwrap();
    std::fs::create_dir_all(compat_dir.join("public_key.hex")).unwrap();

    let err = refresh_compat_bundle(tmp.path(), Some(&base)).unwrap_err();
    assert!(err.contains("public_key.hex") || !err.is_empty());
    server.join().unwrap();
}
