use fnv::FnvHashMap;
use log::debug;
use memmap2::Mmap;

use crate::chip8::Chip8;
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
        let block = self.blocks.entry(pc).or_insert_with(|| {
            debug!("Cache miss for {:#x}", pc);
            jit::compile(state)
        });

        debug!("Pc after compilation: {:#x}", pc);
        block
    }
}

#[derive(Debug)]
pub struct CompileBlock {
    pub code: Mmap,
    pub start_addr: ChipAddr,
}

impl CompileBlock {
    pub fn execute(&self, state: Rc<RefCell<Chip8State>>) {
        assert!(state.borrow().pc >= Chip8::START_ADDRESS);
        debug!("Before Execution: {:#x}", state.borrow().pc);

        {
            let state = (&mut *state.borrow_mut()) as *mut Chip8State;

            let fnptr: unsafe extern "C" fn(state: *mut Chip8State) =
                unsafe { std::mem::transmute(self.code.as_ptr()) };
            unsafe {
                fnptr(state);
            }
        }

        debug!("After Execution: {:#x}", state.borrow().pc);
    }
}
