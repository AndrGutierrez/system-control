use std::fs::File;
use std::io;
fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Unable to read");
    //let filename = input.trim();
    let mut file = File::create(&input);
    println!("Hello World!");
    return;
}
