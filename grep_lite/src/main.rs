use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io;
use clap::{App, Arg};
use regex::Regex;

fn process_lines<T: BufRead + Sized>(reader: T, rx: Regex) {
    for i in reader.lines() {
        let line = i.unwrap();
        match rx.find(&line) {
            Some (_) => println!("{}", line),
            None => (),
        }
    }
}

fn main() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns.")
        .arg(
            Arg::with_name("pattern")
                .help("the pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
            .help("file to search")
            .takes_value(true)
            .required(true)            
        )
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let rx = Regex::new(pattern).unwrap();
    
    let input = args.value_of("input").unwrap_or("-");
  
    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, rx);
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, rx);
    }   
}