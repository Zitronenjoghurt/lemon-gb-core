use crate::circuitry::Circuitry;
use crate::cpu::CPU;

#[derive(Debug, Default, PartialEq)]
pub struct GameBoy {
    cpu: CPU,
    circuitry: Circuitry
}

impl GameBoy {
    pub fn step(&mut self) {
        self.cpu.step(&mut self.circuitry)
    }
}