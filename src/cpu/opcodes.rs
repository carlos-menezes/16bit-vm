#[repr(u16)]
pub enum Opcodes {
    BR,
    ADD,
    LD,
    ST,
    JSR,
    AND,
    LDR,
    STR,
    RTI,
    NOT,
    LDI,
    STI,
    JMP,
    RES,
    LEA,
    TRAP,
}

impl From<u16> for Opcodes {
    fn from(val: u16) -> Self {
        match val {
            0x0001 => Self::ADD,
            0x0101 => Self::AND,
            0x0000 => Self::BR,
            0x1100 => Self::JMP,
            0x0100 => Self::JSR,
            0x0010 => Self::LD,
            0x1010 => Self::LDI,
            0x0110 => Self::LDR,
            0x1110 => Self::LEA,
            0x1001 => Self::NOT,
            0x1000 => Self::RTI,
            0x0011 => Self::ST,
            0x1011 => Self::STI,
            0x0111 => Self::STR,
            0x1111 => Self::TRAP,
            0x1101 => Self::RES,
            _ => panic!("invalid opcode"),
        }
    }
}
