use cpu::{Cpu, InstructionSet};
use logos::Logos;
use parser::Token;

mod cpu;
mod memory;
mod parser;

fn main() {
    let source = "MVI A,90";

    let mut lexer = Token::lexer(source);
    for token in lexer.into_iter() {
        dbg!(token);
    }
}
