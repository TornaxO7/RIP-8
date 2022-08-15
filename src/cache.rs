use fnv::FnvHashMap;
use memmap2::Mmap;

use crate::chip8::Chip8State;
use crate::jit;
use crate::ChipAddr;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Cache {
    blocks: FnvHashMap<ChipAddr, CompileBlock>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            blocks: FnvHashMap::default(),
        }
    }

    pub fn get_or_compile(&mut self, state: Rc<RefCell<Chip8State>>) -> &CompileBlock {
        let pc = state.borrow().pc;
        self.blocks.entry(pc).or_insert(jit::compile(state))
    }
}

#[derive(Debug)]
pub struct CompileBlock {
    pub code: Mmap,
    pub start_addr: ChipAddr,
}

impl CompileBlock {
    pub fn execute(&self, state: Rc<RefCell<Chip8State>>) {
        let state = (&mut *state.borrow_mut()) as *mut Chip8State;

        let fnptr: unsafe extern "C" fn(state: *mut Chip8State) =
            unsafe { std::mem::transmute(self.code.as_ptr()) };
        unsafe {
            fnptr(state);
        }
    }
}
