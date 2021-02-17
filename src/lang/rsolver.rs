use crate::*;
use crate::lang::*;

#[derive(Debug)]
pub enum RevalModel {
    Model_Blk,
    Model_Str,
}

#[derive(Debug)]
pub struct Rsolver {
    pub inp_strs: Vec<String>,
    pub inp_blk: Vec<PRtoken>,
    pub inp_len: usize,
    pub idx: usize,
    pub ans_tk: PRtoken,
    pub model: RevalModel
}

impl Rsolver {
    pub fn new()-> Rsolver{
        Rsolver{
            inp_strs: vec![],
            inp_blk: vec![],
            inp_len: 0,
            idx: 0,
            ans_tk: PRtoken::new_nil(),
            model: RevalModel::Model_Blk
        }
    }

    pub fn clear(&mut self){
        self.inp_strs = vec![];
        self.inp_blk = vec![];
        self.inp_len = 0;
        self.idx = 0;
        self.ans_tk = PRtoken::new_nil();
    }

    pub fn input_strs(&mut self, inp: Vec<String>){
        self.inp_strs = inp.clone();
        self.inp_len = inp.len();
        self.idx = 0;
        self.ans_tk = PRtoken::new_nil();
        self.model = RevalModel::Model_Str;
    }

    pub fn input_blk(&mut self, inp: Vec<PRtoken>){
        self.inp_blk = inp.clone();
        self.inp_len = inp.len();
        self.idx = 0;
        self.ans_tk = PRtoken::new_nil();
        self.model = RevalModel::Model_Blk;
    }

    pub fn eval_str_static(s: String, ctx: PRtable)-> PRtoken{
        let mut result = PRtoken::new_nil();
        let strs = StrKit::cut_strs(s.chars().collect());
        let mut solver = Rsolver::new();
        solver.input_strs(strs);
        while solver.idx < solver.inp_len {
            result = solver.eval_one(ctx.clone());
            match *ptr_raw!(result) {
                Rtoken::Err(_) => return result.clone(),
                Rtoken::Flow{ref msg, ref tk} => return result.clone(),
                _ => (),
            }
        }

        return result
    }

    pub fn eval_str(&mut self, s: String, ctx: PRtable)-> PRtoken{
        let mut result = PRtoken::new_nil();
        let strs = StrKit::cut_strs(s.chars().collect());
        self.input_strs(strs);
        while self.idx < self.inp_len {
            result = self.eval_one(ctx.clone());
            match *ptr_raw!(result) {
                Rtoken::Err(_) => return result.clone(),
                Rtoken::Flow{ref msg, ref tk} => return result.clone(),
                _ => (),
            }
        }

        return result
    }

    pub fn eval_blk_static(blk: Vec<PRtoken>, ctx: PRtable)-> PRtoken{
        let mut result = PRtoken::new_nil();
        let mut solver = Rsolver::new();
        solver.input_blk(blk);
        while solver.idx < solver.inp_len {
            result = solver.eval_one(ctx.clone());
            match *ptr_raw!(result) {
                Rtoken::Err(_) => return result.clone(),
                Rtoken::Flow{ref msg, ref tk} => return result.clone(),
                _ => (),
            }
        }

        return result
    }

    pub fn eval_blk(&mut self, blk: Vec<PRtoken>, ctx: PRtable)-> PRtoken{
        let mut result = PRtoken::new_nil();
        self.input_blk(blk);
        while self.idx < self.inp_len {
            result = self.eval_one(ctx.clone());
            match *ptr_raw!(result) {
                Rtoken::Err(_) => return result.clone(),
                Rtoken::Flow{ref msg, ref tk} => return result.clone(),
                _ => (),
            }
        }

        return result
    }

    pub fn reduce_str_static(s: String, ctx: PRtable)-> Vec<PRtoken>{
        let mut result : Vec<PRtoken> = vec![];
        let strs = StrKit::cut_strs(s.chars().collect());
        let mut solver = Rsolver::new();
        solver.input_strs(strs);
        let pre_ans = solver.ans_tk.clone();
        while solver.idx < solver.inp_len {
            let temp = solver.eval_one(ctx.clone());
            match &*ptr_raw!(pre_ans) {
                Rtoken::Op(_)=> {result.pop();},
                _ => (),
            }
            result.push(temp);
        }

        return result
    }

    pub fn reduce_str(&mut self, s: String, ctx: PRtable)-> Vec<PRtoken>{
        let mut result : Vec<PRtoken> = vec![];
        let strs = StrKit::cut_strs(s.chars().collect());
        self.input_strs(strs);
        let pre_ans = self.ans_tk.clone();
        while self.idx < self.inp_len {
            let temp = self.eval_one(ctx.clone());
            match &*ptr_raw!(pre_ans) {
                Rtoken::Op(_)=> {result.pop();},
                _ => (),
            }
            result.push(temp);
        }

        return result
    }

    pub fn reduce_blk_static(blk: Vec<PRtoken>, ctx: PRtable)-> Vec<PRtoken>{
        let mut result : Vec<PRtoken> = vec![];
        let mut solver = Rsolver::new();
        solver.input_blk(blk);
        let pre_ans = solver.ans_tk.clone();
        while solver.idx < solver.inp_len {
            let temp = solver.eval_one(ctx.clone());
            match &*ptr_raw!(pre_ans) {
                Rtoken::Op(_)=> {result.pop();},
                _ => (),
            }
            result.push(temp);
        }

        return result
    }

    pub fn reduce_blk(&mut self, blk: Vec<PRtoken>, ctx: PRtable)-> Vec<PRtoken>{
        let mut result : Vec<PRtoken> = vec![];
        self.input_blk(blk);
        let pre_ans = self.ans_tk.clone();
        while self.idx < self.inp_len {
            let temp = self.eval_one(ctx.clone());
            match &*ptr_raw!(pre_ans) {
                Rtoken::Op(_)=> {result.pop();},
                _ => (),
            }
            result.push(temp);
        }

        return result
    }


    pub fn eval_one(&mut self, ctx: PRtable)-> PRtoken{
        let mut curr_tk : PRtoken;
        match self.model {
            RevalModel::Model_Blk => {
                curr_tk = self.inp_blk.get(self.idx).unwrap().clone();
            },
            RevalModel::Model_Str => {
                curr_tk = RtokenKit::make_rtoken(self.inp_strs.get(self.idx).unwrap().clone(), Some(ctx.clone()))
            }
        } 

        self.idx += 1;
        curr_tk = curr_tk.get_val(ctx.clone());
        // curr_tk.echo();

        let mut result = PRtoken::new_nil();

        match *ptr_raw!(curr_tk) {
            Rtoken::SetWord(ref s) => {
                let temp = self.eval_one(ctx.clone());
                ctx.clone().put(s.clone(), temp.clone());
                result = temp;
            },
            Rtoken::Native(ref n) => {
                let mut expr = vec![curr_tk.clone()];
                self.eval_n(ctx.clone(), &mut expr, n.expr_len);
                if expr.len() < n.expr_len {
                    return PRtoken::new_err("Incomplete expression!".to_string())
                }
                let native = n.exec;
                result = native(expr, ctx.clone());
            },

            Rtoken::Func(ref f) => {
                let mut args = vec![];
                self.eval_n(ctx.clone(), &mut args, f.expr_len - 1);
                if args.len() < f.expr_len - 1 {
                    return PRtoken::new_err("Incomplete expression!".to_string())
                }
                result = f.exec(args, ctx.clone());
            },

            Rtoken::Op(ref n) => {
                let mut expr = vec![curr_tk.clone(), self.ans_tk.clone()];
                self.eval_n(ctx.clone(), &mut expr, n.expr_len);
                let native = n.exec;
                result = native(expr, ctx.clone());
            }

            _ => {
                result = curr_tk.clone()
            }
        }

        self.ans_tk = result.clone();
        return result
    }

    pub fn eval_n(&mut self, ctx: PRtable, blk: &mut Vec<PRtoken>, num: usize){
        while blk.len() < num && self.idx < self.inp_len {
            blk.push(self.eval_one(ctx.clone()))
        }
    }

}












