use crate::circuitry::interface::CircuitryInterface;

pub mod interface;

#[derive(Debug, Default, PartialEq)]
pub struct Circuitry {

}

impl CircuitryInterface for Circuitry {
    fn tick(&self) {
        todo!()
    }
}