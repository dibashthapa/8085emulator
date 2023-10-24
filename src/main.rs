use cpu::Cpu;
use parser::parse_instructions;

mod cpu;
mod memory;
mod parser;
mod token;

struct Dibash {
    name: String,
    age: u8,
}

fn main() {
    let source = r#"
        MVI A, 20
        INR A
        "#;
    let instructions = parse_instructions(source);
    let mut cpu = Cpu::new(instructions);
    cpu.run();
    println!("accumulator is {}", cpu.accumulator);
}
