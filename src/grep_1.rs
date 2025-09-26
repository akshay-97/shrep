use std::{collections::HashSet, iter::Enumerate};

use nom::character;

pub fn match_me(input : &str, regex : &str) -> bool{
    if input.len() ==0 || regex.len() == 0{
        return false
    }

    if regex.starts_with('^'){
        return match_here(input, &regex[1..])
    }

    let mut iter = 0;
    loop{
        if match_here(&input[iter..], regex){
            return true
        }
        
        iter +=1;
        if iter >= input.len(){
            break;
        }
    }

    return false
}

fn match_here(input : &str, regex: &str) -> bool{
    if regex.len() == 0{
        return true
    }

    if regex.len() > 1 && regex.chars().nth(1).unwrap() == '+'{
        return match_star(regex.chars().nth(0).unwrap(),
                input,
                &regex[2..])
    }

    let first_char = regex.chars().nth(0).unwrap();
    if first_char == '['{
        return match_character_set(input, &regex[1..])
    }

    if first_char == '\\'{
        if regex.chars().nth(1) == Some('d'){
            return match_character_digit(input, &regex[2..])
        }
        if regex.chars().nth(1) == Some('w'){
            return match_character_alu(input, &regex[2..])
        }
    }

    if first_char == '$' && regex.len() == 1{
        return input.len() == 0
    }

    if input.len() > 0 && first_char == input.chars().nth(0).unwrap(){
        return match_here(&input[1..], &regex[1..])
    }

    return false

}

fn match_character_alu(input : &str, regex: &str) -> bool{
    if input.len() == 0{
        return false
    }

    if !(input.chars().nth(0).unwrap().is_alphanumeric() ||
        input.chars().nth(0).unwrap() == '_')
    {
        return false
    }

    return match_here(&input[1..], regex)
}

fn match_character_digit(input : &str, regex: &str) -> bool{
    if input.len() ==0 {
        return false
    }

    if !input.chars().nth(0).unwrap().is_ascii_digit(){
        return false
    }

    return match_here(&input[1..], regex)

}

fn match_character_set(input : &str, regex: &str) -> bool{
    if input.len() == 0{
        return false
    }

    let is_negate = regex.chars().nth(0) == Some('^');
    let mut hashset = HashSet::new();
    let mut regex_chars = regex.chars();

    while let Some(c) = regex_chars.next(){
        if c == ']'{
            break;
        }
        hashset.insert(c);
    }

    let res = hashset.contains(&input.chars().nth(0).unwrap()) ^ is_negate;
    if !res{
        return false
    }

    return match_here(&input[1..], regex_chars.as_str())

}

fn match_star(c : char, input : &str, regex : &str) -> bool{
    let mut input_chars = input.chars();
    loop{
        if match_here(input_chars.as_str(), regex){
            return true
        }
        let ch = input_chars.next();
        if Some(c) != ch{
            break;
        }
    }

    return false
}