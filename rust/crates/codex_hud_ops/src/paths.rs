#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagedPaths {
    pub root: String,
    pub bin_dir: String,
}

pub fn managed_paths(home: &str, windows: bool) -> ManagedPaths {
    if windows {
        let root = format!("{}/.codex-hud", home.trim_end_matches('/'));
        let bin_dir = format!("{root}/bin");
        ManagedPaths { root, bin_dir }
    } else {
        let root = format!("{}/.codex-hud", home.trim_end_matches('/'));
        let bin_dir = format!("{root}/bin");
        ManagedPaths { root, bin_dir }
    }
}
