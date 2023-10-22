const MEMORY_SIZE: usize = 0x10000;

#[derive(Clone)]
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
        if addr < MEMORY_SIZE {
            self.addresses[addr] = val;
        }
    }

    pub fn read(&self, address: usize) -> u8 {
        self.addresses[address]
    }
}
