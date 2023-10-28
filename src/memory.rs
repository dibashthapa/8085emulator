const MEMORY_SIZE: usize = 0xFFFF;

#[derive(Clone, Debug)]
pub struct Memory {
    pub addresses: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            addresses: [0; MEMORY_SIZE],
        }
    }

    pub fn write(&mut self, addr: usize, val: u8) {
        if addr < MEMORY_SIZE - 1 {
            self.addresses[addr] = val;
        }
    }

    pub fn write_word(&mut self, addr: usize, val: u16) {
        if addr < MEMORY_SIZE - 1 {
            self.addresses[addr] = (val >> 8) as u8;
            self.addresses[addr + 1] = val as u8;
        }
    }

    pub fn read(&self, address: usize) -> u8 {
        self.addresses[address]
    }

    pub fn print(&self) {
        for (i, address) in self.addresses.iter().enumerate() {
            if *address != 0 {
                println!("----------------");
                print!("| ");
                println!("{:#06x}: {:#04x} |", i, address);
            }
        }
        println!("----------------");
    }
}
