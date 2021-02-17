use std::str::Chars;




#[derive(Debug)]
pub struct StrKit();

impl StrKit {

    pub fn cut_strs(inp: Vec<char>)-> Vec<String>{
        let mut result: Vec<String> = Vec::new();
        let mut idx: usize = 0;
        while idx < inp.len() {
            if !inp.get(idx).unwrap().is_whitespace() {
                result.push(Self::take_token(inp.clone(), &mut idx));
            }else{
               idx += 1; 
            }
        }

        return result;
    }


    pub fn take_token(inp: Vec<char>, idx: &mut usize)-> String{
        let c = inp.get(*idx).unwrap();
        match *c {
            '"' => Self::take_string(inp.clone(), idx),
            '[' => Self::take_block(inp.clone(), idx),
            '(' => Self::take_paren(inp.clone(), idx),
            '{' => Self::take_object(inp.clone(), idx),

            _ => Self::take_word(inp.clone(), idx)
        }

    }

    pub fn take_word(inp: Vec<char>, idx: &mut usize)-> String{
        let start = *idx;
        while *idx < inp.len() {
            if inp.get(*idx).unwrap().is_whitespace() {
                break;
            }
            (*idx) += 1;
        }

        return inp[start..*idx].iter().collect();
    }

    pub fn take_string(inp: Vec<char>, idx: &mut usize)-> String{
        let start = *idx;
        (*idx) += 1;
        while *idx < inp.len() {
            let c = inp.get(*idx).unwrap();
            if *c == '^' {
                (*idx) += 2;
                continue;
            }else if *c == '"' {
                (*idx) += 1;
                break;
            }

            (*idx) += 1;
        }

        return inp[start..*idx].iter().collect();
    }

    pub fn take_block(inp: Vec<char>, idx: &mut usize)-> String{
        let start = *idx;
        let mut floor = 0;
        while *idx < inp.len() {
            let c = inp.get(*idx).unwrap();
            if *c == '"' {
                Self::take_string(inp.clone(), idx);
            }else if *c == '[' {
                floor += 1;
            }else if *c == ']' {
                floor -= 1;
            }

            (*idx) += 1;

            if floor == 0 {
                break;
            }
        }

        return inp[start..*idx].iter().collect();
    }

    pub fn take_paren(inp: Vec<char>, idx: &mut usize)-> String{
        let start = *idx;
        let mut floor = 0;
        while *idx < inp.len() {
            let c = inp.get(*idx).unwrap();
            if *c == '"' {
                Self::take_string(inp.clone(), idx);
            }else if *c == '(' {
                floor += 1;
            }else if *c == ')' {
                floor -= 1;
            }

            (*idx) += 1;

            if floor == 0 {
                break;
            }
        }

        return inp[start..*idx].iter().collect();
    }

    pub fn take_object(inp: Vec<char>, idx: &mut usize)-> String{
        let start = *idx;
        let mut floor = 0;
        while *idx < inp.len() {
            let c = inp.get(*idx).unwrap();
            if *c == '"' {
                Self::take_string(inp.clone(), idx);
            }else if *c == '{' {
                floor += 1;
            }else if *c == '}' {
                floor -= 1;
            }

            (*idx) += 1;

            if floor == 0 {
                break;
            }
        }

        return inp[start..*idx].iter().collect();
    }

    pub fn is_num_str(inp: String)-> i32{
        if inp.len() == 0 {
            return -1
        }

        if inp == "-" {
            return -1
        }

        let mut dot = 0;
        let cs: Vec<char> = inp.chars().collect();
        let mut idx = 0;
        if cs.get(0).unwrap() == &'-' {
            idx = 1;
        }
        
        while idx < cs.len() {
            let c = cs.get(idx).unwrap();
            if c == &'.' {
                dot += 1;
            }else if !c.is_ascii_digit() {
                return -1
            }

            idx += 1;
        }

        return dot
    }



}






