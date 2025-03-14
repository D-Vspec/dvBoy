use crate::hardware::cpu::CPU;

pub fn execute_opcode(cpu: &mut CPU, opcode: u8) -> u8 {
    match opcode {
        0x00 => nop(cpu),
        0x01 => ld_bc_d16(cpu),
        0x02 => ld_bc_a(cpu),
        _ => {
            panic!("unimplicated opcode : {:02x}", opcode);
        }
    }
}

fn nop(_cpu: &mut CPU) -> u8 {
    4
}

fn ld_bc_d16(cpu: &mut CPU) -> u8 {
    let lower_byte = cpu.fetch_byte_increment();
    let upper_byte = cpu.fetch_byte_increment();
    let value = ((upper_byte as u16) << 8) | (lower_byte as u16);
    cpu.registers.set_bc(value);
    3
}

fn ld_bc_a(cpu: &mut CPU) -> u8 {
    let locn = cpu.registers.get_bc();
    let value = cpu.fetch_byte_increment();
    cpu.memory.write_byte(locn, value);
    2
}
