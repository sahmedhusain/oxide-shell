pub mod echo;
pub mod cd;
pub mod ls;
pub mod pwd;
pub mod cat;
pub mod cp;
pub mod rm;
pub mod mv;
pub mod mkdir;
pub mod exit;
use crate::types::command::ParsedCommand;
use crate::types::errors::ShellError;
use crate::shell::state::ShellState;
pub fn dispatch_command(cmd: &ParsedCommand, state: &mut ShellState) -> Result<(), ShellError> {
    match cmd.name.as_str() {
        "echo" => echo::run(cmd, state),
        "cd" => cd::run(cmd, state),
        "ls" => ls::run(cmd, state),
        "pwd" => pwd::run(cmd, state),
        "cat" => cat::run(cmd, state),
        "cp" => cp::run(cmd, state),
        "rm" => rm::run(cmd, state),
        "mv" => mv::run(cmd, state),
        "mkdir" => mkdir::run(cmd, state),
        "exit" => exit::run(cmd, state),
        _ => Err(ShellError::CommandNotFound(cmd.name.clone())),
    }
}
