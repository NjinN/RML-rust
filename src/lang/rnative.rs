use crate::*;
use crate::lang::*;


#[derive(Debug, Clone)]
pub struct Rnative {
    pub name: String,
    pub expr_len: usize,
    pub exec: fn(Vec<PRtoken>, PRtable)->PRtoken,
}


impl Rnative {
    pub fn new(n: String, l: usize, e: fn(Vec<PRtoken>, PRtable)->PRtoken)-> Rnative{
        Rnative{name: n, expr_len: l, exec: e}
    }

    
}



