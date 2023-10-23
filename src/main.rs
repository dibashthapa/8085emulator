use cpu::Cpu;
use parser::parse_instructions;

mod cpu;
mod memory;
mod parser;

fn main() {
    let source = r#"
        LXI H,2000
        "#;
    let instructions = parse_instructions(source);
    let mut cpu = Cpu::new(instructions);
    cpu.run();
}
