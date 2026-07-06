use crate::types::command::ParsedCommand;
use crate::types::errors::ShellError;
use crate::shell::state::ShellState;
use std::path::Path;
pub fn run(cmd: &ParsedCommand, _state: &mut ShellState) -> Result<(), ShellError> {
    if cmd.args.is_empty() {
        let err_msg = crate::constants::fallback::ERR_MISSING_OPERAND.replacen("{}", "cp", 1);
        return Err(ShellError::Generic(err_msg));
    }
    if cmd.args.len() == 1 {
        let err_msg = crate::constants::fallback::ERR_MISSING_DEST_OPERAND
            .replacen("{}", "cp", 1)
            .replacen("{}", &cmd.args[0], 1);
        return Err(ShellError::Generic(err_msg));
    }
    let last_arg = cmd.args.last().unwrap();
    let dest_path = Path::new(last_arg);
    let dest_is_dir = dest_path.is_dir();
    let sources = &cmd.args[0..cmd.args.len() - 1];
    let mut has_failed = false;
    if dest_is_dir {
        for src_str in sources {
            let src_path = Path::new(src_str);
            let metadata = match std::fs::metadata(src_path) {
                Ok(m) => m,
                Err(_) => {
                    has_failed = true;
                    let err_msg = crate::constants::fallback::ERR_NO_SUCH_FILE_OR_DIR
                        .replacen("{}", "cp", 1)
                        .replacen("{}", src_str, 1);
                    eprintln!("{}", err_msg);
                    continue;
                }
            };
            if metadata.is_dir() {
                has_failed = true;
                let err_msg = crate::constants::fallback::ERR_CP_OMITTING_DIR.replacen("{}", src_str, 1);
                eprintln!("{}", err_msg);
                continue;
            }
            if let Some(file_name) = src_path.file_name() {
                let target = dest_path.join(file_name);
                if let Err(e) = std::fs::copy(src_path, &target) {
                    has_failed = true;
                    let err_msg = crate::constants::fallback::ERR_GENERIC
                        .replacen("{}", "cp", 1)
                        .replacen("{}", src_str, 1)
                        .replacen("{}", &e.to_string(), 1);
                    eprintln!("{}", err_msg);
                }
            }
        }
    } else {
        if sources.len() > 1 {
            return Err(ShellError::Generic(format!("cp: target '{}' is not a directory", last_arg)));
        }
        let src_str = &sources[0];
        let src_path = Path::new(src_str);
        let metadata = match std::fs::metadata(src_path) {
            Ok(m) => m,
            Err(_) => {
                let err_msg = crate::constants::fallback::ERR_NO_SUCH_FILE_OR_DIR
                    .replacen("{}", "cp", 1)
                    .replacen("{}", src_str, 1);
                return Err(ShellError::Generic(err_msg));
            }
        };
        if metadata.is_dir() {
            let err_msg = crate::constants::fallback::ERR_CP_OMITTING_DIR.replacen("{}", src_str, 1);
            return Err(ShellError::Generic(err_msg));
        }
        if let Err(e) = std::fs::copy(src_path, dest_path) {
            let err_msg = crate::constants::fallback::ERR_GENERIC
                .replacen("{}", "cp", 1)
                .replacen("{}", src_str, 1)
                .replacen("{}", &e.to_string(), 1);
            return Err(ShellError::Generic(err_msg));
        }
    }
    if has_failed {
        Err(ShellError::Generic(String::new()))
    } else {
        Ok(())
    }
}
