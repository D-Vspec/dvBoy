mod hardware;
use hardware::cpu::CPU;

fn main() {
    let mut cpu = CPU::new();

    cpu.memory.write_byte(0x100, 0x01);
    cpu.memory.write_byte(0x101, 0x34);
    cpu.memory.write_byte(0x102, 0x12);

    cpu.run();
    println!("{}", cpu.registers.get_bc());
    println!("{:?}", cpu.registers);
}
