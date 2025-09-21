use std::env;
use std::io;
use std::process;
use clap::{arg, command};
use anyhow::{Result,anyhow};
use std::collections::HashSet;

enum GrepPatterns<'a>{
    Number,
    AlphaNumerUnderscore,
    Contains(&'a str),
    PositiveCharacterGroups(HashSet<char>),
    Default
}

impl <'a> GrepPatterns<'a>{
    fn find(self, input: &str) -> bool{
        match self{
            GrepPatterns::AlphaNumerUnderscore =>
                input.chars().any(|c| c.is_alphanumeric() || c == '_'),
            GrepPatterns::Number =>
                input.chars().any(|c| c.is_ascii_digit()),
            GrepPatterns::Contains(c) =>
                input.contains(c),
            GrepPatterns::PositiveCharacterGroups(strlist) => {
                let chars = input.chars();
                for c in chars{
                    if strlist.contains(&c){
                        return true
                    }
                }

                false
            }
            GrepPatterns::Default => false
        }
    }
}

trait Grep{
    fn grep<'a>(pattern: &'a str) -> GrepPatterns<'a>;
}

impl <'b> Grep for GrepPatterns<'b>{
    fn grep<'a>(pattern: &'a str) -> GrepPatterns<'a>{
        match pattern{
            "\\d" => GrepPatterns::Number,
            "\\w" => GrepPatterns::AlphaNumerUnderscore,
            a => {
                if a.len() == 1{
                    return GrepPatterns::Contains(a)
                }
                if a.len() >= 2 && a.starts_with('[') && a.ends_with(']'){
                    let mut hashset = HashSet::new();
                    let mut chars = a.chars();
                    chars.next();
                    chars.next_back();
                    for c in chars{
                        hashset.insert(c);
                    }
                    return GrepPatterns::PositiveCharacterGroups(hashset)
                }

                GrepPatterns::Default
            }
        }
    }
}

fn match_pattern(input_line: &str, pattern: GrepPatterns<'_>) -> bool
where
{
    pattern.find(input_line)
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

    // let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    let grep_pattern = GrepPatterns::grep(pattern.as_str());

    if match_pattern(&input_line, grep_pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
