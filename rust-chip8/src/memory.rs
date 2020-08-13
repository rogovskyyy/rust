pub struct Memory {
    mem: Box<[u8; 4095]>
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            mem: Box::new([0x0; 4095])
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.mem[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.mem[address as usize] = value;
    }
}