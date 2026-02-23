pub mod patch_contract;
pub use patch_contract::patch_contract;
pub mod command;
pub use command::{parse_statusline_invocation, StatuslineAction};

pub fn statusline_ready() -> bool {
    true
}
