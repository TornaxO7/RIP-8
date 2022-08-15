use crate::chip8::Chip8State;

use super::{Vx, Vy, InstructionResult, Addr};

pub unsafe extern "C" fn cls(state: * mut Chip8State) {
    todo!()
}

pub unsafe extern "C" fn ld_i(state: * mut Chip8State, addr: Addr) {
    todo!()
}

pub unsafe extern "C" fn drw(state: * mut Chip8State, vx: Vx, vy: Vy, nibble: u8) {
    todo!()
}

pub unsafe extern "C" fn skp(state: * mut Chip8State, vx: Vx) -> bool {
    todo!()
}

pub unsafe extern "C" fn sknp(state: * mut Chip8State, vx: Vx, vy: Vy, nibble: u8) -> bool {
    todo!()
}

pub unsafe extern "C" fn ld_k(state: * mut Chip8State, vx: Vx) {
    todo!()
}

pub unsafe extern "C" fn ld_f(state: * mut Chip8State, vx: Vx) {
    todo!()
}

pub unsafe extern "C" fn ld_b(state: * mut Chip8State, vx: Vx) {
    todo!()
}
