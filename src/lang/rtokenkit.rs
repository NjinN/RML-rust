use crate::lang::*;

pub struct RtokenKit();

impl RtokenKit {
    pub fn make_rtoken(s: String, ctx: Option<PRtable>)-> PRtoken{
        let s = s.as_str().trim();
        let len = s.len();
        if len == 0 {
            return PRtoken::new_word(s.to_string(), ctx) 
        }

        let first_c = s.chars().nth(0).unwrap();
        let last_c = s.chars().nth(len-1).unwrap();

        if s == "none" {
            return PRtoken::new_none()
        }

        if s == "false" {
            return PRtoken::new_bool(false)
        }

        if s == "true" {
            return PRtoken::new_bool(true)
        }

        if last_c == '!' {
            return PRtoken::str_to_datatype(s.to_string())
        }

        let num_dots = StrKit::is_num_str(s.to_string());
        if num_dots == 0 {
            return PRtoken::new_int(s.parse().unwrap())
        }else if num_dots == 1 {
            return PRtoken::new_float(s.parse().unwrap())
        }

        if first_c == '\'' {
            return PRtoken::new_lit_word(s[1..len].to_string())
        }

        if first_c == '"' {
            return PRtoken::new_str(s[1..len-1].to_string())
        }

        if last_c == ':' {
            return PRtoken::new_set_word(s[0..len-1].to_string())
        }


        let opt : Option<PRtable>;
        match ctx {
            Some(ref c) => opt = Some(c.clone()),
            None => opt = None
        }

        if first_c == '[' {
            return PRtoken::new_block(RtokenKit::make_rtoken_vec(s[1..len-1].to_string(), opt))
        }

        if first_c == '(' {
            return PRtoken::new_paren(RtokenKit::make_rtoken_vec(s[1..len-1].to_string(), opt))
        }



        return PRtoken::new_word(s.to_string(), ctx)
    }


    pub fn make_rtoken_vec(s: String, ctx: Option<PRtable>)-> Vec<PRtoken>{
        let s = s.as_str().trim();
        let strs = StrKit::cut_strs(s.chars().collect());
        let mut result: Vec<PRtoken> = vec![];
        for item in strs.iter(){
            match ctx {
                Some(ref c) => result.push(RtokenKit::make_rtoken(item.clone(), Some(c.clone()))),
                None => result.push(RtokenKit::make_rtoken(item.clone(), None)),
            }
        }

        return result
    }

}




















