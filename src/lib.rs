use std::fs;
use std::process;

pub fn analyze_file(filename: &str) {
    let data = fs::read_to_string(filename).unwrap_or_else(|err| {
            eprintln!("Unable to read file: {}", err);
            process::exit(1);
    });

    println!("{}", data.lines().count());
}
