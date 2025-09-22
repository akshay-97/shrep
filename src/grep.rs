use std::{collections::HashSet, str::Chars, iter::Peekable};

pub enum GrepPatterns{
    Number,
    AlphaNumerUnderscore,
    Contains(char),
    PositiveCharacterGroups(HashSet<char>),
    NegativeCharacterGroups(HashSet<char>),
    Default
}

impl GrepPatterns{
    pub fn find(self, input: &str) -> bool{
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
            }
            GrepPatterns::Default => false
        }
    }

    pub fn find_by_step<'a>(self, mut input : Peekable<Chars<'a>>) -> Option<bool>{
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
                Some(next_char == ch)
            },
            GrepPatterns::PositiveCharacterGroups(strlist) => {
                
                // for c in chars{
                //     if strlist.contains(&c){
                //         return true
                //     }
                // }
                while let Some(c) = input.next(){

                }
            },
            GrepPatterns::NegativeCharacterGroups(strlist) => {
                let chars = input.chars();
                for c in chars{
                    if !strlist.contains(&c){
                        return true 
                    }
                }
                false
            }
            GrepPatterns::Default => false
        }
    }
}

// pub trait Grep{
//     fn grep<'a>(pattern: &'a str) -> GrepPatterns<'a>;
// }

// impl <'b> Grep for GrepPatterns<'b>{
//     fn grep<'a>(pattern: &'a str) -> GrepPatterns<'a>{
//         match pattern{
//             "\\d" => GrepPatterns::Number,
//             "\\w" => GrepPatterns::AlphaNumerUnderscore,
//             a => {
//                 if a.len() == 1{
//                     return GrepPatterns::Contains(a)
//                 }
//                 if a.len() >= 2 && a.starts_with('[') && a.ends_with(']'){
//                     let mut hashset = HashSet::new();
//                     let mut chars = a.chars();
//                     chars.next();
//                     chars.next_back();
//                     let mut is_negate = false;

//                     if let Some(first_char) = chars.next(){
//                         if first_char == '^'{
//                             is_negate = true;
//                         }else{
//                             hashset.insert(first_char);
//                         }
//                     }

//                     for c in chars{
//                         hashset.insert(c);
//                     }

//                     if is_negate{
//                         return GrepPatterns::NegativeCharacterGroups(hashset);
//                     }
//                     return GrepPatterns::PositiveCharacterGroups(hashset)
//                 }

//                 GrepPatterns::Default
//             }
//         }
//     }
// }


//todo: rewrite with nom parser
struct RegEx<'a>{
    chars: Chars<'a>,
}

impl <'a> RegEx<'a>{
    fn init(pattern: &'a str) -> Self{
        Self{
            chars : pattern.chars()
        }
    }
}

impl <'a> Iterator for RegEx<'a>{
    type Item = GrepPatterns;

    fn next(&mut self) -> Option<Self::Item> {
        let first_char = self.chars.next()?;
        match first_char{
            '\\' => {
                let second_char = self.chars.next()?;
                if second_char == 'd'{
                    return Some(GrepPatterns::Number);
                }else if second_char == 'w'{
                    return Some(GrepPatterns::AlphaNumerUnderscore);
                }
                None
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
            c => {
                return Some(GrepPatterns::Contains(c))
            }
        }
    }
}


struct GrepFinder<'a>{
    input : Peekable<Chars<'a>>,
    regex_pattern : RegEx<'a>,
    prev_pattern : Option<GrepPatterns>,
}

impl <'a> GrepFinder<'a>{
    pub fn init(input: &'a str, pattern : &'a str) -> Self{
        Self{
            input : input.chars().peekable(),
            regex_pattern : RegEx::init(pattern),
            prev_pattern : None
        }
    }

    pub fn find(&mut self) -> bool{
        
    }
}