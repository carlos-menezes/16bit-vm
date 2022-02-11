use crate::{
    cpu::{opcodes::Opcodes, CPU},
    memory::Memory,
};

pub struct VM<'c, 'm> {
    cpu: &'c mut CPU,
    memory: &'m mut Memory,
}

impl<'c, 'm> VM<'c, 'm> {
    pub fn new(cpu: &'c mut CPU, memory: &'m mut Memory) -> Self {
        Self { cpu, memory }
    }

    pub fn run(&mut self) {
        // Fetch
        let instruction = self.memory.read(self.cpu.pc);
        self.cpu.pc += 1;
        let opcode: Opcodes = (instruction >> 12).into();

        match opcode {
            Opcodes::BR => self.cpu.br(instruction),
            Opcodes::ADD => self.cpu.add(instruction),
            Opcodes::LD => todo!(),
            Opcodes::ST => todo!(),
            Opcodes::JSR => todo!(),
            Opcodes::AND => self.cpu.and(instruction),
            Opcodes::LDR => todo!(),
            Opcodes::STR => todo!(),
            Opcodes::RTI => todo!(),
            Opcodes::NOT => todo!(),
            Opcodes::LDI => todo!(),
            Opcodes::STI => todo!(),
            Opcodes::JMP => todo!(),
            Opcodes::RES => todo!(),
            Opcodes::LEA => todo!(),
            Opcodes::TRAP => todo!(),
        }
    }
}
