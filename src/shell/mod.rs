use std::io::{self, Write};
use crate::parser::parse_input;
use crate::commands::dispatch_command;
use crate::types::errors::ShellError;
use crate::shell::state::ShellState;
pub mod state;
unsafe extern "C" {
    fn signal(sig: i32, handler: usize) -> usize;
}
const SIGINT: i32 = 2;
const SIG_IGN: usize = 1;
pub fn start_shell() {
    unsafe {
        signal(SIGINT, SIG_IGN);
    }
    let mut state = ShellState::new();
    loop {
        print!("{}", get_prompt());
        if io::stdout().flush().is_err() {
            break;
        }
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                println!();
                break;
            }
            Ok(_) => {
                let trimmed = input.trim();
                if trimmed.is_empty() {
                    continue;
                }
                if let Some(cmd) = parse_input(trimmed) {
                    match dispatch_command(&cmd, &mut state) {
                        Ok(_) => {}
                        Err(ShellError::Exit(code)) => {
                            state.exit_code = code;
                            break;
                        }
                        Err(ShellError::CommandNotFound(name)) => {
                            eprintln!("{}", crate::constants::fallback::CMD_NOT_FOUND.replace("{}", &name));
                        }
                        Err(ShellError::Generic(msg)) => {
                            if !msg.is_empty() {
                                eprintln!("{}", msg);
                            }
                        }
                    }
                }
            }
            Err(_) => {
                break;
            }
        }
        if state.exit_requested {
            break;
        }
    }
    std::process::exit(state.exit_code);
}
fn get_prompt() -> String {
    let current_dir = std::env::current_dir()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| "/".to_string());
    let prompt_path = if let Ok(home) = std::env::var("HOME") {
        if current_dir == home {
            "~".to_string()
        } else if current_dir.starts_with(&format!("{}/", home)) {
            format!("~{}", &current_dir[home.len()..])
        } else {
            current_dir
        }
    } else {
        current_dir
    };
    format!("{} $ ", prompt_path)
}
