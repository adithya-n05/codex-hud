#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisualOptions {
    pub warn_percent: u8,
    pub critical_percent: u8,
    pub show_severity_symbols: bool,
    pub show_confidence_markers: bool,
    pub colorblind_mode: bool,
}

impl Default for VisualOptions {
    fn default() -> Self {
        Self {
            warn_percent: 70,
            critical_percent: 85,
            show_severity_symbols: false,
            show_confidence_markers: false,
            colorblind_mode: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrivacyOptions {
    pub redact_auth_identity: bool,
    pub persist_redaction_toggle: bool,
}

impl Default for PrivacyOptions {
    fn default() -> Self {
        Self {
            redact_auth_identity: false,
            persist_redaction_toggle: true,
        }
    }
}
