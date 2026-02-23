pub mod patch_contract;
pub use patch_contract::patch_contract;
pub mod command;
pub use command::{parse_statusline_invocation, StatuslineAction};
pub use command::initial_screen;
pub use command::{parse_statusline_command, validate_statusline_command_name};
pub mod ui_state;
pub use ui_state::{ConfigUiState, Key};
pub use ui_state::ConfigUiEvent;
pub use ui_state::config_action_labels;
pub mod load_state;
pub use load_state::{map_load_result, UiLoadResult, UiMode};
pub mod integration_constraints;

pub fn statusline_ready() -> bool {
    true
}
