#[derive(Debug, Clone, Default)]
pub struct StatusSnapshot {
    pub installed: bool,
    pub shim_present: bool,
    pub rc_block_present: bool,
    pub compatible: bool,
    pub codex_version: Option<String>,
    pub codex_sha256: Option<String>,
    pub managed_root: Option<String>,
    pub stock_codex_path: Option<String>,
    pub patch_mode: Option<String>,
    pub patch_reason: Option<String>,
    pub compat_key: Option<String>,
    pub compat_refresh_source: Option<String>,
}

fn redact_secret_like(value: &str) -> String {
    let lower = value.to_ascii_lowercase();
    if lower.contains("sk-") || lower.contains("token") || lower.contains("bearer") {
        "[redacted]".to_string()
    } else {
        value.to_string()
    }
}

pub fn render_status_summary(s: &StatusSnapshot) -> String {
    let installed = if s.installed { "yes" } else { "no" };
    let shim = if s.shim_present { "present" } else { "missing" };
    let compatibility = if s.compatible { "supported" } else { "unsupported" };
    let patch_mode = s.patch_mode.as_deref().unwrap_or("unknown");
    let compat_key = s.compat_key.as_deref().unwrap_or("unknown");
    let refresh_source = s.compat_refresh_source.as_deref().unwrap_or("unknown");
    format!(
        "codex-hud status\ninstalled: {installed}\nshim: {shim}\ncompatibility: {compatibility}\npatch_mode: {patch_mode}\ncompat_key: {compat_key}\nrefresh_source: {refresh_source}"
    )
}

pub fn render_status_details(s: &StatusSnapshot) -> String {
    let unsupported_notice = if s.compatible {
        "not_applicable"
    } else {
        "shown_once"
    };
    format!(
        "codex-hud status details\ninstalled: {}\nshim_present: {}\nrc_block_present: {}\ncompatible: {}\ncodex_version: {}\ncodex_sha256: {}\nmanaged_root: {}\nstock_codex_path: {}\npatch_mode: {}\npatch_reason: {}\ncompat_key: {}\ncompat_refresh_source: {}\nunsupported_notice: {}",
        s.installed,
        s.shim_present,
        s.rc_block_present,
        s.compatible,
        s.codex_version.clone().unwrap_or_else(|| "unknown".to_string()),
        s.codex_sha256
            .as_deref()
            .map(redact_secret_like)
            .unwrap_or_else(|| "unknown".to_string()),
        s.managed_root
            .as_deref()
            .map(redact_secret_like)
            .unwrap_or_else(|| "unknown".to_string()),
        s.stock_codex_path
            .as_deref()
            .map(redact_secret_like)
            .unwrap_or_else(|| "unknown".to_string()),
        s.patch_mode
            .as_deref()
            .map(redact_secret_like)
            .unwrap_or_else(|| "unknown".to_string()),
        s.patch_reason
            .as_deref()
            .map(redact_secret_like)
            .unwrap_or_else(|| "unknown".to_string()),
        s.compat_key
            .as_deref()
            .map(redact_secret_like)
            .unwrap_or_else(|| "unknown".to_string()),
        s.compat_refresh_source
            .as_deref()
            .map(redact_secret_like)
            .unwrap_or_else(|| "unknown".to_string()),
        unsupported_notice,
    )
}
