use crate::hardware::memory::Memory;
use crate::hardware::registers::Registers;

#[derive(Debug)]
pub struct CPU {
    pub registers: Registers,
    pub memory: Memory,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            registers: Registers::new(),
        }
    }

    pub fn reset(&mut self) {
        self.registers = Registers::new();
    }
}
