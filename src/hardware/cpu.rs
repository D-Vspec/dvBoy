use crate::hardware::instructions;
use crate::hardware::memory::Memory;
use crate::hardware::registers::Registers;
use std::thread::yield_now;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct CPU {
    pub registers: Registers,
    pub memory: Memory,
    pub clock_cycles: u32, //change to 64 if not refreshing
}

const GAMEBOY_CLOCK_HZ: u64 = 4_194_304;
const CYCLES_PER_FRAME: u64 = 70224;

impl CPU {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            registers: Registers::new(),
            clock_cycles: 0,
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

        loop {
            let mut cycles_this_frame = 0;

            while cycles_this_frame < CYCLES_PER_FRAME {
                let opcode = 0x00;
                let cycles = instructions::execute_opcode(self, opcode);

                cycles_this_frame += cycles as u64;
            }

            let elapsed = start_time.elapsed();
            let frame_duration = Duration::from_secs_f64(1.0 / 60.0);

            if elapsed < frame_duration {
                Self::precise_delay(frame_duration - elapsed);
            }

            start_time = Instant::now();
        }
    }

    pub fn reset(&mut self) {
        self.registers = Registers::new();
        self.memory = Memory::new();
        self.clock_cycles = 0;
    }
}
