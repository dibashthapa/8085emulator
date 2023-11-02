use super::{cpu::Registers, token::Token};
use logos::Logos;

pub fn parse_instructions(code: &str) -> Vec<u8> {
    let mut lexer = Token::lexer(code);
    let mut instructions = Vec::new();

    while let Some(token) = lexer.next() {
        match token {
            Ok(token) => match token {
                Token::OpCode => match lexer.slice() {
                    "MVI" => {
                        if let Some(Ok(Token::Register)) = lexer.next() {
                            let register = Registers::from(lexer.slice());
                            match register {
                                Registers::RegA => {
                                    instructions.push(0x3E);
                                    if let Some(Ok(Token::Number(value))) = lexer.next() {
                                        instructions.push(value);
                                    }
                                }
                                Registers::RegB => {
                                    instructions.push(0x06);
                                    if let Some(Ok(Token::Number(value))) = lexer.next() {
                                        instructions.push(value);
                                    }
                                }
                                Registers::RegC => {
                                    instructions.push(0x0E);
                                    if let Some(Ok(Token::Number(value))) = lexer.next() {
                                        instructions.push(value);
                                    }
                                }
                                Registers::RegD => {
                                    instructions.push(0x16);
                                    if let Some(Ok(Token::Number(value))) = lexer.next() {
                                        instructions.push(value);
                                    }
                                }
                                Registers::RegE => {
                                    instructions.push(0x1E);
                                    if let Some(Ok(Token::Number(value))) = lexer.next() {
                                        instructions.push(value);
                                    }
                                }
                                Registers::RegH => {
                                    instructions.push(0x26);
                                    if let Some(Ok(Token::Number(value))) = lexer.next() {
                                        instructions.push(value);
                                    }
                                }
                                Registers::RegL => {
                                    instructions.push(0x2E);
                                    if let Some(Ok(Token::Number(value))) = lexer.next() {
                                        instructions.push(value);
                                    }
                                }
                            }
                        }
                    }
                    "MOV" => {
                        if let Some(Ok(Token::Register)) = lexer.next() {
                            let register = Registers::from(lexer.slice());
                            match register {
                                Registers::RegA => {
                                    if let Some(Ok(Token::Register)) = lexer.next() {
                                        let register = Registers::from(lexer.slice());
                                        match register {
                                            Registers::RegA => {
                                                instructions.push(0x7F);
                                            }
                                            Registers::RegB => {
                                                instructions.push(0x78);
                                            }
                                            Registers::RegC => {
                                                instructions.push(0x79);
                                            }
                                            Registers::RegD => {
                                                instructions.push(0x7A);
                                            }
                                            Registers::RegE => {
                                                instructions.push(0x7B);
                                            }
                                            Registers::RegH => {
                                                instructions.push(0x7C);
                                            }
                                            Registers::RegL => {
                                                instructions.push(0x7D);
                                            }
                                        }
                                    }
                                }
                                Registers::RegB => {
                                    if let Some(Ok(Token::Register)) = lexer.next() {
                                        let register = Registers::from(lexer.slice());
                                        match register {
                                            Registers::RegA => {
                                                instructions.push(0x47);
                                            }
                                            Registers::RegB => {
                                                instructions.push(0x40);
                                            }
                                            Registers::RegC => {
                                                instructions.push(0x41);
                                            }
                                            Registers::RegD => {
                                                instructions.push(0x42);
                                            }
                                            Registers::RegE => {
                                                instructions.push(0x43);
                                            }
                                            Registers::RegH => {
                                                instructions.push(0x44);
                                            }
                                            Registers::RegL => {
                                                instructions.push(0x45);
                                            }
                                        }
                                    }
                                }
                                Registers::RegC => {
                                    if let Some(Ok(Token::Register)) = lexer.next() {
                                        let register = Registers::from(lexer.slice());
                                        match register {
                                            Registers::RegA => {
                                                instructions.push(0x4F);
                                            }
                                            Registers::RegB => {
                                                instructions.push(0x48);
                                            }
                                            Registers::RegC => {
                                                instructions.push(0x49);
                                            }
                                            Registers::RegD => {
                                                instructions.push(0x4A);
                                            }
                                            Registers::RegE => {
                                                instructions.push(0x4B);
                                            }
                                            Registers::RegH => {
                                                instructions.push(0x4C);
                                            }
                                            Registers::RegL => {
                                                instructions.push(0x4D);
                                            }
                                        }
                                    }
                                }
                                Registers::RegD => {
                                    if let Some(Ok(Token::Register)) = lexer.next() {
                                        let register = Registers::from(lexer.slice());
                                        match register {
                                            Registers::RegA => {
                                                instructions.push(0x57);
                                            }
                                            Registers::RegB => {
                                                instructions.push(0x50);
                                            }
                                            Registers::RegC => {
                                                instructions.push(0x51);
                                            }
                                            Registers::RegD => {
                                                instructions.push(0x52);
                                            }
                                            Registers::RegE => {
                                                instructions.push(0x53);
                                            }
                                            Registers::RegH => {
                                                instructions.push(0x54);
                                            }
                                            Registers::RegL => {
                                                instructions.push(0x55);
                                            }
                                        }
                                    }
                                }
                                Registers::RegE => {
                                    if let Some(Ok(Token::Register)) = lexer.next() {
                                        let register = Registers::from(lexer.slice());
                                        match register {
                                            Registers::RegA => {
                                                instructions.push(0x5F);
                                            }
                                            Registers::RegB => {
                                                instructions.push(0x58);
                                            }
                                            Registers::RegC => {
                                                instructions.push(0x59);
                                            }
                                            Registers::RegD => {
                                                instructions.push(0x5A);
                                            }
                                            Registers::RegE => {
                                                instructions.push(0x5B);
                                            }
                                            Registers::RegH => {
                                                instructions.push(0x5C);
                                            }
                                            Registers::RegL => {
                                                instructions.push(0x5D);
                                            }
                                        }
                                    }
                                }
                                Registers::RegH => {
                                    if let Some(Ok(Token::Register)) = lexer.next() {
                                        let register = Registers::from(lexer.slice());
                                        match register {
                                            Registers::RegA => {
                                                instructions.push(0x67);
                                            }
                                            Registers::RegB => {
                                                instructions.push(0x60);
                                            }
                                            Registers::RegC => {
                                                instructions.push(0x61);
                                            }
                                            Registers::RegD => {
                                                instructions.push(0x62);
                                            }
                                            Registers::RegE => {
                                                instructions.push(0x63);
                                            }
                                            Registers::RegH => {
                                                instructions.push(0x64);
                                            }
                                            Registers::RegL => {
                                                instructions.push(0x65);
                                            }
                                        }
                                    }
                                }
                                Registers::RegL => {
                                    if let Some(Ok(Token::Register)) = lexer.next() {
                                        let register = Registers::from(lexer.slice());
                                        match register {
                                            Registers::RegA => {
                                                instructions.push(0x6F);
                                            }
                                            Registers::RegB => {
                                                instructions.push(0x68);
                                            }
                                            Registers::RegC => {
                                                instructions.push(0x69);
                                            }
                                            Registers::RegD => {
                                                instructions.push(0x6A);
                                            }
                                            Registers::RegE => {
                                                instructions.push(0x6B);
                                            }
                                            Registers::RegH => {
                                                instructions.push(0x6C);
                                            }
                                            Registers::RegL => {
                                                instructions.push(0x6D);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    "ADD" => {
                        if let Some(Ok(Token::Register)) = lexer.next() {
                            let register = Registers::from(lexer.slice());
                            match register {
                                Registers::RegA => {
                                    instructions.push(0x87);
                                }
                                Registers::RegB => {
                                    instructions.push(0x80);
                                }
                                Registers::RegC => {
                                    instructions.push(0x81);
                                }
                                Registers::RegD => {
                                    instructions.push(0x82);
                                }
                                Registers::RegE => {
                                    instructions.push(0x83);
                                }
                                Registers::RegH => {
                                    instructions.push(0x84);
                                }
                                Registers::RegL => {
                                    instructions.push(0x85);
                                }
                            }
                        }
                    }
                    "CMP" => {
                        instructions.push(0xBF);
                    }
                    "HLT" => {
                        instructions.push(0x76);
                    }
                    "STA" => {
                        instructions.push(0x32);
                        if let Some(Ok(Token::Address(address))) = lexer.next() {
                            let high_byte = ((address & 0xFF00) >> 8) as u8;
                            let low_byte = (address & 0x00FF) as u8;
                            instructions.push(low_byte);
                            instructions.push(high_byte);
                        }
                    }
                    "STAX" => {
                        if let Some(Ok(Token::Register)) = lexer.next() {
                            let register = Registers::from(lexer.slice());
                            match register {
                                Registers::RegB => {
                                    instructions.push(0x02);
                                }
                                Registers::RegD => {
                                    instructions.push(0x12);
                                }
                                _ => {
                                    unimplemented!(
                                        "{}",
                                        format!("Invalid opcode: {}", lexer.slice())
                                    );
                                }
                            }
                        }
                    }
                    "LHLD" => {
                        if let Some(Ok(Token::Address(address))) = lexer.next() {
                            instructions.push(0x2A);
                            let high_byte = ((address & 0xFF00) >> 8) as u8;
                            let low_byte = (address & 0x00FF) as u8;
                            instructions.push(low_byte);
                            instructions.push(high_byte);
                        }
                    }
                    "XCHG" => {
                        instructions.push(0xEB);
                    }
                    "ADC" => {
                        if let Some(Ok(Token::Register)) = lexer.next() {
                            let register = Registers::from(lexer.slice());
                            match register {
                                Registers::RegA => {
                                    instructions.push(0xC6);
                                }
                                Registers::RegB => {
                                    instructions.push(0x8F);
                                }
                                Registers::RegC => {
                                    instructions.push(0x88);
                                }
                                Registers::RegD => {
                                    instructions.push(0x89);
                                }
                                Registers::RegE => {
                                    instructions.push(0x8B);
                                }
                                Registers::RegH => {
                                    instructions.push(0x8C);
                                }
                                Registers::RegL => {
                                    instructions.push(0x8D);
                                }
                            }
                        }
                    }
                    "SHLD" => {
                        if let Some(Ok(Token::Address(address))) = lexer.next() {
                            instructions.push(0x22);
                            let high_byte = ((address & 0xFF00) >> 8) as u8;
                            let low_byte = (address & 0x00FF) as u8;
                            instructions.push(low_byte);
                            instructions.push(high_byte);
                        }
                    }
                    "DAD" => {
                        if let Some(Ok(Token::Register)) = lexer.next() {
                            let register = Registers::from(lexer.slice());
                            match register {
                                Registers::RegB => {
                                    instructions.push(0x09);
                                }
                                Registers::RegD => {
                                    instructions.push(0x19);
                                }
                                Registers::RegH => {
                                    instructions.push(0x29);
                                }
                                _ => {
                                    unimplemented!(
                                        "{}",
                                        format!("Invalid opcode: {}", lexer.slice())
                                    );
                                }
                            }
                        }
                    }

                    "LDA" => {
                        instructions.push(0x3A);
                        if let Some(Ok(Token::Address(address))) = lexer.next() {
                            let high_byte = ((address & 0xFF00) >> 8) as u8;
                            let low_byte = (address & 0x00FF) as u8;
                            instructions.push(low_byte);
                            instructions.push(high_byte);
                        }
                    }
                    "CMA" => {
                        instructions.push(0x2F);
                    }
                    _ => {
                        unimplemented!("{}", format!("Invalid opcode: {}", lexer.slice()));
                    }
                },
                _ => {}
            },
            Err(_) => {}
        }
    }
    instructions
}

#[cfg(test)]
mod tests {
    macro_rules! test_opcode {
        ($name:ident, $source:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let instructions = super::parse_instructions($source);
                assert_eq!(instructions, $expected);
            }
        };
        () => {};
    }

    test_opcode!(test_mvi_a, "MVI A, 15", vec![0x3E, 0x15]);

    test_opcode!(test_mvi_b, "MVI B, 15", vec![0x06, 0x15]);

    test_opcode!(test_mov, "MOV A,B", vec![0x78]);

    test_opcode!(test_lhld, "LHLD 2501", vec![0x2A, 0x01, 0x25]);

    test_opcode!(test_add, "ADD B", vec![0x80]);

    test_opcode!(test_shld, "SHLD 2500", vec![0x22, 0x00, 0x25]);

    test_opcode!(test_xchg, "XCHG", vec![0xEB]);

    test_opcode!(test_adc_a, "ADC A", vec![0xC6]);
    test_opcode!(test_adc_b, "ADC B", vec![0x8F]);

    test_opcode!(test_adc_c, "ADC C", vec![0x88]);

    test_opcode!(test_adc_d, "ADC D", vec![0x89]);

    test_opcode!(test_adc_e, "ADC E", vec![0x8B]);

    test_opcode!(test_adc_h, "ADC H", vec![0x8C]);

    test_opcode!(test_adc_l, "ADC L", vec![0x8D]);
    test_opcode!(test_dad_b, "DAD B", vec![0x09]);
}
