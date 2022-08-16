use simple::Key;

use crate::chip8::{
    Chip8State, INSTRUCTION_SIZE_BYTES, KEY_LAYOUT
};

fn key_value(key: Key) -> u8 {
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
    state.window.clear_to_color(0, 0, 0);
}

pub unsafe extern "C" fn drw(state: *mut Chip8State, vx: u32, vy: u32, nibble: u8) {
    let mut index = usize::try_from(vx * vy).unwrap();

    for _ in 0..nibble {
        (*state).fb[index] = true;
        index += 1;
    }
    panic!("pls");
}

pub unsafe extern "C" fn skp(state: *mut Chip8State, vx: u32) {
    let state = &mut *state;

    if state.keys[usize::try_from(vx).unwrap()] {
        state.pc += INSTRUCTION_SIZE_BYTES;
    }
}

pub unsafe extern "C" fn sknp(state: *mut Chip8State, vx: u32) {
    let state = &mut *state;

    if !state.keys[usize::try_from(vx).unwrap()] {
        state.pc += INSTRUCTION_SIZE_BYTES;
    }
}

pub unsafe extern "C" fn ld_k(state: *mut Chip8State, vx: u32) {
    let mut found_key = false;
    let state = &mut *state;
    let vx = usize::try_from(vx).unwrap();

    while !found_key {
        for &key in KEY_LAYOUT.iter() {
            if state.window.is_key_down(key) {
                state.regs[vx] = key_value(key);
                found_key = true;
            }
        }
    }
}

pub unsafe extern "C" fn ld_f(state: *mut Chip8State, vx: u32) {
    let index = u16::try_from(vx * 5).unwrap();
    (*state).i = index;
}

pub unsafe extern "C" fn ld_b(state: *mut Chip8State, vx: u32) {
    let state = &mut *state;
    let start_index = usize::from(state.i);
    state.mem[start_index] = u8::try_from(vx / 100).unwrap();
    state.mem[start_index + 1] = u8::try_from((vx % 100) / 10).unwrap();
    state.mem[start_index + 1] = u8::try_from(vx % 10).unwrap();
}
