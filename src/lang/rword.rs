use crate::*;
use crate::lang::*;

#[derive(Debug)]
pub struct Rword {
    pub key: String,
    pub ctx: Option<PRtable>,
}

impl Rword {
    pub fn new(k: String, c: Option<PRtable>)-> Rword{
        Rword{key: k, ctx: c}
    }

    pub fn set_ctx(&mut self, c: Option<PRtable>){
        self.ctx = c
    }
}



impl Clone for Rword {
    fn clone(&self) -> Self {
        match self.ctx {
            Some(ref c) => Rword{key: self.key.clone(), ctx: Some(c.clone())},
            _ => Rword{key: self.key.clone(), ctx: None},
        }
    }
}

