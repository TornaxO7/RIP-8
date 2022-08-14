use crate::cache::Cache;

use std::rc::Rc;
use std::cell::RefCell;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Chip8State {
    pub mem: [u8; Chip8::MEM_SIZE],
    pub regs: [u8; Chip8::AMOUNT_REGISTERS],
    pub i: u16,
    pub delay: u8,
    pub sound: u8,
    pub pc: u16,
    pub sp: u8,
    pub stack: [u16; Chip8::MAX_AMOUNT_STACK],
}

#[derive(Debug)]
pub struct Chip8 {
    state: Rc<RefCell<Chip8State>>,
    cache: Cache,
}

impl Chip8 {
    pub const MEM_SIZE: usize = 4096;
    pub const AMOUNT_REGISTERS: usize = 16;
    pub const START_ADDRESS: u16 = 0x200;
    pub const MAX_AMOUNT_STACK: usize = 16;

    pub fn new(binary_content: Vec<u8>) -> Self {
        if !binary_is_valid(&binary_content) {
            panic!("ROM is too big");
        }

        let mut mem = [0; Chip8::MEM_SIZE];
        for (index, &value) in binary_content.iter().enumerate() {
            mem[index] = value;
        }

        Self {
            state: Rc::new(RefCell::new(Chip8State {
                mem,
                regs: [0; Chip8::AMOUNT_REGISTERS],
                i: 0,
                delay: 0,
                sound: 0,
                pc: Self::START_ADDRESS,
                sp: 0,
                stack: [0; Chip8::MAX_AMOUNT_STACK],
            })),
            cache: Cache::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            let block = self.cache.get_or_compile(&self.state);
            block.execute();
        }
    }
}

fn binary_is_valid(binary: &Vec<u8>) -> bool {
    binary.len() <= Chip8::MEM_SIZE
}
