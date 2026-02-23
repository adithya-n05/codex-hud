#[derive(Debug, Clone)]
pub struct PatchContract {
    pub command_symbol: &'static str,
    pub source_files: Vec<&'static str>,
}

pub fn patch_contract() -> PatchContract {
    PatchContract {
        command_symbol: "SlashCommand::Statusline",
        source_files: vec![
            "codex-rs/tui/src/slash_command.rs",
            "codex-rs/tui/src/chatwidget.rs",
            "codex-rs/tui/src/app.rs",
            "codex-rs/tui/src/bottom_pane/status_line_setup.rs",
        ],
    }
}
