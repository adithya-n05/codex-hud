use std::path::Path;

const HUD_ITEMS: &[&str] = &[
    "model-with-reasoning",
    "git-branch",
    "permission-mode",
    "auth-chip",
    "tool-calls",
    "ctx-bar",
    "five-hour-bar",
    "weekly-bar",
];

const LEGACY_HUD_ITEMS: &[&str] = &[
    "model-with-reasoning",
    "git-branch",
    "permission-mode",
    "auth-chip",
    "tool-calls",
    "ctx-bar",
    "five-hour-bar",
    "weekly-bar",
    "context-remaining",
    "context-used",
    "five-hour-limit",
    "weekly-limit",
    "current-dir",
    "project-root",
    "codex-version",
    "context-window-size",
    "used-tokens",
    "total-input-tokens",
    "total-output-tokens",
    "session-id",
    "model-name",
];

const HUD_SIGNALS: &[&str] = &[
    "permission-mode",
    "auth-chip",
    "tool-calls",
    "ctx-bar",
    "five-hour-bar",
    "weekly-bar",
];

fn hud_items_value() -> toml::Value {
    toml::Value::Array(
        HUD_ITEMS
            .iter()
            .map(|item| toml::Value::String((*item).to_string()))
            .collect::<Vec<_>>(),
    )
}

fn has_hud_signal(array: &[toml::Value]) -> bool {
    array.iter().any(|value| {
        value
            .as_str()
            .map(|item| HUD_SIGNALS.contains(&item))
            .unwrap_or(false)
    })
}

fn array_matches_exact_items(array: &[toml::Value], expected: &[&str]) -> bool {
    if array.len() != expected.len() {
        return false;
    }
    array
        .iter()
        .zip(expected.iter())
        .all(|(value, expected_item)| value.as_str() == Some(*expected_item))
}

pub fn ensure_hud_statusline_config(home: &Path) -> Result<bool, String> {
    let config_path = home.join(".codex/config.toml");
    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    if !config_path.exists() {
        let mut root = toml::value::Table::new();
        let mut tui = toml::value::Table::new();
        tui.insert("status_line".to_string(), hud_items_value());
        root.insert("tui".to_string(), toml::Value::Table(tui));
        let out = toml::to_string_pretty(&toml::Value::Table(root)).map_err(|e| e.to_string())?;
        std::fs::write(&config_path, out).map_err(|e| e.to_string())?;
        return Ok(true);
    }

    let raw = std::fs::read_to_string(&config_path).map_err(|e| e.to_string())?;
    let mut root: toml::Value = match raw.parse::<toml::Value>() {
        Ok(v) => v,
        Err(_) => return Ok(false),
    };

    let Some(root_table) = root.as_table_mut() else {
        return Ok(false);
    };
    let tui = root_table
        .entry("tui")
        .or_insert_with(|| toml::Value::Table(toml::value::Table::new()));
    let Some(tui_table) = tui.as_table_mut() else {
        return Ok(false);
    };

    if let Some(existing) = tui_table.get("status_line").and_then(|v| v.as_array()) {
        if has_hud_signal(existing) {
            if array_matches_exact_items(existing, LEGACY_HUD_ITEMS) {
                tui_table.insert("status_line".to_string(), hud_items_value());
                let out = toml::to_string_pretty(&root).map_err(|e| e.to_string())?;
                std::fs::write(&config_path, out).map_err(|e| e.to_string())?;
                return Ok(true);
            }
            return Ok(false);
        }
    }

    tui_table.insert("status_line".to_string(), hud_items_value());
    let out = toml::to_string_pretty(&root).map_err(|e| e.to_string())?;
    std::fs::write(&config_path, out).map_err(|e| e.to_string())?;
    Ok(true)
}
