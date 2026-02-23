use codex_hud_domain::{count_tool_events, ToolCounterEvent, ToolCounterOptions};

#[test]
fn tool_counter_counts_default_families_in_session_scope() {
    let events = vec![
        ToolCounterEvent::CoreCall,
        ToolCounterEvent::McpCall,
        ToolCounterEvent::WebCall,
        ToolCounterEvent::PatchApply,
        ToolCounterEvent::Failure,
        ToolCounterEvent::Other,
    ];

    let defaults = ToolCounterOptions::default();
    assert_eq!(count_tool_events(&events, &defaults), 5);

    let non_session_scope = ToolCounterOptions {
        scope: "current_turn".to_string(),
        ..ToolCounterOptions::default()
    };
    assert_eq!(count_tool_events(&events, &non_session_scope), 0);
}
