use crate::Command;

pub trait CommandHandlers {
    fn run_status(&self) -> Result<String, String>;
    fn run_status_details(&self) -> Result<String, String>;
    fn run_install(&self) -> Result<String, String>;
    fn run_uninstall(&self) -> Result<String, String>;
    fn run_shim(&self, stock_codex_path: &str, passthrough_args: &[String]) -> Result<String, String>;
}

pub fn dispatch_command<H: CommandHandlers>(cmd: &Command, handlers: &H) -> Result<String, String> {
    match cmd {
        Command::Status => handlers.run_status(),
        Command::StatusDetails => handlers.run_status_details(),
        Command::Install => handlers.run_install(),
        Command::Uninstall => handlers.run_uninstall(),
        Command::Run {
            stock_codex_path,
            passthrough_args,
        } => handlers.run_shim(stock_codex_path, passthrough_args),
    }
}
