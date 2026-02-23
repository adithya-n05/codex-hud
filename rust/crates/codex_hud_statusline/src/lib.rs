pub mod patch_contract;
pub use patch_contract::patch_contract;
pub mod command;
pub use command::{parse_statusline_invocation, StatuslineAction};
pub mod ui_state;
pub use ui_state::{ConfigUiState, Key};
pub use ui_state::ConfigUiEvent;

pub fn statusline_ready() -> bool {
    true
}
