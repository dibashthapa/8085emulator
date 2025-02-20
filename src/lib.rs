use logos::Logos;
pub mod core;
pub mod gui;
pub mod tests;
use core::{assembler::assemble, cpu::Cpu, parser::parse, token::Token, *};

pub fn execute_code(code: &str) -> (Cpu, usize) {
    let lex = Token::lexer(code);
    let tokens: Vec<_> = lex.filter_map(|token| token.ok()).collect();
    let instructions = parse(tokens);
    match instructions {
        Ok(instructions) => {
            let assembled_instructions = assemble(&instructions);
            let mut cpu = Cpu::new();

            let assembled_count = assembled_instructions.iter().len();
            for (index, inst) in assembled_instructions.iter().enumerate() {
                cpu.write_memory(index, *inst);
            }
            (cpu, assembled_count)
        }
        Err(err) => panic!("{}", err),
    }
}
