use minifb::{Key, Window, WindowOptions};

use crate::cache::Cache;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Instant, Duration};

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

pub const FACTOR: usize = 10;
pub const AMOUNT_KEYS: usize = 16;

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
    Helper(u8),
}

#[derive(Debug)]
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
    pub fb: Vec<bool>,
    pub keys: [bool; AMOUNT_KEYS],
    pub tick: Instant,
    pub help_regs: [u64; Chip8::AMOUNT_REGISTERS],
    should_run: bool,
}

#[derive(Debug)]
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

        let window = Window::new(
            "RIP-8",
            WINDOW_WIDTHusize * FACTOR,
            WINDOW_HEIGHTusize * FACTOR,
            WindowOptions::default(),
        )
        .unwrap();

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
                fb: [false; WINDOW_WIDTHusize * WINDOW_HEIGHTusize].to_vec(),
                keys: [false; AMOUNT_KEYS],
                help_regs: [0; Self::AMOUNT_REGISTERS],
                tick: Instant::now(),
                window,
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
        self.refresh_window();
        self.refresh_keys();

        std::thread::sleep(Self::FREQUENCY.saturating_sub(self.state.borrow().tick.elapsed()));
    }

    pub fn refresh_window(&mut self) {
        let mut state = self.state.borrow_mut();

        let mut buffer: Vec<u32> = [0; WINDOW_HEIGHTusize * WINDOW_WIDTHusize * FACTOR].to_vec();
        for entry in state.fb.clone().into_iter().enumerate() {
            let (index, should_place) = entry;

            if should_place {
                for fb_index in index * FACTOR..index * 2 * FACTOR {
                    buffer[fb_index] = PIXEL_DRAW;
                }
            }
        }

        state.window.update_with_buffer(
            &buffer,
            WINDOW_WIDTHusize * FACTOR,
            WINDOW_HEIGHTusize,
        ).unwrap();

        if let Some(key) = state.window.get_keys_pressed(minifb::KeyRepeat::No).into_iter().next() {
            state.should_run = key == Key::Q;
        }
    }

    pub fn refresh_keys(&mut self) {
        let mut state = self.state.borrow_mut();

        state.window.get_keys_pressed(minifb::KeyRepeat::No).into_iter().for_each(|key: Key| {
            state.keys[key_value(key) as usize] = true;
        });
        state.window.get_keys_released().into_iter().for_each(|key: Key| {
            state.keys[key_value(key) as usize] = false;
        });

        state.should_run = !state.keys[key_value(Key::A) as usize];
    }
}

fn binary_is_valid(binary: &Vec<u8>) -> bool {
    binary.len() <= Chip8::MEM_SIZE
}

fn key_value(key: Key) -> u8 {
    match key {
        Key::Key1 => 0x1,
        Key::Key2 => 0x2,
        Key::Key3 => 0x3,
        Key::Key4 => 0x4,
        Key::Key5 => 0x5,
        Key::Key6 => 0x6,
        Key::Key7 => 0x7,
        Key::Key8 => 0x8,
        Key::Key9 => 0x9,
        Key::A => 0xA,
        Key::B => 0xB,
        Key::C => 0xC,
        Key::E => 0xE,
        Key::F => 0xF,
        _ => unreachable!("Unknown key: {:?}", key),
    }
}

