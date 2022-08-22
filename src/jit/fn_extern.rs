use bit_iter::BitIter;
use simple::Key;

use crate::chip8::{
    Chip8State, WINDOW_HEIGHTusize, WINDOW_WIDTHusize, FACTOR, INSTRUCTION_SIZE_BYTES, PIXEL_CLEAN, WINDOW_WIDTHu16, WINDOW_SIZE, KEY_LAYOUT,
};

fn key_value(key: Key) -> u64 {
    match key {
        Key::Num1 => 0x1,
        Key::Num2 => 0x2,
        Key::Num3 => 0x3,
        Key::Num4 => 0x4,
        Key::Num5 => 0x5,
        Key::Num6 => 0x6,
        Key::Num7 => 0x7,
        Key::Num8 => 0x8,
        Key::Num9 => 0x9,
        Key::A => 0xA,
        Key::B => 0xB,
        Key::C => 0xC,
        Key::E => 0xE,
        Key::F => 0xF,
        _ => unreachable!(""),
    }
}

pub unsafe extern "C" fn cls(state: *mut Chip8State) {
    let state = &mut *state;
    state.fb = [false; WINDOW_SIZE];
    state.need_window_update = true;
}

pub unsafe extern "C" fn drw(state: *mut Chip8State, vx: u64, vy: u64, nibble: u64) {
    let state = &mut *state;
    let vx_value = state.regs[vx as usize];
    let vy_value = state.regs[vy as usize];
    state.regs[0xf] = 0;

    let mut x_coord = vx_value;
    let mut y_coord = vy_value;

    for byte_index in state.i..state.i + nibble {
        let byte = state.mem[byte_index as usize];

        for bit in BitIter::from(byte) {
            let addr: usize = (x_coord + y_coord * u64::from(WINDOW_WIDTHu16)) as usize;
            let prev_value = state.fb[addr];

            state.fb[addr] ^= bit_to_bool(bit);

            if state.fb[addr] != prev_value {
                state.regs[0xf] = 1;
            }

            x_coord += 1;
        }
        y_coord += 1;
    }

    state.need_window_update = true;
}

pub unsafe extern "C" fn skp(state: *mut Chip8State, vx: u64) {
    let vx = u8::try_from(vx & 0xff).unwrap();
    let state = &mut *state;

    if state.keys[usize::try_from(vx).unwrap()] {
        state.pc += INSTRUCTION_SIZE_BYTES;
    }
}

pub unsafe extern "C" fn sknp(state: *mut Chip8State, vx: u64) {
    let vx = u8::try_from(vx & 0xff).unwrap();
    let state = &mut *state;

    if !state.keys[usize::try_from(vx).unwrap()] {
        state.pc += INSTRUCTION_SIZE_BYTES;
    }
}

pub unsafe extern "C" fn ld_k(state: *mut Chip8State, vx: u64) {
    let vx = u8::try_from(vx & 0xff).unwrap();
    let state = &mut *state;
    let vx = usize::try_from(vx).unwrap();

    let mut pressed_key = false;
    while !pressed_key {
        for &key in KEY_LAYOUT.iter() {
            if state.window.is_key_down(key) {
                state.regs[vx] = key_value(key);
                pressed_key = true;
            }
        }
    }
}

pub unsafe extern "C" fn ld_f(state: *mut Chip8State, vx: u64) {
    let vx = u8::try_from(vx & 0xff).unwrap();
    let index = u64::from(vx * 5);
    (*state).i = index;
}

pub unsafe extern "C" fn ld_b(state: *mut Chip8State, vx: u64) {
    let vx = u8::try_from(vx & 0xff).unwrap();
    let state = &mut *state;
    let start_index = usize::try_from(state.i).unwrap();
    state.mem[start_index] = u8::try_from(vx / 100).unwrap();
    state.mem[start_index + 1] = u8::try_from((vx % 100) / 10).unwrap();
    state.mem[start_index + 1] = u8::try_from(vx % 10).unwrap();
}

fn bit_to_bool(bit: usize) -> bool {
    (bit & 1) == 1
}
