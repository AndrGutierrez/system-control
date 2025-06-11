use std::fs;

fn get_value(input: &str) -> i32 {
    let arr: Vec<&str> = input.split(" ").collect();
    let filtered: Vec<&&str> = arr.iter().filter(|&item| !item.is_empty()).collect();
    let res = filtered[1].parse().unwrap();
    return res;
}

fn main() {
    let result = fs::read_to_string("/proc/meminfo");
    if let Ok(contents) = result {
        let info: Vec<&str> = contents.split("\n").collect();
        let memory_total = get_value(info[0]);
        let memory_free = get_value(info[1]);
        let buffers = get_value(info[3]);
        let cached = get_value(info[4]);
        let sreclaimable = get_value(info[25]);
        let memory_used =
            (memory_total - memory_free - buffers - cached - sreclaimable) as f64 / 1024.0;
        println!("{:#?} MiB", memory_used);
    }
}
