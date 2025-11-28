use crate::cpu::registers::Registers;

pub struct CPU {
    registers: Registers,
}
impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: Registers::new(),
        }
    }
}
