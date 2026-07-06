use crate::types::command::ParsedCommand;
use crate::types::errors::ShellError;
use crate::shell::state::ShellState;
pub fn run(cmd: &ParsedCommand, _state: &mut ShellState) -> Result<(), ShellError> {
    if cmd.args.is_empty() {
        let err_msg = crate::constants::fallback::ERR_MISSING_OPERAND.replacen("{}", "mkdir", 1);
        return Err(ShellError::Generic(err_msg));
    }
    let mut has_failed = false;
    for dir in &cmd.args {
        if let Err(e) = std::fs::create_dir(dir) {
            has_failed = true;
            let err_msg = if e.kind() == std::io::ErrorKind::AlreadyExists {
                crate::constants::fallback::ERR_FILE_EXISTS.replacen("{}", dir, 1)
            } else {
                crate::constants::fallback::ERR_GENERIC
                    .replacen("{}", "mkdir", 1)
                    .replacen("{}", dir, 1)
                    .replacen("{}", &e.to_string(), 1)
            };
            eprintln!("{}", err_msg);
        }
    }
    if has_failed {
        Err(ShellError::Generic(String::new()))
    } else {
        Ok(())
    }
}
