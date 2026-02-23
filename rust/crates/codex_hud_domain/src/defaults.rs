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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormatOptions {
    pub context_mode: String,
    pub usage_mode: String,
    pub path_depth_mode: String,
}

impl Default for FormatOptions {
    fn default() -> Self {
        Self {
            context_mode: "percent".to_string(),
            usage_mode: "bars".to_string(),
            path_depth_mode: "compact".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolCounterOptions {
    pub scope: String,
    pub include_core: bool,
    pub include_mcp: bool,
    pub include_web: bool,
    pub include_patch: bool,
    pub include_failures: bool,
}

impl Default for ToolCounterOptions {
    fn default() -> Self {
        Self {
            scope: "session_total".to_string(),
            include_core: true,
            include_mcp: true,
            include_web: true,
            include_patch: true,
            include_failures: true,
        }
    }
}
