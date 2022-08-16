use minifb::{Key, KeyRepeat};

use crate::chip8::{
    Chip8State, INSTRUCTION_SIZE_BYTES, PIXEL_CLEAR, PIXEL_DRAW, WINDOW_HEIGHT, WINDOW_WIDTH,
};

use super::{Vx, Vy};

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
        _ => unreachable!(""),
    }
}

pub unsafe extern "C" fn cls(state: *mut Chip8State) {
    (*state).fb = [PIXEL_CLEAR; WINDOW_WIDTH * WINDOW_HEIGHT].to_vec();
}

pub unsafe extern "C" fn drw(state: *mut Chip8State, vx: Vx, vy: Vy, nibble: u8) {
    let mut index = usize::from(vx.0 * vy.0);

    for _ in 0..nibble {
        (*state).fb[index] = PIXEL_DRAW;
        index += 1;
    }
}

pub unsafe extern "C" fn skp(state: *mut Chip8State, vx: Vx) {
    let state = &mut *state;

    if let Some(key) = state
        .window
        .get_keys_pressed(KeyRepeat::No)
        .into_iter()
        .next()
    {
        if vx.0 == key_value(key) {
            state.pc += INSTRUCTION_SIZE_BYTES;
        }
    }
}

pub unsafe extern "C" fn sknp(state: *mut Chip8State, vx: Vx) {
    let state = &mut *state;

    if let Some(key) = state
        .window
        .get_keys_pressed(KeyRepeat::No)
        .into_iter()
        .next()
    {
        if vx.0 != key_value(key) {
            state.pc += INSTRUCTION_SIZE_BYTES;
        }
    }
}

pub unsafe extern "C" fn ld_k(state: *mut Chip8State, vx: Vx) {
    let mut pressed_key = None;

    while pressed_key.is_none() {
        pressed_key = (*state)
            .window
            .get_keys_pressed(KeyRepeat::No)
            .into_iter()
            .next();
    }

    let pressed_key = pressed_key.unwrap();
    (*state).regs[usize::from(vx.0)] = key_value(pressed_key);
}

pub unsafe extern "C" fn ld_f(state: *mut Chip8State, vx: Vx) {
    let index = u16::from(vx.0 * 5);
    (*state).i = index;
}

pub unsafe extern "C" fn ld_b(state: *mut Chip8State, vx: Vx) {
    let state = &mut *state;
    let start_index = usize::from(state.i);
    state.mem[start_index] = vx.0 / 100;
    state.mem[start_index + 1] = (vx.0 % 100) / 10;
    state.mem[start_index + 1] = vx.0 % 10;
}
