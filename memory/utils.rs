use std::{fs, path::Path};

pub fn get_value(input: &str) -> i32 {
    let arr: Vec<&str> = input.split(" ").collect();
    let filtered: Vec<&&str> = arr.iter().filter(|&item| !item.is_empty()).collect();
    let res = filtered[1].parse().unwrap();
    return res;
}

pub fn get_libraries(pid: String) -> (Vec<String>, Vec<String>) {
    let path = format!("/proc/{}/maps", pid);
    let content = fs::read_to_string(&path).expect("Failed to read process maps");
    
    let mut libraries = Vec::new();
    let mut files = Vec::new();
    
    for line in content.lines() {
        if let Some(path) = extract_path(line) {
            if is_shared_library(&path) {
                if !libraries.contains(&path) {
                    libraries.push(path);
                }
            } else if !files.contains(&path) {
                files.push(path);
            }
        }
    }
    
    (libraries, files)
}

fn extract_path(line: &str) -> Option<String> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 6 {
        let path = parts[5];
        if !path.is_empty() && path != "0" && Path::new(path).exists() {
            return Some(path.to_string());
        }
    }
    None
}

fn is_shared_library(path: &str) -> bool {
    path.ends_with(".so") || path.contains(".so.")
}

pub fn get_memory_usage(pid: &str) -> f64 {
    // returns memory used by process in MiB
    let path = format!("/proc/{}/status", pid);
    let content = fs::read_to_string(&path).expect("Error");
    let mut memory_usage: f64 = 0.0;
    for line in content.lines() {
        if line.starts_with("VmRSS") {
            memory_usage = get_value(line) as f64 / 1024.0;
        }
    }
    return memory_usage;
}

pub fn list_processes() -> std::io::Result<Vec<(f64, String, String)>> {
    let proc_path = Path::new("/proc");
    let mut pids = Vec::new();
    for entry in fs::read_dir(proc_path)? {
        let entry = entry?;
        let file_name = entry.file_name();
        if let Ok(pid) = file_name.to_string_lossy().parse::<u32>() {
            let str_pid: String = pid.to_string();
            let name = get_process_name(&str_pid);
            let memory_usage = get_memory_usage(&str_pid);
            let process_info = format!("{:<8}{:>8.2} MiB\t{:<8} ", str_pid, memory_usage, name);
            pids.push((memory_usage, process_info, str_pid));
        }
    }
    pids.sort_by(|a, b| a.0.partial_cmp(&b.0).expect("Could not compare f64 values"));
    Ok(pids)
}

pub fn get_process_name(pid: &str) -> String {
    let path: String = format!("/proc/{}/comm", pid);
    let proc_path = Path::new(&path);
    let mut name: String = fs::read_to_string(proc_path).expect("Something failed");
    let len = name.len();
    name.truncate(len - 1);

    return name;
}
