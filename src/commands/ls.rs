use crate::types::command::ParsedCommand;
use crate::types::errors::ShellError;
use crate::shell::state::ShellState;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::os::unix::fs::MetadataExt;
use std::path::Path;
extern "C" {
    fn ctime(timep: *const i64) -> *mut std::os::raw::c_char;
}
#[cfg(target_os = "macos")]
fn major_minor(rdev: u64) -> (u32, u32) {
    (((rdev >> 24) & 0xff) as u32, (rdev & 0xffffff) as u32)
}
#[cfg(not(target_os = "macos"))]
fn major_minor(rdev: u64) -> (u32, u32) {
    ((((rdev >> 8) & 0xfff) | ((rdev >> 32) & !0xfff)) as u32,
     ((rdev & 0xff) | ((rdev >> 12) & !0xff)) as u32)
}
fn get_users() -> HashMap<u32, String> {
    let mut map = HashMap::new();
    if let Ok(file) = File::open("/etc/passwd") {
        let reader = BufReader::new(file);
        for line_result in reader.lines() {
            if let Ok(line) = line_result {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() >= 3 {
                    if let Ok(uid) = parts[2].parse::<u32>() {
                        map.insert(uid, parts[0].to_string());
                    }
                }
            }
        }
    }
    map
}
fn get_groups() -> HashMap<u32, String> {
    let mut map = HashMap::new();
    if let Ok(file) = File::open("/etc/group") {
        let reader = BufReader::new(file);
        for line_result in reader.lines() {
            if let Ok(line) = line_result {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() >= 3 {
                    if let Ok(gid) = parts[2].parse::<u32>() {
                        map.insert(gid, parts[0].to_string());
                    }
                }
            }
        }
    }
    map
}
fn format_mode(mode: u32, is_dir: bool, is_symlink: bool) -> String {
    let mut chars = vec!['-'; 10];
    if is_dir {
        chars[0] = 'd';
    } else if is_symlink {
        chars[0] = 'l';
    } else {
        let file_type_bits = mode & 0o170000;
        if file_type_bits == 0o060000 {
            chars[0] = 'b';
        } else if file_type_bits == 0o020000 {
            chars[0] = 'c';
        } else if file_type_bits == 0o010000 {
            chars[0] = 'p';
        } else if file_type_bits == 0o140000 {
            chars[0] = 's';
        }
    }
    if mode & 0o400 != 0 { chars[1] = 'r'; }
    if mode & 0o200 != 0 { chars[2] = 'w'; }
    if mode & 0o100 != 0 { chars[3] = 'x'; }
    if mode & 0o040 != 0 { chars[4] = 'r'; }
    if mode & 0o020 != 0 { chars[5] = 'w'; }
    if mode & 0o010 != 0 { chars[6] = 'x'; }
    if mode & 0o004 != 0 { chars[7] = 'r'; }
    if mode & 0o002 != 0 { chars[8] = 'w'; }
    if mode & 0o001 != 0 { chars[9] = 'x'; }
    chars.into_iter().collect()
}
fn format_mtime(mtime: i64) -> String {
    unsafe {
        let ptr = ctime(&mtime);
        if !ptr.is_null() {
            let c_str = std::ffi::CStr::from_ptr(ptr);
            let s = c_str.to_string_lossy();
            let parts: Vec<&str> = s.split_whitespace().collect();
            if parts.len() >= 5 {
                let month = parts[1];
                let day = parts[2];
                let time_parts: Vec<&str> = parts[3].split(':').collect();
                if time_parts.len() >= 2 {
                    return format!("{} {:>2} {}:{}", month, day, time_parts[0], time_parts[1]);
                }
            }
        }
    }
    "Jan  1  1970".to_string()
}
struct EntryInfo {
    name: String,
    metadata: std::fs::Metadata,
}
pub fn run(cmd: &ParsedCommand, _state: &mut ShellState) -> Result<(), ShellError> {
    let mut all = false;
    let mut long = false;
    let mut classify = false;
    let mut targets = Vec::new();
    for arg in &cmd.args {
        if arg.starts_with('-') && arg.len() > 1 {
            for c in arg.chars().skip(1) {
                match c {
                    'a' => all = true,
                    'l' => long = true,
                    'F' => classify = true,
                    _ => return Err(ShellError::Generic(format!("ls: invalid option -- '{}'", c))),
                }
            }
        } else {
            targets.push(arg.as_str());
        }
    }
    if targets.is_empty() {
        targets.push(".");
    }
    let users = get_users();
    let groups = get_groups();
    let multiple = targets.len() > 1;
    let mut has_failed = false;
    for (idx, target) in targets.iter().enumerate() {
        let path = Path::new(target);
        let metadata = match std::fs::symlink_metadata(path) {
            Ok(m) => m,
            Err(_) => {
                has_failed = true;
                let err_msg = crate::constants::fallback::ERR_NO_SUCH_FILE_OR_DIR
                    .replacen("{}", "ls", 1)
                    .replacen("{}", target, 1);
                eprintln!("{}", err_msg);
                continue;
            }
        };
        if multiple {
            if idx > 0 {
                println!();
            }
            println!("{}:", target);
        }
        if metadata.is_dir() {
            let mut entries = Vec::new();
            if all {
                if let Ok(m) = std::fs::metadata(path) {
                    entries.push(EntryInfo { name: ".".to_string(), metadata: m });
                }
                if let Ok(m) = std::fs::metadata(path.parent().unwrap_or(path)) {
                    entries.push(EntryInfo { name: "..".to_string(), metadata: m });
                }
            }
            if let Ok(dir_entries) = std::fs::read_dir(path) {
                for entry_res in dir_entries {
                    if let Ok(entry) = entry_res {
                        let name = entry.file_name().to_string_lossy().into_owned();
                        if !all && name.starts_with('.') {
                            continue;
                        }
                        if let Ok(m) = entry.metadata() {
                            entries.push(EntryInfo { name, metadata: m });
                        }
                    }
                }
            }
            entries.sort_by(|a, b| a.name.cmp(&b.name));
            if entries.is_empty() {
                continue;
            }
            if long {
                let mut total_blocks = 0;
                for entry in &entries {
                    total_blocks += entry.metadata.blocks();
                }
                println!("total {}", total_blocks);
                let mut link_width = 0;
                let mut owner_width = 0;
                let mut group_width = 0;
                let mut size_width = 0;
                let mut formatted_entries = Vec::new();
                for entry in &entries {
                    let mode = entry.metadata.mode();
                    let is_dir = entry.metadata.is_dir();
                    let is_symlink = entry.metadata.file_type().is_symlink();
                    let perms = format_mode(mode, is_dir, is_symlink);
                    let links = entry.metadata.nlink().to_string();
                    let uid = entry.metadata.uid();
                    let gid = entry.metadata.gid();
                    let owner = users.get(&uid).cloned().unwrap_or_else(|| uid.to_string());
                    let group = groups.get(&gid).cloned().unwrap_or_else(|| gid.to_string());
                    let file_type_bits = mode & 0o170000;
                    let is_device = file_type_bits == 0o060000 || file_type_bits == 0o020000;
                    let size_str = if is_device {
                        let (maj, min) = major_minor(entry.metadata.rdev());
                        format!("{}, {}", maj, min)
                    } else {
                        entry.metadata.len().to_string()
                    };
                    let mtime = entry.metadata.mtime();
                    let time_str = format_mtime(mtime);
                    let mut display_name = entry.name.clone();
                    if classify {
                        if is_dir {
                            display_name.push('/');
                        } else if is_symlink {
                            display_name.push('@');
                        } else if !is_dir && mode & 0o111 != 0 {
                            display_name.push('*');
                        }
                    }
                    link_width = link_width.max(links.len());
                    owner_width = owner_width.max(owner.len());
                    group_width = group_width.max(group.len());
                    size_width = size_width.max(size_str.len());
                    formatted_entries.push((perms, links, owner, group, size_str, time_str, display_name));
                }
                for (perms, links, owner, group, size_str, time_str, display_name) in formatted_entries {
                    println!(
                        "{} {:>lw$} {:<ow$} {:<gw$} {:>sw$} {} {}",
                        perms,
                        links,
                        owner,
                        group,
                        size_str,
                        time_str,
                        display_name,
                        lw = link_width,
                        ow = owner_width,
                        gw = group_width,
                        sw = size_width
                    );
                }
            } else {
                let mut display_names = Vec::new();
                for entry in &entries {
                    let mut name = entry.name.clone();
                    if classify {
                        let mode = entry.metadata.mode();
                        let is_dir = entry.metadata.is_dir();
                        let is_symlink = entry.metadata.file_type().is_symlink();
                        if is_dir {
                            name.push('/');
                        } else if is_symlink {
                            name.push('@');
                        } else if !is_dir && mode & 0o111 != 0 {
                            name.push('*');
                        }
                    }
                    display_names.push(name);
                }
                println!("{}", display_names.join("  "));
            }
        } else {
            if long {
                let mode = metadata.mode();
                let is_dir = metadata.is_dir();
                let is_symlink = metadata.file_type().is_symlink();
                let perms = format_mode(mode, is_dir, is_symlink);
                let links = metadata.nlink().to_string();
                let uid = metadata.uid();
                let gid = metadata.gid();
                let owner = users.get(&uid).cloned().unwrap_or_else(|| uid.to_string());
                let group = groups.get(&gid).cloned().unwrap_or_else(|| gid.to_string());
                let file_type_bits = mode & 0o170000;
                let is_device = file_type_bits == 0o060000 || file_type_bits == 0o020000;
                let size_str = if is_device {
                    let (maj, min) = major_minor(metadata.rdev());
                    format!("{}, {}", maj, min)
                } else {
                    metadata.len().to_string()
                };
                let mtime = metadata.mtime();
                let time_str = format_mtime(mtime);
                let mut display_name = target.to_string();
                if classify {
                    if is_dir {
                        display_name.push('/');
                    } else if is_symlink {
                        display_name.push('@');
                    } else if !is_dir && mode & 0o111 != 0 {
                        display_name.push('*');
                    }
                }
                println!(
                    "{} {} {} {} {} {} {}",
                    perms, links, owner, group, size_str, time_str, display_name
                );
            } else {
                let mut display_name = target.to_string();
                if classify {
                    let mode = metadata.mode();
                    let is_dir = metadata.is_dir();
                    let is_symlink = metadata.file_type().is_symlink();
                    if is_dir {
                        display_name.push('/');
                    } else if is_symlink {
                        display_name.push('@');
                    } else if !is_dir && mode & 0o111 != 0 {
                        display_name.push('*');
                    }
                }
                println!("{}", display_name);
            }
        }
    }
    if has_failed {
        Err(ShellError::Generic(String::new()))
    } else {
        Ok(())
    }
}
