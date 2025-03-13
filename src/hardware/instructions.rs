use crate::CPU;

pub fn execute_opcode(cpu: &mut CPU, opcode: u8) -> u8 {
    match opcode {
        0x00 => nop(cpu),

        _ => {
            panic!("Unimplicated opcode : {:02X}", opcode);
        }
    }
}

fn nop(cpu: &mut CPU) -> u8 {
    4
}
