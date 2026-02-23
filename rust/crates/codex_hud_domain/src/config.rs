use crate::{parse::reject_unknown_top_keys, validate::validate_threshold_range};

pub type HudConfig = crate::HudConfig;

pub fn parse_hud_config(src: &str) -> Result<HudConfig, String> {
    let value = src
        .parse::<toml::Value>()
        .map_err(|e| format!("syntax error: {e}"))?;

    if value
        .get("project")
        .and_then(|v| v.get("codex_hud"))
        .is_some()
    {
        return Err("project-level codex_hud config is not supported".to_string());
    }

    reject_unknown_top_keys(&value)?;

    let warn = value
        .get("tui")
        .and_then(|v| v.get("codex_hud"))
        .and_then(|v| v.get("visual"))
        .and_then(|v| v.get("warn_percent"))
        .and_then(toml::Value::as_integer)
        .unwrap_or(70);

    let critical = value
        .get("tui")
        .and_then(|v| v.get("codex_hud"))
        .and_then(|v| v.get("visual"))
        .and_then(|v| v.get("critical_percent"))
        .and_then(toml::Value::as_integer)
        .unwrap_or(85);

    validate_threshold_range(warn, critical)?;
    if warn > critical {
        return Err("warn_percent must be <= critical_percent".to_string());
    }

    let mut cfg = HudConfig::default();
    if let Some(table) = value
        .get("tui")
        .and_then(|v| v.get("codex_hud"))
        .and_then(|v| v.get("native"))
        .and_then(toml::Value::as_table)
    {
        cfg.native.model_name = table
            .get("model_name")
            .and_then(toml::Value::as_bool)
            .unwrap_or(cfg.native.model_name);
        cfg.native.model_with_reasoning = table
            .get("model_with_reasoning")
            .and_then(toml::Value::as_bool)
            .unwrap_or(cfg.native.model_with_reasoning);
    }
    Ok(cfg)
}
