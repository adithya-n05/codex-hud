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
        "main_config"
    }
}

pub fn validate_statusline_command_name(command_name: &str) -> Result<(), String> {
    if command_name == "/statusline" {
        return Ok(());
    }
    if command_name == "/hud" {
        return Err("`/hud` is not available in v1. Use `/statusline`.".to_string());
    }
    Err("unknown statusline command".to_string())
}

pub fn parse_statusline_command<const N: usize>(
    command_name: &str,
    args: [&str; N],
) -> Result<StatuslineAction, String> {
    validate_statusline_command_name(command_name)?;
    parse_statusline_invocation(args)
}
