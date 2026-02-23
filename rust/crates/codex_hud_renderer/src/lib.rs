#[derive(Debug, Clone, Default)]
pub struct RenderInput {
    pub repo: Option<String>,
    pub branch: Option<String>,
    pub permission: Option<String>,
    pub auth_label: Option<String>,
    pub provider_label: Option<String>,
    pub model_label: Option<String>,
    pub tool_count: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderOutput {
    pub logical_lines: Vec<String>,
    pub wrapped_lines: Vec<String>,
}

pub fn renderer_ready() -> bool {
    true
}

pub fn render_hud(_input: &RenderInput) -> RenderOutput {
    let logical_lines = vec![String::new(), String::new()];
    RenderOutput {
        logical_lines: logical_lines.clone(),
        wrapped_lines: logical_lines,
    }
}
