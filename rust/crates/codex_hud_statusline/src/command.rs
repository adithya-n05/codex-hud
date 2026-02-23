#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatuslineAction {
    OpenInteractiveUi,
}

pub fn parse_statusline_invocation<const N: usize>(
    args: [&str; N],
) -> Result<StatuslineAction, String> {
    let collected = args.into_iter().map(str::to_string).collect::<Vec<_>>();
    if !collected.is_empty() {
        return Err("/statusline does not accept arguments in v1".to_string());
    }

    if collected.is_empty() {
        Ok(StatuslineAction::OpenInteractiveUi)
    } else {
        Ok(StatuslineAction::OpenInteractiveUi)
    }
}

pub fn initial_screen(is_first_run: bool) -> &'static str {
    if is_first_run {
        "preset_wizard"
    } else {
        "preset_wizard"
    }
}
