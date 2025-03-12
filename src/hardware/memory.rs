#[derive(Debug)]
pub struct Memory {
    pub ram: [u8; 65536],
}

impl Memory {
    pub fn new() -> Self {
        self { ram: [0; 65536] }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.ram[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.ram[address as usize] = value;
    }
}
