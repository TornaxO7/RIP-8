use std::convert::From;
use super::chip8::Chip8Instruction;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Register {
    RAX,
    RBX,
    RCX,
    RDX,
    RSI,
    RDI,
    RSP,
    RBP,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum x86Instruction {
    RET,
    MOVE_64,
    CALL(u32),
}

impl x86Instruction {
    pub fn encode(&self) -> Vec<u8> {
        match self {
            _ => todo!(),
        }
    }

    pub fn from_chip8(chip8_instruction: &Chip8Instruction) -> Vec<Self> {
        match chip8_instruction {
            Chip8Instruction::CLS => todo!(),
            Chip8Instruction::RET => todo!(),
            Chip8Instruction::SYS | Chip8Instruction::JP1 => todo!(),
            Chip8Instruction::CALL(addr) => vec![x86Instruction::CALL(addr)],
            Chip8Instruction::SE1 => vec![],
            Chip8Instruction::SNE1 => todo!(),
            Chip8Instruction::SE2 => todo!(),
            Chip8Instruction::LD1 => todo!(),
            Chip8Instruction::ADD1 => todo!(),
            Chip8Instruction::LD2 => todo!(),
            Chip8Instruction::OR => todo!(),
            Chip8Instruction::AND => todo!(),
            Chip8Instruction::XOR => todo!(),
            Chip8Instruction::ADD2 => todo!(),
            Chip8Instruction::SUB => todo!(),
            Chip8Instruction::SHR => todo!(),
            Chip8Instruction::SUBN => todo!(),
            Chip8Instruction::SHL => todo!(),
            Chip8Instruction::SNE2 => todo!(),
            Chip8Instruction::LD3 => todo!(),
            Chip8Instruction::JP2 => todo!(),
            Chip8Instruction::RND => todo!(),
            Chip8Instruction::DRW => todo!(),
            Chip8Instruction::SKP => todo!(),
            Chip8Instruction::SKNP => todo!(),
            Chip8Instruction::LD4 => todo!(),
            Chip8Instruction::LD5 => todo!(),
            Chip8Instruction::LD6 => todo!(),
            Chip8Instruction::LD7 => todo!(),
            Chip8Instruction::ADD3 => todo!(),
            Chip8Instruction::LD8 => todo!(),
            Chip8Instruction::LD9 => todo!(),
            Chip8Instruction::LD10 => todo!(),
            Chip8Instruction::LD11 => todo!(),
        }
    }
}
