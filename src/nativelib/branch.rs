use crate::*;
use crate::lang::*;


pub fn rif(args: Vec<PRtoken>, ctx: PRtable)-> PRtoken{
    match *ptr_raw!(args.get(1).unwrap()) {
        Rtoken::Bool(b) => {
            match *ptr_raw!(args.get(2).unwrap()){
                Rtoken::Block(ref v) => {
                    if b {
                        return Rsolver::eval_blk_static(v.clone(), ctx.clone());
                    }
                    PRtoken::new_nil()
                },

                _ => PRtoken::new_err("Types mismatch for native::if".to_string())
            }
        },
        _ => PRtoken::new_err("Types mismatch for native::if".to_string())
    }
}

pub fn either(args: Vec<PRtoken>, ctx: PRtable)-> PRtoken{
    match *ptr_raw!(args.get(1).unwrap()) {
        Rtoken::Bool(b) => {
            match *ptr_raw!(args.get(2).unwrap()){
                Rtoken::Block(ref v1) => {
                   match *ptr_raw!(args.get(3).unwrap()){
                        Rtoken::Block(ref v2) => {
                            if b {
                                return Rsolver::eval_blk_static(v1.clone(), ctx.clone());
                            }else{
                                return Rsolver::eval_blk_static(v2.clone(), ctx.clone());
                            }
                        },

                        _ => PRtoken::new_err("Types mismatch for native::if".to_string())
                   }
                },

                _ => PRtoken::new_err("Types mismatch for native::if".to_string())
            }
        },
        _ => PRtoken::new_err("Types mismatch for native::if".to_string())
    }
}