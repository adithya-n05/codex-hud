use codex_hud_domain::DerivedToggles;

#[test]
fn derived_toggles_contains_all_required_fields() {
    let d = DerivedToggles::default();
    let _ = d.permission_chip;
    let _ = d.auth_chip;
    let _ = d.provider_chip;
    let _ = d.context_bar;
    let _ = d.five_hour_bar;
    let _ = d.weekly_bar;
    let _ = d.tool_counter;
    let _ = d.failure_count;
    let _ = d.activity_summary;
    let _ = d.git_dirty;
    let _ = d.git_ahead_behind;
    let _ = d.git_file_stats;
    let _ = d.duration_metric;
    let _ = d.speed_metric;
    let _ = d.plan_progress;
    let _ = d.config_count;
}
