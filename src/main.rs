use cpu::Cpu;
use parser::parse_instructions;

mod cpu;
mod memory;
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
    let mut cpu = Cpu::new(instructions);
    cpu.memory.write(0x2501, 0x15);
    cpu.memory.write(0x2502, 0x1C);
    cpu.memory.write(0x2503, 0xB7);
    cpu.memory.write(0x2504, 0x5A);
    cpu.run();
    cpu.print();
    cpu.print_memory();
}
