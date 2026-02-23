pub mod preflight;
pub use preflight::{preflight, PreflightInput};
pub use preflight::preflight_guarded_install_root;
pub mod shim;
pub use shim::write_codex_shim;
pub mod shell_rc;
pub use shell_rc::ensure_rc_block;
pub use shell_rc::remove_rc_block;
pub mod compatibility;
pub mod install_message;
pub mod uninstall;
pub use uninstall::run_uninstall_with_rc;
pub use uninstall::reverse_patch_if_exact_state;
pub mod unsupported_notice;
pub mod status;
pub mod manifest_signing;

pub fn ops_ready() -> bool {
    true
}

pub fn uninstall_mode() -> &'static str {
    "non_interactive_no_prompt"
}
