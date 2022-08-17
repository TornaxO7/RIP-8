pub mod cache;
pub mod chip8;
pub mod jit;

use std::fs::read;

pub type Addr = u64;

use chip8::Chip8;

pub fn run(path: &str) {
    let binary_content = read(path).unwrap();
    Chip8::new(binary_content).run();
}
