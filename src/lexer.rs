use crate::OpCode;

pub struct Lexer {
    pub source_code: String,
}

impl Lexer {
    pub fn lex(&self) -> Vec<OpCode> {
        let mut tokens = Vec::new();
        let source_code = self.source_code.split(" ");
        for token in source_code {
            match token {
                "+" => tokens.push(OpCode::OpAdd),
                "-" => tokens.push(OpCode::OpSub),
                "." => tokens.push(OpCode::OpDump),
                _ => {
                    if let Ok(number) = token.parse::<i64>() {
                        tokens.push(OpCode::OpPush(number));
                    } else {
                        println!("Not handled: {}", token);
                        todo!()
                    }
                }
            }
        }
        tokens
    }
}
