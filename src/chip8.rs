use log::debug;
use simple::{Point, Window, Key};

use crate::cache::Cache;

use std::cell::RefCell;
use std::rc::Rc;

pub const INSTRUCTION_SIZE_BYTES: u16 = 2;

#[allow(non_upper_case_globals)]
pub const WINDOW_WIDTHu16: u16 = 64;
#[allow(non_upper_case_globals)]
pub const WINDOW_HEIGHTu16: u16 = 32;
#[allow(non_upper_case_globals)]
pub const WINDOW_WIDTHusize: usize = WINDOW_WIDTHu16 as usize;
#[allow(non_upper_case_globals)]
pub const WINDOW_HEIGHTusize: usize = WINDOW_HEIGHTu16 as usize;

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

pub const AMOUNT_KEYS: usize = 16;
pub const KEY_LAYOUT: [Key; AMOUNT_KEYS] =
[
    Key::Num1, Key::Num2, Key::Num3, Key::Num4,
    Key::Q, Key::W, Key::E, Key::R,
    Key::A, Key::S, Key::D, Key::F,
    Key::Z, Key::X, Key::C, Key::V
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
    pub fb: Vec<bool>,
    pub keys: [bool; AMOUNT_KEYS],
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

        let mut window = Window::new("RIP-8", WINDOW_WIDTHu16, WINDOW_HEIGHTu16);
        window.set_color(u8::MAX, u8::MAX, u8::MAX, u8::MAX);

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
                window: Window::new("RIP-8", WINDOW_WIDTHu16, WINDOW_HEIGHTu16),
            })),
            cache: Cache::new(),
        }
    }

    pub fn run(&mut self) {
        while self.state.borrow().should_run {
            let block = self.cache.get_or_compile(self.state.clone());
            debug!("pc: {:#x}", self.state.borrow().pc);
            block.execute(self.state.clone());
            self.state.borrow_mut().pc += 1;

            self.refresh_window();
            self.refresh_keys();
        }
    }

    pub fn refresh_window(&mut self) {
        self.clear_screen();
        let mut state = self.state.borrow_mut();

        for entry in state.fb.clone().into_iter().enumerate() {
            let (index, should_place) = entry;
            if should_place {
                let x = i32::try_from(index % WINDOW_WIDTHusize).unwrap();
                let y = i32::try_from(index / WINDOW_HEIGHTusize).unwrap();
                let point = Point::new(x, y);
                state.window.draw_point(point.clone());
            }
       }
    }

    fn clear_screen(&mut self) {
        let mut state = self.state.borrow_mut();
        state.window.clear_to_color(0, 0, 0);
    }

    fn refresh_keys(&mut self) {
        let mut state = self.state.borrow_mut();


        for (i, key) in KEY_LAYOUT.iter().enumerate() {
            if state.window.is_key_down(*key) {
                state.keys[i] = true;
            } else {
                state.keys[i] = false;
            }
        }
    }
}

fn binary_is_valid(binary: &Vec<u8>) -> bool {
    binary.len() <= Chip8::MEM_SIZE
}
