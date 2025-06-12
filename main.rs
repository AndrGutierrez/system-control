use std::{fs, path::Path};
mod utils;
use utils::list_processes;
use utils::get_value;

fn main() {
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
