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
