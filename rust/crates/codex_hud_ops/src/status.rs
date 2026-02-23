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
