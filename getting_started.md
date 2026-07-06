# Getting Started with 0-shell 🚀

This guide will help you build, run, and take a quick tour of our custom shell implementation.

---

## 🛠 Prerequisites

You'll need the Rust compiler and Cargo installed on your system. If you don't have them yet, head over to [rustup.rs](https://rustup.rs/) to set them up.

Verify your installation by running:
```bash
cargo --version
```

---

## 🏗 Building the Project

Compile the source code from the project root:

```bash
cargo build
```

This compiles a debug binary located at `./target/debug/src`. If you want to build an optimized release binary, you can run:

```bash
cargo build --release
```

---

## 🧭 Taking a Tour of the Shell

Here is a quick walkthrough to get you familiar with how the shell runs.

### 1. Launch the Shell
Run the compiled binary:
```bash
./target/debug/src
```
You'll see a prompt showing your current directory, ending with a `$`:
```text
~/Desktop/reboot01/projects/0-shell $ 
```

### 2. Navigate and Create Folders
Try creating a test workspace directory, cd into it, and verify with `pwd`:
```text
$ mkdir test_workspace
$ cd test_workspace
~/Desktop/reboot01/projects/0-shell/test_workspace $ pwd
/Users/sayed/Desktop/reboot01/projects/0-shell/test_workspace
```

### 3. Creating and Viewing Files
Use `echo` to print text, and redirect or copy it. (Since our shell doesn't implement redirection operators like `>`, you can test `echo` argument grouping with quotes):
```text
$ echo "Hello from our modular shell!"
Hello from our modular shell!
```
Let's copy or write some text to a file. In standard terminal, you can write a file, then use our shell to view it.
Outside the shell (in your main terminal), write a file inside `test_workspace/doc.txt`:
```bash
echo "Modular design works great" > test_workspace/doc.txt
```
Back inside our shell, print its content using `cat`:
```text
$ cat doc.txt
Modular design works great
```

### 4. Copying and Renaming Files
Create a backup directory, copy `doc.txt` into it, and then rename it:
```text
$ mkdir backup
$ cp doc.txt backup
$ ls backup
doc.txt
$ mv backup/doc.txt backup/doc_old.txt
$ ls backup
doc_old.txt
```

### 5. Listing Detailed Info
Run `ls -laF backup` to see hidden files, indicators, and long list alignment:
```text
$ ls -laF backup
total 8
drwxr-xr-x 3 501 staff  96 Jul  6 21:05 ./
drwxr-xr-x 4 501 staff 128 Jul  6 21:05 ../
-rw-r--r-- 1 501 staff  27 Jul  6 21:05 doc_old.txt
```

### 6. Exit
When you are done, run `exit` to hand control back to your computer terminal:
```text
$ exit
```

---

## 🧠 Behind the Scenes: Core Logic

### The Input Parser
When you type a command, the parser splits it by whitespace. However, if it encounters quotes (`"` or `'`), it halts whitespace splitting and groups everything within the quotes into a single argument. It also supports backslash `\` escapes so you can pass literal quotes or spaces.

### Dynamic Owner/Group Lookups
Instead of relying on heavy unsafe bindings to OS structures, `ls -l` parses the local `/etc/passwd` and `/etc/group` files directly to resolve numeric UIDs and GIDs into actual usernames and groups. If your current user isn't in those files, it falls back gracefully to formatting the numeric ID.
