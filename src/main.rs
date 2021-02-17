mod lang;
use lang::*;

mod nativelib;
use nativelib::*;

mod oplib;
use oplib::*;

use std::io;
use std::io::Write;


fn main(){
    let mut lib_ctx = PRtable::new(RtableType::SYS_TABLE);

    InitNative::init(lib_ctx.clone());
    InitOp::init(lib_ctx.clone());

    let mut usr_ctx = PRtable::new(RtableType::USR_TABLE);
    usr_ctx.set_father(lib_ctx.clone());

    let mut main_solver = Rsolver::new();
    main_solver.clear();

    loop {
        print!("{}", ">> ");
        io::stdout().flush().unwrap();

        let mut buf = String::new();
        io::stdin().read_line(&mut buf);

        let result = main_solver.eval_str(buf, usr_ctx.clone());

        match *ptr_raw!(result) {
            Rtoken::Nil => (),
            _ => result.echo(),
        }

        println!("{}", "");
    }

}


// fn main() {
//     println!("Hello, world!");
//     println!("{:?}", StrKit::cut_strs("123 456 [789] \"hello world\"".to_string().chars().collect()));
//     PRtoken::new_word("token".to_string(), None).echo();
//     RtokenKit::make_rtoken("abc:".to_string(), None).echo();
//     println!("{:?}", PRtable::new(RtableType::SYS_TABLE))
// }
