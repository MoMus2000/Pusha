use crate::OpCode;

pub struct Lexer {
    pub source_code: String,
}

impl Lexer {
    pub fn lex(&self) -> Vec<OpCode> {
        let mut tokens = Vec::new();
        let source_code = self.source_code.split("\n");
        for line in source_code {
            for token in line.split(" ") {
                match token {
                    "+" => tokens.push(OpCode::OpAdd),
                    "-" => tokens.push(OpCode::OpSub),
                    "." => tokens.push(OpCode::OpDump),
                    "end" => tokens.push(OpCode::OpEnd),
                    "if" => tokens.push(OpCode::OpIf((0, 0))),
                    "=" => tokens.push(OpCode::OpEq),
                    _ => {
                        if let Ok(number) = token.parse::<i64>() {
                            tokens.push(OpCode::OpPush(number));
                        } else {
                        }
                    }
                }
            }
        }
        tokens
    }
}
