pub mod parse;

pub use parse::{parse_args, Command};
pub use parse::automation_script_contract_supported;

pub fn cli_ready() -> bool {
    true
}
