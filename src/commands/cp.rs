use crate::types::command::ParsedCommand;
use crate::types::errors::ShellError;
use crate::shell::state::ShellState;
pub fn run(_cmd: &ParsedCommand, _state: &mut ShellState) -> Result<(), ShellError> {
    Ok(())
}
