pub mod parse;

pub use parse::{parse_args, Command};

pub fn cli_ready() -> bool {
    true
}
