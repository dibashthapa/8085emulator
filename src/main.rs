use cpu::Cpu;
use parser::parse_instructions;

mod cpu;
mod memory;
mod parser;

fn main() {
    let source = r#"
        MVI B,29
        MVI C,29
        INX B
        "#;
    let instructions = parse_instructions(source);
    let mut cpu = Cpu::new(instructions);
    cpu.run();
}
