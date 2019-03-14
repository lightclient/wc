use std::fs;
use std::process;

struct Results {
    bytes: u32,
    lines: u32,
    words: u32,
    in_word: bool,
}

impl Results {
    fn new() -> Results {
       Results {
           bytes: 0,
           lines: 0,
           words: 0,
           in_word: false,
       }
    }
}

pub fn analyze_file(filename: &str) {
    let data = fs::read_to_string(filename).unwrap_or_else(|err| {
            eprintln!("Unable to read file: {}", err);
            process::exit(1);
    });

    let results = data.chars().fold(Results::new(), |mut acc, c| {
        acc.bytes += 1;

        if c == '\n' {
            acc.lines += 1;
            if acc.in_word { acc.words += 1 }
            acc.in_word = false;
        } else if c.is_whitespace() && acc.in_word {
            acc.words += 1;
            acc.in_word = false;
        } else if !acc.in_word {
            acc.in_word = true;
        }

        acc
    });

    println!("{:8}{:8}{:8} {:8}", results.lines, results.words, results.bytes, filename);
}
