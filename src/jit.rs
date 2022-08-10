use crate::chip8::Chip8State;
use crate::cache::CompileBlock;

pub fn compile(state: &mut Chip8State) -> CompileBlock {
    let mut jit = JIT::new(state);
    jit.compile()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JIT {
    chip_state: Chip8State,
    x86_code: Vec<u8>,
}

impl JIT {
    fn new(state: &mut Chip8State) -> Self {
        // Self {
        //     chip_state: 
        // }
        todo!()
    }

    fn compile(&mut self) -> CompileBlock {
        todo!()
    }
}
