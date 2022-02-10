pub mod opcodes;

enum CondFlags {
    Pos = 1 << 0,
    Zro = 1 << 1,
    Neg = 1 << 2,
}

pub struct CPU {
    pub gpr: [u16; 8], // general purpose registers
    pub pc: u16,       // program counter register
    pub cond: u16,     // condition flag register
}

impl CPU {
    pub fn new() -> Self {
        Self {
            gpr: [0u16; 8],
            pc: 0x3000,
            cond: 0,
        }
    }
}
