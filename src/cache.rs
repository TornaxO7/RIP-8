use memmap2::MmapMut;
use fnv::FnvHashMap;

use crate::ChipAddr;
use crate::chip8::Chip8State;
use crate::jit;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Cache {
    blocks: FnvHashMap<ChipAddr, CompileBlock>
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
    pub code: MmapMut,
    pub start_addr: ChipAddr,
}

impl CompileBlock {
    pub fn execute(&self) {
    }
}
