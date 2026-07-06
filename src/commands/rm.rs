use crate::types::command::ParsedCommand;
use crate::types::errors::ShellError;
use crate::shell::state::ShellState;
use std::path::Path;
pub fn run(cmd: &ParsedCommand, _state: &mut ShellState) -> Result<(), ShellError> {
    let mut recursive = false;
    let mut targets = Vec::new();
    for arg in &cmd.args {
        if arg == "-r" || arg == "-R" || arg == "--recursive" {
            recursive = true;
        } else if arg.starts_with('-') && arg.len() > 1 {
            for c in arg.chars().skip(1) {
                if c == 'r' || c == 'R' {
                    recursive = true;
                }
            }
        } else {
            targets.push(arg);
        }
    }
    if targets.is_empty() {
        let err_msg = crate::constants::fallback::ERR_MISSING_OPERAND.replacen("{}", "rm", 1);
        return Err(ShellError::Generic(err_msg));
    }
    let mut has_failed = false;
    for target in targets {
        let path = Path::new(target);
        let metadata = match std::fs::symlink_metadata(path) {
            Ok(m) => m,
            Err(_) => {
                has_failed = true;
                let err_msg = crate::constants::fallback::ERR_NO_SUCH_FILE_OR_DIR
                    .replacen("{}", "rm", 1)
                    .replacen("{}", target, 1);
                eprintln!("{}", err_msg);
                continue;
            }
        };
        if metadata.is_dir() {
            if recursive {
                if let Err(e) = std::fs::remove_dir_all(path) {
                    has_failed = true;
                    let err_msg = crate::constants::fallback::ERR_GENERIC
                        .replacen("{}", "rm", 1)
                        .replacen("{}", target, 1)
                        .replacen("{}", &e.to_string(), 1);
                    eprintln!("{}", err_msg);
                }
            } else {
                has_failed = true;
                let err_msg = crate::constants::fallback::ERR_IS_A_DIRECTORY
                    .replacen("{}", "rm", 1)
                    .replacen("{}", target, 1);
                eprintln!("{}", err_msg);
            }
        } else {
            if let Err(e) = std::fs::remove_file(path) {
                has_failed = true;
                let err_msg = crate::constants::fallback::ERR_GENERIC
                    .replacen("{}", "rm", 1)
                    .replacen("{}", target, 1)
                    .replacen("{}", &e.to_string(), 1);
                eprintln!("{}", err_msg);
            }
        }
    }
    if has_failed {
        Err(ShellError::Generic(String::new()))
    } else {
        Ok(())
    }
}
