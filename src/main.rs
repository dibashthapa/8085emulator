use cpu::Cpu;
use parser::parse_instructions;

mod cpu;
mod parser;
mod token;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <source file>", args[0]);
        std::process::exit(1);
    }
    let source_file = &args[1];
    let source = std::fs::read_to_string(source_file).expect("Failed to read source file");
    let instructions = parse_instructions(&source);
    let mut cpu = Cpu::new();
    let memory_count = instructions.iter().len();
    for (i, instruction) in instructions.iter().enumerate() {
        cpu.write_memory(i, *instruction);
    }
    cpu.print_memory();
    println!("memory count is 0x{:X}", memory_count);

    loop {
        match cpu.eval() {
            Some(pc) => {
                // println!("PC is {:X}", pc);
                if pc as usize >= memory_count {
                    break;
                }
            }
            None => break,
        }
    }

    cpu.print()
}
