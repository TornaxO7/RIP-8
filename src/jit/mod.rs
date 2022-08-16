mod fn_extern;
mod fn_implementation;
mod fn_trait_impl;
mod fn_traits;
mod frames;

use frames::StackFrame;
use log::debug;

use std::cell::RefCell;
use std::convert::From;
use std::rc::Rc;

use crate::cache::CompileBlock;
use crate::chip8::{Chip8Field, Chip8State, INSTRUCTION_SIZE_BYTES};
use crate::ChipAddr;

use iced_x86::code_asm::CodeAssembler;
use memmap2::MmapMut;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Vx(pub u8);

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Vy(pub u8);

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Byte(pub u8);

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Addr(pub u16);

pub fn compile(state: Rc<RefCell<Chip8State>>) -> CompileBlock {
    let mut jit = JIT::new(state);

    jit.compile()
}

pub trait Frame {
    fn prolog(&self, jit: &mut JIT);

    fn epilog(&self, jit: &mut JIT);
}

#[repr(C)]
pub struct JIT {
    start_pc: u16,
    pub chip_state: Rc<RefCell<Chip8State>>,
    pub x86: CodeAssembler,
}

impl JIT {
    pub const QUAD_WORD: i32 = 8;

    const BITNESS: u32 = 16;
    const STEPS: [&'static dyn Frame; 1] = [&StackFrame as &dyn Frame];

    fn new(chip_state: Rc<RefCell<Chip8State>>) -> Self {
        let start_pc = chip_state.borrow().pc;
        Self {
            start_pc,
            chip_state,
            x86: CodeAssembler::new(Self::BITNESS).unwrap(),
        }
    }

    fn compile(&mut self) -> CompileBlock {
        self.recompile_chip8();

        self.epilog();

        debug!("Finished compiled block!");
        self.get_compiled_block()
    }

    fn get_compiled_block(&mut self) -> CompileBlock {
        let pc = self.chip_state.borrow().pc;
        let bytes = self.x86.assemble(u64::from(pc)).unwrap();
        let mut code = MmapMut::map_anon(bytes.len()).unwrap();
        code.copy_from_slice(&bytes);
        let code = code.make_exec().unwrap();

        CompileBlock {
            code,
            start_addr: self.start_pc,
        }
    }

    fn epilog(&mut self) {
        for step in Self::STEPS.into_iter().rev() {
            step.epilog(self);
        }
    }

    fn recompile_chip8(&mut self) {
        let mut pc: ChipAddr = self.chip_state.borrow().pc;

        while self.compile_next_instruction(pc) {
            debug!("Recompiling instruction next at {:#x}", pc);
            pc += u16::from(INSTRUCTION_SIZE_BYTES);
        }
    }

    fn compile_next_instruction(&mut self, addr: ChipAddr) -> bool {
        let start_addr = usize::from(addr);
        let end_addr = start_addr + usize::from(INSTRUCTION_SIZE_BYTES);
        let mem = self.chip_state.borrow().mem;

        let slice: [u8; 2] = mem[start_addr..end_addr]
            .try_into()
            .unwrap();
        let wordbyte = u16::from_be_bytes(slice);

        self.compile_instruction(wordbyte)
    }

    fn compile_instruction(&mut self, instruction: u16) -> bool {
        debug!("Recompiling '{:#x}'", instruction);
        let nibbles: [u8; 4] = [
            u8::try_from((instruction & 0xf000) >> 12).unwrap(),
            u8::try_from((instruction & 0x0f00) >> 8).unwrap(),
            u8::try_from((instruction & 0x00f0) >> 4).unwrap(),
            u8::try_from(instruction & 0x000f).unwrap(),
        ];

        let x: Vx = Vx(nibbles[1]);
        let y: Vy = Vy(nibbles[2]);
        let kk: Byte = Byte(u8::try_from(instruction & 0x00ff).unwrap());
        let nnn: Addr = Addr(instruction & 0x0fff);
        match (nibbles[0], nibbles[1], nibbles[2], nibbles[3]) {
            (0x0, 0x0, 0xe, 0x0) => self.cls(),
            (0x0, 0x0, 0xe, 0xe) => self.ret(),
            (0x0, _, _, _) => self.sys(nnn),
            (0x1, _, _, _) => self.jp(nnn),
            (0x2, _, _, _) => self.call(nnn),
            (0x3, _, _, _) => self.se(x, kk),
            (0x4, _, _, _) => self.sne(x, kk),
            (0x5, _, _, _) => self.se(x, y),
            (0x6, _, _, _) => self.ld(x, kk),
            (0x7, _, _, _) => self.add(x, kk),
            (0x8, _, _, 0) => self.ld(x, y),
            (0x8, _, _, 1) => self.or(x, y),
            (0x8, _, _, 2) => self.and(x, y),
            (0x8, _, _, 3) => self.xor(x, y),
            (0x8, _, _, 4) => self.add(x, y),
            (0x8, _, _, 5) => self.sub(x, y),
            (0x8, _, _, 6) => self.shr(x, y),
            (0x8, _, _, 7) => self.subn(x, y),
            (0x8, _, _, 0xe) => self.shl(x, y),
            (0x9, _, _, 0) => self.sne(x, y),
            (0xa, _, _, _) => self.ld_i(nnn),
            (0xb, _, _, _) => self.ld_v0(nnn),
            (0xc, _, _, _) => self.rnd(x, kk),
            (0xd, _, _, nibble) => self.drw(x, y, nibble),
            (0xe, _, 0x9, 0xe) => self.skp(x),
            (0xe, _, 0xa, 0x1) => self.sknp(x),
            (0xf, _, 0x0, 0x7) => self.ld_x_dt(x),
            (0xf, _, 0x0, 0xa) => self.ld_k(x),
            (0xf, _, 0x1, 0x5) => self.ld_dt_x(x),
            (0xf, _, 0x1, 0x8) => self.ld_st(x),
            (0xf, _, 0x1, 0xe) => self.add_i(x),
            (0xf, _, 0x2, 0x9) => self.ld_f(x),
            (0xf, _, 0x3, 0x3) => self.ld_b(x),
            (0xf, _, 0x5, 0x5) => self.ld_i_x(x),
            (0xf, _, 0x6, 0x5) => self.ld_x_i(x),
            _ => unreachable!("Reached unknown instruction: {:#x}", instruction),
        }
    }

    fn get_field_offset(&self, field: Chip8Field) -> usize {
        let state_addr = &self.chip_state.borrow().deine_mudda as *const u32 as usize;

        let field_addr = match field {
            Chip8Field::I => &self.chip_state.borrow().i as *const u16 as usize,
            Chip8Field::PC => &self.chip_state.borrow().pc as *const u16 as usize,
            Chip8Field::SP => &self.chip_state.borrow().sp as *const u8 as usize,
            Chip8Field::Stack => &self.chip_state.borrow().stack as *const u16 as usize,
            Chip8Field::Reg(index) => self
                .chip_state
                .borrow()
                .regs
                .get(usize::from(index))
                .unwrap() as *const u8 as usize,
            Chip8Field::Delay => &self.chip_state.borrow().delay as *const u8 as usize,
            Chip8Field::Sound => &self.chip_state.borrow().sound as *const u8 as usize,
        };

        field_addr - state_addr
    }
}

#[cfg(test)]
mod tests {
    use std::{rc::Rc, cell::RefCell};

    use crate::chip8::{Chip8State, Chip8, Chip8Field};

    use super::JIT;

    #[test]
    fn test_offset_mem() {
        let state = Rc::new(RefCell::new(Chip8State::default()));
        let jit = JIT::new(state);

        assert_eq!(jit.get_field_offset(Chip8Field::PC), Chip8::MEM_SIZE + Chip8::AMOUNT_REGISTERS + 2 + 1 + 1);
    }
}
