use crate::*;
use crate::lang::*;


#[derive(Debug, Clone)]
pub struct Rfunc {
    pub expr_len: usize,
    pub args: Vec<PRtoken>,
    pub code: Vec<PRtoken>
}

impl Rfunc {
    pub fn new(l: usize, a: Vec<PRtoken>, c: Vec<PRtoken>)-> Rfunc{
        Rfunc{expr_len: l, args: a, code: c}
    }


    pub fn exec(&self, act_args: Vec<PRtoken>, ctx: PRtable)-> PRtoken{
        let mut fctx = PRtable::new(RtableType::USR_TABLE);
        fctx.set_father(ctx.clone());
        let mut idx = 0;
        while idx < self.args.len() {
            fctx.put_now(self.args.get(idx).unwrap().to_str(), act_args.get(idx).unwrap().clone());
            idx += 1;
        }
        
        Rsolver::eval_blk_static(self.code.clone(), fctx)
    }

}
















