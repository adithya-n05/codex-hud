#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimePolicy {
    pub telemetry_enabled: bool,
    pub diagnostics_mode_available: bool,
}

impl Default for RuntimePolicy {
    fn default() -> Self {
        Self {
            telemetry_enabled: false,
            diagnostics_mode_available: false,
        }
    }
}
