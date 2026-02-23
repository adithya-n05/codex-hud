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
}

pub fn render_status_summary(s: &StatusSnapshot) -> String {
    let installed = if s.installed { "yes" } else { "no" };
    let shim = if s.shim_present { "present" } else { "missing" };
    let compatibility = if s.compatible { "supported" } else { "unsupported" };
    format!(
        "codex-hud status\ninstalled: {installed}\nshim: {shim}\ncompatibility: {compatibility}"
    )
}

pub fn render_status_details(s: &StatusSnapshot) -> String {
    let unsupported_notice = if s.compatible {
        "not_applicable"
    } else {
        "shown_once"
    };
    format!(
        "codex-hud status details\ninstalled: {}\nshim_present: {}\nrc_block_present: {}\ncompatible: {}\ncodex_version: {}\ncodex_sha256: {}\nmanaged_root: {}\nstock_codex_path: {}\nunsupported_notice: {}",
        s.installed,
        s.shim_present,
        s.rc_block_present,
        s.compatible,
        s.codex_version.clone().unwrap_or_else(|| "unknown".to_string()),
        s.codex_sha256.clone().unwrap_or_else(|| "unknown".to_string()),
        s.managed_root.clone().unwrap_or_else(|| "unknown".to_string()),
        s.stock_codex_path.clone().unwrap_or_else(|| "unknown".to_string()),
        unsupported_notice,
    )
}
