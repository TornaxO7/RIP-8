use crate::chip8::Chip8State;

use super::{Preparation, JIT};
use iced_x86::code_asm::*;

pub struct ChipState;

impl Preparation for ChipState {
    fn prolog(jit: &mut JIT) -> Result<(), iced_x86::IcedError> {
        let chip_state = &*jit.chip_state.borrow() as * const Chip8State;
        jit.x86.mov(rax, chip_state as u64)?;
        jit.x86.push(rax)?;
        Ok(())
    }

    fn epilog(jit: &mut JIT) -> Result<(), iced_x86::IcedError> {
        jit.x86.pop(rax)?;
        Ok(())
    }
}
