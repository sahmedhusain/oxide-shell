use crate::types::command::ParsedCommand;
use crate::types::errors::ShellError;
use crate::shell::state::ShellState;
use std::io::{self, Read, Write};
pub fn run(cmd: &ParsedCommand, _state: &mut ShellState) -> Result<(), ShellError> {
    if cmd.args.is_empty() {
        let mut buffer = [0; 4096];
        let mut stdin = io::stdin();
        let mut stdout = io::stdout();
        loop {
            match stdin.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {
                    if stdout.write_all(&buffer[..n]).is_err() {
                        break;
                    }
                    let _ = stdout.flush();
                }
                Err(_) => break,
            }
        }
        Ok(())
    } else {
        let mut has_failed = false;
        for path in &cmd.args {
            let metadata = match std::fs::metadata(path) {
                Ok(m) => m,
                Err(e) => {
                    has_failed = true;
                    let err_msg = if e.kind() == io::ErrorKind::NotFound {
                        crate::constants::fallback::ERR_NO_SUCH_FILE_OR_DIR
                            .replacen("{}", "cat", 1)
                            .replacen("{}", path, 1)
                    } else {
                        crate::constants::fallback::ERR_GENERIC
                            .replacen("{}", "cat", 1)
                            .replacen("{}", path, 1)
                            .replacen("{}", &e.to_string(), 1)
                    };
                    eprintln!("{}", err_msg);
                    continue;
                }
            };
            if metadata.is_dir() {
                has_failed = true;
                let err_msg = crate::constants::fallback::ERR_IS_A_DIRECTORY
                    .replacen("{}", "cat", 1)
                    .replacen("{}", path, 1);
                eprintln!("{}", err_msg);
                continue;
            }
            let mut file = match std::fs::File::open(path) {
                Ok(f) => f,
                Err(e) => {
                    has_failed = true;
                    let err_msg = crate::constants::fallback::ERR_GENERIC
                        .replacen("{}", "cat", 1)
                        .replacen("{}", path, 1)
                        .replacen("{}", &e.to_string(), 1);
                    eprintln!("{}", err_msg);
                    continue;
                }
            };
            let mut buffer = [0; 4096];
            let mut stdout = io::stdout();
            loop {
                match file.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(n) => {
                        if stdout.write_all(&buffer[..n]).is_err() {
                            break;
                        }
                    }
                    Err(e) => {
                        has_failed = true;
                        let err_msg = crate::constants::fallback::ERR_GENERIC
                            .replacen("{}", "cat", 1)
                            .replacen("{}", path, 1)
                            .replacen("{}", &e.to_string(), 1);
                        eprintln!("{}", err_msg);
                        break;
                    }
                }
            }
            let _ = stdout.flush();
        }
        if has_failed {
            Err(ShellError::Generic(String::new()))
        } else {
            Ok(())
        }
    }
}
