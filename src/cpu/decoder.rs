use crate::cpu::addressing_modes::AddressingMode;
use crate::cpu::instructions::Instruction;

pub fn Decode(opcode: u8) -> Instruction {
    match opcode {
        0x00 => Instruction::BRK(AddressingMode::Implied),
        0x18 => Instruction::CLC(AddressingMode::Implied),
        0xD8 => Instruction::CLD(AddressingMode::Implied),
        0x58 => Instruction::CLI(AddressingMode::Implied),
        0xB8 => Instruction::CLV(AddressingMode::Implied),
        0xCA => Instruction::DEX(AddressingMode::Implied),
        0x88 => Instruction::DEY(AddressingMode::Implied),
        0xE8 => Instruction::INY(AddressingMode::Implied),
        0xEA => Instruction::NOP(AddressingMode::Implied),
        0x48 => Instruction::PHA(AddressingMode::Implied),
        0x08 => Instruction::PHP(AddressingMode::Implied),
        0x68 => Instruction::PLA(AddressingMode::Implied),
        _ => todo!(),
    }
}
