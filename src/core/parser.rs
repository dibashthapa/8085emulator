use super::{cpu::Registers, token::Token};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JumpTarget<'a> {
    Address(u16),
    Label(&'a str),
}

#[derive(Debug, PartialEq)]
pub enum Ins<'a> {
    Mov(Registers, Registers),
    Mvi(Registers, u8),
    Jmp(JumpTarget<'a>),
    Add(Registers),
    Adi(u8),
    Lxi(Registers, u16),
    Inx(Registers),
    Inr(Registers),
    Dcr(Registers),
    Jnz(JumpTarget<'a>),
    Jnc(JumpTarget<'a>),
    Cmp(Registers),
    Sta(u16),
    Hlt,
}

#[derive(Debug, PartialEq)]
pub struct Instruction<'a> {
    pub label: Option<&'a str>,
    pub ins: Ins<'a>,
}

pub fn create_instruction(ins: Ins) -> Option<Instruction> {
    Some(Instruction { label: None, ins })
}

pub fn parse_instruction<'a>(
    word: &str,
    tokens_iter: &mut impl Iterator<Item = Token<'a>>,
) -> Option<Instruction<'a>> {
    match word {
        "MVI" => {
            let register = tokens_iter.next();
            let value = tokens_iter.next();
            if let (Some(Token::Register(register)), Some(Token::Number(value))) = (register, value)
            {
                return create_instruction(Ins::Mvi(register, value));
            }
            None
        }
        "JMP" => {
            if let Some(Token::Word(label)) = tokens_iter.next() {
                return create_instruction(Ins::Jmp(JumpTarget::Label(label)));
            } else if let Some(Token::Address(address)) = tokens_iter.next() {
                return create_instruction(Ins::Jmp(JumpTarget::Address(address)));
            }
            None
        }
        "MOV" => {
            if let (Some(Token::Register(dest)), Some(Token::Register(source))) =
                (tokens_iter.next(), tokens_iter.next())
            {
                return create_instruction(Ins::Mov(dest, source));
            }
            None
        }
        "ADD" => {
            if let Some(Token::Register(register)) = tokens_iter.next() {
                return create_instruction(Ins::Add(register));
            }
            None
        }
        "ADI" => {
            if let Some(Token::Number(value)) = tokens_iter.next() {
                return create_instruction(Ins::Adi(value));
            }
            None
        }
        "LXI" => {
            if let (Some(Token::Register(register)), Some(Token::Address(address))) =
                (tokens_iter.next(), tokens_iter.next())
            {
                return create_instruction(Ins::Lxi(register, address));
            }
            None
        }
        "INX" => {
            if let Some(Token::Register(register)) = tokens_iter.next() {
                return create_instruction(Ins::Inx(register));
            }
            None
        }
        "INR" => {
            if let Some(Token::Register(register)) = tokens_iter.next() {
                return create_instruction(Ins::Inr(register));
            }
            None
        }
        "DCR" => {
            if let Some(Token::Register(register)) = tokens_iter.next() {
                return create_instruction(Ins::Dcr(register));
            }
            None
        }
        "JNZ" => {
            if let Some(Token::Word(label)) = tokens_iter.next() {
                return create_instruction(Ins::Jnz(JumpTarget::Label(label)));
            } else if let Some(Token::Address(address)) = tokens_iter.next() {
                return create_instruction(Ins::Jnz(JumpTarget::Address(address)));
            }
            None
        }
        "CMP" => {
            if let Some(Token::Register(register)) = tokens_iter.next() {
                return create_instruction(Ins::Cmp(register));
            }
            None
        }
        "JNC" => {
            if let Some(Token::Word(label)) = tokens_iter.next() {
                return create_instruction(Ins::Jnc(JumpTarget::Label(label)));
            } else if let Some(Token::Address(address)) = tokens_iter.next() {
                return create_instruction(Ins::Jnc(JumpTarget::Address(address)));
            }
            None
        }
        "STA" => {
            if let Some(Token::Address(address)) = tokens_iter.next() {
                return create_instruction(Ins::Sta(address));
            }
            None
        }
        "HLT" => {
            return create_instruction(Ins::Hlt);
        }
        _ => None,
    }
}
pub fn parse(tokens: Vec<Token>) -> Vec<Instruction> {
    let mut tokens = tokens.into_iter();
    let mut instructions = Vec::new();

    while let Some(token) = tokens.next() {
        match token {
            Token::Label(label) => {
                if let Some(Token::Word(word)) = tokens.next() {
                    if let Some(mut instruction) = parse_instruction(word, &mut tokens) {
                        instruction.label = Some(label);
                        instructions.push(instruction);
                    }
                }
            }
            Token::Word(word) => {
                if let Some(instruction) = parse_instruction(word, &mut tokens) {
                    instructions.push(instruction);
                }
            }
            _ => {}
        }
    }
    instructions
}

#[cfg(test)]
mod tests {
    use crate::core::{
        parser::{Ins, Instruction},
        token::Token,
    };

    use super::parse;

    use logos::Logos;
    macro_rules! parse_code {
        ($code:expr) => {{
            let lex = Token::lexer($code);
            let tokens: Vec<_> = lex.filter_map(|token| token.ok()).collect();
            parse(tokens)
        }};
    }
    #[test]
    fn test_loop() {
        let code = r#"
            LXI H,2050H
            MVI B,01H
            MVI C,0AH
            X: MOV M,B
        "#;
        let instructions = parse_code!(code);
        assert_eq!(
            instructions,
            vec![
                Instruction {
                    label: None,
                    ins: Ins::Lxi(super::Registers::RegH, 0x2050),
                },
                Instruction {
                    label: None,
                    ins: Ins::Mvi(super::Registers::RegB, 0x01),
                },
                Instruction {
                    label: None,
                    ins: Ins::Mvi(super::Registers::RegC, 0x0A),
                },
                Instruction {
                    label: Some("X"),
                    ins: Ins::Mov(super::Registers::RegM, super::Registers::RegB),
                }
            ]
        );
    }
}
