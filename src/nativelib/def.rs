use crate::*;
use crate::lang::*;


pub fn func(args: Vec<PRtoken>, ctx: PRtable)-> PRtoken{
    match *ptr_raw!(args.get(1).unwrap()){
        Rtoken::Block(ref a) => {
            match *ptr_raw!(args.get(2).unwrap()){
                Rtoken::Block(ref c) => {

                    for item in a.clone().iter(){
                        PRtoken::vec_bind_ctx_for_key(&mut c.clone(), None, item.to_str());
                    }
                    PRtoken::new_func(Rfunc::new(a.len() + 1, a.clone(), c.clone()))
                },
                _ => PRtoken::new_err("Types mismatch for native::func".to_string())
            }
        },

        _ => PRtoken::new_err("Types mismatch for native::func".to_string())
    }


}







