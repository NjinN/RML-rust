use crate::*;
use crate::lang::*;

pub fn add(args: Vec<PRtoken>, ctx: PRtable)-> PRtoken{
    let mut type_str = "".to_string();
    for item in args[1..].iter() {
        type_str += &item.r_type_str();
    }

    match type_str.as_str() {
        "int!int!" => return add_int_int(args),
        "int!float!" => return add_int_float(args),
        "float!int!" => return add_float_int(args),
        "float!float!" => return add_float_float(args),
        _ => return PRtoken::new_err("Types mismatch for native::add".to_string())
    }

}


fn add_int_int(args: Vec<PRtoken>)-> PRtoken{
    let arg1 = args.get(1).unwrap().get_int();
    let arg2 = args.get(2).unwrap().get_int();
    return PRtoken::new_int(arg1 + arg2)
}

fn add_int_float(args: Vec<PRtoken>)-> PRtoken{
    let arg1 = args.get(1).unwrap().get_int();
    let arg2 = args.get(2).unwrap().get_float();
    return PRtoken::new_float(arg1 as f64 + arg2)
}

fn add_float_int(args: Vec<PRtoken>)-> PRtoken{
    let arg1 = args.get(1).unwrap().get_float();
    let arg2 = args.get(2).unwrap().get_int();
    return PRtoken::new_float(arg1 + arg2 as f64)
}

fn add_float_float(args: Vec<PRtoken>)-> PRtoken{
    let arg1 = args.get(1).unwrap().get_float();
    let arg2 = args.get(2).unwrap().get_float();
    return PRtoken::new_float(arg1 + arg2)
}




pub fn sub(args: Vec<PRtoken>, ctx: PRtable)-> PRtoken{
    let mut type_str = "".to_string();
    for item in args[1..].iter() {
        type_str += &item.r_type_str();
    }

    match type_str.as_str() {
        "int!int!" => return sub_int_int(args),
        "int!float!" => return sub_int_float(args),
        "float!int!" => return sub_float_int(args),
        "float!float!" => return sub_float_float(args),
        _ => return PRtoken::new_err("Types mismatch for native::sub".to_string())
    }

}


fn sub_int_int(args: Vec<PRtoken>)-> PRtoken{
    let arg1 = args.get(1).unwrap().get_int();
    let arg2 = args.get(2).unwrap().get_int();
    return PRtoken::new_int(arg1 - arg2)
}

fn sub_int_float(args: Vec<PRtoken>)-> PRtoken{
    let arg1 = args.get(1).unwrap().get_int();
    let arg2 = args.get(2).unwrap().get_float();
    return PRtoken::new_float(arg1 as f64 - arg2)
}

fn sub_float_int(args: Vec<PRtoken>)-> PRtoken{
    let arg1 = args.get(1).unwrap().get_float();
    let arg2 = args.get(2).unwrap().get_int();
    return PRtoken::new_float(arg1 - arg2 as f64)
}

fn sub_float_float(args: Vec<PRtoken>)-> PRtoken{
    let arg1 = args.get(1).unwrap().get_float();
    let arg2 = args.get(2).unwrap().get_float();
    return PRtoken::new_float(arg1 - arg2)
}





pub fn mul(args: Vec<PRtoken>, ctx: PRtable)-> PRtoken{
    let mut type_str = "".to_string();
    for item in args[1..].iter() {
        type_str += &item.r_type_str();
    }

    match type_str.as_str() {
        "int!int!" => return mul_int_int(args),
        "int!float!" => return mul_int_float(args),
        "float!int!" => return mul_float_int(args),
        "float!float!" => return mul_float_float(args),
        _ => return PRtoken::new_err("Types mismatch for native::mul".to_string())
    }

}


fn mul_int_int(args: Vec<PRtoken>)-> PRtoken{
    let arg1 = args.get(1).unwrap().get_int();
    let arg2 = args.get(2).unwrap().get_int();
    return PRtoken::new_int(arg1 * arg2)
}

fn mul_int_float(args: Vec<PRtoken>)-> PRtoken{
    let arg1 = args.get(1).unwrap().get_int();
    let arg2 = args.get(2).unwrap().get_float();
    return PRtoken::new_float(arg1 as f64 * arg2)
}

fn mul_float_int(args: Vec<PRtoken>)-> PRtoken{
    let arg1 = args.get(1).unwrap().get_float();
    let arg2 = args.get(2).unwrap().get_int();
    return PRtoken::new_float(arg1 * arg2 as f64)
}

fn mul_float_float(args: Vec<PRtoken>)-> PRtoken{
    let arg1 = args.get(1).unwrap().get_float();
    let arg2 = args.get(2).unwrap().get_float();
    return PRtoken::new_float(arg1 * arg2)
}





pub fn div(args: Vec<PRtoken>, ctx: PRtable)-> PRtoken{
    let mut type_str = "".to_string();
    for item in args[1..].iter() {
        type_str += &item.r_type_str();
    }

    match type_str.as_str() {
        "int!int!" => return div_int_int(args),
        "int!float!" => return div_int_float(args),
        "float!int!" => return div_float_int(args),
        "float!float!" => return div_float_float(args),
        _ => return PRtoken::new_err("Types mismatch for native::div".to_string())
    }

}


fn div_int_int(args: Vec<PRtoken>)-> PRtoken{
    let arg1 = args.get(1).unwrap().get_int();
    let arg2 = args.get(2).unwrap().get_int();
    return PRtoken::new_float(arg1 as f64/ arg2 as f64)
}

fn div_int_float(args: Vec<PRtoken>)-> PRtoken{
    let arg1 = args.get(1).unwrap().get_int();
    let arg2 = args.get(2).unwrap().get_float();
    return PRtoken::new_float(arg1 as f64 / arg2)
}

fn div_float_int(args: Vec<PRtoken>)-> PRtoken{
    let arg1 = args.get(1).unwrap().get_float();
    let arg2 = args.get(2).unwrap().get_int();
    return PRtoken::new_float(arg1 / arg2 as f64)
}

fn div_float_float(args: Vec<PRtoken>)-> PRtoken{
    let arg1 = args.get(1).unwrap().get_float();
    let arg2 = args.get(2).unwrap().get_float();
    return PRtoken::new_float(arg1 / arg2)
}









