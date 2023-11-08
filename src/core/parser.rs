use super::{cpu::Registers, token::Token};

#[derive(Debug, Clone, Copy)]
pub enum JumpTarget<'a> {
    Address(u16),
    Label(&'a str),
}

#[derive(Debug)]
pub enum Ins<'a> {
    Mov(Registers, Registers),
    Mvi(Registers, u8),
    Jmp(JumpTarget<'a>),
    Add(Registers),
    Adi(u8),
}

#[derive(Debug)]
pub struct Instruction<'a> {
    pub label: Option<&'a str>,
    pub ins: Ins<'a>,
}

pub fn parse_ops<'a>(
    word: &str,
    tokens_iter: &mut impl Iterator<Item = Token<'a>>,
) -> Option<Instruction<'a>> {
    match word {
        "MVI" => {
            if let (Some(Token::Register(register)), Some(Token::Number(value))) =
                (tokens_iter.next(), tokens_iter.next())
            {
                return Some(Instruction {
                    label: None,
                    ins: Ins::Mvi(register, value),
                });
            }
            None
        }
        "JMP" => {
            if let Some(Token::Word(label)) = tokens_iter.next() {
                return Some(Instruction {
                    label: None,
                    ins: Ins::Jmp(JumpTarget::Label(label)),
                });
            } else if let Some(Token::Address(address)) = tokens_iter.next() {
                return Some(Instruction {
                    label: None,
                    ins: Ins::Jmp(JumpTarget::Address(address)),
                });
            }
            None
        }
        "MOV" => {
            if let (Some(Token::Register(dest)), Some(Token::Register(source))) =
                (tokens_iter.next(), tokens_iter.next())
            {
                return Some(Instruction {
                    label: None,
                    ins: Ins::Mov(dest, source),
                });
            }
            None
        }
        "ADD" => {
            if let Some(Token::Register(register)) = tokens_iter.next() {
                return Some(Instruction {
                    label: None,
                    ins: Ins::Add(register),
                });
            }
            None
        }
        "ADI" => {
            if let Some(Token::Number(value)) = tokens_iter.next() {
                return Some(Instruction {
                    label: None,
                    ins: Ins::Adi(value),
                });
            }
            None
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
                    if let Some(mut instruction) = parse_ops(word, &mut tokens) {
                        instruction.label = Some(label);
                        instructions.push(instruction);
                    }
                }
            }
            Token::Word(word) => {
                if let Some(instruction) = parse_ops(word, &mut tokens) {
                    instructions.push(instruction);
                }
            }
            _ => {}
        }
    }
    instructions
}
