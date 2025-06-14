mod utils;
use std::fs;
use utils::{get_libraries, get_value, list_processes};

fn main() {
    let result = fs::read_to_string("/proc/meminfo");
    let mut memory_used: f64 = 0.0;
    if let Ok(contents) = result {
        let info: Vec<&str> = contents.split("\n").collect();
        let memory_total = get_value(info[0]);
        // let memory_free = get_value(info[1]);
        let memory_available = get_value(info[2]);
        //let buffers = get_value(info[3]);
        //let cached = get_value(info[4]);
        //let sreclaimable = get_value(info[25]);
        //memory_used =
        //(memory_total - memory_free - buffers - cached - sreclaimable) as f64 / 1024.0;
        memory_used =
        (memory_total - memory_available) as f64 / 1024.0;
    }
    match list_processes() {
        Ok(processes) => {
            let processes_len = processes.len();
            let most_consuming_process = &processes[processes_len - 1];
            println!("################################################");
            println!("# Proceso que usa la mayor cantidad de memoria #");
            println!("################################################");
            println!("\n{:<9} {:<13} {}", "PID", "Memoria", "Nombre");
            println!("\n{}", most_consuming_process.1);
            let (libraries, files) = get_libraries(most_consuming_process.2.clone());
            println!("\n####################");
            println!("# Librerias usadas #");
            println!("####################\n");
            for lib in libraries {
                println!("{}", lib);
            }

            println!("\n###################");
            println!("# Archivos usados #");
            println!("###################\n");
            for file in files {
                println!("{}", file);
            }
            println!("\n#############################################");
            println!("# Total de Memoria utilizada por el sistema #");
            println!("############################################# \n");
            println!("{:#?} MiB", memory_used);
        }
        Err(e) => eprintln!("Error listing processes: {}", e),
    }
}
