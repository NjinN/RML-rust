use crate::*;
use crate::lang::*;
use crate::nativelib::*;

pub struct InitNative();

impl InitNative {
    pub fn init(ctx: PRtable){
        ctx.clone().put_now("_quit".to_string(), PRtoken::new_native(Rnative::new("_quit".to_string(), 2, quit)));
        ctx.clone().put_now("print".to_string(), PRtoken::new_native(Rnative::new("print".to_string(), 2, print)));
        ctx.clone().put_now("clear".to_string(), PRtoken::new_native(Rnative::new("clear".to_string(), 1, clear)));
        ctx.clone().put_now("type?".to_string(), PRtoken::new_native(Rnative::new("type?".to_string(), 2, typeOf)));
        ctx.clone().put_now("cost".to_string(), PRtoken::new_native(Rnative::new("cost".to_string(), 2, cost)));

        ctx.clone().put_now("break".to_string(), PRtoken::new_native(Rnative::new("break".to_string(), 1, rbreak)));
        ctx.clone().put_now("continue".to_string(), PRtoken::new_native(Rnative::new("continue".to_string(), 1, rcontinue)));
        ctx.clone().put_now("loop".to_string(), PRtoken::new_native(Rnative::new("loop".to_string(), 3, rloop)));
        ctx.clone().put_now("repeat".to_string(), PRtoken::new_native(Rnative::new("repeat".to_string(), 4, repeat)));

        ctx.clone().put_now("add".to_string(), PRtoken::new_native(Rnative::new("add".to_string(), 3, add)));
        ctx.clone().put_now("sub".to_string(), PRtoken::new_native(Rnative::new("sub".to_string(), 3, sub)));
        ctx.clone().put_now("mul".to_string(), PRtoken::new_native(Rnative::new("mul".to_string(), 3, mul)));
        ctx.clone().put_now("div".to_string(), PRtoken::new_native(Rnative::new("div".to_string(), 3, div)));

        ctx.clone().put_now("eq".to_string(), PRtoken::new_native(Rnative::new("eq".to_string(), 3, eq)));
        ctx.clone().put_now("lt".to_string(), PRtoken::new_native(Rnative::new("lt".to_string(), 3, lt)));
        ctx.clone().put_now("gt".to_string(), PRtoken::new_native(Rnative::new("gt".to_string(), 3, gt)));
        ctx.clone().put_now("le".to_string(), PRtoken::new_native(Rnative::new("le".to_string(), 3, le)));
        ctx.clone().put_now("ge".to_string(), PRtoken::new_native(Rnative::new("ge".to_string(), 3, ge)));

        ctx.clone().put_now("func".to_string(), PRtoken::new_native(Rnative::new("func".to_string(), 3, func)));
        
        ctx.clone().put_now("if".to_string(), PRtoken::new_native(Rnative::new("if".to_string(), 3, rif)));
        ctx.clone().put_now("either".to_string(), PRtoken::new_native(Rnative::new("either".to_string(), 4, either)));

    }
}



