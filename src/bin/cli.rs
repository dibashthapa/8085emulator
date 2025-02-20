use std::process::exit;

use emulator_8085::core::{assembler::assemble, cpu::Cpu, parser::parse, token::Token};

pub fn main() {
    use logos::Logos;
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        exit(1);
    }
    let source = std::fs::read_to_string(&args[1]).expect("Couldn;t read file");
    let lexer = Token::lexer(&source);
    let tokens: Vec<_> = lexer.filter_map(|token| token.ok()).collect();
    let instructions = parse(tokens);
    match instructions {
        Ok(instructions) => {
            let assembled_instructions = assemble(&instructions);
            let mut cpu = Cpu::new();
            for (index, inst) in assembled_instructions.iter().enumerate() {
                cpu.write_memory(index, *inst);
            }
            let assembled_count = assembled_instructions.iter().len();
            while let Some(pc) = cpu.eval() {
                if pc as usize >= assembled_count {
                    break;
                }
            }
            cpu.print_memory();
            cpu.print();
        }
        Err(err) => eprintln!("{}", err),
    }
}
