use codex_hud_cli::automation_script_contract_supported;

#[test]
fn automation_script_contract_is_disabled_in_v1() {
    assert!(!automation_script_contract_supported());
}
