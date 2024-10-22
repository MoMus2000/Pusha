mod lexer;

use crate::lexer::Lexer;
use std::{
    env,
    error::Error,
    fs,
    io::{Read, Write},
    process::exit,
};

pub enum OpCode {
    OpPush(i64),
    OpAdd,
    OpSub,
    OpDump,
}

pub struct Program {
    tokens: Vec<OpCode>,
}

impl Program {
    pub fn new(source_code_path: String) -> Result<Program, Box<dyn Error>> {
        let mut source_code = String::new();
        fs::File::open(source_code_path)?.read_to_string(&mut source_code)?;
        let lexer = Lexer { source_code };
        let tokens = lexer.lex();
        Ok(Program { tokens })
    }

    fn run_compilation(&self) -> Result<(), Box<dyn Error>> {
        let mut temp_file = tempfile::tempfile()?;
        temp_file.write(b"section .text\n")?;
        temp_file.write(b"  global _start")?;
        temp_file.write(b"_start:\n")?;
        temp_file.write(b"mov rax, 60\n")?;
        temp_file.write(b"xor rdi, rdi\n")?;
        temp_file.write(b"syscall\n")?;
        for token in &self.tokens {
            match token {
                OpCode::OpAdd => {}
                OpCode::OpPush(i) => {}
                OpCode::OpSub => {}
                OpCode::OpDump => {}
            }
        }
        Ok(())
    }

    fn run_simulation(&self) -> Result<(), Box<dyn Error>> {
        let mut stack: Vec<i64> = vec![];
        for token in &self.tokens {
            match token {
                OpCode::OpAdd => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(a + b);
                }
                OpCode::OpPush(i) => stack.push(*i),
                OpCode::OpSub => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b - a);
                }
                OpCode::OpDump => {
                    let a = stack.pop().unwrap();
                    println!("{}", a);
                }
            }
        }
        Ok(())
    }
}

fn usage(program_name: &str) {
    println!("Usage {} <SUBCOMMAND> [ARGS]", program_name);
    println!("SUBCOMMANDS:");
    println!("  sim <file> Simulate the program.");
    println!("  com <file> Compile the program.");
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let program_name = &args[0];
    if args.len() < 3 {
        usage(&program_name);
        exit(1)
    }
    let command = args[1].as_str();
    let source_path = args[2].as_str();
    let program = Program::new(source_path.to_string())?;
    match command {
        "sim" => {
            program.run_simulation()?;
        }
        "com" => {
            program.run_compilation()?;
        }
        _ => {
            usage(program_name);
            exit(1)
        }
    }
    Ok(())
}
