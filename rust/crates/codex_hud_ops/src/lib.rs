pub mod preflight;
pub use preflight::{preflight, PreflightInput};

pub fn ops_ready() -> bool {
    true
}
