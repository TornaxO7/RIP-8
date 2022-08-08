mod chip8;
mod x86;

use std::path::Path;
use std::fs::read;

use self::{chip8::Chip8Opcode, x86::x86Opcode};

pub type Address = usize;

const INSTRUCTION_SIZE: usize = 16;

#[derive(Debug)]
pub struct JIT {
    chip8_binary_opcodes: Vec<chip8::Opcode>,
    chip8_opcode_buffer: Vec<Chip8Opcode>,
    x86_opcode_buffer: Vec<x86Opcode>,
    pc: Address,
}

impl JIT {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let binary_content = read(path).unwrap();
        Self {
            chip8_binary_opcodes: convert_bytes_to_chip8_opcodes(binary_content),
            chip8_opcode_buffer: Vec::new(),
            x86_opcode_buffer: Vec::new(),
            pc: 0,
        }
    }

    pub fn run(&mut self) {
        loop {
            self.decode_to_branch();
            self.get_x86_opcodes();
            self.execute_rest();
        }
    }

    fn decode_to_branch(&mut self) {
        self.chip8_opcode_buffer.clear();
        self.decode_next_opcode();

        while !self.chip8_opcode_buffer.last().unwrap().is_branch() {
            self.decode_next_opcode();
        }
    }

    fn decode_next_opcode(&mut self) {
        let next_opcode: chip8::Opcode = self.chip8_binary_opcodes[self.pc];
        let next_opcode: Chip8Opcode = Chip8Opcode::from(next_opcode);
        self.chip8_opcode_buffer.push(next_opcode);

        if next_opcode.is_branch() {
            self.adjust_pc_for_branch(next_opcode);
        } else {
            self.pc += INSTRUCTION_SIZE;
        }
    }

    fn adjust_pc_for_branch(&mut self, branch_opcode: Chip8Opcode) {
        todo!("Yeetus deletus!");
    }

    fn get_x86_opcodes(&mut self) {}

    fn execute_rest(&mut self) {}
}

fn convert_bytes_to_chip8_opcodes(binary_content: Vec<u8>) -> Vec<chip8::Opcode> {
    let mut chip8_binary_opcodes = Vec::new();
    let mut iterator = binary_content.iter();
    while let Some(&byte) = iterator.next() {
        let mut opcode: chip8::Opcode = u16::from(byte) << 8;

        if let Some(&byte2) = iterator.next() {
            opcode |= u16::from(byte2);
        }

        chip8_binary_opcodes.push(opcode);
    }

    chip8_binary_opcodes
}

#[cfg(test)]
mod tests {

    use super::convert_bytes_to_chip8_opcodes;

    #[test]
    fn test_new_general() {
        let binaries: Vec<u8> = vec![0xaa, 0xbb, 0xcc, 0xdd];
        let opcodes = convert_bytes_to_chip8_opcodes(binaries);

        let expected = vec![0xaabb, 0xccdd];

        assert_eq!(expected, opcodes);
    }

    #[test]
    fn test_missing_half() {
        let binaries: Vec<u8> = vec![0xaa, 0xbb, 0xcc];
        let opcodes = convert_bytes_to_chip8_opcodes(binaries);

        let expected = vec![0xaabb, 0xcc00];

        assert_eq!(expected, opcodes);
    }
}
