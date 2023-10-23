use crate::memory::Memory;

#[derive(PartialEq, Clone, Debug)]
pub enum InstructionSet {
    Add(Registers),
    Sub(Registers),
    Mov(Registers, Registers),
    Mvi(Registers, u8),
    Sta(u16),
    Lxi(Registers, u16),
    Lda(u16),
    Hlt,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Registers {
    RegB,
    RegC,
    RegD,
    RegE,
    RegH,
    RegL,
    RegA,
}

impl Registers {
    pub fn from(reg: &str) -> Registers {
        match reg {
            "A" => Registers::RegA,
            "B" => Registers::RegB,
            "C" => Registers::RegC,
            "D" => Registers::RegD,
            "E" => Registers::RegE,
            "H" => Registers::RegH,
            "L" => Registers::RegH,
            _ => panic!("unknown register"),
        }
    }
}

#[derive(Clone, Debug)]
struct FlagRegisters {
    pub sign: bool,
    pub zero: bool,
    pub auxiliary_carry: bool,
    pub parity: bool,
    pub carry: bool,
}
impl FlagRegisters {
    fn new() -> Self {
        Self {
            sign: false,
            zero: false,
            auxiliary_carry: false,
            parity: false,
            carry: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Cpu {
    pub ip: usize,
    stack: Vec<i32>,
    program: Vec<InstructionSet>,
    pub accumulator: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    memory: Memory,
    flags: FlagRegisters,
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
            flags: FlagRegisters::new(),
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

    pub fn run(&mut self) {
        while self.ip < self.program.len() {
            match self.fetch() {
                InstructionSet::Lda(address) => {
                    self.accumulator = self.memory.read(*address as usize);
                }
                InstructionSet::Sub(register) => match register {
                    Registers::RegB => self.accumulator = self.accumulator - self.b,
                    Registers::RegC => {
                        self.accumulator = self.accumulator - self.c;
                    }
                    Registers::RegD => {
                        self.accumulator = self.accumulator - self.d;
                    }
                    Registers::RegE => {
                        self.accumulator = self.accumulator - self.e;
                    }
                    Registers::RegH => {
                        self.accumulator = self.accumulator - self.h;
                    }
                    Registers::RegL => {
                        self.accumulator = self.accumulator - self.l;
                    }
                    Registers::RegA => {
                        self.accumulator = self.accumulator - self.accumulator;
                    }
                },
                InstructionSet::Add(register) => match register {
                    Registers::RegB => {
                        self.accumulator = self.accumulator + self.b;
                    }
                    Registers::RegC => {
                        self.accumulator = self.accumulator + self.c;
                    }
                    Registers::RegD => {
                        self.accumulator = self.accumulator + self.d;
                    }
                    Registers::RegE => {
                        self.accumulator = self.accumulator + self.e;
                    }
                    Registers::RegH => {
                        self.accumulator = self.accumulator + self.h;
                    }
                    Registers::RegL => {
                        self.accumulator = self.accumulator + self.l;
                    }
                    Registers::RegA => {
                        self.accumulator = self.accumulator + self.accumulator;
                    }
                },
                InstructionSet::Lxi(rpair, value) => match rpair {
                    Registers::RegB => {
                        let high_byte: u8 = ((value & 0xFF00) >> 8) as u8;
                        let low_byte: u8 = (value & 0x00FF) as u8;
                        self.b = high_byte;
                        self.c = low_byte;
                    }
                    Registers::RegD => {
                        let high_byte: u8 = ((value & 0xFF00) >> 8) as u8;
                        let low_byte: u8 = (value & 0x00FF) as u8;
                        self.d = high_byte;
                        self.e = low_byte;
                    }
                    Registers::RegH => {
                        let high_byte: u8 = ((value & 0xFF00) >> 8) as u8;
                        let low_byte: u8 = (value & 0x00FF) as u8;
                        self.h = high_byte;
                        self.l = low_byte;
                    }
                    _ => {
                        eprintln!("{:#?} not supported ", rpair);
                    }
                },
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
                    Registers::RegC => {
                        self.c = *value;
                    }
                    Registers::RegD => {
                        self.d = *value;
                    }
                    Registers::RegE => {
                        self.e = *value;
                    }
                    Registers::RegH => {
                        self.h = *value;
                    }
                    Registers::RegL => {
                        self.l = *value;
                    }
                },

                InstructionSet::Hlt => break,
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

    #[test]
    fn test_lxi_b() {
        let mut cpu = Cpu::new(vec![InstructionSet::Lxi(Registers::RegB, 0x2050)]);
        cpu.run();
        assert_eq!(cpu.b, 0x20);
        assert_eq!(cpu.c, 0x50);
    }

    #[test]
    fn test_lxi_d() {
        let mut cpu = Cpu::new(vec![InstructionSet::Lxi(Registers::RegD, 0x2051)]);
        cpu.run();
        assert_eq!(cpu.d, 0x20);
        assert_eq!(cpu.e, 0x51);
    }

    #[test]
    fn test_lxi_h() {
        let mut cpu = Cpu::new(vec![InstructionSet::Lxi(Registers::RegH, 0xFFFF)]);
        cpu.run();
        assert_eq!(cpu.h, 0xFF);
        assert_eq!(cpu.l, 0xFF);
    }

    #[test]
    fn test_add_b() {
        let mut cpu = Cpu::new(vec![
            InstructionSet::Mvi(Registers::RegA, 0x20),
            InstructionSet::Mvi(Registers::RegB, 0x30),
            InstructionSet::Add(Registers::RegB),
        ]);
        cpu.run();
        assert_eq!(cpu.accumulator, 0x50)
    }

    #[test]
    fn test_add_c() {
        let mut cpu = Cpu::new(vec![
            InstructionSet::Mvi(Registers::RegA, 0x60),
            InstructionSet::Mvi(Registers::RegC, 0x90),
            InstructionSet::Add(Registers::RegC),
        ]);
        cpu.run();
        assert_eq!(cpu.accumulator, 0xF0);
    }

    #[test]
    fn test_sub_b() {
        let mut cpu = Cpu::new(vec![
            InstructionSet::Mvi(Registers::RegA, 0x90),
            InstructionSet::Mvi(Registers::RegC, 0x60),
            InstructionSet::Sub(Registers::RegC),
        ]);
        cpu.run();
        assert_eq!(cpu.accumulator, 0x30);
    }
}
