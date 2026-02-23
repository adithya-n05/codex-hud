pub mod parse;
pub mod dispatch;

pub use parse::{parse_args, Command};
pub use parse::automation_script_contract_supported;
pub use dispatch::dispatch_command;

pub fn cli_ready() -> bool {
    true
}
