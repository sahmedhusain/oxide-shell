use crate::types::command::ParsedCommand;
use crate::types::errors::ShellError;
use crate::shell::state::ShellState;
pub fn run(cmd: &ParsedCommand, _state: &mut ShellState) -> Result<(), ShellError> {
    println!("{}", cmd.args.join(" "));
    Ok(())
}
