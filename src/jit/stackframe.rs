use super::{Preparation, JIT};

use iced_x86::code_asm::*;

pub struct StackFrame;

impl Preparation for StackFrame {
    fn prolog(jit: &mut JIT) -> Result<(), iced_x86::IcedError> {
        jit.x86.push(rbp)?;
        jit.x86.mov(rsp, rbp)?;

        jit.x86.push(rbx)?;
        jit.x86.push(r12)?;
        jit.x86.push(r13)?;
        jit.x86.push(r14)?;
        jit.x86.push(r15)?;

        Ok(())
    }

    fn epilog(jit: &mut JIT) -> Result<(), iced_x86::IcedError> {
        jit.x86.pop(r15)?;
        jit.x86.pop(r14)?;
        jit.x86.pop(r13)?;
        jit.x86.pop(r12)?;
        jit.x86.pop(rbx)?;

        jit.x86.mov(rbp, rsp)?;
        jit.x86.pop(rbp)?;
        Ok(())
    }
}
