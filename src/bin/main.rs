extern crate clap;
use clap::{App, Arg};
use wc::{Result, analyze_file};
use std::process;

fn main() {
   let matches = App::new("wc")
       .arg(Arg::with_name("filename")
             .help("the name of the input file")
             .index(1)
       )
       .arg(Arg::with_name("print_lines").short("l"))
       .arg(Arg::with_name("print_words").short("w"))
       .arg(Arg::with_name("print_bytes").short("c"))
       .arg(Arg::with_name("print_chars").short("m"))
       .get_matches();


    let result: Result;
    if let Some(filename) = matches.value_of("filename") {
        result = analyze_file(filename); 
    } else {
        println!("STDIN not implemented yet");
        process::exit(1);
    }

    write_counts(result, matches); 
}

fn write_counts(result: Result, matches: clap::ArgMatches) {
    let no_args = !(
        matches.is_present("print_lines") ||
        matches.is_present("print_words") ||
        matches.is_present("print_bytes")
    );


    if no_args || matches.is_present("print_lines") {
        print!("{:8}", result.lines);
    }

    if no_args || matches.is_present("print_words") {
        print!("{:8}", result.words);
    }

    /*
    if matches.is_present("print_chars") {
        print!("{:8}", result.chars);
    }
    */

    if no_args || matches.is_present("print_bytes") {
        print!("{:8}", result.bytes);
    }

    if let Some(filename) = matches.value_of("filename") {
        print!(" {:8}", filename);
    }
}
