use super::addressing_modes::AddressingMode;

#[allow(non_snake_case)]
pub enum Instruction {
    /* Load/Store Operations */
    LDA(AddressingMode),
    LDX(AddressingMode),
    LDY(AddressingMode),
    STA(AddressingMode),
    STX(AddressingMode),
    STY(AddressingMode),
    /* Register Transfer */
    TAX(AddressingMode),
    TAY(AddressingMode),
    TXA(AddressingMode),
    TYA(AddressingMode),
    /* Stack Operations */
    TSX(AddressingMode),
    TXS(AddressingMode),
    PHA(AddressingMode),
    PHP(AddressingMode),
    PLA(AddressingMode),
    PLP(AddressingMode),
    /* Logical */
    AND(AddressingMode),
    EOR(AddressingMode),
    ORA(AddressingMode),
    BIT(AddressingMode),
    /* Arithmetic */
    ADC(AddressingMode),
    SBC(AddressingMode),
    CMP(AddressingMode),
    CPX(AddressingMode),
    CPY(AddressingMode),
    /* Increments & Decrements */
    INC(AddressingMode),
    INX(AddressingMode),
    INY(AddressingMode),
    DEC(AddressingMode),
    DEX(AddressingMode),
    DEY(AddressingMode),
    /* Shifts */
    ASL(AddressingMode),
    LSR(AddressingMode),
    ROL(AddressingMode),
    ROR(AddressingMode),
    /* Jumps and Calls */
    JMP(AddressingMode),
    JSR(AddressingMode),
    RTS(AddressingMode),
    /* Branches */
    BCC(AddressingMode),
    BCS(AddressingMode),
    BEQ(AddressingMode),
    BMI(AddressingMode),
    BNE(AddressingMode),
    BPL(AddressingMode),
    BVC(AddressingMode),
    BVS(AddressingMode),
    /* Processor Status Flag Changes */
    CLC(AddressingMode),
    CLD(AddressingMode),
    CLI(AddressingMode),
    CLV(AddressingMode),
    SEC(AddressingMode),
    SED(AddressingMode),
    SEI(AddressingMode),
    /* System Functions */
    BRK(AddressingMode),
    NOP(AddressingMode),
    RTI(AddressingMode),
}
