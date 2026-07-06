use crate::types::command::ParsedCommand;
use crate::types::errors::ShellError;
use crate::shell::state::ShellState;
pub fn run(cmd: &ParsedCommand, _state: &mut ShellState) -> Result<(), ShellError> {
    if cmd.args.len() > 1 {
        return Err(ShellError::Generic(crate::constants::fallback::ERR_CD_TOO_MANY_ARGS.to_string()));
    }
    let target_dir = if cmd.args.is_empty() {
        match std::env::var("HOME") {
            Ok(home) => home,
            Err(_) => return Err(ShellError::Generic(crate::constants::fallback::ERR_CD_HOME_NOT_SET.to_string())),
        }
    } else {
        cmd.args[0].clone()
    };
    if let Err(_) = std::env::set_current_dir(&target_dir) {
        let err_msg = crate::constants::fallback::ERR_NO_SUCH_FILE_OR_DIR
            .replacen("{}", "cd", 1)
            .replacen("{}", &target_dir, 1);
        return Err(ShellError::Generic(err_msg));
    }
    Ok(())
}
