#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimePolicy {
    pub telemetry_enabled: bool,
}

impl Default for RuntimePolicy {
    fn default() -> Self {
        Self {
            telemetry_enabled: false,
        }
    }
}
