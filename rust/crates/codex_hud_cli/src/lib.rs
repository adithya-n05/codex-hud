pub mod dispatch;
pub mod parse;
pub mod runtime;

pub use dispatch::dispatch_command;
pub use parse::automation_script_contract_supported;
pub use parse::{parse_args, Command};

pub fn cli_ready() -> bool {
    true
}
