#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatuslineAction {
    OpenInteractiveUi,
}

pub fn parse_statusline_invocation<const N: usize>(
    args: [&str; N],
) -> Result<StatuslineAction, String> {
    let collected = args.into_iter().map(str::to_string).collect::<Vec<_>>();

    if collected.is_empty() {
        Ok(StatuslineAction::OpenInteractiveUi)
    } else {
        Ok(StatuslineAction::OpenInteractiveUi)
    }
}
