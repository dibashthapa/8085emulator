use logos::Logos;

use crate::cpu::{InstructionSet, Registers};

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n,]+")]
pub enum Token {
    #[token("false", |_| false)]
    #[token("true", |_| true)]
    Bool(bool),

    #[token("MVI")]
    #[token("MOV")]
    #[token("LDA")]
    #[token("STA")]
    #[token("LXI")]
    #[token("ADD")]
    #[token("SUB")]
    OpCode,

    #[regex(r"[ABCDEHL]")]
    Register,

    #[regex(r";.*", logos::skip)]
    Comment,

    #[regex(r"[0-9]{2}", |lex| lex.slice().parse::<u8>().unwrap())]
    Number(u8),

    #[regex(r"[0-9]{4}", |lex| lex.slice().parse::<u16>().unwrap())]
    Address(u16),
}

pub fn parse_instructions(code: &str) -> Vec<InstructionSet> {
    let mut lexer = Token::lexer(code);
    let mut instructions = Vec::new();

    while let Some(token) = lexer.next() {
        match token {
            Ok(token) => match token {
                Token::OpCode => match lexer.slice() {
                    "MVI" => {
                        if let Some(Ok(Token::Register)) = lexer.next() {
                            let register = Registers::from(lexer.slice());
                            if let Some(Ok(Token::Number(value))) = lexer.next() {
                                instructions.push(InstructionSet::Mvi(register, value));
                            }
                        }
                    }
                    "MOV" => {
                        if let Some(Ok(Token::Register)) = lexer.next() {
                            let desination = Registers::from(lexer.slice());

                            if let Some(Ok(Token::Register)) = lexer.next() {
                                let source = Registers::from(lexer.slice());
                                instructions.push(InstructionSet::Mov(desination, source));
                            }
                        }
                    }
                    "STA" => {
                        if let Some(Ok(Token::Address(address))) = lexer.next() {
                            instructions.push(InstructionSet::Sta(address));
                        }
                    }
                    "LDA" => {
                        if let Some(Ok(Token::Address(address))) = lexer.next() {
                            instructions.push(InstructionSet::Lda(address));
                        }
                    }
                    _ => {
                        panic!("unkown opcode");
                    }
                },
                _ => {}
            },
            Err(_) => {}
        }
    }
    instructions
}

#[cfg(test)]
mod tests {
    use super::Token;
    use logos::Logos;

    #[test]
    fn test_mvi() {
        let mut lex = Token::lexer(
            r#"MVI A,20
            "#,
        );
        assert_eq!(lex.next(), Some(Ok(Token::OpCode)));
        assert_eq!(lex.slice(), "MVI");
        assert_eq!(lex.next(), Some(Ok(Token::Register)));
        assert_eq!(lex.slice(), "A");
        assert_eq!(lex.next(), Some(Ok(Token::Number(20))));
    }

    #[test]
    fn test_mov() {
        let mut lex = Token::lexer(
            r#"MOV A,B
            "#,
        );
        assert_eq!(lex.next(), Some(Ok(Token::OpCode)));
        assert_eq!(lex.slice(), "MOV");
        assert_eq!(lex.next(), Some(Ok(Token::Register)));
        assert_eq!(lex.slice(), "A");
        assert_eq!(lex.next(), Some(Ok(Token::Register)));
        assert_eq!(lex.slice(), "B");
    }

    #[test]
    fn test_comment() {
        let mut lex = Token::lexer(
            r#"; Hello this is dibash
            "#,
        );
        assert_eq!(lex.next(), None);
    }
}
