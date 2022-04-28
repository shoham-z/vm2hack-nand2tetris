use crate::FILE_NAME;

pub unsafe fn label(label: &str) -> String{
    return "(".to_string() + FILE_NAME + "." + label + ")\n"
}

pub unsafe fn goto(label: &str) -> String{
    return "@".to_string() + FILE_NAME + "." + label + "\n0;JMP\n"
}

pub unsafe fn if_goto(label: &str) -> String{
    return "@SP\nM=M-1\nA=M\nD=M\n@".to_string() + FILE_NAME + "." + label + "\nD;JNE\n"
}