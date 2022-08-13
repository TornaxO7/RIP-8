use std::cell::RefCell;
use std::rc::Rc;

use crate::chip8::Chip8State;
use crate::cache::CompileBlock;

pub fn compile(state: &Rc<RefCell<Chip8State>>) -> CompileBlock {
    let mut jit = JIT::new(state);
    jit.compile()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JIT<'a> {
    chip_state: &'a Rc<RefCell<Chip8State>>,
    x86_code: Vec<u8>,
}

impl<'a> JIT<'a> {
    fn new(chip_state: &'a Rc<RefCell<Chip8State>>) -> Self {
        Self {
            chip_state,
            x86_code: Vec::new()
        }
    }

    fn compile(&mut self) -> CompileBlock {
        todo!()
    }

    fn prolog(&mut self) {
        todo!();
    }

    fn epilog(&mut self) {
        todo!();
    }
}
