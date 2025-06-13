mod utils;
use std::{fs};
use utils::{list_processes, get_value, get_libraries};

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
        Ok(processes) => {
            let processes_len = processes.len();
            let most_consuming_process = &processes[processes_len-1];
            println!("Most memory-consuming process: \n {}", most_consuming_process.1);
            let (libraries, files) = get_libraries(most_consuming_process.2.clone());
            println!("Librerias usadas:");
            for lib in libraries {
                println!("{}", lib);
            }
            
            println!("\nArchivos usados:");
            for file in files {
                println!("{}", file);
            }
            println!("{:#?} MiB", memory_used);
        }
        Err(e) => eprintln!("Error listing processes: {}", e),
    }    
}
