use crate::types::command::ParsedCommand;
use crate::types::errors::ShellError;
use crate::shell::state::ShellState;
pub fn run(cmd: &ParsedCommand, state: &mut ShellState) -> Result<(), ShellError> {
    let mut code = 0;
    if let Some(arg) = cmd.args.first() {
        if let Ok(parsed) = arg.parse::<i32>() {
            code = parsed;
        }
    }
    state.exit_requested = true;
    Err(ShellError::Exit(code))
}
