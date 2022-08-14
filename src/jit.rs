use std::cell::RefCell;
use std::rc::Rc;

use crate::chip8::Chip8State;
use crate::cache::CompileBlock;

use iced_x86::code_asm::*;
use memmap2::MmapMut;

pub fn compile(state: &Rc<RefCell<Chip8State>>) -> CompileBlock {
    let mut jit = JIT::new(state);
    jit.compile()
}

pub struct JIT<'a> {
    chip_state: &'a Rc<RefCell<Chip8State>>,
    x86: CodeAssembler,
}

impl<'a> JIT<'a> {
    const BITNESS: u32 = 16;
    const STACK_OFFSET: u32 = 5 * 64;

    fn new(chip_state: &'a Rc<RefCell<Chip8State>>) -> Self {
        Self {
            chip_state,
            x86: CodeAssembler::new(Self::BITNESS).unwrap(),
        }
    }

    fn compile(&mut self) -> CompileBlock {
        self.stack_frame_head();
        self.stack_frame_tail();

        self.get_compiled_block()
    }

    fn get_compiled_block(&mut self) -> CompileBlock {
        let pc = self.chip_state.borrow().pc;
        let bytes = self.x86.assemble(u64::from(pc)).unwrap();
        let mut code = MmapMut::map_anon(bytes.len()).unwrap();
        code.copy_from_slice(&bytes);

        CompileBlock {
            code,
            start_addr: pc,
        }
    }

    fn stack_frame_head(&mut self) -> Result<(), IcedError> {
        self.x86.push(rbp)?;
        self.x86.mov(rsp, rbp)?;
        self.x86.sub(10_i32, rsp)?;

        self.x86.push(rbx)?;
        self.x86.push(r12)?;
        self.x86.push(r13)?;
        self.x86.push(r14)?;
        self.x86.push(r15)?;

        Ok(())
    }

    fn stack_frame_tail(&mut self) -> Result<(), IcedError> {
        self.x86.pop(r15)?;
        self.x86.pop(r14)?;
        self.x86.pop(r13)?;
        self.x86.pop(r12)?;
        self.x86.pop(rbx)?;

        self.x86.mov(rbp, rsp)?;
        self.x86.pop(rbp)?;

        Ok(())
    }

    fn reserve_regs(&mut self) {
    }
}
