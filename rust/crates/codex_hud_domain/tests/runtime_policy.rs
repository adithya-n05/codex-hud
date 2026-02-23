use codex_hud_domain::RuntimePolicy;

#[test]
fn telemetry_disabled_by_default() {
    let p = RuntimePolicy::default();
    assert!(!p.telemetry_enabled);
}

#[test]
fn diagnostics_mode_not_available_in_v1() {
    let p = RuntimePolicy::default();
    assert!(!p.diagnostics_mode_available);
}
