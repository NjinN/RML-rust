use crate::*;
use crate::lang::*;

pub fn eq(args: Vec<PRtoken>, ctx: PRtable)-> PRtoken{
    let mut type_str = "".to_string();
    for item in args[1..].iter() {
        type_str += &item.r_type_str();
    }

    match type_str.as_str() {
        "int!int!" => return eq_int_int(args),
        "int!float!" => return eq_int_float(args),
        "float!int!" => return eq_float_int(args),
        "float!float!" => return eq_float_float(args),
        _ => return PRtoken::new_err("Types mismatch for native::eq".to_string())
    }

}


fn eq_int_int(args: Vec<PRtoken>)-> PRtoken{
    let arg1 = args.get(1).unwrap().get_int();
    let arg2 = args.get(2).unwrap().get_int();
    return PRtoken::new_bool(arg1 == arg2)
}

fn eq_int_float(args: Vec<PRtoken>)-> PRtoken{
    let arg1 = args.get(1).unwrap().get_int();
    let arg2 = args.get(2).unwrap().get_float();
    return PRtoken::new_bool(arg1 as f64 == arg2)
}

fn eq_float_int(args: Vec<PRtoken>)-> PRtoken{
    let arg1 = args.get(1).unwrap().get_float();
    let arg2 = args.get(2).unwrap().get_int();
    return PRtoken::new_bool(arg1 == arg2 as f64)
}

fn eq_float_float(args: Vec<PRtoken>)-> PRtoken{
    let arg1 = args.get(1).unwrap().get_float();
    let arg2 = args.get(2).unwrap().get_float();
    return PRtoken::new_bool(arg1 == arg2)
}




pub fn lt(args: Vec<PRtoken>, ctx: PRtable)-> PRtoken{
    let mut type_str = "".to_string();
    for item in args[1..].iter() {
        type_str += &item.r_type_str();
    }

    match type_str.as_str() {
        "int!int!" => return lt_int_int(args),
        "int!float!" => return lt_int_float(args),
        "float!int!" => return lt_float_int(args),
        "float!float!" => return lt_float_float(args),
        _ => return PRtoken::new_err("Types mismatch for native::lt".to_string())
    }

}


fn lt_int_int(args: Vec<PRtoken>)-> PRtoken{
    let arg1 = args.get(1).unwrap().get_int();
    let arg2 = args.get(2).unwrap().get_int();
    return PRtoken::new_bool(arg1 < arg2)
}

fn lt_int_float(args: Vec<PRtoken>)-> PRtoken{
    let arg1 = args.get(1).unwrap().get_int();
    let arg2 = args.get(2).unwrap().get_float();
    return PRtoken::new_bool((arg1 as f64) < arg2)
}

fn lt_float_int(args: Vec<PRtoken>)-> PRtoken{
    let arg1 = args.get(1).unwrap().get_float();
    let arg2 = args.get(2).unwrap().get_int();
    return PRtoken::new_bool(arg1 < arg2 as f64)
}

fn lt_float_float(args: Vec<PRtoken>)-> PRtoken{
    let arg1 = args.get(1).unwrap().get_float();
    let arg2 = args.get(2).unwrap().get_float();
    return PRtoken::new_bool(arg1 < arg2)
}




pub fn gt(args: Vec<PRtoken>, ctx: PRtable)-> PRtoken{
    let mut type_str = "".to_string();
    for item in args[1..].iter() {
        type_str += &item.r_type_str();
    }

    match type_str.as_str() {
        "int!int!" => return gt_int_int(args),
        "int!float!" => return gt_int_float(args),
        "float!int!" => return gt_float_int(args),
        "float!float!" => return gt_float_float(args),
        _ => return PRtoken::new_err("Types mismatch for native::gt".to_string())
    }

}


fn gt_int_int(args: Vec<PRtoken>)-> PRtoken{
    let arg1 = args.get(1).unwrap().get_int();
    let arg2 = args.get(2).unwrap().get_int();
    return PRtoken::new_bool(arg1 > arg2)
}

fn gt_int_float(args: Vec<PRtoken>)-> PRtoken{
    let arg1 = args.get(1).unwrap().get_int();
    let arg2 = args.get(2).unwrap().get_float();
    return PRtoken::new_bool(arg1 as f64 > arg2)
}

fn gt_float_int(args: Vec<PRtoken>)-> PRtoken{
    let arg1 = args.get(1).unwrap().get_float();
    let arg2 = args.get(2).unwrap().get_int();
    return PRtoken::new_bool(arg1 > arg2 as f64)
}

fn gt_float_float(args: Vec<PRtoken>)-> PRtoken{
    let arg1 = args.get(1).unwrap().get_float();
    let arg2 = args.get(2).unwrap().get_float();
    return PRtoken::new_bool(arg1 > arg2)
}



pub fn le(args: Vec<PRtoken>, ctx: PRtable)-> PRtoken{
    let mut type_str = "".to_string();
    for item in args[1..].iter() {
        type_str += &item.r_type_str();
    }

    match type_str.as_str() {
        "int!int!" => return le_int_int(args),
        "int!float!" => return le_int_float(args),
        "float!int!" => return le_float_int(args),
        "float!float!" => return le_float_float(args),
        _ => return PRtoken::new_err("Types mismatch for native::le".to_string())
    }

}


fn le_int_int(args: Vec<PRtoken>)-> PRtoken{
    let arg1 = args.get(1).unwrap().get_int();
    let arg2 = args.get(2).unwrap().get_int();
    return PRtoken::new_bool(arg1 <= arg2)
}

fn le_int_float(args: Vec<PRtoken>)-> PRtoken{
    let arg1 = args.get(1).unwrap().get_int();
    let arg2 = args.get(2).unwrap().get_float();
    return PRtoken::new_bool(arg1 as f64 <= arg2)
}

fn le_float_int(args: Vec<PRtoken>)-> PRtoken{
    let arg1 = args.get(1).unwrap().get_float();
    let arg2 = args.get(2).unwrap().get_int();
    return PRtoken::new_bool(arg1 <= arg2 as f64)
}

fn le_float_float(args: Vec<PRtoken>)-> PRtoken{
    let arg1 = args.get(1).unwrap().get_float();
    let arg2 = args.get(2).unwrap().get_float();
    return PRtoken::new_bool(arg1 <= arg2)
}




pub fn ge(args: Vec<PRtoken>, ctx: PRtable)-> PRtoken{
    let mut type_str = "".to_string();
    for item in args[1..].iter() {
        type_str += &item.r_type_str();
    }

    match type_str.as_str() {
        "int!int!" => return ge_int_int(args),
        "int!float!" => return ge_int_float(args),
        "float!int!" => return ge_float_int(args),
        "float!float!" => return ge_float_float(args),
        _ => return PRtoken::new_err("Types mismatch for native::ge".to_string())
    }

}


fn ge_int_int(args: Vec<PRtoken>)-> PRtoken{
    let arg1 = args.get(1).unwrap().get_int();
    let arg2 = args.get(2).unwrap().get_int();
    return PRtoken::new_bool(arg1 >= arg2)
}

fn ge_int_float(args: Vec<PRtoken>)-> PRtoken{
    let arg1 = args.get(1).unwrap().get_int();
    let arg2 = args.get(2).unwrap().get_float();
    return PRtoken::new_bool(arg1 as f64 >= arg2)
}

fn ge_float_int(args: Vec<PRtoken>)-> PRtoken{
    let arg1 = args.get(1).unwrap().get_float();
    let arg2 = args.get(2).unwrap().get_int();
    return PRtoken::new_bool(arg1 >= arg2 as f64)
}

fn ge_float_float(args: Vec<PRtoken>)-> PRtoken{
    let arg1 = args.get(1).unwrap().get_float();
    let arg2 = args.get(2).unwrap().get_float();
    return PRtoken::new_bool(arg1 >= arg2)
}