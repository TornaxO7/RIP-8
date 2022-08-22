use simple::{Key, Rect, Window};

use crate::cache::Cache;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

pub const INSTRUCTION_SIZE_BYTES: u64 = 2;

pub const PIXEL_DRAW: u32 = u32::MAX;
pub const PIXEL_CLEAN: u32 = 0;

#[allow(non_upper_case_globals)]
pub const WINDOW_WIDTHu16: u16 = 64;
#[allow(non_upper_case_globals)]
pub const WINDOW_HEIGHTu16: u16 = 32;
#[allow(non_upper_case_globals)]
pub const WINDOW_WIDTHusize: usize = WINDOW_WIDTHu16 as usize;
#[allow(non_upper_case_globals)]
pub const WINDOW_HEIGHTusize: usize = WINDOW_HEIGHTu16 as usize;

pub const WINDOW_SIZE: usize = (WINDOW_WIDTHu16 * WINDOW_HEIGHTu16) as usize;

pub const FACTOR: u16 = 10;
pub const AMOUNT_KEYS: usize = 16;

pub const KEY_LAYOUT: [Key; 16] = [
    Key::Num1,
    Key::Num2,
    Key::Num3,
    Key::Num4,
    Key::Q,
    Key::W,
    Key::E,
    Key::R,
    Key::A,
    Key::S,
    Key::D,
    Key::F,
    Key::Z,
    Key::X,
    Key::C,
    Key::V,
];

pub const SPRITES: [u8; 16 * 5] = [
    0xf0, 0x90, 0x90, 0x90, 0xf0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xf0, 0x10, 0xf0, 0x80, 0xf0, // 2
    0xf0, 0x10, 0xf0, 0x10, 0xf0, // 3
    0xf0, 0x80, 0xf0, 0x90, 0xf0, // 4
    0xf0, 0x80, 0xf0, 0x10, 0xf0, // 5
    0xf0, 0x80, 0xf0, 0x90, 0xf0, // 6
    0xf0, 0x10, 0x20, 0x40, 0x40, // 7
    0xf0, 0x90, 0xf0, 0x90, 0xf0, // 8
    0xf0, 0x90, 0xf0, 0x10, 0xf0, // 9
    0xf0, 0x90, 0xf0, 0x90, 0x90, // A
    0xe0, 0xe0, 0xe0, 0x90, 0xe0, // B
    0xf0, 0x80, 0x80, 0x80, 0xf0, // C
    0xe0, 0x90, 0x90, 0x90, 0xe0, // D
    0xf0, 0x80, 0xf0, 0x80, 0xf0, // E
    0xf0, 0x80, 0xf0, 0x80, 0x80, // F
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
    pub regs: [u64; Chip8::AMOUNT_REGISTERS],
    pub i: u64,
    pub delay: u64,
    pub sound: u64,
    pub pc: u64,
    pub sp: u64,
    pub stack: [u64; Chip8::MAX_AMOUNT_STACK],
    pub window: Window,
    pub need_window_update: bool,
    pub fb: [bool; WINDOW_SIZE],
    pub keys: [bool; AMOUNT_KEYS],
    pub tick: Instant,
    pub help_regs: [u64; Chip8::AMOUNT_REGISTERS],
    should_run: bool,
}

pub struct Chip8 {
    state: Rc<RefCell<Chip8State>>,
    cache: Cache,
}

impl Chip8 {
    pub const MEM_SIZE: usize = 4096;
    pub const AMOUNT_REGISTERS: usize = 16;
    pub const START_ADDRESS: u64 = 0x200;
    pub const MAX_AMOUNT_STACK: usize = 16;
    pub const FREQUENCY: Duration = Duration::new(0, 16000000);
    pub const REG_MAX_VALUE: i32 = 0xff;

    pub fn new(binary_content: Vec<u8>) -> Self {
        if !binary_is_valid(&binary_content) {
            panic!("ROM is too big");
        }

        let mut mem = [0u8; Chip8::MEM_SIZE];
        for (index, &value) in SPRITES.iter().enumerate() {
            mem[index] = value;
        }
        for (index, &value) in binary_content.iter().enumerate() {
            mem[Self::START_ADDRESS as usize + index] = value;
        }

        let window = Window::new("RIP-8", WINDOW_WIDTHu16 * FACTOR, WINDOW_HEIGHTu16 * FACTOR);

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
                fb: [false; WINDOW_SIZE],
                keys: [false; AMOUNT_KEYS],
                help_regs: [0; Self::AMOUNT_REGISTERS],
                tick: Instant::now(),
                window,
                need_window_update: true,
            })),
            cache: Cache::new(),
        }
    }

    pub fn run(&mut self) {
        while self.state.borrow().should_run {
            let block = self.cache.get_or_compile(self.state.clone());
            block.execute(self.state.clone());

            self.tick();
        }
    }

    pub fn tick(&mut self) {
        if self.state.borrow().need_window_update {
            self.refresh_window();
        }
        self.refresh_keys();

        std::thread::sleep(Self::FREQUENCY.saturating_sub(self.state.borrow().tick.elapsed()));
    }

    pub fn refresh_window(&mut self) {
        let mut state = self.state.borrow_mut();
        state.window.clear();

        let buffer = state.fb.clone();
        for (index, &pixel) in buffer.iter().enumerate() {
            if pixel {
                let index = index as u16;
                let height = index / WINDOW_WIDTHu16 * FACTOR;
                let width = index % WINDOW_WIDTHu16 * FACTOR;
                let rect = Rect::new(width as i32, height as i32, FACTOR as u32, FACTOR as u32);
                state.window.fill_rect(rect);
            }
        }
    }

    pub fn refresh_keys(&mut self) {
        let mut state = self.state.borrow_mut();
        for (index, &key) in KEY_LAYOUT.iter().enumerate() {
            if state.window.is_key_down(key) {
                state.keys[index] = true;
            } else {
                state.keys[index] = false;
            }
        }

        for _ in 0..10 {
            if state.window.is_key_down(KEY_LAYOUT[4]) {
                state.should_run = false;
            }
        }
    }
}

fn binary_is_valid(binary: &Vec<u8>) -> bool {
    binary.len() <= Chip8::MEM_SIZE
}
