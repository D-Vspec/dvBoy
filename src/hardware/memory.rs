#[derive(Debug)]
pub struct Memory {
    pub memory_map: [u8; 65536],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            memory_map: [0; 65536],
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory_map[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.memory_map[address as usize] = value;
    }
}
