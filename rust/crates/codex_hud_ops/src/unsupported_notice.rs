use std::path::Path;

pub fn should_show_unsupported_notice(key: &str, state_path: &Path) -> Result<bool, String> {
    let existing = match std::fs::read_to_string(state_path) {
        Ok(v) => v,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(e) => return Err(e.to_string()),
    };

    let mut keys = existing
        .lines()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .collect::<Vec<_>>();

    if keys.iter().any(|v| v == key) {
        return Ok(false);
    }

    keys.push(key.to_string());
    let next = format!("{}\n", keys.join("\n"));
    std::fs::write(state_path, next).map_err(|e| e.to_string())?;
    Ok(true)
}

pub fn build_unsupported_notice_message(key: &str) -> String {
    format!(
        "codex-hud is not yet compatible with {key}. Run `codex-hud status details` to check compatibility support."
    )
}
