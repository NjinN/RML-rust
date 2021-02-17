use crate::*;
use crate::lang::*;
use crate::nativelib::*;

pub struct InitOp();

impl InitOp {
    pub fn init(ctx: PRtable){
        ctx.clone().put_now("+".to_string(), PRtoken::new_op(Rnative::new("+".to_string(), 3, add)));
        ctx.clone().put_now("-".to_string(), PRtoken::new_op(Rnative::new("-".to_string(), 3, sub)));
        ctx.clone().put_now("*".to_string(), PRtoken::new_op(Rnative::new("*".to_string(), 3, mul)));
        ctx.clone().put_now("/".to_string(), PRtoken::new_op(Rnative::new("/".to_string(), 3, div)));

        ctx.clone().put_now("=".to_string(), PRtoken::new_op(Rnative::new("=".to_string(), 3, eq)));
        ctx.clone().put_now("<".to_string(), PRtoken::new_op(Rnative::new("<".to_string(), 3, lt)));
        ctx.clone().put_now(">".to_string(), PRtoken::new_op(Rnative::new(">".to_string(), 3, gt)));
        ctx.clone().put_now("<=".to_string(), PRtoken::new_op(Rnative::new("<=".to_string(), 3, le)));
        ctx.clone().put_now(">=".to_string(), PRtoken::new_op(Rnative::new(">=".to_string(), 3, ge)));
    }
}