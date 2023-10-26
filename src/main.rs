use cpu::Cpu;
use parser::parse_instructions;

mod cpu;
mod memory;
mod parser;
mod token;


fn main() {
    let source = r#"
        MVI A, FF
        MVI B, 01
        ADD B
        "#;
    let instructions = parse_instructions(source);
    let mut cpu = Cpu::new(instructions);
    cpu.run();
    cpu.print();
}
