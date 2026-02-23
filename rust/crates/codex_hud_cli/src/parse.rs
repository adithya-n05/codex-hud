#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Install,
}

pub fn parse_args<I, S>(args: I) -> Result<Command, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let collected = args
        .into_iter()
        .map(|s| s.as_ref().to_string())
        .collect::<Vec<_>>();

    match collected.get(1).map(String::as_str) {
        Some("install") => Ok(Command::Install),
        Some(_) => Err("unsupported command".to_string()),
        None => Err("missing command".to_string()),
    }
}
