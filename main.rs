use std::{fs, path::Path};

fn get_value(input: &str) -> i32 {
    let arr: Vec<&str> = input.split(" ").collect();
    let filtered: Vec<&&str> = arr.iter().filter(|&item| !item.is_empty()).collect();
    let res = filtered[1].parse().unwrap();
    return res;
}

fn get_libraries(pid: &str) -> String {}

fn get_memory_usage(pid: &str) -> f64 {
    // returns memory used by process in MiB
    let path = format!("/proc/{}/status", pid);
    //println!("{}", path);
    //let proc_path = Path::new(&path);
    let content = fs::read_to_string(&path).expect("Error");
    let mut memory_usage: f64 = 0.0;
    for line in content.lines() {
        if line.starts_with("VmRSS") {
            memory_usage = get_value(line) as f64 / 1024.0;
        }
    }
    return memory_usage;
}

fn list_processes() -> std::io::Result<Vec<(f64, String)>> {
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
            pids.push((memory_usage, process_info));
        }
    }
    pids.sort_by(|a, b| a.0.partial_cmp(&b.0).expect("Could not compare f64 values"));
    Ok(pids)
}
fn get_process_name(pid: &str) -> String {
    let path: String = format!("/proc/{}/comm", pid);
    let proc_path = Path::new(&path);
    let mut name: String = fs::read_to_string(proc_path).expect("Something failed");
    let len = name.len();
    name.truncate(len - 1);

    return name;
}
fn main() {
    //get_memory_usage("2081");
    let result = fs::read_to_string("/proc/meminfo");
    let mut memory_used: f64 = 0.0;
    if let Ok(contents) = result {
        let info: Vec<&str> = contents.split("\n").collect();
        let memory_total = get_value(info[0]);
        let memory_free = get_value(info[1]);
        let buffers = get_value(info[3]);
        let cached = get_value(info[4]);
        let sreclaimable = get_value(info[25]);
        memory_used =
            (memory_total - memory_free - buffers - cached - sreclaimable) as f64 / 1024.0;
    }
    match list_processes() {
        Ok(pids) => {
            println!("Running processes ({} total):", pids.len());
            for pid in pids {
                println!("{}", pid.1);
            }
        }
        Err(e) => eprintln!("Error listing processes: {}", e),
    }
    println!("{:#?} MiB", memory_used);
}
