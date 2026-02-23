#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Install,
    Uninstall,
    Status,
    StatusDetails,
    Run {
        stock_codex_path: String,
        passthrough_args: Vec<String>,
    },
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
        "run" => {
            if collected.get(2).map(|s| s.as_str()) != Some("--stock-codex") {
                return Err("missing --stock-codex".to_string());
            }
            let stock_codex_path = collected
                .get(3)
                .cloned()
                .ok_or_else(|| "missing stock codex path".to_string())?;
            let passthrough_start = collected
                .iter()
                .position(|arg| arg == "--")
                .map(|idx| idx + 1)
                .unwrap_or(4);
            let passthrough_args = if passthrough_start < collected.len() {
                collected[passthrough_start..].to_vec()
            } else {
                Vec::new()
            };
            Ok(Command::Run {
                stock_codex_path,
                passthrough_args,
            })
        }
        _ => Err("unknown command".to_string()),
    }
}

pub fn automation_script_contract_supported() -> bool {
    false
}
