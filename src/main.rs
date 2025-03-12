mod hardware;

use hardware::cpu::CPU;

fn main() {
    let mut cpu = CPU::new();
    println!("{:?}", cpu.registers);
    cpu.reset();
    println!("{:?}", cpu);
}
