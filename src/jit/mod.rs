mod chip8;
mod x86;

use std::path::Path;
use std::fs::read;
use std::collections::HashMap;

use chip8::Chip8Instruction;
use x86::x86Instruction;

pub type Addr = usize;
pub type Chip8Block = Vec<Chip8Instruction>;
pub type x86Block = Vec<x86Instruction>;

const INSTRUCTION_SIZE: usize = 2;

#[derive(Debug)]
pub struct JIT {
    file_content: Vec<u8>,
    blocks: HashMap<Addr, x86Block>,
    chip_addr: Addr,
}

impl JIT {
    const START_ADDR: Addr = 0x200;

    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            file_content: read(path).expect("Couldn't load binary file"),
            blocks: HashMap::new(),
            chip_addr: Self::START_ADDR,
        }
    }

    pub fn run(&mut self) {
        loop {
            if self.block_not_already_compiled() {
                let block: x86Block = self.compile_block();
                self.blocks.insert(self.chip_addr, block);
            }

            let execution_block = self.blocks.get(&self.chip_addr)
                .unwrap();

            self.execute_block(execution_block);
        }
    }

    fn block_not_already_compiled(&self) -> bool {
        self.blocks.get(&self.chip_addr).is_none()
    }

    fn compile_block(&mut self) -> x86Block {
        let parsed_block = chip8::parse_until_next_branch(&self.file_content, &mut self.chip_addr);

        self.recompile_to_x86(parsed_block)
    }

    fn execute_block(&mut self, execution_block: &x86Block) {
        self.chip_addr = unsafe {
            (*x86Block[0])()
        };
    }

    fn recompile_to_x86(&self, chip_block: Chip8Block) -> x86Block {
        chip_block
            .iter()
            .flat_map(x86Instruction::from_chip8)
            .collect()
    }

}
