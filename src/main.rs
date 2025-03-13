mod hardware;

use hardware::cpu::CPU;

fn main() {
    let mut cpu = CPU::new();
    println!("{:?}", cpu.registers);
    cpu.reset();
    cpu.memory.write_byte(0, 25);
    println!("{:?}", cpu);
}
