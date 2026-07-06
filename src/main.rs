pub mod constants;
pub mod types;
pub mod parser;
pub mod shell;
pub mod commands;
fn main() {
    shell::start_shell();
}
