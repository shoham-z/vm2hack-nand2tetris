/*
student 1: Shoham Zeharya, id 325177533, #group 42
student 2: Gur Arie Leviav, id 325699890, #group 42
 */

mod stack;
mod constants;
mod arithmetic;
mod program_flow;
mod function_call;

use stack::*;
use constants::*;
use arithmetic::*;
use program_flow::*;
use function_call::*;

use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;


fn main(){
    let args: Vec<String> = env::args().collect();
    let asm_file_path = create_asm(args[1].as_str());
    let mut asm_file = match File::create(&asm_file_path) {
        Err(why) => panic!("couldn't create {}: {}", asm_file_path.to_owned()+".asm", why),
        Ok(file) => file,
    };
    println!("{:?}", args[1]);
    let vm_files:Vec<String> = search_vm_files(args[1].as_str());
    println!("{}", vm_files.len());
    if vm_files.len() > 1{
        write!(asm_file, "@256\nD=A\n@SP\nM=D\n{}", sys_init());
    }
    for file in vm_files{
        unsafe {
            update_FILE_NAME(file.as_ptr(), file.len());

            let contents = fs::read_to_string(&file)
                .expect("Something went wrong reading the file");
            let lines: Vec<&str> = contents.split("\n").collect();
            for line in lines {
                if !line.starts_with(r"//") && line.len() > 1 {
                    println!("translated line: {}\n", line);

                    write!(asm_file, "//{}\n\n{}\n\n",line,  parse(line.to_string()));
                }
            }
        }
    }
}


unsafe fn update_FILE_NAME(ptr: *const u8, len: usize) {
    FUNC_LABEL_NUM = -1;
    let path = std::str::from_utf8(std::slice::from_raw_parts(ptr, len)).unwrap();
    let lst: Vec<&str> = path.split("\\").collect();
    let filename: Vec<&str> = lst[lst.len() - 1].split(".").collect();
    FILE_NAME = filename[0];
}

fn sys_init() -> String {
    return "// Initializing\n@Sys.init.returnAdd\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n".to_string() +
        "@LCL\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n" + //push LCL
        "@ARG\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n" + //push ARG
        "@THIS\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n" + //push THIS
        "@THAT\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n" + //push THAT
        "@SP\nD=M\n@0\nD=D-A\n@5\nD=D-A\n@ARG\nM=D" + //ARG = SP-n-5
        "\n@SP\nD=M\n@LCL\nM=D\n" + //LCL = SP
        "@Sys.init\n0;JMP\n(Sys.init.returnAdd)\n"
}

fn parse(line:String) -> String{
    let words: Vec<&str> = line.split_whitespace().collect();
    unsafe {
        return match words[0].chars().nth(0).unwrap() {
            'a' => return match words[0].chars().nth(1).unwrap(){
                'd' => write_binary_op(ADD),
                'n' => write_binary_op(AND),
                _ => "".to_string()
            },

            'c' => return match words[0].chars().nth(1).unwrap(){
                'a' => call(words[1], words[2]),
                _ => "".to_string()
            },

            'e' => return match words[0].chars().nth(1).unwrap(){
                'q' => comparison(EQ),
                _ => "".to_string()
            },

            'f' => return match words[0].chars().nth(1).unwrap(){
                'u' => function(words[1], words[2]),
                _ => "".to_string()
            },

            'g' => return match words[0].chars().nth(1).unwrap(){
                't' => comparison(GT),
                'o' => goto(words[1]),
                _ => "".to_string()
            },

            'i' => return match words[0].chars().nth(1).unwrap(){
                'f' => if_goto(words[1]),
                _ => "".to_string()
            },

            'l' => return match words[0].chars().nth(1).unwrap(){
                't' => comparison(LT),
                'a' => label(words[1]),
                _ => "".to_string()
            },

            'n' => return match words[0].chars().nth(1).unwrap(){
                'o' => write_unary_op(NOT),
                'e' => write_unary_op(NEG),
                _ => "".to_string()
            },

            'o' => return match words[0].chars().nth(1).unwrap(){
                'r' => write_binary_op(OR),
                _ => "".to_string()
            },

            'p' => return match words[0].chars().nth(1).unwrap(){
                'o' => pop(words[1], words[2]),
                'u' => push(words[1], words[2]),
            _ => "".to_string()
        },

            'r' => return match words[0].chars().nth(1).unwrap(){
                'e' => _return(),
                _ => "".to_string()
            },

            's' => return match words[0].chars().nth(1).unwrap(){
                'u' => write_binary_op(SUB),
                _ => "".to_string()
            },

            _ => "Command not yet supported".to_string()
        }

    }
}

fn create_asm(file_path:&str)-> String{
    let folder_name = file_path.split("\\");
    let vec:Vec<&str> = folder_name.collect();
    let tmp = vec[vec.len()-1];
    let f_name= tmp.to_string();
    let path_to_create = file_path.to_owned() + "\\" + &f_name + ".asm";
    // Open a file in write-only mode, returns `io::Result<File>`
    let _file = match File::create(&path_to_create) {
        Err(why) => panic!("couldn't create {}: {}", f_name+".asm", why),
        Ok(file) => file,
    };
    path_to_create
}

fn search_vm_files(file_path:&str)-> Vec<String>{
    let mut vm_files:Vec<String> =Vec::new();
    let paths = fs::read_dir(file_path).unwrap();
    for path in paths {
        if path.as_ref().unwrap().path().display().to_string().contains(".vm"){
            vm_files.push(path.unwrap().path().display().to_string())
        }
    }
    vm_files
}

