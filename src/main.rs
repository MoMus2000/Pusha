mod lexer;

use crate::lexer::Lexer;
use std::{
    env,
    error::Error,
    fs,
    io::{Read, Write},
    process::{exit, Command, Stdio},
};

pub enum OpCode {
    OpPush(i64),
    OpAdd,
    OpSub,
    OpDump,
    OpEq,
    OpIf((i64, i64)),
    OpEnd,
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
        let mut temp_file = fs::File::create("./output.asm")?;
        temp_file.write(b"section .text\n")?;

        // Shameless copy from tsoding..
        temp_file.write(b"dump:\n")?;
        temp_file.write(b"    mov     r9, -3689348814741910323\n")?;
        temp_file.write(b"    sub     rsp, 40\n")?;
        temp_file.write(b"    mov     BYTE [rsp+31], 10\n")?;
        temp_file.write(b"    lea     rcx, [rsp+30]\n")?;
        temp_file.write(b".L2:\n")?;
        temp_file.write(b"    mov     rax, rdi\n")?;
        temp_file.write(b"    lea     r8, [rsp+32]\n")?;
        temp_file.write(b"    mul     r9\n")?;
        temp_file.write(b"    mov     rax, rdi\n")?;
        temp_file.write(b"    sub     r8, rcx\n")?;
        temp_file.write(b"    shr     rdx, 3\n")?;
        temp_file.write(b"    lea     rsi, [rdx+rdx*4]\n")?;
        temp_file.write(b"    add     rsi, rsi\n")?;
        temp_file.write(b"    sub     rax, rsi\n")?;
        temp_file.write(b"    add     eax, 48\n")?;
        temp_file.write(b"    mov     BYTE [rcx], al\n")?;
        temp_file.write(b"    mov     rax, rdi\n")?;
        temp_file.write(b"    mov     rdi, rdx\n")?;
        temp_file.write(b"    mov     rdx, rcx\n")?;
        temp_file.write(b"    sub     rcx, 1\n")?;
        temp_file.write(b"    cmp     rax, 9\n")?;
        temp_file.write(b"    ja      .L2\n")?;
        temp_file.write(b"    lea     rax, [rsp+32]\n")?;
        temp_file.write(b"    mov     edi, 1\n")?;
        temp_file.write(b"    sub     rdx, rax\n")?;
        temp_file.write(b"    xor     eax, eax\n")?;
        temp_file.write(b"    lea     rsi, [rsp+32+rdx]\n")?;
        temp_file.write(b"    mov     rdx, r8\n")?;
        temp_file.write(b"    mov     rax, 1\n")?;
        temp_file.write(b"    syscall\n")?;
        temp_file.write(b"    add     rsp, 40\n")?;
        temp_file.write(b"    ret\n")?;

        temp_file.write(b"  global _start\n")?;
        temp_file.write(b"_start:\n")?;

        for token in &self.tokens {
            match token {
                OpCode::OpAdd => {
                    temp_file.write(b";; -- plus -- \n")?;
                    temp_file.write(b"pop rax\n")?;
                    temp_file.write(b"pop rbx\n")?;
                    temp_file.write(b"add rax, rbx\n")?;
                    temp_file.write(b"push rax\n")?;
                }
                OpCode::OpPush(i) => {
                    temp_file.write(b";; -- push -- \n")?;
                    temp_file.write(format!("push {}", i).as_bytes())?;
                }
                OpCode::OpSub => {
                    temp_file.write(b";; -- sub -- \n")?;
                    temp_file.write(b"pop rax\n")?;
                    temp_file.write(b"pop rbx\n")?;
                    temp_file.write(b"sub rbx, rax\n")?;
                    temp_file.write(b"push rbx\n")?;
                }
                OpCode::OpDump => {
                    temp_file.write(b";; -- dump -- \n")?;
                    temp_file.write(b"pop rdi\n")?; // Convention to provide function arguments in rdi
                    temp_file.write(b"call dump\n")?; // Convention to provide function arguments in rdi
                }
                OpCode::OpEq => {
                    temp_file.write(b";; -- eq -- \n")?;
                    temp_file.write(b"mov rcx, 0\n")?;
                    temp_file.write(b"mov rdx, 1\n")?;
                    temp_file.write(b"pop rax\n")?;
                    temp_file.write(b"pop rbx\n")?;
                    //If RAX is equal to RBX, the Zero Flag (ZF) is set.
                    temp_file.write(b"cmp rax, rbx\n")?;
                    // This instruction conditionally moves
                    // the value in RDX (which is 1) into RCX if the Zero Flag (ZF) is set
                    temp_file.write(b"cmove rcx, rdx\n")?;
                }
                _ => {
                    panic!("Unreachable")
                }
            }
        }

        temp_file.write(b"mov rax, 60\n")?;
        temp_file.write(b"mov rdi, 1\n")?;
        temp_file.write(b"syscall\n")?;

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
                OpCode::OpEq => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push((a == b) as i64);
                }
                OpCode::OpIf((predicate, end)) => {
                    let predicate = stack.pop().unwrap();
                    if predicate == 1 {
                        // do something
                    } else {
                        // go the end index
                    }
                }
                OpCode::OpEnd => {}
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
            let output = Command::new("nasm")
                .args(vec!["-f", "elf64", "output.asm"])
                .stdout(Stdio::piped()) // Capture standard output
                .stderr(Stdio::piped())
                .output()?;

            let stderr = String::from_utf8_lossy(&output.stderr);

            // Print any error messages
            if !stderr.is_empty() {
                eprintln!("Error:\n{}", stderr);
                exit(1)
            }

            let output = Command::new("ld")
                .args(vec!["-o", "output", "output.o"])
                .stdout(Stdio::piped()) // Capture standard output
                .stderr(Stdio::piped())
                .output()?;

            let stderr = String::from_utf8_lossy(&output.stderr);

            // Print any error messages
            if !stderr.is_empty() {
                eprintln!("Error:\n{}", stderr);
                exit(1)
            }

            let output = Command::new("rm")
                .args(vec!["output.asm", "output.o"])
                .stdout(Stdio::piped()) // Capture standard output
                .stderr(Stdio::piped())
                .output()?;

            // Print any error messages
            if !stderr.is_empty() {
                eprintln!("Error:\n{}", stderr);
                exit(1)
            }
        }
        _ => {
            usage(program_name);
            exit(1)
        }
    }
    Ok(())
}
