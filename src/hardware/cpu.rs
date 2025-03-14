use crate::hardware::instructions;
use crate::hardware::memory::Memory;
use crate::hardware::registers::Registers;
use std::thread::yield_now;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct CPU {
    pub registers: Registers,
    pub memory: Memory,
}

const GAMEBOY_CLOCK_HZ: u64 = 4_194_304;
const CYCLES_PER_FRAME: u64 = 70224;

impl CPU {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            registers: Registers::new(),
        }
    }

    fn precise_delay(time: Duration) {
        let start = Instant::now();
        while start.elapsed() < time {
            yield_now();
        }
    }

    pub fn run(&mut self) {
        let mut start_time = Instant::now();
        let mut counter = 0;
        loop {
            let mut cycles_this_frame = 0;

            println!("{}", self.registers.get_pc_address());

            while cycles_this_frame < CYCLES_PER_FRAME {
                let opcode = self.fetch_curr_byte();
                let cycles = instructions::execute_opcode(self, opcode);

                println!("{}", opcode);
                println!("{}", cycles);

                cycles_this_frame += cycles as u64;
            }

            let elapsed = start_time.elapsed();
            let frame_duration = Duration::from_secs_f64(1.0 / 60.0);

            if elapsed < frame_duration {
                Self::precise_delay(frame_duration - elapsed);
            }

            start_time = Instant::now();
            break;
        }
    }

    pub fn fetch_byte_increment(&mut self) -> u8 {
        self.memory.read_byte(self.registers.increment_pc())
    }

    pub fn fetch_curr_byte(&self) -> u8 {
        self.memory.read_byte(self.registers.get_pc_address())
    }

    pub fn reset(&mut self) {
        self.registers = Registers::new();
        self.memory = Memory::new();
    }
}
