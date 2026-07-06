pub enum ShellError {
    Exit(i32),
    CommandNotFound(String),
    Generic(String),
}
