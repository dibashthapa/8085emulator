use crate::{cpu::Registers, token::Token};
use std::fmt;
static MNEMONICS: &[&str] = &[
    "ACI", "ADC", "ADD", "ADI", "ANA", "ANI", "CALL", "CC", "CM", "CMA", "CMC", "CMP", "CNC",
    "CNZ", "CP", "CPE", "CPI", "CPO", "CZ", "DAA", "DAD", "DCR", "DCX", "DI", "EI", "HLT", "IN",
    "INR", "INX", "JC", "JM", "JMP", "JNC", "JNZ", "JP", "JPE", "JPO", "JZ", "LDA", "LDAX", "LHLD",
    "LXI", "MOV", "MVI", "NOP", "ORA", "ORI", "OUT", "PCHL", "POP", "PUSH", "RAL", "RAR", "RC",
    "RET", "RLC", "RM", "RNC", "RNZ", "RP", "RPE", "RPO", "RRC", "RST", "RZ", "SBB", "SBI", "SHLD",
    "SPHL", "STA", "STAX", "STC", "SUB", "SUI", "XCHG", "XRA", "XRI", "XTHL",
];
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
    Adc(Registers),
    Sub(Registers),
    Lxi(Registers, u16),
    Lda(u16),
    Inx(Registers),
    Inr(Registers),
    Dcr(Registers),
    Jnz(JumpTarget<'a>),
    Lhld(u16),
    Xchg,
    Shld(u16),
    Jnc(JumpTarget<'a>),
    Cmp(Registers),
    Sta(u16),
    Ani(u8),
    Hlt,
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(String),
    MissingToken,
    UnimplementedInstruction(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken(token) => write!(f, "Unexpected token: {}", token),
            ParseError::MissingToken => write!(f, "Missing token"),
            ParseError::UnimplementedInstruction(instruction) => {
                write!(f, "Instruction '{}' is not implemented", instruction)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction<'a> {
    pub label: Option<&'a str>,
    pub ins: Ins<'a>,
}

pub fn create_instruction(ins: Ins) -> Instruction {
    Instruction { label: None, ins }
}

fn next_register<'a>(
    tokens_iter: &mut impl Iterator<Item = Token<'a>>,
) -> Result<Registers, ParseError> {
    match tokens_iter.next() {
        Some(Token::Register(register)) => Ok(register),
        Some(token) => Err(ParseError::UnexpectedToken(format!(
            "Expected register, but received {:?}",
            token
        ))),
        None => Err(ParseError::MissingToken),
    }
}

fn next_number<'a>(tokens_iter: &mut impl Iterator<Item = Token<'a>>) -> Result<u8, ParseError> {
    match tokens_iter.next() {
        Some(Token::Number(value)) => Ok(value),
        Some(token) => Err(ParseError::UnexpectedToken(format!(
            "Expected number, but received {:?}",
            token
        ))),
        None => Err(ParseError::MissingToken),
    }
}

fn next_address<'a>(tokens_iter: &mut impl Iterator<Item = Token<'a>>) -> Result<u16, ParseError> {
    match tokens_iter.next() {
        Some(Token::Address(address)) => Ok(address),
        Some(token) => Err(ParseError::UnexpectedToken(format!(
            "Expected address, but received {:?}",
            token
        ))),
        None => Err(ParseError::MissingToken),
    }
}

fn next_jump_target<'a>(
    tokens_iter: &mut impl Iterator<Item = Token<'a>>,
) -> Result<JumpTarget<'a>, ParseError> {
    match tokens_iter.next() {
        Some(Token::Word(label)) => Ok(JumpTarget::Label(label)),
        Some(Token::Address(address)) => Ok(JumpTarget::Address(address)),
        Some(token) => Err(ParseError::UnexpectedToken(format!(
            "Expected address or label, but received {:?}",
            token
        ))),
        None => Err(ParseError::MissingToken),
    }
}

pub fn parse_instruction<'a>(
    word: &str,
    tokens_iter: &mut impl Iterator<Item = Token<'a>>,
) -> Result<Instruction<'a>, ParseError> {
    match word {
        "MVI" => {
            let register = next_register(tokens_iter)?;
            let value = next_number(tokens_iter)?;
            Ok(create_instruction(Ins::Mvi(register, value)))
        }
        "JMP" => {
            let target = next_jump_target(tokens_iter)?;
            Ok(create_instruction(Ins::Jmp(target)))
        }
        "MOV" => {
            let dest = next_register(tokens_iter)?;
            let source = next_register(tokens_iter)?;
            Ok(create_instruction(Ins::Mov(dest, source)))
        }
        "ADD" => {
            let register = next_register(tokens_iter)?;
            Ok(create_instruction(Ins::Add(register)))
        }
        "ADI" => {
            let value = next_number(tokens_iter)?;
            Ok(create_instruction(Ins::Adi(value)))
        }
        "SUB" => {
            let register = next_register(tokens_iter)?;
            Ok(create_instruction(Ins::Sub(register)))
        }
        "LXI" => {
            let register = next_register(tokens_iter)?;
            let address = next_address(tokens_iter)?;
            Ok(create_instruction(Ins::Lxi(register, address)))
        }
        "INX" => {
            let register = next_register(tokens_iter)?;
            Ok(create_instruction(Ins::Inx(register)))
        }
        "INR" => {
            let register = next_register(tokens_iter)?;
            Ok(create_instruction(Ins::Inr(register)))
        }
        "DCR" => {
            let register = next_register(tokens_iter)?;
            Ok(create_instruction(Ins::Dcr(register)))
        }
        "JNZ" => {
            let target = next_jump_target(tokens_iter)?;
            Ok(create_instruction(Ins::Jnz(target)))
        }
        "CMP" => {
            let register = next_register(tokens_iter)?;
            Ok(create_instruction(Ins::Cmp(register)))
        }
        "JNC" => {
            let target = next_jump_target(tokens_iter)?;
            Ok(create_instruction(Ins::Jnc(target)))
        }
        "STA" => {
            let address = next_address(tokens_iter)?;
            Ok(create_instruction(Ins::Sta(address)))
        }
        "LHLD" => {
            let address = next_address(tokens_iter)?;
            Ok(create_instruction(Ins::Lhld(address)))
        }
        "SHLD" => {
            let address = next_address(tokens_iter)?;
            Ok(create_instruction(Ins::Shld(address)))
        }
        "XCHG" => Ok(create_instruction(Ins::Xchg)),
        "ADC" => {
            let register = next_register(tokens_iter)?;
            Ok(create_instruction(Ins::Adc(register)))
        }
        "LDA" => {
            let address = next_address(tokens_iter)?;
            Ok(create_instruction(Ins::Lda(address)))
        }
        "ANI" => {
            let value = next_number(tokens_iter)?;
            Ok(create_instruction(Ins::Ani(value)))
        }
        "HLT" => Ok(create_instruction(Ins::Hlt)),
        remaining => Err(ParseError::UnimplementedInstruction(format!(
            "Instruction {} hasn't been implemented yet",
            remaining
        ))),
    }
}
pub fn parse(tokens: Vec<Token>) -> Result<Vec<Instruction>, ParseError> {
    let mut tokens = tokens.into_iter();
    let mut instructions = Vec::new();

    while let Some(token) = tokens.next() {
        match token {
            Token::Label(label) => {
                if let Some(Token::Word(word)) = tokens.next() {
                    let mut instruction = parse_instruction(word, &mut tokens)?;
                    instruction.label = Some(label);
                    instructions.push(instruction);
                }
            }
            Token::Word(word) => {
                if MNEMONICS.contains(&word) {
                    let instruction = parse_instruction(word, &mut tokens)?;
                    instructions.push(instruction);
                }
            }
            _ => {}
        }
    }
    Ok(instructions)
}

#[macro_export]
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
            instructions.unwrap(),
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
