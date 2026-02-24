use std::path::Path;
use std::time::Duration;

const DEFAULT_RELEASE_BASE_URL: &str =
    "https://github.com/personal/codex-hud/releases/latest/download";

fn fetch_text(client: &reqwest::blocking::Client, url: &str) -> Result<String, String> {
    let response = client.get(url).send().map_err(|e| e.to_string())?;
    if !response.status().is_success() {
        return Err(format!("download failed: {url} ({})", response.status()));
    }
    response.text().map_err(|e| e.to_string())
}

pub fn refresh_compat_bundle(home: &Path, release_base_url: Option<&str>) -> Result<(), String> {
    let base = release_base_url.unwrap_or(DEFAULT_RELEASE_BASE_URL).trim_end_matches('/');
    let compat_dir = home.join(".codex-hud").join("compat");
    std::fs::create_dir_all(&compat_dir).map_err(|e| e.to_string())?;

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .map_err(|e| e.to_string())?;

    let manifest_url = format!("{base}/compat.json");
    let pubkey_url = format!("{base}/public_key.hex");
    let manifest = fetch_text(&client, &manifest_url)?;
    let pubkey = fetch_text(&client, &pubkey_url)?;

    std::fs::write(compat_dir.join("compat.json"), manifest).map_err(|e| e.to_string())?;
    std::fs::write(compat_dir.join("public_key.hex"), pubkey).map_err(|e| e.to_string())?;
    Ok(())
}
