use std::{collections::HashSet, str::Chars, iter::Peekable};

#[derive(Debug,Clone)]
pub enum GrepPatterns<'a>{
    Number,
    AlphaNumerUnderscore,
    Contains(char),
    PositiveCharacterGroups(HashSet<char>),
    NegativeCharacterGroups(HashSet<char>),
    BeginWith(&'a str),
    End,
    Default
}

impl <'a>  GrepPatterns<'a> {
    fn can_continue(&self) -> bool{
        match self{
            Self::BeginWith(_) => false,
            Self::End  => false,
            _ => true,
        }
    }

    pub fn find(self, input: &'a str) -> bool{
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
            },
            GrepPatterns::NegativeCharacterGroups(strlist) => {
                let chars = input.chars();
                for c in chars{
                    if !strlist.contains(&c){
                        return true 
                    }
                }
                false
            },
            GrepPatterns::BeginWith(slice) => {
                input.starts_with(slice)
            },
            GrepPatterns::End => {
                input == ""
            }
            GrepPatterns::Default => false
        }
    }

    pub fn find_by_step(&self, input : &mut Chars<'a>) -> Option<bool>{
        match self{
            GrepPatterns::AlphaNumerUnderscore =>{
                let next_char= input.next()?;
                Some(next_char.is_alphanumeric() || next_char == '_')
            },
            GrepPatterns::Number =>{
                let next_char = input.next()?;
                Some(next_char.is_ascii_digit())
            },
            GrepPatterns::Contains(ch) => {
                let next_char = input.next()?;
                Some(next_char == *ch)
            },
            GrepPatterns::PositiveCharacterGroups(strlist) => {
                let next_char = input.next()?;
                Some(strlist.contains(&next_char))
            },
            GrepPatterns::NegativeCharacterGroups(strlist) => {
                let next_char = input.next()?;
                Some(!strlist.contains(&next_char))
            },
            GrepPatterns::BeginWith(slice) => {
                let strslice = input
                    .as_str();
                let result = strslice
                    .starts_with(slice);
                if result{
                    chars_advance_by(input, strslice.len());
                }
                Some(result)
            },
            GrepPatterns::End => {
                Some(input.next() == None)
            }
            GrepPatterns::Default => None
        }
    }
}

const PATTERN_DELIMITER : [char;4] = ['\\', '[', '^','$'];

//todo: rewrite with nom parser
#[derive(Clone)]
struct RegEx<'a>{
    chars: Chars<'a>,
    next_character : Option<char>,
    original_slice : &'a str
}

impl <'a> RegEx<'a>{
    fn init(pattern: &'a str) -> Self{
        Self{
            chars : pattern.chars(),
            next_character : None,
            original_slice : pattern
        }
    }
}

impl <'a> Iterator for RegEx<'a>{
    type Item = GrepPatterns<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        //let mut cloned_iter = self.chars.clone();
        // let last_char = cloned_iter.next_back()?;

        // if last_char == '$'{
        //     let pattern = GrepPatterns::EndsWith(cloned_iter.as_str());
        //     self.chars = "".chars();
        //     return Some(pattern)
        // }

        let first_char = self.next_character.or(self.chars.next())?;
        match first_char{
            '\\' => {
                let second_char = self.chars.next();
                if second_char == Some('d'){
                    return Some(GrepPatterns::Number);
                }else if second_char == Some('w'){
                    return Some(GrepPatterns::AlphaNumerUnderscore);
                }else{
                    self.next_character = second_char;
                    return Some(GrepPatterns::Contains(first_char))
                }
            },
            '[' => {
                // let mut cloned_iter = self.chars.clone();
                // //todo: handle case where ] is not found
                // if cloned_iter.any(|c| c == ']'){
                let mut hashset = HashSet::new();
                
                let mut is_negate = false;
                let second_char = self.chars.next()?;
                
                if second_char == '^' {
                    is_negate = true;
                }else {
                    hashset.insert(second_char);
                }

                while let Some(c) = self.chars.next(){
                    if c == ']'{
                        break
                    }
                    hashset.insert(c);
                }
                
                if is_negate{
                    return Some(GrepPatterns::NegativeCharacterGroups(hashset));
                }
                return Some(GrepPatterns::PositiveCharacterGroups(hashset))
    
            },
            '^' => {
                let mut end_index = 0;
                while let Some(c) = self.chars.next(){
                    if PATTERN_DELIMITER.contains(&c){
                        self.next_character = Some(c);
                        break;
                    }
                    end_index += 1;
                }
                Some(GrepPatterns::BeginWith(&self.original_slice[1..(end_index+1)]))
            },
            '$' => {
                self.next_character = None;
                Some(GrepPatterns::End)
            },
            c => {
                return Some(GrepPatterns::Contains(c))
            }
        }
    }
}


pub struct GrepFinder<'a>{
    input :Chars<'a>,
    regex_pattern : RegEx<'a>,
}

impl <'a> GrepFinder<'a>{
    pub fn init(input: &'a str, pattern : &'a str) -> Self{
        Self{
            input : input.chars(),
            regex_pattern : RegEx::init(pattern),
        }
    }

    pub fn find(&mut self) -> bool{
        loop {
            let regex_pattern = self.regex_pattern.clone();
            let mut cloned_input = self.input.clone();
            let (result, should_continue) = 
                Self::match_me(&mut cloned_input, regex_pattern);
                
            if result {return true;}

            if !should_continue || self.input.next() == None{
                break;
            }
        }
        false
    }

    fn match_me(input : &mut Chars<'a>, mut regex : RegEx<'a>) -> (bool, bool) {
        loop{
            if let Some(pattern) = regex.next(){
                if !pattern.find_by_step(input).unwrap_or(false){    
                    return (false, pattern.can_continue())
                }
            }else{
                break;
            }
        }
        (true, false)
    }
}


//utils

fn chars_advance_by<'a>(chars : &mut Chars<'a>, mut advance_by : usize){
    while advance_by > 0{
        if chars.next() == None{
            break;
        }
        advance_by -= 1;
    }
}