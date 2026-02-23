#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UiMode {
    Interactive,
    ReadOnlyWarning,
}

#[derive(Debug, Clone)]
pub enum UiLoadResult {
    Ok,
    ParseError(String),
}

#[derive(Debug, Clone)]
pub struct UiViewState {
    pub mode: UiMode,
    pub message: String,
}

pub fn map_load_result(input: UiLoadResult) -> UiViewState {
    match input {
        UiLoadResult::Ok => UiViewState {
            mode: UiMode::Interactive,
            message: String::new(),
        },
        UiLoadResult::ParseError(_) => UiViewState {
            mode: UiMode::ReadOnlyWarning,
            message: "Fix syntax in ~/.codex/config.toml and reopen /statusline".to_string(),
        },
    }
}
