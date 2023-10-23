use cpu::{Cpu, InstructionSet};
use logos::Logos;
use parser::{parse_instructions, Token};

mod cpu;
mod memory;
mod parser;

fn main() {
    let source = r#"
        MVI B,20
        MOV A,B
        "#;
    let instructions = parse_instructions(source);
    let mut cpu = Cpu::new(instructions);
    cpu.run();
    println!("accumulator is {}", cpu.accumulator);
}
