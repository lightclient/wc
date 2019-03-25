use std::fs;
use std::process;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Result {
    pub bytes: u32,
    pub chars: u32,
    pub lines: u32,
    pub words: u32,
    in_word: bool,
}

impl Result {
    fn new() -> Result {
       Result {
           bytes: 0,
           chars: 0,
           lines: 0,
           words: 0,
           in_word: false,
       }
    }
}

pub fn analyze_file(filename: &str) -> Result {
    let data = fs::read(filename).unwrap_or_else(|err| {
            eprintln!("Unable to read file: {}", err);
            process::exit(1);
    });

    analyze_bytes(data)
}

pub fn analyze_bytes(data: Vec<u8>) -> Result {
    let mut result = data.iter().fold(Result::new(), |mut acc, c| {
        acc.bytes += 1;

        if *c == '\n' as u8 {
            acc.lines += 1;
            if acc.in_word { acc.words += 1 }
            acc.in_word = false;
        } else if (*c as char).is_whitespace() && acc.in_word {
            acc.words += 1;
            acc.in_word = false;
        } else if !acc.in_word {
            acc.in_word = true;
        }

        if (c & 0xC0) != 0x80 {
            acc.chars += 1;
        }

        acc
    });

    if result.in_word {
       result.words += 1;
       result.in_word = false;
    }

    result
}

#[cfg(test)] 
mod tests {
    use super::*; 

    #[test]
    fn test_empty_string() {
        let test_bytes = "".as_bytes().to_vec();
        let result = Result { bytes: 0, chars: 0, lines: 0, words: 0, in_word: false };
        assert_eq!(analyze_bytes(test_bytes), result);
    }

    #[test]
    fn test_single_line() {
        let test_bytes = "Hello world, this is a string!".as_bytes().to_vec();
        let result = Result { bytes: 30, chars: 30, lines: 0, words: 6, in_word: false };
        assert_eq!(analyze_bytes(test_bytes), result);
    }

    #[test]
    fn test_emojis() {
        // this emoji should be 4 bytes
        let test_bytes = "rust is 🔥".as_bytes().to_vec();
        let result = Result { bytes: 12, chars: 9, lines: 0, words: 3, in_word: false };
        assert_eq!(analyze_bytes(test_bytes), result);
    }

    #[test]
    fn test_newlines() {
        let test_bytes = "a\n\nb\nc\n\n\nd".as_bytes().to_vec();
        let result = Result { bytes: 10, chars: 10, lines: 6, words: 4, in_word: false };
        assert_eq!(analyze_bytes(test_bytes) , result);
    }
}
