use std::convert::TryFrom;
use super::Chip8Block;
use super::Addr;

pub type Opcode = u16;
pub type Vx = u8;
pub type Vy = u8;

pub fn parse_until_next_branch(binary_content: &Vec<u8>, addr: &mut Addr) -> Chip8Block {
    let mut block = Chip8Block::new();

    let instruction = get_next_instruction(binary_content, addr);
    while !instruction.is_branch() {
        block.push(instruction);
    }
    block.push(instruction);
    block
}

fn get_next_instruction(binary_content: &Vec<u8>, addr: &mut Addr) -> Chip8Instruction {
    Chip8Instruction::from(get_next_opcode(binary_content, addr))
}

fn get_next_opcode(binary_content: &Vec<u8>, addr: &mut Addr) -> Opcode {
    let mut opcode: Opcode = 0;

    if let Some(&byte) = binary_content.get(*addr) {
        opcode = u16::from(byte);
        (*addr) += 1;

        if let Some(&byte2) = binary_content.get(*addr) {
            opcode  |= (u16::from(byte2) << 8);
            (*addr) += 1;
        }
    }

    opcode
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Chip8Instruction {
    CLS,
    RET,
    SYS(Addr),
    JP1(Addr),
    CALL(Addr),
    SE1(Vx, u8),
    SNE1(Vx, u8),
    SE2(Vx, Vy),
    LD1(Vx, u8),
    ADD1(Vx, u8),
    LD2(Vx, Vy),
    OR(Vx, Vy),
    AND(Vx, Vy),
    XOR(Vx, Vy),
    ADD2(Vx, Vy),
    SUB(Vx, Vy),
    SHR(Vx),
    SUBN(Vx, Vy),
    SHL(Vx),
    SNE2(Vx, Vy),
    LD3(Addr),
    JP2(Addr),
    RND(Vx, u8),
    DRW(Vx, Vy, u8),
    SKP(Vx),
    SKNP(Vx),
    LD4(Vx),
    LD5(Vx),
    LD6(Vx),
    LD7(Vx),
    ADD3(Vx),
    LD8(Vx),
    LD9(Vx),
    LD10(Vx),
    LD11(Vx),
}

impl From<Opcode> for Chip8Instruction {
    fn from(opcode: Opcode) -> Self {
        let first_hex = opcode >> 12;
        let subhex = opcode & 0xfff;

        match first_hex {
            0x0 => handle_0(subhex),
            0x1 => Self::JP1(subhex.into()),
            0x2 => Self::CALL(subhex.into()),
            0x3 => Self::SE1(get_x(opcode), get_kk_or_byte(opcode)),
            0x4 => Self::SNE1(get_x(opcode), get_kk_or_byte(opcode)),
            0x5 => Self::SE2(get_x(opcode), get_y(opcode)),
            0x6 => Self::LD1(get_x(opcode), get_kk_or_byte(opcode)),
            0x7 => Self::ADD1(get_x(opcode), get_kk_or_byte(opcode)),
            0x8 => handle_8(opcode),
            0x9 => Self::SNE2(get_x(opcode), get_y(opcode)),
            0xA => Self::LD3(get_nnn_or_addr(opcode).into()),
            0xB => Self::JP2(get_nnn_or_addr(opcode).into()),
            0xC => Self::RND(get_x(opcode), get_kk_or_byte(opcode)),
            0xD => Self::DRW(get_x(opcode), get_y(opcode), get_n_or_nibble(opcode)),
            0xE => handle_e(opcode),
            0xF => handle_f(opcode),
            _ => unreachable!(),

        }
    }
}

impl Chip8Instruction {
    pub fn is_branch(&self) -> bool {
        match self {
            Self::RET | Self::SYS(_) | Self::JP1(_) | Self::CALL(_) | Self::JP2(_) => true,
            _ => false,
        }
    }
}

fn handle_0(sub: u16) -> Chip8Instruction {
    if sub == 0x0e0 {
        Chip8Instruction::CLS
    } else if sub == 0xee {
        Chip8Instruction::RET
    } else {
        Chip8Instruction::SYS(sub.into())
    }
}

fn handle_8(opcode: Opcode) -> Chip8Instruction {
    let last_hex = opcode & 0xf;

    let x = get_x(opcode);
    let y = get_y(opcode);

    match last_hex {
        0x0 => Chip8Instruction::LD2(x, y),
        0x1 => Chip8Instruction::OR(x, y),
        0x2 => Chip8Instruction::AND(x, y),
        0x3 => Chip8Instruction::XOR(x, y),
        0x4 => Chip8Instruction::ADD2(x, y),
        0x5 => Chip8Instruction::SUB(x, y),
        0x6 => Chip8Instruction::SHR(x),
        0x7 => Chip8Instruction::SUBN(x, y),
        0xe => Chip8Instruction::SHL(x),
        _ => unreachable!(),
    }
}

fn handle_e(opcode: Opcode) -> Chip8Instruction {
    let first_byte = opcode & 0xff;

    match first_byte {
        0x9E => Chip8Instruction::SKP(get_x(opcode)),
        0xA1 => Chip8Instruction::SKNP(get_x(opcode)),
        _ => unreachable!(),
    }
}

fn handle_f(opcode: Opcode) -> Chip8Instruction {
    let first_byte = opcode & 0xff;
    let x = get_x(opcode);

    match first_byte {
        0x07 => Chip8Instruction::LD4(x),
        0x0A => Chip8Instruction::LD5(x),
        0x15 => Chip8Instruction::LD6(x),
        0x18 => Chip8Instruction::LD7(x),
        0x1E => Chip8Instruction::ADD3(x),
        0x29 => Chip8Instruction::LD8(x),
        0x33 => Chip8Instruction::LD9(x),
        0x55 => Chip8Instruction::LD10(x),
        0x65 => Chip8Instruction::LD11(x),
        _ => unreachable!(),
    }
}

fn get_nnn_or_addr(opcode: Opcode) -> u16 {
    opcode & 0xfff
}

fn get_n_or_nibble(opcode: Opcode) -> u8 {
    u8::try_from(opcode & 0b1111).unwrap()
}

fn get_x(opcode: Opcode) -> u8 {
    u8::try_from((opcode >> 8) & 0b1111).unwrap()
}

fn get_y(opcode: Opcode) -> u8 {
    u8::try_from(opcode & 0b1111).unwrap()
}

fn get_kk_or_byte(opcode: Opcode) -> u8 {
    u8::try_from(opcode & 0b1111_1111).unwrap()
}

#[cfg(test)]
mod tests {

    use super::{get_next_opcode, parse_until_next_branch};

    #[test]
    fn test_get_next_opcode() {
        let binary: Vec<u8> = vec![0xaa, 0xbb, 0xcc, 0xdd];
        let mut index = 0;
        let value = get_next_opcode(&binary, &mut index);
        let expected = 0xbbaa;

        assert_eq!(expected, value, "{:#x}", value);
        assert_eq!(index, 2);
    }

    #[test]
    fn test_parse_until_next_branch() {
        let binary: Vec<u8> = vec![];
    }
}
