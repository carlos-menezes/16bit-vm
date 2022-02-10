use crate::{cpu::CPU, memory::Memory, vm::VM};

mod cpu;
mod memory;
mod vm;

fn main() {
    let mut cpu = CPU::new();
    let mut memory = Memory::new();
    let mut vm = VM::new(&mut cpu, &mut memory);
    vm.run();
}
