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
            Opcodes::LD => self.cpu.ld(instruction),
            Opcodes::ST => self.cpu.st(instruction, self.memory),
            Opcodes::JSR => self.cpu.jsr(instruction),
            Opcodes::AND => self.cpu.and(instruction),
            Opcodes::LDR => self.cpu.ldr(instruction),
            Opcodes::STR => self.cpu.str(instruction, self.memory),
            Opcodes::NOT => self.cpu.not(instruction),
            Opcodes::LDI => self.cpu.ldi(instruction),
            Opcodes::STI => self.cpu.sti(instruction, self.memory),
            Opcodes::JMP => self.cpu.jmp(instruction),
            Opcodes::LEA => self.cpu.lea(instruction),
            Opcodes::TRAP => todo!(),
        }
    }
}
