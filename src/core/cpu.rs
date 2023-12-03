const MEMORY_SIZE: usize = 0xFFFF;

#[derive(PartialEq, Clone, Debug, Copy)]
pub enum Registers {
    RegB,
    RegC,
    RegD,
    RegE,
    RegH,
    RegL,
    RegA,
    RegM,
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
            "L" => Registers::RegL,
            "M" => Registers::RegM,
            _ => panic!("unknown register"),
        }
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
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
#[allow(dead_code)]
pub struct Cpu {
    pub pc: u16,
    sp: usize,
    pub accumulator: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub memory: [u8; MEMORY_SIZE],
    flags: FlagRegisters,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            pc: 0,
            sp: 0,
            accumulator: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            memory: [0; MEMORY_SIZE],
            flags: FlagRegisters::new(),
        }
    }

    #[allow(dead_code)]
    pub fn print_memory(&self) {
        for (i, address) in self.memory.iter().enumerate() {
            if *address != 0 {
                println!("----------------");
                print!("| ");
                println!("{:#06x}: {:#04x} |", i, address);
            }
        }
        println!("----------------");
    }

    pub fn reset_pc(&mut self) {
        self.pc = 0;
    }

    pub fn reset_flags(&mut self) {
        self.flags = FlagRegisters::new()
    }

    pub fn reset_registers(&mut self) {
        self.pc = 0;
        self.sp = 0;
        self.accumulator = 0;
        self.b = 0;
        self.c = 0;
        self.d = 0;
        self.e = 0;
        self.h = 0;
        self.l = 0;
    }

    pub fn reset_memory(&mut self) {
        self.memory = [0; MEMORY_SIZE];
    }

    pub fn write_memory(&mut self, address: usize, value: u8) {
        self.memory[address] = value;
    }

    pub fn read_memory(&self, address: usize) -> u8 {
        self.memory[address]
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        println!(
            r#" 
|-------------|-------------|
|accumulator: {:<4x}        | 
|-------------|-------------|
|b: {:<4x}      | c: {:<4x} | 
|-------------|-------------|
|d: {:<4x}      | e: {:<4x} | 
|-------------|-------------|
|h: {:<4x}      | l: {:<4x} | 
|-------------|-------------|
|stack pointer:  {:<4x}     | 
|-------------|-------------|
| PC : {:<4x}                |
|-------------|-------------|
| Sign: {:<4}  | Zero: {:<4} |
|-------------|-------------|
| Aux: {:<4}   | Parity: {:<4}|
|-------------|-------------|
| Carry: {:<4} |             |
|-------------|-------------|
        "#,
            self.accumulator,
            self.b,
            self.c,
            self.d,
            self.e,
            self.h,
            self.l,
            self.sp,
            self.pc,
            i8::from(self.flags.sign),
            i8::from(self.flags.zero),
            i8::from(self.flags.auxiliary_carry),
            i8::from(self.flags.parity),
            i8::from(self.flags.carry),
        );
    }

    pub fn fetch(&mut self) -> u8 {
        self.memory[self.pc as usize]
    }

    pub fn eval(&mut self) -> Option<u16> {
        match self.memory[self.pc as usize] {
            // Dad B
            0x09 => {
                self.h = self.h.wrapping_add(self.b);
                self.l = self.l.wrapping_add(self.c);
                self.pc += 1;
            }

            // Dad D
            0x19 => {
                self.h = self.h.wrapping_add(self.d);
                self.l = self.l.wrapping_add(self.e);
                self.pc += 1;
            }

            // Dad H
            0x29 => {
                self.h = self.h.wrapping_add(self.h);
                self.l = self.l.wrapping_add(self.l);
                self.pc += 1;
            }

            // ADC A
            0xC6 => {
                self.accumulator =
                    self.accumulator + self.accumulator + if self.flags.carry { 1 } else { 0 };
            }

            // ADC B
            0x8F => {
                self.accumulator = self.accumulator + self.b + if self.flags.carry { 1 } else { 0 };
            }

            // ADC C
            0x88 => {
                self.accumulator = self.accumulator + self.c + if self.flags.carry { 1 } else { 0 };
            }

            // ADC D
            0x89 => {
                self.accumulator = self.accumulator + self.d + if self.flags.carry { 1 } else { 0 };
            }

            // ADC E
            0x8B => {
                self.accumulator = self.accumulator + self.e + if self.flags.carry { 1 } else { 0 };
            }

            // ADC H
            0x8C => {
                self.accumulator = self.accumulator + self.h + if self.flags.carry { 1 } else { 0 };
                self.pc += 1;
            }

            // ADC L
            0x8D => {
                self.accumulator = self.accumulator + self.l + if self.flags.carry { 1 } else { 0 };
            }

            // XCHG
            0xEB => {
                std::mem::swap(&mut self.d, &mut self.h);
                std::mem::swap(&mut self.e, &mut self.l);
                self.pc += 1;
            }

            // STAX B
            0x02 => {
                let address = u16::from_be_bytes([self.b, self.c]);
                self.write_memory(address as usize, self.accumulator);
            }

            // STAX D
            0x12 => {
                let address = u16::from_be_bytes([self.d, self.e]);
                self.write_memory(address as usize, self.accumulator);
            }

            // LDAX B
            0x0A => {
                let address = u16::from_be_bytes([self.b, self.c]);
                self.accumulator = self.read_memory(address as usize);
            }

            // LDAX D
            0x1A => {
                let address = u16::from_be_bytes([self.d, self.e]);
                self.accumulator = self.read_memory(address as usize);
            }

            // LHLD address
            0x2A => {
                self.pc += 1;
                let low_byte_address = self.fetch();
                self.pc += 1;
                let high_byte_address = self.fetch();
                let address = u16::from_be_bytes([high_byte_address, low_byte_address]);
                let low_byte = self.read_memory(address as usize);
                let high_byte = self.read_memory((address + 1) as usize);
                self.l = low_byte;
                self.h = high_byte;
                self.pc += 1;
            }

            // DCR A
            0x3D => {
                self.accumulator = self.accumulator.wrapping_sub(1);
            }

            // DCR B
            0x05 => {
                self.b = self.b.wrapping_sub(1);

                if self.b == 0 {
                    self.flags.zero = true;
                }
                self.pc += 1;
            }

            // DCR C
            0x0D => {
                self.c = self.c.wrapping_sub(1);
                if self.c == 0 {
                    self.flags.zero = true;
                }
                self.pc += 1;
            }

            // DCR D
            0x15 => {
                self.d = self.d.wrapping_sub(1);
                if self.d == 0 {
                    self.flags.zero = true;
                }
                self.pc += 1;
            }

            // DCR E
            0x1D => {
                self.e = self.e.wrapping_sub(1);
            }

            // DCR H
            0x25 => {
                self.h = self.h.wrapping_sub(1);
            }

            // DCR L
            0x2D => {
                self.l = self.l.wrapping_sub(1);
            }

            // INR A
            0x3C => {
                self.accumulator = self.accumulator.wrapping_add(1);
            }

            // INR B
            0x04 => {
                self.b = self.b.wrapping_add(1);

                if self.b == 0 {
                    self.flags.zero = true;
                }
                self.pc += 1;
            }

            // INR C
            0x0C => {
                self.c = self.c.wrapping_add(1);
            }

            // INR D
            0x14 => {
                self.d = self.d.wrapping_add(1);
            }

            // INR E
            0x1C => {
                self.e = self.e.wrapping_add(1);
            }

            // INR H
            0x24 => {
                self.h = self.h.wrapping_add(1);
            }

            // INR L
            0x2C => {
                self.l = self.l.wrapping_add(1);
            }

            // INX B
            0x03 => {
                let address = u16::from_be_bytes([self.b, self.c]);
                let new_address = address.wrapping_add(1);
                self.b = (new_address >> 8) as u8;
                self.c = new_address as u8;
            }

            // INX D
            0x13 => {
                let address = u16::from_be_bytes([self.d, self.e]);
                let new_address = address.wrapping_add(1);
                self.d = (new_address >> 8) as u8;
                self.e = new_address as u8;
            }

            // INX H
            0x23 => {
                let address = u16::from_be_bytes([self.h, self.l]);
                let new_address = address.wrapping_add(1);

                self.h = (new_address >> 8) as u8;
                self.l = new_address as u8;
                self.pc += 1;
            }

            // LDA address
            0x3A => {
                self.pc += 1;
                let low_byte_address = self.fetch();
                self.pc += 1;
                let high_byte_address = self.fetch();
                let address = u16::from_be_bytes([high_byte_address, low_byte_address]);
                self.accumulator = self.read_memory(address as usize);
                self.pc += 1;
            }

            // SUB A
            0x97 => {
                self.accumulator = 0;
            }

            // SUB B
            0x90 => {
                self.accumulator -= self.b;
                self.pc += 1;
            }

            // SUB C
            0x91 => {
                self.accumulator -= self.c;
                self.pc += 1;
            }

            // SUB D
            0x92 => {
                self.accumulator -= self.d;
                self.pc += 1;
            }

            // SUB E
            0x93 => {
                self.accumulator -= self.e;
                self.pc += 1;
            }

            // SUB H
            0x94 => {
                self.accumulator -= self.h;
                self.pc += 1;
            }

            // SUB L
            0x95 => {
                self.accumulator -= self.l;
                self.pc += 1;
            }

            // SUB M
            0x96 => {
                let address = u16::from_be_bytes([self.h, self.l]);
                let value = self.read_memory(address as usize);
                self.accumulator -= value;
                self.pc += 1;
            }

            // ADD A
            0x87 => {
                if let Some(result) = self.accumulator.checked_add(self.accumulator) {
                    self.accumulator = result;
                } else {
                    self.flags.carry = true;
                }
                self.pc += 1;
            }

            // ADD B
            0x80 => {
                if let Some(result) = self.accumulator.checked_add(self.b) {
                    self.accumulator = result;
                } else {
                    self.flags.carry = true;
                }
                self.pc += 1;
            }

            // ADD C
            0x81 => {
                if let Some(result) = self.accumulator.checked_add(self.c) {
                    self.accumulator = result;
                } else {
                    self.flags.carry = true;
                }
            }

            // ADD D
            0x82 => {
                if let Some(result) = self.accumulator.checked_add(self.d) {
                    self.accumulator = result;
                } else {
                    self.flags.carry = true;
                }
            }

            // ADD E
            0x83 => {
                if let Some(result) = self.accumulator.checked_add(self.e) {
                    self.accumulator = result;
                } else {
                    self.flags.carry = true;
                }
            }

            // ADD H
            0x84 => {
                if let Some(result) = self.accumulator.checked_add(self.h) {
                    self.accumulator = result;
                } else {
                    self.flags.carry = true;
                }
            }

            // ADD L
            0x85 => {
                if let Some(result) = self.accumulator.checked_add(self.l) {
                    self.accumulator = result;
                } else {
                    self.flags.carry = true;
                }
                self.pc += 1;
            }

            // ADD M
            0x86 => {
                let address = u16::from_be_bytes([self.h, self.l]);
                let value = self.read_memory(address as usize);

                if let Some(result) = self.accumulator.checked_add(value) {
                    self.accumulator = result;
                } else {
                    self.flags.carry = true;
                }

                self.pc += 1;
            }
            // LXI B, value
            0x01 => {
                let high_byte = self.fetch();
                let low_byte = self.fetch();
                self.b = high_byte;
                self.c = low_byte;
            }

            // LXI D, value
            0x11 => {
                let high_byte = self.fetch();
                let low_byte = self.fetch();
                self.d = high_byte;
                self.e = low_byte;
            }

            // LXI H, value
            0x21 => {
                self.pc += 1;
                let high_byte = self.fetch();
                self.pc += 1;
                let low_byte = self.fetch();
                self.h = low_byte;
                self.l = high_byte;
                self.pc += 1;
            }

            // MOV A, A
            0x7F => {
                self.pc += 1;
            }
            // MOV A, B
            0x78 => {
                self.accumulator = self.b;
                self.pc += 1;
            }
            // MOV A, C
            0x79 => {
                self.accumulator = self.c;
                self.pc += 1;
            }

            // MOV A, D
            0x7A => {
                self.accumulator = self.d;
                self.pc += 1;
            }
            // MOV A, E
            0x7B => {
                self.accumulator = self.e;
                self.pc += 1;
            }
            // MOV A, H
            0x7C => {
                self.accumulator = self.h;
                self.pc += 1;
            }
            // MOV A, L
            0x7D => {
                self.accumulator = self.l;
                self.pc += 1;
            }
            // MOV A, M
            0x7E => {
                let address = u16::from_be_bytes([self.h, self.l]);
                self.accumulator = self.read_memory(address as usize);
                self.pc += 1;
            }
            // MOV B, A
            0x47 => {
                self.b = self.accumulator;
                self.pc += 1;
            }
            // MOV B, B
            0x40 => {
                self.pc += 1;
            }
            // MOV B, C
            0x41 => {
                self.b = self.c;
                self.pc += 1;
            }
            // MOV B, D
            0x42 => {
                self.b = self.d;
                self.pc += 1;
            }
            // MOV B, E
            0x43 => {
                self.b = self.e;
                self.pc += 1;
            }
            // MOV B, H
            0x44 => {
                self.b = self.h;
                self.pc += 1;
            }
            // MOV B, L
            0x45 => {
                self.b = self.l;
                self.pc += 1;
            }
            // MOV B, M
            0x46 => {
                let address = u16::from_be_bytes([self.h, self.l]);
                self.b = self.read_memory(address as usize);
                self.pc += 1;
            }
            // MOV C, A
            0x4F => {
                self.c = self.accumulator;
                self.pc += 1;
            }
            // MOV C, B
            0x48 => {
                self.c = self.b;
                self.pc += 1;
            }
            // MOV C, C
            0x49 => {
                self.pc += 1;
            }
            // MOV C, D
            0x4A => {
                self.c = self.d;
                self.pc += 1;
            }
            // MOV C, E
            0x4B => {
                self.c = self.e;
                self.pc += 1;
            }
            // MOV C, H
            0x4C => {
                self.c = self.h;
                self.pc += 1;
            }
            // MOV C, L
            0x4D => {
                self.c = self.l;
                self.pc += 1;
            }
            // MOV D, A
            0x57 => {
                self.d = self.accumulator;
            }
            // MOV D, B
            0x50 => {
                self.d = self.b;
            }
            // MOV D, C
            0x51 => {
                self.d = self.c;
            }

            // MOV D, D
            0x52 => {
                self.pc += 1;
            }
            // MOV D, E
            0x53 => {
                self.d = self.e;
            }
            // MOV D, H
            0x54 => {
                self.d = self.h;
            }
            // MOV D, L
            0x55 => {
                self.d = self.l;
            }

            // MOV E, A
            0x5F => {
                self.e = self.accumulator;
            }

            // MOV E, B
            0x58 => {
                self.e = self.b;
            }

            // MOV E, C
            0x59 => {
                self.e = self.c;
            }

            // MOV E, D
            0x5A => {
                self.e = self.d;
            }

            // MOV E, E
            0x5B => {
                self.pc += 1;
            }

            // MOV E, H
            0x5C => {
                self.e = self.h;
            }

            // MOV E, L
            0x5D => {
                self.e = self.l;
            }

            // MOV H, A
            0x67 => {
                self.h = self.accumulator;
                self.pc += 1;
            }

            // MOV H, B
            0x60 => {
                self.h = self.b;
            }

            // MOV H, C
            0x61 => {
                self.h = self.c;
            }

            // MOV H, D
            0x62 => {
                self.h = self.d;
            }

            // MOV H, E
            0x63 => {
                self.h = self.e;
            }

            // MOV H, L
            0x65 => {
                self.h = self.l;
            }

            // MOV L, A
            0x6F => {
                self.l = self.accumulator;
                self.pc += 1;
            }

            // MOV L, B
            0x68 => {
                self.l = self.b;
            }

            // MOV L, C
            0x69 => {
                self.l = self.c;
            }

            // MOV L, D
            0x6A => {
                self.l = self.d;
            }

            // MOV L, E
            0x6B => {
                self.l = self.e;
            }

            // MOV L, H
            0x6C => {
                self.l = self.h;
            }

            // MOV L, L
            0x6D => {
                self.pc += 1;
            }

            // MOV M, A
            0x77 => {
                let address = u16::from_be_bytes([self.h, self.l]);
                self.write_memory(address as usize, self.accumulator);
                self.pc += 1;
            }

            // MOV M, B
            0x70 => {
                let address = u16::from_be_bytes([self.h, self.l]);
                self.write_memory(address as usize, self.b);
                self.pc += 1;
            }

            // MVI A, value
            0x3E => {
                self.pc += 1;
                self.accumulator = self.fetch();
                self.pc += 1;
            }

            // MVI B, value
            0x06 => {
                self.pc += 1;
                self.b = self.fetch();
                self.pc += 1;
            }
            // MVI C, value
            0x0E => {
                self.pc += 1;
                self.c = self.fetch();
                self.pc += 1;
            }
            // MVI D, value
            0x16 => {
                self.d = self.fetch();
            }
            // MVI E, value
            0x1E => {
                self.e = self.fetch();
            }
            // MVI H, value
            0x26 => {
                self.pc += 1;
                self.h = self.fetch();
                self.pc += 1;
            }
            // MVI L, value
            0x2E => {
                self.l = self.fetch();
            }

            // ANA A
            0xA7 => {
                self.accumulator &= self.accumulator;
            }
            // ANA B
            0xA0 => {
                self.accumulator &= self.b;
            }
            // ANA C
            0xA1 => {
                self.accumulator &= self.c;
            }
            // ANA D
            0xA2 => {
                self.accumulator &= self.d;
            }
            // ANA E
            0xA3 => {
                self.accumulator &= self.e;
            }
            // ANA H
            0xA4 => {
                self.accumulator &= self.h;
            }
            // ANA L
            0xA5 => {
                self.accumulator &= self.l;
            }
            // ANA M
            0xA6 => {
                let address = u16::from_be_bytes([self.h, self.l]);
                self.accumulator &= self.read_memory(address as usize);
            }

            // ORA A
            0xB7 => {
                self.accumulator |= self.accumulator;
            }
            // ORA B
            0xB0 => {
                self.accumulator |= self.b;
            }
            // ORA C
            0xB1 => {
                self.accumulator |= self.c;
            }
            // ORA D
            0xB2 => {
                self.accumulator |= self.d;
            }
            // ORA E
            0xB3 => {
                self.accumulator |= self.e;
            }
            // ORA H
            0xB4 => {
                self.accumulator |= self.h;
            }
            // ORA L
            0xB5 => {
                self.accumulator |= self.l;
            }

            // CMA
            0x2F => {
                self.accumulator = !self.accumulator;
            }

            // CMP A
            0xBF => {
                self.flags.sign = (self.accumulator.wrapping_sub(self.accumulator) & 0x80) != 0;
            }

            // CMP B
            0xB8 => {
                self.flags.zero = self.accumulator == self.b;
                self.flags.sign = (self.accumulator.wrapping_sub(self.b) & 0x80) != 0;
                self.flags.carry = self.b > self.accumulator;
            }

            // CMP C
            0xB9 => {
                self.flags.zero = self.accumulator == self.c;
                self.flags.sign = (self.accumulator.wrapping_sub(self.c) & 0x80) != 0;
                self.flags.carry = self.c > self.accumulator;
            }
            // CMP D
            0xBA => {
                self.flags.zero = self.accumulator == self.d;
                self.flags.sign = (self.accumulator.wrapping_sub(self.d) & 0x80) != 0;
                self.flags.carry = self.d > self.accumulator;
            }
            // CMP E
            0xBB => {
                self.flags.zero = self.accumulator == self.e;
                self.flags.sign = (self.accumulator.wrapping_sub(self.e) & 0x80) != 0;
                self.flags.carry = self.e > self.accumulator;
            }
            // CMP H
            0xBC => {
                self.flags.zero = self.accumulator == self.h;
                self.flags.sign = (self.accumulator.wrapping_sub(self.h) & 0x80) != 0;
                self.flags.carry = self.h > self.accumulator;
            }

            // CMP L
            0xBD => {
                self.flags.zero = self.accumulator == self.l;
                self.flags.sign = (self.accumulator.wrapping_sub(self.l) & 0x80) != 0;
                self.flags.carry = self.l > self.accumulator;
            }

            // CMP M
            0xBE => {
                let address = u16::from_be_bytes([self.h, self.l]);
                self.flags.zero = self.accumulator == self.read_memory(address as usize);
                self.flags.sign = (self
                    .accumulator
                    .wrapping_sub(self.read_memory(address as usize))
                    & 0x80)
                    != 0;
                self.flags.carry = self.read_memory(address as usize) > self.accumulator;
                self.pc += 1;
            }

            // SHLD address
            0x22 => {
                self.pc += 1;
                let low_byte_address = self.fetch();
                self.pc += 1;
                let high_byte_address = self.fetch();
                let address = u16::from_be_bytes([high_byte_address, low_byte_address]);
                self.write_memory(address as usize, self.l);
                self.write_memory((address + 1) as usize, self.h);
                self.pc += 1;
            }
            // STA Address
            0x32 => {
                self.pc += 1;
                let low_byte_address = self.fetch();
                self.pc += 1;
                let high_byte_address = self.fetch();
                let address = u16::from_be_bytes([high_byte_address, low_byte_address]);
                self.write_memory(address as usize, self.accumulator);
                self.pc += 1;
            }

            // JMP Address
            0xC3 => {
                self.pc += 1;
                let low_byte_address = self.fetch();
                self.pc += 1;
                let high_byte_address = self.fetch();
                let address = u16::from_be_bytes([high_byte_address, low_byte_address]);
                self.pc = address;
            }

            // JNZ address
            0xC2 => {
                self.pc += 1;
                let low_byte_address = self.fetch();
                self.pc += 1;
                let high_byte_address = self.fetch();
                let address = u16::from_be_bytes([high_byte_address, low_byte_address]);
                if !self.flags.zero {
                    self.pc = address;
                } else {
                    self.pc += 1;
                }
            }

            // JNC Address
            0xD2 => {
                self.pc += 1;
                let low_byte_address = self.fetch();
                self.pc += 1;
                let high_byte_address = self.fetch();
                let address = u16::from_be_bytes([high_byte_address, low_byte_address]);
                if !self.flags.carry {
                    self.pc = address;
                } else {
                    self.pc += 1;
                }
            }

            // ANI value
            0xE6 => {
                self.pc += 1;
                let value = self.fetch();
                self.accumulator = self.accumulator & value;
                self.pc += 1;
            }
            0x76 => return None,
            _ => {}
        }

        Some(self.pc)
    }
}
