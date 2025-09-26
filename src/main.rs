use std::env;
use std::io;
use std::process;
use clap::{arg, command};
use anyhow::{Result,anyhow};

//mod grep;
mod grep_1;
use grep_1::*;
//use grep::GrepFinder;

fn match_pattern(input_line: &str, pattern: &str) -> bool
{
    match_wrapper(input_line, pattern)
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() -> Result<(), anyhow::Error> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");

    let grep = command!()
        .about("shit grep -shrep")
        .arg(arg!(-E <pattern> "give your pattern"))
        .get_matches();

    let pattern = grep.get_one::<String>("pattern").ok_or(anyhow!("Missing Attribute"))?;

    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}

#[cfg(test)]
mod tests{
    #[test]
    fn test1(){

    }
}