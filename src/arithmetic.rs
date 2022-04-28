use crate::{LABEL_NUM};

/// Returns the commands for binary operations in the hack assembly language
///
/// # Arguments
///
/// * `op` - A string with the operation
///
/// # Example
/// Add, Sub, And, Or
#[inline(always)]
pub fn write_binary_op(op: &str) -> String{
    return "@SP\nAM=M-1\nD=M\n@SP\nAM=M-1\nD=M".to_string() + op + "D\nM=D\n@SP\nM=M+1\n"
}

/// Returns the commands for unary operations in the hack assembly language
///
/// # Arguments
///
/// * `op` - A string with the operation
///
/// # Example
/// Neg, Not
#[inline(always)]
pub fn write_unary_op(op: &str) -> String{
    return "@SP\nA=M-1\nM=".to_string() + op + "M\n"
}


/// Returns the commands for unary operations in the hack assembly language
///
/// # Arguments
///
/// * `comp` - A string with the comparison operation
///
/// # Example
/// JEQ ,JGT ,JLT
#[inline(always)]
pub unsafe fn comparison(comp: &str) -> String{
    LABEL_NUM +=1;
    let label_num = LABEL_NUM.to_string().to_owned();
    return "@SP\nAM=M-1\nD=M\nAM=M-1\nD=M-D\n@IF_TRUE".to_string() + &label_num
        + "\nD;J" + comp + "\n@SP\nA=M\nM=0\n@IF_FALSE" +  &label_num
        + "\n0;JMP\n(IF_TRUE" + &label_num + ")\n@SP\nA=M\nM=-1\n(IF_FALSE" +  &label_num
        + ")\n@SP\nM=M+1\n"
}

