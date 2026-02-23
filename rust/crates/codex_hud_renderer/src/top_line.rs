use crate::RenderInput;

pub fn render_top_line(input: &RenderInput) -> String {
    let mut parts = Vec::new();
    if let Some(v) = input.repo.as_ref() {
        parts.push(format!("repo {v}"));
    }
    if let Some(v) = input.branch.as_ref() {
        parts.push(format!("branch {v}"));
    }
    parts.join(" | ")
}
