use std::collections::HashSet;

pub fn match_me(input : &str, regex : &str) -> bool{
    if input.len() ==0 || regex.len() == 0{
        return false
    }

    if regex.starts_with('^'){
        return match_here(input, &regex[1..]).0
    }

    let mut input_chars = input.chars();
    loop{
        if match_here(input_chars.as_str(), regex).0{
            return true
        }
        
        if input_chars.next() == None{
            break
        }
    }

    return false
}

fn match_here<'a>(mut input : &'a str, mut regex: &'a str) -> (bool, &'a str, &'a str){
    if regex.starts_with('('){
        if let Some(i) = regex[1..].find(')'){
            if regex[1..i].contains('|'){
                let (result, input_, regex_) = match_either(input , &regex[(i+1)..], &regex[1..i]);
                if !result{
                    return (false, input, regex)
                }
                input = input_;
                regex = regex_;
            }
        }
    }

    if regex.len() == 0{
        return (true, input, regex)
    }

    if regex.len() > 1 && regex.chars().nth(1).unwrap() == '+'{
        return match_plus(regex.chars().nth(0).unwrap(),
                input,
                &regex[2..])
    }

    if regex.len() > 1 && regex.chars().nth(1).unwrap() == '?'{
        return match_option(regex.chars().nth(0).unwrap(),
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
        return (input.len() == 0, input , &regex[1..])
    }

    if input.len() > 0 && (first_char == '.' || first_char == input.chars().nth(0).unwrap()){
        return match_here(&input[1..], &regex[1..])
    }

    return (false, input, regex)

}

fn match_character_alu<'a>(input : &'a str, regex: &'a str) -> (bool, &'a str, &'a str){
    if input.len() == 0{
        return (false, input, regex)
    }

    if !(input.chars().nth(0).unwrap().is_alphanumeric() ||
        input.chars().nth(0).unwrap() == '_')
    {
        return (false, input, regex)
    }

    return (true, &input[1..], regex)
}

fn match_character_digit<'a>(input : &'a str, regex: &'a str) -> (bool, &'a str, &'a str){
    if input.len() ==0 {
        return (false, input, regex)
    }

    if !input.chars().nth(0).unwrap().is_ascii_digit(){
        return (false, input, regex)
    }

    return (false, &input[1..], regex)

}

fn match_character_set<'a>(input : &'a str, regex: &'a str) -> (bool, &'a str, &'a str){
    if input.len() == 0{
        return (false, input, regex)
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
        return (false, input, regex)
    }

    return (true, &input[1..], regex_chars.as_str())

}

fn match_plus<'a>(c : char, input : &'a str, regex : &'a str) -> (bool, &'a str, &'a str){
    let mut input_chars = input.chars();
    loop{
        let ch = input_chars.next();
        if ch == None{
            break;
        }

        if c != '.' && Some(c) != ch{
            break;
        }

        let (result, input_, regex_) = match_here(input_chars.as_str(), regex);
        if result{
            return (true, input_, regex_)
        } 
    }

    return (false, input, regex)
}

fn match_option<'a>(c :char, input : &'a str, regex : &'a str) -> (bool, &'a str ,&'a str){
    let mut input_chars = input.chars();

    let (result, input_, regex_) = match_here(input, regex);
    if result{
        return (true, input_, regex_)
    }
    if input_chars.next() == Some(c){
        return match_here(input_chars.as_str(), regex)
    }

    return (false, input, regex)
}

fn match_either<'a>(mut input : &'a str, mut regex : &'a str, either_list : &'a str) -> (bool, &'a str, &'a str){
    let mut splitted = either_list.split('|');

    let mut result = false;
    while let Some(slice) = splitted.next(){
        let (result_, input_, regex_) = match_here(input, slice);
        if result_{
            input = input_;
            regex = regex_;
            result = true;
            break;
        }
    }
    return (result,input ,regex) 
}