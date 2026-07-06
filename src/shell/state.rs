pub struct ShellState {
    pub exit_requested: bool,
    pub exit_code: i32,
}
impl ShellState {
    pub fn new() -> Self {
        ShellState {
            exit_requested: false,
            exit_code: 0,
        }
    }
}
