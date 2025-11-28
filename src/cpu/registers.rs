#[allow(non_snake_case)]
pub struct Registers {
    program_counter: u16,
    stack_pointer: u8,
    /* Accumulator Register */
    A: u8,
    /* Index Register X */
    X: u8,
    /* Index Register Y */
    Y: u8,
    /* Flag (Processor Status) Register */
    F: u8,
}
impl Registers {
    pub fn new() -> Self {
        Registers {
            program_counter: 0,
            stack_pointer: 0,
            A: 0,
            X: 0,
            Y: 0,
            F: 0,
        }
    }
}
