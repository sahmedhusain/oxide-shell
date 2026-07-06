# 0-shell 🐚

A minimalist, Unix-like shell implemented from scratch in Rust. 

The coolest part? It doesn't rely on or spawn external system binaries (like `/bin/ls` or `/bin/cp`). Instead, every command is built directly into the shell using Rust's standard library and Unix system APIs.

---

## What does it support?

Here are the commands built into the shell:

*   **`exit`**: Exits the shell. You can optionally provide an exit code (e.g., `exit 0` or `exit 1`).
*   **`pwd`**: Prints your current working directory path.
*   **`cd`**: Changes the active directory. If you run it without arguments (`cd`), it takes you straight to your `$HOME` directory.
*   **`echo`**: Prints your arguments back to you, separated by spaces. It handles both single (`'`) and double (`"`) quotes so you can pass spaces within a single argument.
*   **`mkdir`**: Creates one or more directories.
*   **`cat`**: Reads and prints file contents. If you run it without parameters, it falls back to streaming standard input.
*   **`cp`**: Copies files to a target path or directory.
*   **`mv`**: Moves or renames files and directories.
*   **`rm`**: Removes files. Add the `-r` flag to delete directories and their contents recursively.
*   **`ls`**: Lists directory contents alphabetically. It supports:
    *   `-a`: Includes hidden files (as well as `.` and `..`).
    *   `-l`: Long listing format showing permissions, link counts, UID/GID owner details, file sizes (or major/minor device numbers), and formatted modification times.
    *   `-F`: Appends file type indicators (`/` for directories, `*` for executables, `@` for symlinks).

---

## ⚡ Nice Touches (Bonus Features)

*   **Ctrl+C Grace**: Pressing `Ctrl+C` won't crash or kill your shell. It is ignored gracefully.
*   **Pretty Prompt**: Shows your current directory relative to your home folder (e.g., replacing `/Users/username` with `~`).
*   **Clean Outputs**: Column layouts in `ls -l` are dynamically aligned to keep the layout readable regardless of file sizes or username lengths.

---

## 📂 Project Architecture

We structured the project to follow scalable rust patterns:

```text
src/
├── main.rs                 # Initializes and boots the shell
├── constants/
│   ├── mod.rs              # Re-exports constants
│   └── fallback.rs         # Holds all user-facing fallback messages and templates
├── types/
│   ├── mod.rs              # Re-exports types
│   ├── command.rs          # Command representation structures
│   └── errors.rs           # Shell execution error enums
├── parser/
│   └── mod.rs              # Text command parser (interprets quotes & escapes)
├── shell/
│   ├── mod.rs              # REPL loop coordinator & prompt builder
│   └── state.rs            # Struct managing the active shell environment
└── commands/
    ├── mod.rs              # Directs inputs to correct modules
    ├── cd.rs
    ├── cat.rs
    ├── cp.rs
    ├── echo.rs
    ├── exit.rs
    ├── ls.rs
    ├── mkdir.rs
    ├── mv.rs
    ├── pwd.rs
    └── rm.rs
```

---

## 🚀 Quick Setup & Run

### 1. Build
Make sure you have Cargo and the Rust toolchain installed. Build the project using:

```bash
cargo build --release
```

### 2. Run
Start the shell using the compiled binary:

```bash
./target/release/src
```