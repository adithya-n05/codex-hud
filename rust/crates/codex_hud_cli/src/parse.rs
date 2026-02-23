#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Install,
    Uninstall,
    Status,
    StatusDetails,
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

    if collected.len() < 2 {
        return Err("missing command".to_string());
    }

    match collected[1].as_str() {
        "install" => Ok(Command::Install),
        "uninstall" => Ok(Command::Uninstall),
        "status" => {
            if collected.get(2).map(|s| s.as_str()) == Some("details") {
                Ok(Command::StatusDetails)
            } else {
                Ok(Command::Status)
            }
        }
        _ => Err("unknown command".to_string()),
    }
}
