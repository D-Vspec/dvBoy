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

    pub fn get_af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16)
    }

    pub fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    pub fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    pub fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = (value & 0xF0) as u8;
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }

    const Z_FLAG: u8 = 0b1000_0000;
    const N_FLAG: u8 = 0b0100_0000;
    const H_FLAG: u8 = 0b0010_0000;
    const C_FLAG: u8 = 0b0001_0000;

    pub fn get_z(&self) -> bool {
        self.f & Self::Z_FLAG != 0
    }

    pub fn get_n(&self) -> bool {
        self.f & Self::N_FLAG != 0
    }

    pub fn get_h(&self) -> bool {
        self.f & Self::H_FLAG != 0
    }

    pub fn get_c(&self) -> bool {
        self.f & Self::C_FLAG != 0
    }

    pub fn set_z(&mut self, value: bool) {
        if value {
            self.f |= Self::Z_FLAG;
        } else {
            self.f &= !Self::Z_FLAG;
        }
    }

    pub fn set_n(&mut self, value: bool) {
        if value {
            self.f |= Self::N_FLAG;
        } else {
            self.f &= !Self::N_FLAG;
        }
    }

    pub fn set_h(&mut self, value: bool) {
        if value {
            self.f |= Self::H_FLAG;
        } else {
            self.f &= !Self::H_FLAG;
        }
    }

    pub fn set_c(&mut self, value: bool) {
        if value {
            self.f |= Self::C_FLAG;
        } else {
            self.f &= !Self::C_FLAG;
        }
    }
}
