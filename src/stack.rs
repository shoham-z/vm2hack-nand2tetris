use crate::{FILE_NAME, STATIC_BASE, TEMP_BASE};

pub unsafe fn push(segment: &str, index: &str) -> String{
    let num = index.parse::<i64>().unwrap();
    return match segment{
        "constant" => push_constant(num),

        "local" => push_arg_lcl_this_that("LCL", num),

        "argument" => push_arg_lcl_this_that("ARG", num),

        "this" => push_arg_lcl_this_that("THIS", num),

        "that" => push_arg_lcl_this_that("THAT", num),

        "temp" => push_temp(num),

        "pointer" => push_pointer(num),

        "static" => push_static(num),

        _ => panic!("Invalid Command Found!"),
    }
}

fn push_constant(num: i64) -> String{
    return "@".to_string() + num.to_string().as_str() +
        "\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
}

fn push_arg_lcl_this_that(which: &str, num: i64) -> String{
    return "@".to_string() + num.to_string().as_str() + "\nD=A\n@" + which.to_string().as_str() +
        "\nA=M+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1"
}

fn push_pointer(num: i64) -> String {
    return match num {
        0 => "@THIS\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n".to_string(),

        1 => "@THAT\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n".to_string(),

        _ => "".to_string()
    };
}

fn push_temp(num: i64) -> String{
    if num > 7 || num < 0 {
        panic!("INVALID 'pop temp num' - when num > 7 or num < 0")
    }
    return "@".to_string() + (TEMP_BASE + num).to_string().as_str() + "\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n";
}

unsafe fn push_static(num:i64) -> String{
    return "@".to_string() + FILE_NAME + "." + num.to_string().as_str()
        + "\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n"
}



pub unsafe fn pop(segment: &str, index: &str) -> String {
    let num = index.parse::<i64>().unwrap();

    match segment {
        "local" => pop_arg_lcl_this_that("LCL", num),

        "argument" => pop_arg_lcl_this_that("ARG", num),

        "this" => pop_arg_lcl_this_that("THIS", num),

        "that" => pop_arg_lcl_this_that("THAT", num),

        "temp" => pop_temp(num),

        "pointer" => pop_pointer(num),

        "static" => pop_static(num),

        _ => panic!("Invalid Command Found!"),
    }
}


fn pop_arg_lcl_this_that(which: &str, num: i64) -> String{
    let mut s: String = "@SP\nA=M-1\nD=M\n@".to_string() + which + "\nA=M\n";
    for _i in 0..num{
        s = s + "A=A+1\n"
    }
    return s + "M=D\n@SP\nM=M-1\n"
}

fn pop_temp(num: i64) -> String {
    if num > 7 || num < 0 {
        panic!("INVALID 'pop temp num' - when num > 7 or num < 0")
    }
    return  "@SP\nA=M-1\nD=M\n@".to_string() +
        num.to_string().as_str() +
        "\nA=A+1\nA=A+1\nA=A+1\nA=A+1\nA=A+1\n" +
        "M=D\n@SP\nM=M-1\n"

}

fn pop_pointer(num: i64) -> String {
    return match num {
        0 => "@SP\nA=M-1\nD=M\n@THIS\nM=D\n@SP\nM=M-1\n".to_string(),
        1 => "@SP\nA=M-1\nD=M\n@THAT\nM=D\n@SP\nM=M-1\n".to_string(),
        _ => "".to_string()
    }
}

unsafe fn pop_static(num: i64) -> String {
    return "@SP\nM=M-1\nA=M\nD=M\n@".to_string() + FILE_NAME + "." + num.to_string().as_str() + "\nM=D\n";
}