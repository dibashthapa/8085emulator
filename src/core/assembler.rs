use std::collections::HashMap;

use super::{
    cpu::Registers,
    parser::{Ins, Instruction, JumpTarget},
};

fn split_address(address: u16) -> (u8, u8) {
    let high_byte = ((address & 0xFF00) >> 8) as u8;
    let low_byte = (address & 0x00FF) as u8;
    (low_byte, high_byte)
}

pub fn assemble(instructions: &Vec<Instruction>) -> Vec<u8> {
    let mut assembled_instructions: Vec<u8> = Vec::new();
    let mut symbol_table: HashMap<&str, u16> = HashMap::new();
    let mut unresolved_labels: HashMap<&str, Vec<usize>> = HashMap::new();

    for instruction in instructions {
        if let Some(label) = instruction.label {
            symbol_table.insert(label, assembled_instructions.len() as u16);
        }
        match instruction.ins {
            Ins::Mov(destination, source) => match (destination, source) {
                (Registers::RegA, Registers::RegB) => {
                    assembled_instructions.push(0x78);
                }
                (Registers::RegA, Registers::RegC) => {
                    assembled_instructions.push(0x79);
                }
                (Registers::RegA, Registers::RegD) => {
                    assembled_instructions.push(0x7A);
                }
                (Registers::RegA, Registers::RegE) => {
                    assembled_instructions.push(0x7B);
                }
                (Registers::RegA, Registers::RegH) => {
                    assembled_instructions.push(0x7C);
                }
                (Registers::RegA, Registers::RegL) => {
                    assembled_instructions.push(0x7D);
                }
                (Registers::RegA, Registers::RegA) => {
                    assembled_instructions.push(0x47);
                }
                (Registers::RegB, Registers::RegA) => {
                    assembled_instructions.push(0x7F);
                }
                (Registers::RegB, Registers::RegB) => {
                    assembled_instructions.push(0x40);
                }
                (Registers::RegB, Registers::RegC) => {
                    assembled_instructions.push(0x41);
                }
                (Registers::RegB, Registers::RegD) => {
                    assembled_instructions.push(0x42);
                }
                (Registers::RegB, Registers::RegE) => {
                    assembled_instructions.push(0x43);
                }
                (Registers::RegB, Registers::RegH) => {
                    assembled_instructions.push(0x44);
                }
                (Registers::RegB, Registers::RegL) => {
                    assembled_instructions.push(0x45);
                }
                (Registers::RegC, Registers::RegA) => {
                    assembled_instructions.push(0x4F);
                }
                (Registers::RegC, Registers::RegB) => {
                    assembled_instructions.push(0x48);
                }
                (Registers::RegC, Registers::RegC) => {
                    assembled_instructions.push(0x49);
                }
                (Registers::RegC, Registers::RegD) => {
                    assembled_instructions.push(0x4A);
                }
                (Registers::RegC, Registers::RegE) => {
                    assembled_instructions.push(0x4B);
                }
                (Registers::RegC, Registers::RegH) => {
                    assembled_instructions.push(0x4C);
                }
                (Registers::RegC, Registers::RegL) => {
                    assembled_instructions.push(0x4D);
                }
                (Registers::RegD, Registers::RegA) => {
                    assembled_instructions.push(0x57);
                }
                (Registers::RegD, Registers::RegB) => {
                    assembled_instructions.push(0x50);
                }
                (Registers::RegD, Registers::RegC) => {
                    assembled_instructions.push(0x51);
                }
                (Registers::RegD, Registers::RegD) => {
                    assembled_instructions.push(0x52);
                }
                (Registers::RegD, Registers::RegE) => {
                    assembled_instructions.push(0x53);
                }
                (Registers::RegD, Registers::RegH) => {
                    assembled_instructions.push(0x54);
                }
                (Registers::RegD, Registers::RegL) => {
                    assembled_instructions.push(0x55);
                }
                (Registers::RegE, Registers::RegA) => {
                    assembled_instructions.push(0x5F);
                }
                (Registers::RegE, Registers::RegB) => {
                    assembled_instructions.push(0x58);
                }
                (Registers::RegE, Registers::RegC) => {
                    assembled_instructions.push(0x59);
                }
                (Registers::RegE, Registers::RegD) => {
                    assembled_instructions.push(0x5A);
                }
                (Registers::RegE, Registers::RegE) => {
                    assembled_instructions.push(0x5B);
                }
                (Registers::RegE, Registers::RegH) => {
                    assembled_instructions.push(0x5C);
                }
                (Registers::RegE, Registers::RegL) => {
                    assembled_instructions.push(0x5D);
                }
                (Registers::RegH, Registers::RegA) => {
                    assembled_instructions.push(0x67);
                }
                (Registers::RegH, Registers::RegB) => {
                    assembled_instructions.push(0x60);
                }
                (Registers::RegH, Registers::RegC) => {
                    assembled_instructions.push(0x61);
                }
                (Registers::RegH, Registers::RegD) => {
                    assembled_instructions.push(0x62);
                }
                (Registers::RegH, Registers::RegE) => {
                    assembled_instructions.push(0x63);
                }
                (Registers::RegH, Registers::RegH) => {
                    assembled_instructions.push(0x64);
                }
                (Registers::RegH, Registers::RegL) => {
                    assembled_instructions.push(0x65);
                }
                (Registers::RegL, Registers::RegA) => {
                    assembled_instructions.push(0x6F);
                }
                (Registers::RegL, Registers::RegB) => {
                    assembled_instructions.push(0x68);
                }
                (Registers::RegL, Registers::RegC) => {
                    assembled_instructions.push(0x69);
                }
                (Registers::RegL, Registers::RegD) => {
                    assembled_instructions.push(0x6A);
                }
                (Registers::RegL, Registers::RegE) => {
                    assembled_instructions.push(0x6B);
                }
                (Registers::RegL, Registers::RegH) => {
                    assembled_instructions.push(0x6C);
                }
                (Registers::RegL, Registers::RegL) => {
                    assembled_instructions.push(0x6D);
                }
            },
            Ins::Mvi(register, value) => match register {
                Registers::RegA => {
                    assembled_instructions.push(0x3E);

                    assembled_instructions.push(value);
                }
                Registers::RegB => {
                    assembled_instructions.push(0x06);

                    assembled_instructions.push(value);
                }
                Registers::RegC => {
                    assembled_instructions.push(0x0E);
                    assembled_instructions.push(value);
                }
                Registers::RegD => {
                    assembled_instructions.push(0x16);
                    assembled_instructions.push(value);
                }
                Registers::RegE => {
                    assembled_instructions.push(0x1E);
                    assembled_instructions.push(value);
                }
                Registers::RegH => {
                    assembled_instructions.push(0x26);
                    assembled_instructions.push(value);
                }
                Registers::RegL => {
                    assembled_instructions.push(0x2E);
                    assembled_instructions.push(value);
                }
            },
            Ins::Adi(value) => {
                assembled_instructions.push(0xC6);
                assembled_instructions.push(value);
            }
            Ins::Add(Registers::RegA) => {
                assembled_instructions.push(0x87);
            }
            Ins::Add(Registers::RegB) => {
                assembled_instructions.push(0x80);
            }
            Ins::Add(Registers::RegC) => {
                assembled_instructions.push(0x81);
            }
            Ins::Add(Registers::RegD) => {
                assembled_instructions.push(0x82);
            }
            Ins::Add(Registers::RegE) => {
                assembled_instructions.push(0x83);
            }
            Ins::Add(Registers::RegH) => {
                assembled_instructions.push(0x84);
            }
            Ins::Jmp(value) => {
                if let JumpTarget::Address(address) = value {
                    assembled_instructions.push(0xC3);

                    let (low_byte, high_byte) = split_address(address);
                    assembled_instructions.push(low_byte);

                    assembled_instructions.push(high_byte);
                } else if let JumpTarget::Label(label) = value {
                    if let Some(address) = symbol_table.get(label) {
                        let (low_byte, high_byte) = split_address(*address);
                        assembled_instructions.push(0xC3);
                        assembled_instructions.push(low_byte);
                        assembled_instructions.push(high_byte);
                    } else {
                        let location = assembled_instructions.len();
                        assembled_instructions.push(0xC3);
                        assembled_instructions.push(0x00);
                        assembled_instructions.push(0x00);
                        unresolved_labels.entry(label).or_default().push(location);
                    }
                }
            }
            _ => {}
        }
    }

    for (label, locations) in unresolved_labels {
        if let Some(address) = symbol_table.get(label) {
            let (low_byte, high_byte) = split_address(*address);
            println!(
                "Addres is {:04X}",
                u16::from_be_bytes([high_byte, low_byte])
            );
            for location in locations {
                assembled_instructions[location + 1] = low_byte;
                assembled_instructions[location + 2] = high_byte;
            }
        }
    }

    assembled_instructions
}
