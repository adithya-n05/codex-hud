use codex_hud_domain::RuntimePolicy;

#[test]
fn telemetry_disabled_by_default() {
    let p = RuntimePolicy::default();
    assert!(!p.telemetry_enabled);
}
