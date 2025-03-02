use crate::circuitry::interface::CircuitryInterface;
use crate::cpu::registers::{CPURegisters, CpuRegistersAccessTrait};

mod registers;

#[derive(Debug, Default, PartialEq)]
pub struct CPU {
    registers: CPURegisters,
}

impl CPU {
    pub fn step(&mut self, c: &mut impl CircuitryInterface) {

    }
}

impl CpuRegistersAccessTrait for CPU {
    fn get_registers(&self) -> &CPURegisters {
        &self.registers
    }

    fn get_registers_mut(&mut self) -> &mut CPURegisters {
        &mut self.registers
    }
}