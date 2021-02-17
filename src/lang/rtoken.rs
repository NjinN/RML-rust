use std::sync::{Arc, RwLock};

use crate::*;

#[derive(Debug)]
pub enum Rtoken {
    Nil,
    None,
    Err(String),
    Datatype(String),
    Bool(bool),
    Byte(i8),
    Char(char),
    Int(isize),
    Float(f64),
    Str(String),
    Block(Vec<PRtoken>),
    Paren(Vec<PRtoken>),
    Flow{msg: String, tk: Option<PRtoken>},
    Word(Rword),
    LitWord(String),
    SetWord(String),
    Func(Rfunc),
    Native(Rnative),
    Op(Rnative),

}



#[derive(Debug, Clone)]
pub struct PRtoken(pub Arc<RwLock<Rtoken>>); 

impl PRtoken {
    pub fn new(tk: Rtoken)-> PRtoken{
        PRtoken(Arc::new(RwLock::new(tk)))
    }

    pub fn new_nil()-> PRtoken{
        PRtoken(Arc::new(RwLock::new(Rtoken::Nil{})))
    }

    pub fn new_none()-> PRtoken{
        PRtoken(Arc::new(RwLock::new(Rtoken::None{})))
    }

    pub fn new_err(s: String)-> PRtoken{
        PRtoken(Arc::new(RwLock::new(Rtoken::Err(s))))
    }

    pub fn new_datatype(s: String)-> PRtoken{
        PRtoken(Arc::new(RwLock::new(Rtoken::Datatype(s))))
    }

    pub fn new_bool(b: bool)-> PRtoken{
        PRtoken(Arc::new(RwLock::new(Rtoken::Bool(b))))
    }

    pub fn new_byte(b: i8)-> PRtoken{
        PRtoken(Arc::new(RwLock::new(Rtoken::Byte(b))))
    }

    pub fn new_char(c: char)-> PRtoken{
        PRtoken(Arc::new(RwLock::new(Rtoken::Char(c))))
    }

    pub fn new_int(i: isize)-> PRtoken{
        PRtoken(Arc::new(RwLock::new(Rtoken::Int(i))))
    }

    pub fn new_float(f: f64)-> PRtoken{
        PRtoken(Arc::new(RwLock::new(Rtoken::Float(f))))
    }

    pub fn new_str(s: String)-> PRtoken{
        PRtoken(Arc::new(RwLock::new(Rtoken::Str(s))))
    }

    pub fn new_block(v: Vec<PRtoken>)-> PRtoken{
        PRtoken(Arc::new(RwLock::new(Rtoken::Block(v))))
    }

    pub fn new_paren(v: Vec<PRtoken>)-> PRtoken{
        PRtoken(Arc::new(RwLock::new(Rtoken::Paren(v))))
    }

    pub fn new_flow(s: String, t: Option<PRtoken>)-> PRtoken{
        PRtoken(Arc::new(RwLock::new(Rtoken::Flow{msg: s, tk: t})))
    }

    pub fn new_word(s: String, tb: Option<PRtable>)-> PRtoken{
        PRtoken(Arc::new(RwLock::new(Rtoken::Word(Rword::new(s, tb)))))
    }

    pub fn new_lit_word(s: String)-> PRtoken{
        PRtoken(Arc::new(RwLock::new(Rtoken::LitWord(s))))
    }

    pub fn new_set_word(s: String)-> PRtoken{
        PRtoken(Arc::new(RwLock::new(Rtoken::SetWord(s))))
    }

    pub fn new_func(f: Rfunc)-> PRtoken{
        PRtoken(Arc::new(RwLock::new(Rtoken::Func(f))))
    }

    pub fn new_native(n: Rnative)-> PRtoken{
        PRtoken(Arc::new(RwLock::new(Rtoken::Native(n))))
    }

    pub fn new_op(n: Rnative)-> PRtoken{
        PRtoken(Arc::new(RwLock::new(Rtoken::Op(n))))
    }

    // pub fn clone(&self)-> PRtoken{
    //     PRtoken(self.0.clone())
    // }

    pub fn show(&self) {
        println!("{}", self.to_str())
    }

    pub fn echo(&self){
        match *ptr_raw!(self) {
            Rtoken::Str(_) => {
                let mut str = self.to_str();
                str.remove(0);
                str.pop();
                println!("{}", str)
            },
            _ => println!("{}", self.to_str())
        }
    }

    pub fn is_nil(&self)-> bool{
        match *ptr_raw!(self) {
            Rtoken::Nil => true,
            _ => false,
        }
    }

    pub fn is_int(&self)-> bool{
        match *ptr_raw!(self) {
            Rtoken::Int(_) => true,
            _ => false,
        }
    }

    pub fn is_word(&self)-> bool{
        match *ptr_raw!(self) {
            Rtoken::Word{..} => true,
            _ => false,
        }
    }



    pub fn to_str(&self)-> String{
        match &*ptr_raw!(self) {
            Rtoken::Nil => "nil".to_string(),
            Rtoken::None => "none".to_string(),
            Rtoken::Err(s) => s.to_string(),
            Rtoken::Datatype(s) => s.to_string(),
            Rtoken::Bool(b) =>  b.to_string(),
            Rtoken::Byte(i) => i.to_string(),
            Rtoken::Char(c) => c.to_string(),
            Rtoken::Int(i) => i.to_string(),
            Rtoken::Float(f) => f.to_string(),
            Rtoken::Str(s) => "\"".to_string() + s + &"\"",
            Rtoken::Block(v) => {
                let mut result = "[".to_string();
                for item in v.iter() {
                    result += &item.to_str();
                    result += " ";
                }
                if result.len() > 1 {
                    result.pop();
                }
                result += &"]";
                return result
            },
            Rtoken::Paren(v) => {
                let mut result = "(".to_string();
                for item in v.iter() {
                    result += &item.to_str();
                    result += " ";
                }
                if result.len() > 1 {
                    result.pop();
                }
                result += &")";
                return result
            },
            Rtoken::Flow{msg, tk} => msg.clone(),
            Rtoken::Word(w) => w.key.clone(),
            Rtoken::LitWord(s) => "'".to_string() + s,
            Rtoken::SetWord(s) => s.to_string() + ":",
            Rtoken::Func(f) => "function".to_string(),
            Rtoken::Native(n) => "Native::".to_string() + n.name.as_str(),
            Rtoken::Op(n) => "Op::".to_string() + n.name.as_str(),

            _ => "Rtoken".to_string(),
        }
    }


    pub fn r_type_str(&self)-> String{
        match &*ptr_raw!(self) {
            Rtoken::Nil => "nil!".to_string(),
            Rtoken::None => "none!".to_string(),
            Rtoken::Err(_) => "err!".to_string(),
            Rtoken::Datatype(_) => "datatype!".to_string(),
            Rtoken::Bool(_) =>  "bool!".to_string(),
            Rtoken::Byte(_) => "byte!".to_string(),
            Rtoken::Char(_) => "char!".to_string(),
            Rtoken::Int(_) => "int!".to_string(),
            Rtoken::Float(_) => "float!".to_string(),
            Rtoken::Str(_) => "str!".to_string(),
            Rtoken::Block(_) => "block!".to_string(),
            Rtoken::Paren(_) => "paren!".to_string(),
            Rtoken::Flow{msg: _, tk: _} => "flow!".to_string(),
            Rtoken::Word(_) => "word!".to_string(),
            Rtoken::LitWord(_) => "lit-word!".to_string(),
            Rtoken::SetWord(_) => "set-word!".to_string(),
            Rtoken::Func(_) => "func!".to_string(),
            Rtoken::Native(_) => "native!".to_string(),
            Rtoken::Op(_) => "op!".to_string(),

            _ => "undefined!".to_string(),
        }
    }

    pub fn str_to_datatype(s: String)-> PRtoken{
        let mut tp = "undefined!";
        let str = s.as_str();
        match str {
            "nil!" => tp = str,
            "none!" => tp = str,
            "err!" => tp = str,
            "datatype!" => tp = str,
            "bool!" => tp = str,
            "byte!" => tp = str,
            "char!" => tp = str,
            "int!" => tp = str,
            "float!" => tp = str,
            "str!" => tp = str,
            "block!" => tp = str,
            "paren!" => tp = str,
            "flow!" => tp = str,
            "word!" => tp = str,
            "lit-word!" => tp = str,
            "set-word!" => tp = str,
            "func!" => tp = str,
            "native!" => tp = str,
            "op!" => tp = str,

            _ => tp = "undefined!",
        }

        PRtoken::new_datatype(tp.to_string())
    }






    pub fn get_val(&self, ctx: PRtable)-> PRtoken{
        match &*ptr_raw!(self){
            Rtoken::Word(w) => {
                let mut result: PRtoken;
                if let Some(ref x) = w.ctx {
                    result = x.get(w.key.clone()).clone()
                }else{
                    result = ctx.get(w.key.clone())
                }
                if result.is_nil(){
                    return PRtoken::new_none()
                }
                return result
            },
            Rtoken::LitWord(s) => {
                PRtoken::new_word(s.clone(), Some(ctx.clone()))
            },
            Rtoken::Paren(v) => {
                // self.echo();
                Rsolver::eval_blk_static(v.clone(), ctx.clone())
            },

            _ => self.clone()
        }

    }


    pub fn get_int(&self)-> isize{
        match *ptr_raw!(self){
            Rtoken::Int(i) => return i,
            _ => panic!("{}", "No int! token of ".to_string() + &self.to_str()),
        }
    }

    pub fn get_float(&self)-> f64{
        match *ptr_raw!(self){
            Rtoken::Float(f) => return f,
            _ => panic!("{}", "No float! token of ".to_string() + &self.to_str()),
        }
    }



    pub fn bind_ctx(&mut self, ctx: PRtable){
        match *ptr_raw_mut!(self) {
            Rtoken::Word(ref mut w) => w.set_ctx(Some(ctx)),
            Rtoken::Block(ref mut v) => {
                PRtoken::vec_bind_ctx(v, ctx.clone())
            },
            Rtoken::Paren(ref mut v) => {
                PRtoken::vec_bind_ctx(v, ctx.clone())
            }
            _ => ()
        }
    }

    pub fn vec_bind_ctx(v: &mut Vec<PRtoken>, ctx: PRtable){
        for item in v.iter_mut() {
            item.bind_ctx(ctx.clone())
        }
    }

    pub fn bind_ctx_for_key(&mut self, ctx: Option<PRtable>, key: String){
        match *ptr_raw_mut!(self) {
            Rtoken::Word(ref mut w) => {
                if w.key == key {
                    w.set_ctx(ctx)
                }
            },
            Rtoken::Block(ref mut v) => {
                match ctx {
                    Some(c) => PRtoken::vec_bind_ctx_for_key(v, Some(c), key.clone()),
                    None => PRtoken::vec_bind_ctx_for_key(v, None, key.clone())
                }
                
            },
            Rtoken::Paren(ref mut v) => {
                match ctx {
                    Some(c) => PRtoken::vec_bind_ctx_for_key(v, Some(c), key.clone()),
                    None => PRtoken::vec_bind_ctx_for_key(v, None, key.clone())
                }
            }
            _ => ()
        }
    }

    pub fn vec_bind_ctx_for_key(v: &mut Vec<PRtoken>, ctx: Option<PRtable>, key: String){
        for item in v.iter_mut() {
            match ctx {
                Some(ref c) => item.bind_ctx_for_key(Some(c.clone()), key.clone()),
                None => item.bind_ctx_for_key(None, key.clone())
            }

        }
    }

}












