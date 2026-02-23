use codex_hud_domain::NativeToggles;

#[test]
fn native_toggles_contains_all_required_fields() {
    let n = NativeToggles::default();
    let _ = n.model_name;
    let _ = n.model_with_reasoning;
    let _ = n.current_dir;
    let _ = n.project_root;
    let _ = n.git_branch;
    let _ = n.context_remaining;
    let _ = n.context_used;
    let _ = n.five_hour_limit;
    let _ = n.weekly_limit;
    let _ = n.codex_version;
    let _ = n.context_window_size;
    let _ = n.used_tokens;
    let _ = n.total_input_tokens;
    let _ = n.total_output_tokens;
    let _ = n.session_id;
}
