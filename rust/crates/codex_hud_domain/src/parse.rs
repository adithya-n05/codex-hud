pub fn reject_unknown_top_keys(value: &toml::Value) -> Result<(), String> {
    let allowed = ["native", "derived", "visual", "privacy", "format", "tool_counter", "preset"];
    if let Some(table) = value
        .get("tui")
        .and_then(|v| v.get("codex_hud"))
        .and_then(toml::Value::as_table)
    {
        for key in table.keys() {
            if !allowed.contains(&key.as_str()) {
                return Err("unknown key in tui.codex_hud".to_string());
            }
        }
    }
    Ok(())
}
