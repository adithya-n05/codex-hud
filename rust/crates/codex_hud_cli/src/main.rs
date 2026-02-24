use codex_hud_cli::runtime::RealHandlers;
use codex_hud_cli::{dispatch_command, parse_args};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = match parse_args(args) {
        Ok(command) => command,
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(2);
        }
    };

    let handlers = RealHandlers;
    match dispatch_command(&cmd, &handlers) {
        Ok(out) => {
            println!("{out}");
        }
        Err(err) => {
            if let Some(code) = err
                .strip_prefix("stock codex exited with ")
                .and_then(|value| value.parse::<i32>().ok())
            {
                std::process::exit(code);
            }
            eprintln!("{err}");
            std::process::exit(1);
        }
    }
}
