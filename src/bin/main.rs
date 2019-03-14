extern crate clap;
use clap::{App, Arg};
use wc::analyze_file;

fn main() {
   let matches = App::new("wc")
       .arg(Arg::with_name("filename")
             .help("the name of the input file")
             .index(1)
       )
       .get_matches();


    if let Some(filename) = matches.value_of("filename") {
        analyze_file(filename); 
    } else {
        println!("STDIN not implemented yet");
    }
}
