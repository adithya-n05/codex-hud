use codex_hud_cli::dispatch::{dispatch_command, CommandHandlers};
use codex_hud_cli::Command;
use std::cell::RefCell;

#[derive(Default)]
struct FakeHandlers {
    calls: RefCell<Vec<String>>,
}

impl CommandHandlers for FakeHandlers {
    fn run_status(&self) -> Result<String, String> {
        self.calls.borrow_mut().push("status".to_string());
        Ok("status-ok".to_string())
    }

    fn run_status_details(&self) -> Result<String, String> {
        self.calls.borrow_mut().push("status_details".to_string());
        Ok("status-details-ok".to_string())
    }

    fn run_install(&self) -> Result<String, String> {
        self.calls.borrow_mut().push("install".to_string());
        Ok("install-ok".to_string())
    }

    fn run_uninstall(&self) -> Result<String, String> {
        self.calls.borrow_mut().push("uninstall".to_string());
        Ok("uninstall-ok".to_string())
    }

    fn run_shim(
        &self,
        stock_codex_path: &str,
        passthrough_args: &[String],
    ) -> Result<String, String> {
        self.calls.borrow_mut().push(format!(
            "run:{}:{}",
            stock_codex_path,
            passthrough_args.join(",")
        ));
        Ok("run-ok".to_string())
    }
}

#[test]
fn dispatch_calls_status_orchestrator() {
    let handlers = FakeHandlers::default();
    let out = dispatch_command(&Command::Status, &handlers).unwrap();
    assert_eq!(out, "status-ok");
    assert_eq!(handlers.calls.borrow().as_slice(), ["status"]);
}
