#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RuntimePolicy {
    pub telemetry_enabled: bool,
    pub diagnostics_mode_available: bool,
}
