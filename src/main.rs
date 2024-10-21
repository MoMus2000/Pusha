enum OpCode {
    OpPush(i64),
    OpAdd,
    OpSub,
    OpDump,
}

impl OpCode {
    fn compile(&self) {}
}

struct Program {
    source_code: Vec<OpCode>,
}

impl Program {
    fn run_simulation(&self) {
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
    }
}

fn main() {
    let mut source_code: Vec<OpCode> = Vec::new();
    source_code.push(OpCode::OpPush(5));
    source_code.push(OpCode::OpPush(6));
    source_code.push(OpCode::OpAdd);
    source_code.push(OpCode::OpDump);

    let program = Program { source_code };
    program.run_simulation();
}
