pub mod cache;
pub mod chip8;
pub mod jit;

pub type ChipAddr = u16;

use std::fs::read;

use chip8::Chip8;

pub fn run(path: &str) {
    let binary_content = read(path).unwrap();
    Chip8::new(binary_content).run();
}
