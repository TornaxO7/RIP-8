use minifb::{Window, WindowOptions, ScaleMode, Key};

use crate::cache::Cache;

use std::cell::RefCell;
use std::rc::Rc;

pub const INSTRUCTION_SIZE_BYTES: u16 = 2;

pub const WINDOW_WIDTH: usize = 64;
pub const WINDOW_HEIGHT: usize = 32;
pub const PIXEL_CLEAR: u32 = 0;
pub const PIXEL_DRAW: u32 = u32::MAX;

pub const SPRITES: [u8; 16 * 5] = [
    0xf0, 0x90, 0x90, 0x90, 0xf0,   // 0
    0x20, 0x60, 0x20, 0x20, 0x70,   // 1
    0xf0, 0x10, 0xf0, 0x80, 0xf0,   // 2
    0xf0, 0x10, 0xf0, 0x10, 0xf0,   // 3
    0xf0, 0x80, 0xf0, 0x90, 0xf0,   // 4
    0xf0, 0x80, 0xf0, 0x10, 0xf0,   // 5
    0xf0, 0x80, 0xf0, 0x90, 0xf0,   // 6
    0xf0, 0x10, 0x20, 0x40, 0x40,   // 7
    0xf0, 0x90, 0xf0, 0x90, 0xf0,   // 8
    0xf0, 0x90, 0xf0, 0x10, 0xf0,   // 9
    0xf0, 0x90, 0xf0, 0x90, 0x90,   // A
    0xe0, 0xe0, 0xe0, 0x90, 0xe0,   // B
    0xf0, 0x80, 0x80, 0x80, 0xf0,   // C
    0xe0, 0x90, 0x90, 0x90, 0xe0,   // D
    0xf0, 0x80, 0xf0, 0x80, 0xf0,   // E
    0xf0, 0x80, 0xf0, 0x80, 0x80,   // F
];

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Chip8Field {
    I,
    PC,
    SP,
    Stack,
    Reg(u8),
    Delay,
    Sound,
}

#[repr(C)]
pub struct Chip8State {
    pub mem: [u8; Chip8::MEM_SIZE],
    pub regs: [u8; Chip8::AMOUNT_REGISTERS],
    pub i: u16,
    pub delay: u8,
    pub sound: u8,
    pub pc: u16,
    pub sp: u8,
    pub stack: [u16; Chip8::MAX_AMOUNT_STACK],
    pub window: Window,
    pub fb: Vec<u32>,
    should_run: bool,
}

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
        for (index, &value) in SPRITES.iter().enumerate() {
            mem[index] = value;
        }
        for (index, &value) in binary_content.iter().enumerate() {
            mem[usize::from(Self::START_ADDRESS) + index] = value;
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
                should_run: true,
                fb: [PIXEL_CLEAR; WINDOW_HEIGHT * WINDOW_WIDTH].to_vec(),
                window: Window::new("RIP-8", WINDOW_WIDTH, WINDOW_HEIGHT, WindowOptions {
                    borderless: true,
                    title: true,
                    resize: false,
                    scale: minifb::Scale::FitScreen,
                    scale_mode: ScaleMode::Center,
                    topmost: true,
                    transparency: false,
                    none: false,
                }).unwrap(),
            })),
            cache: Cache::new(),
        }
    }

    pub fn run(&mut self) {
        while self.state.borrow().should_run {
            let block = self.cache.get_or_compile(self.state.clone());
            block.execute(self.state.clone());

            self.refresh_window();
        }
    }

    pub fn refresh_window(&self) {
        let buffer = &self.state.borrow().fb.clone();
        let mut state = self.state.borrow_mut();
        state.window.update_with_buffer(buffer, WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();

        state.should_run = !state.window.is_key_down(Key::Escape);
    }
}

fn binary_is_valid(binary: &Vec<u8>) -> bool {
    binary.len() <= Chip8::MEM_SIZE
}
