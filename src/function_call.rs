use std::ops::Add;
use crate::{FILE_NAME, FUNC_LABEL_NUM, push};

pub unsafe fn call(func: &str, num_args: &str) -> String{
    FUNC_LABEL_NUM += 1;
    return "@".to_string() + func + ".ReturnAddress" + FUNC_LABEL_NUM.to_string().as_str()
        + "\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n" +
        "@LCL\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n" +
        "@ARG\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n" +
        push("pointer", "0").as_str() +
        push("pointer", "1").as_str() +
        "@SP\nD=M\n@" + num_args.to_string().as_str() + "\nD=D-A\n@5\nD=D-A\n@ARG\nM=D\n" +
        "@SP\nD=M\n@LCL\nM=D\n" +
        "@" + func + "\n0;JMP\n(" + func  + ".ReturnAddress" + FUNC_LABEL_NUM.to_string().as_str() + ")\n"
}


pub unsafe fn function(func: &str, num_params: &str) -> String{
    return "(".to_string() + func + ")\n" + "@" + num_params + "\nD=A\n@" + func + ".END\nD;JEQ\n(" +
        func + ".Loop)\n@SP\nA=M\nM=0\n@SP\nM=M+1\n@" + func + ".Loop\nD=D-1;JNE\n(" +
        func + ".END)\n";
}

pub fn _return()-> String{
    return "@LCL\nD=M\n@5\nA=D-A\nD=M\n@13\nM=D\n@SP\nM=M-1\nA=M\nD=M\n@ARG\nA=M\nM=D\n".to_string() +
        "@ARG\nD=M\n@SP\nM=D+1\n@LCL\nM=M-1\nA=M\nD=M\n@THAT\nM=D\n@LCL\nM=M-1\nA=M\nD=M\n@THIS\nM=D\n" +
        "@LCL\nM=M-1\nA=M\nD=M\n@ARG\nM=D\n@LCL\nM=M-1\nA=M\nD=M\n@LCL\nM=D\n@13\nA=M\n0;JMP\n"
}
