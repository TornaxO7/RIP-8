
use iced_x86::code_asm::*;

use crate::jit::{Frame, JIT};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StackFrame;

impl Frame for StackFrame {
    fn prolog(&self, jit: &mut JIT) -> Result<(), iced_x86::IcedError> {
        jit.x86.push(rbp)?;
        jit.x86.mov(rsp, rbp)?;

        jit.x86.push(rbx)?;
        jit.x86.push(r12)?;
        jit.x86.push(r13)?;
        jit.x86.push(r14)?;
        jit.x86.push(r15)?;

        Ok(())
    }

    fn epilog(&self, jit: &mut JIT) -> Result<(), iced_x86::IcedError> {
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
