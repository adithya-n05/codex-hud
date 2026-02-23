pub mod preflight;
pub use preflight::{preflight, PreflightInput};
pub mod shim;
pub use shim::write_codex_shim;

pub fn ops_ready() -> bool {
    true
}
