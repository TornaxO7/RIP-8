use crate::jit::{Frame, JIT};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StackFrame;

impl Frame for StackFrame {
    fn prolog(&self, _jit: &mut JIT) {
    }

    fn epilog(&self, jit: &mut JIT) {
        jit.x86.ret().unwrap();
    }
}
