use crate::token::Token;
use logos::Logos;

use crate::cpu::{InstructionSet, Registers};

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
                    "INX" => {
                        if let Some(Ok(Token::Register)) = lexer.next() {
                            let register = Registers::from(lexer.slice());
                            instructions.push(InstructionSet::Inx(register));
                        }
                    }
                    "LXI" => {
                        if let Some(Ok(Token::Register)) = lexer.next() {
                            let register = Registers::from(lexer.slice());
                            if let Some(Ok(Token::Address(address))) = lexer.next() {
                                instructions.push(InstructionSet::Lxi(register, address));
                            }
                        }
                    }
                    "INR" => {
                        if let Some(Ok(Token::Register)) = lexer.next() {
                            let register = Registers::from(lexer.slice());
                            instructions.push(InstructionSet::Inr(register));
                        }
                    }
                    "DCR" => {
                        if let Some(Ok(Token::Register)) = lexer.next() {
                            let register = Registers::from(lexer.slice());
                            instructions.push(InstructionSet::Dcr(register));
                        }
                    }
                    "ADI" => {
                        if let Some(Ok(Token::Number(value))) = lexer.next() {
                            instructions.push(InstructionSet::Adi(value));
                        }
                    }
                    "ADD" => {
                        if let Some(Ok(Token::Register)) = lexer.next() {
                            let register = Registers::from(lexer.slice());
                            instructions.push(InstructionSet::Add(register));
                        }
                    }
                    "SUB" => {
                        if let Some(Ok(Token::Register)) = lexer.next() {
                            let register = Registers::from(lexer.slice());
                            instructions.push(InstructionSet::Sub(register));
                        }
                    }
                    "LHLD" => {
                        if let Some(Ok(Token::Address(address))) = lexer.next() {
                            instructions.push(InstructionSet::Lhld(address));
                        }
                    }
                    "SHLD" => {
                        if let Some(Ok(Token::Address(address))) = lexer.next() {
                            instructions.push(InstructionSet::Shld(address));
                        }
                    }
                    "LDAX" => {
                        if let Some(Ok(Token::Register)) = lexer.next() {
                            let register = Registers::from(lexer.slice());
                            instructions.push(InstructionSet::Ldax(register));
                        }
                    }
                    "HLT" => {
                        instructions.push(InstructionSet::Hlt);
                    }
                    "STAX" => {
                        if let Some(Ok(Token::Register)) = lexer.next() {
                            let register = Registers::from(lexer.slice());
                            instructions.push(InstructionSet::Stax(register));
                        }
                    }
                    "XCHG" => {
                        instructions.push(InstructionSet::Xchg);
                    }
                    "ADC" => {
                        if let Some(Ok(Token::Register)) = lexer.next() {
                            let register = Registers::from(lexer.slice());
                            instructions.push(InstructionSet::Adc(register));
                        }
                    }
                    "DAD" => {
                        if let Some(Ok(Token::Register)) = lexer.next() {
                            let register = Registers::from(lexer.slice());
                            instructions.push(InstructionSet::Dad(register));
                        }
                    }
                    "ANA" => {
                        if let Some(Ok(Token::Register)) = lexer.next() {
                            let register = Registers::from(lexer.slice());
                            instructions.push(InstructionSet::Ana(register));
                        }
                    }
                    "ORA" => {
                        if let Some(Ok(Token::Register)) = lexer.next() {
                            let register = Registers::from(lexer.slice());
                            instructions.push(InstructionSet::Ora(register));
                        }
                    }
                    "CMA" => {
                        instructions.push(InstructionSet::Cma);
                    }
                    "CMP" => {
                        if let Some(Ok(Token::Register)) = lexer.next() {
                            let register = Registers::from(lexer.slice());
                            instructions.push(InstructionSet::Cmp(register));
                        }
                    }
                    _ => {
                        unimplemented!("{}", format!("Invalid opcode: {}", lexer.slice()));
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
            r#"MVI A,FF
            "#,
        );
        assert_eq!(lex.next(), Some(Ok(Token::OpCode)));
        assert_eq!(lex.slice(), "MVI");
        assert_eq!(lex.next(), Some(Ok(Token::Register)));
        assert_eq!(lex.slice(), "A");
        assert_eq!(lex.next(), Some(Ok(Token::Number(0xFF))));
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
    fn test_lxi() {
        let mut lex = Token::lexer(r#"LXI 2000"#);
        assert_eq!(lex.next(), Some(Ok(Token::OpCode)));
        assert_eq!(lex.slice(), "LXI");
        assert_eq!(lex.next(), Some(Ok(Token::Address(0x2000))));
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
