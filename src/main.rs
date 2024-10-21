mod lexer;
use std::{error::Error, fs, io::Read};

use crate::lexer::Lexer;

pub enum OpCode {
    OpPush(i64),
    OpAdd,
    OpSub,
    OpDump,
}

pub struct Program {
    source_code: Vec<OpCode>,
}

impl Program {
    fn run_compilation(&self) {}
    fn run_simulation(&self) -> Result<(), Box<dyn Error>> {
        let mut stack: Vec<i64> = vec![];
        for token in &self.source_code {
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

fn main() -> Result<(), Box<dyn Error>> {
    let mut source_code = String::new();
    fs::File::open("./examples/test.psh")?.read_to_string(&mut source_code)?;
    let lexer = Lexer { source_code };
    let tokens = lexer.lex();

    let program = Program {
        source_code: tokens,
    };
    program.run_simulation();
    Ok(())
}
