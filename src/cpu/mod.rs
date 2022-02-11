pub mod opcodes;

#[repr(u16)]
#[derive(Copy)]
pub enum CondFlags {
    Uninitialized = 0,
    Pos = (1 << 0),
    Zro = 1 << 1,
    Neg = 1 << 2,
}

pub struct CPU {
    pub gpr: [u16; 8],   // general purpose registers
    pub pc: u16,         // program counter register
    pub cond: CondFlags, // condition flag register
}

impl CPU {
    pub fn new() -> Self {
        Self {
            gpr: [0u16; 8],
            pc: 0x3000,
            cond: CondFlags::Uninitialized,
        }
    }

    pub fn set_gpr(&mut self, register: u16, value: u16) {
        self.gpr[register as usize] = value
    }

    pub fn get_gpr(&self, register: u16) -> u16 {
        self.gpr[register as usize]
    }

    pub fn sext(mut x: u16, bits: i32) -> u16 {
        if ((x >> (bits - 1)) & 1) == 1 {
            // number is negative
            x |= 0xFFFF << bits;
        }

        x
    }

    pub fn set_cc(&mut self, register: u16) {
        if self.get_gpr(register) == 0 {
            self.cond = CondFlags::Zro;
        } else if (self.get_gpr(register) >> 15) == 1 {
            // Left-most digit is 1,indicating a negative number in 2's Complement
            self.cond = CondFlags::Neg
        } else {
            self.cond = CondFlags::Pos
        }
    }

    pub fn add(&mut self, instruction: u16) {
        let dr = (instruction >> 0x9) & 0x7;
        let sr1 = (instruction >> 0x6) & 0x7;
        let immediate_mode = ((instruction >> 5) & 0x1) == 1;
        if immediate_mode {
            // instruction[5] is 1
            let imm5 = Self::sext(instruction & 0x1F, 5);
            self.set_gpr(dr, self.get_gpr(sr1) + imm5)
        } else {
            let sr2 = instruction >> 0x7;
            self.set_gpr(dr, self.get_gpr(sr1) + self.get_gpr(sr2))
        }
        self.set_cc(self.get_gpr(dr));
    }

    pub fn and(&mut self, instruction: u16) {
        let dr = (instruction >> 0x9) & 0x7;
        let sr1 = (instruction >> 0x6) & 0x7;
        let immediate_mode = ((instruction >> 5) & 0x1) == 1;
        if immediate_mode {
            // instruction[5] is 1
            let imm5 = Self::sext(instruction & 0x1F, 5);
            self.set_gpr(dr, self.get_gpr(sr1) & imm5)
        } else {
            let sr2 = instruction >> 0x7;
            self.set_gpr(dr, self.get_gpr(sr1) & self.get_gpr(sr2))
        }
        self.set_cc(self.get_gpr(dr));
    }

    pub fn br(&mut self, instruction: u16) {
        let cond_flag = (instruction >> 0x09) & 0x7;
        let pc_offset = Self::sext(instruction >> 0x1FF, 9);
        if (self.cond & cond_flag) == 1 {
            self.pc += pc_offset;
        }
    }
}
