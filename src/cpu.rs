use crate::memory::Memory;

#[derive(PartialEq, Clone, Debug)]
pub enum InstructionSet {
    Add(Registers),
    Mov(Registers, Registers),
    Mvi(Registers, u8),
    Sta(u16),
    Lda(u16),
    Hlt,
}

#[derive(PartialEq, Clone, Debug)]
enum Registers {
    RegB,
    RegC,
    RegD,
    RegE,
    RegH,
    RegL,
    RegA,
}

impl Registers {}

struct FlagRegisters {
    sign: bool,
    zero: bool,
    auxiliary_carry: bool,
    parity: bool,
    carry: bool,
}

#[derive(Clone)]
pub struct Cpu {
    pub ip: usize,
    stack: Vec<i32>,
    program: Vec<InstructionSet>,
    accumulator: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    memory: Memory,
}

impl Cpu {
    pub fn new(program: Vec<InstructionSet>) -> Self {
        Self {
            ip: 0,
            stack: vec![],
            program,
            accumulator: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            memory: Memory::new(),
        }
    }

    pub fn fetch(&self) -> &InstructionSet {
        &self.program[self.ip]
    }

    fn pop(&mut self) -> Option<InstructionSet> {
        self.program.pop()
    }

    fn push(&mut self, value: i32) {
        self.stack.push(value);
    }

    fn advance(&mut self) {
        self.ip += 1;
    }

    pub fn eval(&mut self) {}
    pub fn run(&mut self) {
        while self.ip < self.program.len() {
            match self.fetch() {
                InstructionSet::Lda(address) => {
                    self.accumulator = self.memory.read(*address as usize);
                }
                InstructionSet::Mov(destination, source) => match (destination, source) {
                    (Registers::RegB, Registers::RegA) => {
                        self.b = self.accumulator;
                    }
                    (Registers::RegC, Registers::RegA) => {
                        self.c = self.accumulator;
                    }
                    (Registers::RegD, Registers::RegA) => {
                        self.d = self.accumulator;
                    }
                    (Registers::RegE, Registers::RegA) => {
                        self.e = self.accumulator;
                    }
                    (Registers::RegH, Registers::RegA) => {
                        self.h = self.accumulator;
                    }
                    (Registers::RegL, Registers::RegA) => {
                        self.l = self.accumulator;
                    }
                    (Registers::RegA, Registers::RegB) => {
                        self.accumulator = self.b;
                    }
                    (Registers::RegA, Registers::RegC) => {
                        self.accumulator = self.c;
                    }
                    (Registers::RegB, Registers::RegC) => {
                        self.b = self.c;
                    }
                    (Registers::RegD, Registers::RegC) => {
                        self.d = self.c;
                    }
                    (Registers::RegE, Registers::RegC) => {
                        self.e = self.c;
                    }
                    (Registers::RegH, Registers::RegC) => {
                        self.h = self.c;
                    }
                    (Registers::RegL, Registers::RegC) => {
                        self.l = self.c;
                    }
                    (Registers::RegA, Registers::RegD) => {
                        self.accumulator = self.d;
                    }
                    (Registers::RegA, Registers::RegE) => {
                        self.accumulator = self.e;
                    }
                    (Registers::RegA, Registers::RegH) => {
                        self.accumulator = self.h;
                    }
                    (Registers::RegA, Registers::RegL) => {
                        self.accumulator = self.l;
                    }
                    _ => {
                        println!("Unhandled");
                    }
                },
                InstructionSet::Sta(address) => {
                    self.memory.write(*address as usize, self.accumulator);
                }
                InstructionSet::Mvi(register, value) => match register {
                    Registers::RegA => {
                        self.accumulator = *value;
                    }
                    Registers::RegB => {
                        self.b = *value;
                    }
                    _ => {}
                },

                InstructionSet::Hlt => break,
                _ => {}
            }
            self.advance()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    use super::Registers;
    #[test]
    fn test_mvi() {
        let mut cpu = Cpu::new(vec![
            InstructionSet::Mvi(Registers::RegA, 42),
            InstructionSet::Sta(0x0700),
        ]);

        cpu.run();
        assert_eq!(cpu.memory.read(0x0700), 42);
    }

    #[test]
    fn test_mov_b() {
        let mut cpu = Cpu::new(vec![
            InstructionSet::Mvi(Registers::RegA, 32),
            InstructionSet::Mov(Registers::RegB, Registers::RegA),
        ]);
        cpu.run();
        assert_eq!(cpu.b, 32);
    }
    #[test]
    fn test_mov_a() {
        let mut cpu = Cpu::new(vec![
            InstructionSet::Mvi(Registers::RegB, 32),
            InstructionSet::Mov(Registers::RegA, Registers::RegB),
        ]);
        cpu.run();
        assert_eq!(cpu.accumulator, 32);
    }
    #[test]
    fn test_lda() {
        let mut cpu = Cpu::new(vec![
            InstructionSet::Mvi(Registers::RegA, 40),
            InstructionSet::Sta(0x0600),
            InstructionSet::Lda(0x0600),
        ]);
        cpu.run();
        assert_eq!(cpu.accumulator, 40);
    }
}
