use memmap::MmapMut;
use fnv::FnvHashMap;

use crate::ChipAddr;
use crate::chip8::Chip8State;

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

    pub fn get_or_compile(&self, state: &Chip8State) -> CompileBlock {
        todo!()
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
