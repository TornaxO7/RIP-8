use iced_x86::code_asm::*;

use crate::jit::{Frame, JIT};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StackFrame;

impl Frame for StackFrame {
    fn prolog(&self, jit: &mut JIT) {
        jit.x86.push(rbp).unwrap();
        jit.x86.mov(rbp, rsp).unwrap();
    }

    fn epilog(&self, jit: &mut JIT) {
        jit.x86.mov(rsp, rbp).unwrap();
        jit.x86.pop(rbp).unwrap();
        jit.x86.ret().unwrap();
    }
}
