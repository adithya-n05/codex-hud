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
