mod frames;

use frames::{StackFrame, ChipState};

use std::cell::RefCell;
use std::rc::Rc;

use crate::cache::CompileBlock;
use crate::chip8::Chip8State;

use iced_x86::code_asm::CodeAssembler;
use iced_x86::IcedError;
use memmap2::MmapMut;

pub fn compile(state: &Rc<RefCell<Chip8State>>) -> CompileBlock {
    let mut jit = JIT::new(state);

    match jit.compile() {
        Ok(compiled_block) => compiled_block,
        Err(err) => panic!("{}", err),
    }
}

pub trait Frame {
    fn prolog(&self, jit: &mut JIT<'_>) -> Result<(), IcedError>;

    fn epilog(&self, jit: &mut JIT<'_>) -> Result<(), IcedError>;
}

pub struct JIT<'a> {
    pub chip_state: &'a Rc<RefCell<Chip8State>>,
    pub x86: CodeAssembler,
}

impl<'a> JIT<'a> {
    pub const QUAD_WORD: i32 = 64;

    const BITNESS: u32 = 16;
    const STEPS: [&'static dyn Frame; 2] = [
        &StackFrame as &dyn Frame,
        &ChipState as &dyn Frame,
    ];

    fn new(chip_state: &'a Rc<RefCell<Chip8State>>) -> Self {
        Self {
            chip_state,
            x86: CodeAssembler::new(Self::BITNESS).unwrap(),
        }
    }

    fn compile(&mut self) -> Result<CompileBlock, IcedError> {
        self.prolog()?;

        self.epilog()?;
        self.get_compiled_block()
    }

    fn get_compiled_block(&mut self) -> Result<CompileBlock, IcedError> {
        let pc = self.chip_state.borrow().pc;
        let bytes = self.x86.assemble(u64::from(pc))?;
        let mut code = MmapMut::map_anon(bytes.len()).unwrap();
        code.copy_from_slice(&bytes);

        Ok(CompileBlock {
            code,
            start_addr: pc,
        })
    }

    fn prolog(&mut self) -> Result<(), IcedError> {
        for step in Self::STEPS {
            step.prolog(self)?;
        }

        Ok(())
    }

    fn epilog(&mut self) -> Result<(), IcedError> {
        for step in Self::STEPS {
            step.epilog(self)?;
        }

        Ok(())
    }
}
