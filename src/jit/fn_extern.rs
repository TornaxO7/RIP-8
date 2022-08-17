use minifb::Key;

use crate::chip8::{
    Chip8State, WINDOW_HEIGHTusize, WINDOW_WIDTHusize, FACTOR, INSTRUCTION_SIZE_BYTES, PIXEL_CLEAN,
};

fn key_value(key: Key) -> u64 {
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
    let state = &mut *state;
    let buffer = [PIXEL_CLEAN; WINDOW_WIDTHusize * WINDOW_HEIGHTusize * FACTOR];
    state
        .window
        .update_with_buffer(
            &buffer,
            WINDOW_WIDTHusize * FACTOR,
            WINDOW_HEIGHTusize * FACTOR,
        )
        .unwrap();
}

pub unsafe extern "C" fn drw(state: *mut Chip8State, vx: u64, vy: u64, nibble: u64) {
    let mut index = usize::try_from(vx * vy).unwrap();

    for _ in 0..nibble {
        (*state).fb[index] = true;
        index += 1;
    }
    panic!("pls");
}

pub unsafe extern "C" fn skp(state: *mut Chip8State, vx: u64) {
    let state = &mut *state;

    if state.keys[usize::try_from(vx).unwrap()] {
        state.pc += INSTRUCTION_SIZE_BYTES;
    }
}

pub unsafe extern "C" fn sknp(state: *mut Chip8State, vx: u64) {
    let state = &mut *state;

    if !state.keys[usize::try_from(vx).unwrap()] {
        state.pc += INSTRUCTION_SIZE_BYTES;
    }
}

pub unsafe extern "C" fn ld_k(state: *mut Chip8State, vx: u64) {
    let mut found_key = false;
    let state = &mut *state;
    let vx = usize::try_from(vx).unwrap();

    while !found_key {
        state
            .window
            .get_keys_pressed(minifb::KeyRepeat::No)
            .into_iter()
            .for_each(|key: Key| {
                state.regs[vx] = key_value(key);
                found_key = true;
            });
    }
}

pub unsafe extern "C" fn ld_f(state: *mut Chip8State, vx: u64) {
    let index = u64::from(vx * 5);
    (*state).i = index;
}

pub unsafe extern "C" fn ld_b(state: *mut Chip8State, vx: u64) {
    let state = &mut *state;
    let start_index = usize::try_from(state.i).unwrap();
    state.mem[start_index] = u8::try_from(vx / 100).unwrap();
    state.mem[start_index + 1] = u8::try_from((vx % 100) / 10).unwrap();
    state.mem[start_index + 1] = u8::try_from(vx % 10).unwrap();
}
