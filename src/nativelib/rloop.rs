use crate::*;
use crate::lang::*;


pub fn rbreak(args: Vec<PRtoken>, ctx: PRtable)-> PRtoken{
    return PRtoken::new_flow("break".to_string(), None)
}

pub fn rcontinue(args: Vec<PRtoken>, ctx: PRtable)-> PRtoken{
    return PRtoken::new_flow("continue".to_string(), None)
}

pub fn rloop(args: Vec<PRtoken>, ctx: PRtable)-> PRtoken{
    let mut idx = 0;
    match *ptr_raw!(args.get(1).unwrap()){
        Rtoken::Int(i) => {
            let mut temp = PRtoken::new_nil();
            match *ptr_raw!(args.get(2).unwrap()){
                Rtoken::Block(ref v) => {
                    while idx < i {
                        temp = Rsolver::eval_blk_static(v.clone(), ctx.clone());
                        idx += 1;
                        match *ptr_raw!(temp) {
                            Rtoken::Flow{ref msg, ..} => {
                                match msg.as_str(){
                                    "break" => break,
                                    "continue" => continue,
                                    _ => ()
                                }
                            },
                            _ => (),
                        }
                    }
                    PRtoken::new_nil()
                },
                Rtoken::Str(ref s) => {
                    while idx < i {
                        temp = Rsolver::eval_str_static(s.clone(), ctx.clone());
                        idx += 1;
                        match *ptr_raw!(temp) {
                            Rtoken::Flow{ref msg, ..} => {
                                match msg.as_str(){
                                    "break" => break,
                                    "continue" => continue,
                                    _ => ()
                                }
                            },
                            _ => (),
                        }
                    }
                    PRtoken::new_nil()
                },

                _ => PRtoken::new_err("Types mismatch for native::loop".to_string())
            }
        }
        _ => PRtoken::new_err("Types mismatch for native::loop".to_string())
    }

}


pub fn repeat(args: Vec<PRtoken>, ctx: PRtable)-> PRtoken{
    match *ptr_raw!(args.get(1).unwrap()) {
        Rtoken::Word(ref w) => {
            match *ptr_raw!(args.get(2).unwrap()){
                Rtoken::Int(i) => {
                    let mut temp = PRtoken::new_nil();
                    let mut rctx = PRtable::new(RtableType::TMP_TABLE);
                    let mut cond = PRtoken::new_int(1);
                    rctx.set_father(ctx.clone());
                    rctx.put_now(w.key.clone(), cond.clone());
                    match *ptr_raw!(args.get(3).unwrap()){
                        Rtoken::Block(ref v) => {
                            let mut code = v.clone();
                            PRtoken::vec_bind_ctx_for_key(&mut code, Some(rctx.clone()), w.key.clone());
                            while cond.get_int() <= i {
                                temp = Rsolver::eval_blk_static(code.clone(), rctx.clone());
                                cond = rctx.get_now(w.key.clone());
                                if !cond.is_int(){
                                    break
                                }
                                cond = PRtoken::new_int(cond.get_int() + 1);
                                rctx.put_now(w.key.clone(), cond.clone());
                                match *ptr_raw!(temp) {
                                    Rtoken::Flow{ref msg, ..} => {
                                        match msg.as_str(){
                                            "break" => break,
                                            "continue" => continue,
                                            _ => ()
                                        }
                                    },
                                    _ => (),
                                }
                            }

                            return PRtoken::new_nil()
                        },
                        Rtoken::Str(ref s) => {
                            while cond.get_int() <= i {
                                temp = Rsolver::eval_str_static(s.clone(), rctx.clone());
                                cond = rctx.get_now(w.key.clone());
                                if !cond.is_int(){
                                    break
                                }
                                cond = PRtoken::new_int(cond.get_int() + 1);
                                rctx.put_now(w.key.clone(), cond.clone());
                                match *ptr_raw!(temp) {
                                    Rtoken::Flow{ref msg, ..} => {
                                        match msg.as_str(){
                                            "break" => break,
                                            "continue" => continue,
                                            _ => ()
                                        }
                                    },
                                    _ => (),
                                }
                            }

                            return PRtoken::new_nil()
                        },
                        _ => PRtoken::new_err("Types mismatch for native::repeat".to_string())
                    }
                },
                _ => PRtoken::new_err("Types mismatch for native::repeat".to_string())
            }
        },
        _ =>  PRtoken::new_err("Types mismatch for native::repeat".to_string())
    }
}






















