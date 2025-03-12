#[derive(Debug, Default)]
pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    pc: u16,
    sp: u16,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            pc: 0x100,
            sp: 0xFFE,
            ..Default::default()
        }
    }
}
