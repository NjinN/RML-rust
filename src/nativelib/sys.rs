use crate::*;
use crate::lang::*;

use std::process::{exit, Command};
use std::time::{Duration, SystemTime};


pub fn quit(args: Vec<PRtoken>, ctx: PRtable)-> PRtoken{

    match *ptr_raw!(args.get(1).unwrap()){
        Rtoken::Int(i) => {
            exit(i as i32)
        },
        _ => return PRtoken::new_err("Types mismatch for native::quit".to_string())
    }

}


pub fn print(args: Vec<PRtoken>, ctx: PRtable)-> PRtoken{

    args.get(1).unwrap().echo();

    return PRtoken::new_nil()
}

pub fn typeOf(args: Vec<PRtoken>, ctx: PRtable)-> PRtoken{
    return PRtoken::str_to_datatype(args.get(1).unwrap().r_type_str());
}

pub fn cost(args: Vec<PRtoken>, ctx: PRtable)-> PRtoken{
    let now = SystemTime::now();
    match *ptr_raw!(args.get(1).unwrap()){
        Rtoken::Block(ref v) => {
           Rsolver::eval_blk_static(v.clone(), ctx.clone());
        },
        Rtoken::Str(ref s) => {
            Rsolver::eval_str_static(s.clone(), ctx.clone());
        }
        _ => return PRtoken::new_err("Types mismatch for native::cost".to_string())
    }

    match now.elapsed(){
        Ok(elapsed) => PRtoken::new_float(elapsed.as_secs_f64()),
        Err(e) => PRtoken::new_err("Cost fail!".to_string())
    }
}

pub fn clear(args: Vec<PRtoken>, ctx: PRtable)-> PRtoken{

    if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/c").arg("clear").output().expect("cmd exec error!");
    } else {
        Command::new("sh").arg("-c").arg("clear").output().expect("sh exec error!");
    }
    return PRtoken::new_nil()
}
