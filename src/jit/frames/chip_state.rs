use crate::{chip8::Chip8State, jit::{Frame, JIT}};


use iced_x86::code_asm::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChipState;

impl Frame for ChipState {
    fn prolog(&self, jit: &mut JIT) -> Result<(), iced_x86::IcedError> {
        let chip_state = &*jit.chip_state.borrow() as * const Chip8State;
        jit.x86.mov(rax, chip_state as u64)?;
        jit.x86.push(rax)?;
        Ok(())
    }

    fn epilog(&self, jit: &mut JIT) -> Result<(), iced_x86::IcedError> {
        jit.x86.sub(rsp, JIT::QUAD_WORD)?;
        Ok(())
    }
}
