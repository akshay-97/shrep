use std::env;
use std::io;
use std::process;
use clap::{Arg, command};
use anyhow::{Result,anyhow};

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern.chars().count() == 1 {
        return input_line.contains(pattern);
    } else {
        panic!("Unhandled pattern: {}", pattern)
    }
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() -> Result<(), anyhow::Error> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");

    let grep = command!()
        .arg(Arg::new("match_e").short('E'))
        .get_matches();

    let pattern = grep.get_one::<String>("match_e").ok_or(anyhow!("Missing Attribute"))?;
    
    // let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
