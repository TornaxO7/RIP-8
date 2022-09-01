use crate::chip8::{Chip8Field, INSTRUCTION_SIZE_BYTES};

use super::{
    fn_traits::{ArgLd, ArgSe, ArgSne},
    Vx, Vy, JIT, Byte,
};

use iced_x86::code_asm::*;
use log::debug;

impl ArgSe<Byte> for JIT {
    fn se(&mut self, vx: Vx, arg2: Byte) {
        debug!("--> SE V{:X}, {:#X}", vx.0, arg2.0);
        let vx_offset = self.get_field_offset(Chip8Field::Reg(vx.0));
        let pc_offset = self.get_field_offset(Chip8Field::PC);

        // get pc address
        self.x86.mov(r8, pc_offset).unwrap();
        self.x86.add(r8, rdi).unwrap();

        // prepare `pc + 2`
        self.x86.mov(r11, INSTRUCTION_SIZE_BYTES).unwrap();
        self.x86.mov(r9, qword_ptr(r8)).unwrap();
        self.x86.add(r9, r11).unwrap();

        // get vx address
        self.x86.mov(r10, qword_ptr(vx_offset)).unwrap();
        self.x86.add(r10, rdi).unwrap();

        // cmp vx, kk
        self.x86.mov(r11, u64::from(arg2.0)).unwrap();
        self.x86.cmp(r10, r11).unwrap();

        // set pc if vx == kk (update r8 if needed)
        self.x86.cmove(r10, r9).unwrap();
        self.x86.cmovne(r10, qword_ptr(r8)).unwrap();

        self.x86.mov(qword_ptr(r8), r10).unwrap();
    }
}

impl ArgSe<Vy> for JIT {
    fn se(&mut self, vx: Vx, arg2: Vy) {
        debug!("--> SE V{:X}, V{:X}", vx.0, arg2.0);
        let vx_value = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));
        let vy_value = rdi + self.get_field_offset(Chip8Field::Reg(arg2.0));
        let pc_offset = self.get_field_offset(Chip8Field::PC);

        // store vx and vy in registers
        self.x86.mov(r8, vx_value).unwrap();
        self.x86.mov(r9, vy_value).unwrap();

        // get pc address
        self.x86.mov(r10, pc_offset).unwrap();
        self.x86.add(r10, rdi).unwrap();

        // prepare `pc + 2`
        self.x86.mov(r12, INSTRUCTION_SIZE_BYTES).unwrap();
        self.x86.mov(r11, qword_ptr(r10)).unwrap();
        self.x86.add(r11, r12).unwrap();

        // cmp vx, vy
        self.x86.cmp(r8, r9).unwrap();

        // set pc if vx == kk (update r10 if needed)
        self.x86.cmove(r12, r11).unwrap();
        self.x86.cmovne(r12, qword_ptr(r10)).unwrap();

        self.x86.mov(qword_ptr(r10), r12).unwrap();
    }
}

impl ArgSne<Byte> for JIT {
    fn sne(&mut self, vx: Vx, arg2: Byte) {
        debug!("--> SNE V{:X}, {:#X}", vx.0, arg2.0);
        let vx_value = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));
        let pc_offset = self.get_field_offset(Chip8Field::PC);

        // get pc address
        self.x86.mov(r8, pc_offset).unwrap();
        self.x86.add(r8, rdi).unwrap();

        // prepare `pc + 2`
        self.x86.mov(r11, INSTRUCTION_SIZE_BYTES).unwrap();
        self.x86.mov(r9, qword_ptr(r8)).unwrap();
        self.x86.add(r9, r11).unwrap();

        // cmp vx, kk
        self.x86.mov(r11, u64::from(arg2.0)).unwrap();
        self.x86.mov(r10, vx_value).unwrap();
        self.x86.cmp(r10, r11).unwrap();

        // set pc if vx != kk (update r8 if needed)
        self.x86.cmovne(r12, r9).unwrap();
        self.x86.cmove(r12, qword_ptr(r8)).unwrap();

        self.x86.mov(qword_ptr(r8), r12).unwrap();
    }
}

impl ArgSne<Vy> for JIT {
    // IDEA: r8most the same as `se` maybe putting the same lines together.unwrap()
    fn sne(&mut self, vx: Vx, arg2: Vy) {
        debug!("--> SNE V{:X}, V{:X}", vx.0, arg2.0);
        let vx_value = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));
        let vy_value = rdi + self.get_field_offset(Chip8Field::Reg(arg2.0));
        let pc_offset = self.get_field_offset(Chip8Field::PC);

        // store vx and vy in registers
        self.x86.mov(r8, vx_value).unwrap();
        self.x86.mov(r9, vy_value).unwrap();

        self.x86.mov(r10, pc_offset).unwrap();
        self.x86.add(r10, rdi).unwrap();

        // prepare `pc + 2`
        self.x86.mov(r12, INSTRUCTION_SIZE_BYTES).unwrap();
        self.x86.mov(r11, qword_ptr(r10)).unwrap();
        self.x86.add(r11, r12).unwrap();

        // cmp vx, vy
        self.x86.cmp(r8, r9).unwrap();

        // set pc if vx != kk (update r10 if needed)
        self.x86.cmovne(r12, r11).unwrap();
        self.x86.cmove(r12, qword_ptr(r10)).unwrap();

        self.x86.mov(qword_ptr(r10), r12).unwrap();
    }
}

impl ArgLd<Byte> for JIT {
    fn ld(&mut self, vx: Vx, arg2: Byte) {
        debug!("--> LD V{:X}, {:#X}", vx.0, arg2.0);
        let vx_offset = self.get_field_offset(Chip8Field::Reg(vx.0));

        // calculate address of `vx`
        self.x86.mov(r8, vx_offset).unwrap();
        self.x86.add(r8, rdi).unwrap();

        // state.regs[vx] = arg2;
        self.x86.mov(r9, u64::from(arg2.0)).unwrap();
        self.x86.mov(qword_ptr(r8), r9).unwrap();
    }
}

impl ArgLd<Vy> for JIT {
    fn ld(&mut self, vx: Vx, arg2: Vy) {
        debug!("--> LD V{:X}, V{:X}", vx.0, arg2.0);
        let vx_offset = self.get_field_offset(Chip8Field::Reg(vx.0));
        let vy_offset = self.get_field_offset(Chip8Field::Reg(arg2.0));

        // get vx address
        self.x86.mov(r8, vx_offset).unwrap();
        self.x86.add(r8, rdi).unwrap();

        // get vy address
        self.x86.mov(r9, vy_offset).unwrap();
        self.x86.add(r9, rdi).unwrap();

        // actual move
        self.x86.mov(r10, qword_ptr(r9)).unwrap();
        self.x86.mov(qword_ptr(r8), r10).unwrap();
    }
}
