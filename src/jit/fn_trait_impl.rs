use crate::chip8::{Chip8Field, INSTRUCTION_SIZE_BYTES};

use super::{
    fn_traits::{ArgAdd, ArgLd, ArgSe, ArgSne},
    Byte, Vx, Vy, JIT,
};

use iced_x86::code_asm::*;

impl ArgSe<Byte> for JIT {
    fn se(&mut self, vx: Vx, arg2: Byte) -> bool {
        let vx_addr = di + self.get_field_offset(Chip8Field::Reg(vx.0));
        let pc_addr = di + self.get_field_offset(Chip8Field::PC);

        self.x86.mov(ax, word_ptr(pc_addr)).unwrap();
        // prepare `pc + 2`
        self.x86.mov(bx, word_ptr(pc_addr)).unwrap();
        self.x86.add(bx, i32::from(INSTRUCTION_SIZE_BYTES)).unwrap();

        // cmp vx, kk
        self.x86.mov(cl, byte_ptr(vx_addr)).unwrap();
        self.x86.cmp(cl, i32::from(arg2.0)).unwrap();

        // set pc if vx == kk (update ax if needed)
        self.x86.cmove(ax, bx).unwrap();

        self.x86.mov(word_ptr(pc_addr), ax).unwrap();

        false
    }
}

impl ArgSe<Vy> for JIT {
    fn se(&mut self, vx: Vx, arg2: Vy) -> bool {
        let vx_addr = di + self.get_field_offset(Chip8Field::Reg(vx.0));
        let vy_addr = di + self.get_field_offset(Chip8Field::Reg(arg2.0));
        let pc_addr = di + self.get_field_offset(Chip8Field::PC);

        // store vx and vy in registers
        self.x86.mov(al, byte_ptr(vx_addr)).unwrap();
        self.x86.mov(bl, byte_ptr(vy_addr)).unwrap();

        self.x86.mov(cx, word_ptr(pc_addr)).unwrap();
        // prepare `pc + 2`
        self.x86.mov(dx, word_ptr(pc_addr)).unwrap();
        self.x86.add(dx, u32::from(INSTRUCTION_SIZE_BYTES)).unwrap();

        // cmp vx, vy
        self.x86.cmp(al, bl).unwrap();

        // set pc if vx == kk (update cx if needed)
        self.x86.cmove(cx, dx).unwrap();

        self.x86.mov(word_ptr(pc_addr), cx).unwrap();

        false
    }
}

impl ArgSne<Byte> for JIT {
    fn sne(&mut self, vx: Vx, arg2: Byte) -> bool {
        let vx_addr = di + self.get_field_offset(Chip8Field::Reg(vx.0));
        let pc_addr = di + self.get_field_offset(Chip8Field::PC);

        self.x86.mov(ax, word_ptr(pc_addr)).unwrap();
        // prepare `pc + 2`
        self.x86.mov(bx, word_ptr(pc_addr)).unwrap();
        self.x86.add(bx, i32::from(INSTRUCTION_SIZE_BYTES)).unwrap();

        // cmp vx, kk
        self.x86.mov(cx, byte_ptr(vx_addr)).unwrap();
        self.x86.cmp(cx, i32::from(arg2.0)).unwrap();

        // set pc if vx != kk (update ax if needed)
        self.x86.cmovne(ax, bx).unwrap();

        self.x86.mov(word_ptr(pc_addr), ax).unwrap();

        false
    }
}

impl ArgSne<Vy> for JIT {
    // IDEA: almost the same as `se` maybe putting the same lines together.unwrap()
    fn sne(&mut self, vx: Vx, arg2: Vy) -> bool {
        let vx_addr = di + self.get_field_offset(Chip8Field::Reg(vx.0));
        let vy_addr = di + self.get_field_offset(Chip8Field::Reg(arg2.0));
        let pc_addr = di + self.get_field_offset(Chip8Field::PC);

        // store vx and vy in registers
        self.x86.mov(al, byte_ptr(vx_addr)).unwrap();
        self.x86.mov(bl, byte_ptr(vy_addr)).unwrap();

        self.x86.mov(cx, word_ptr(pc_addr)).unwrap();
        // prepare `pc + 2`
        self.x86.mov(dx, word_ptr(pc_addr)).unwrap();
        self.x86.add(dx, i32::from(INSTRUCTION_SIZE_BYTES)).unwrap();

        // cmp vx, vy
        self.x86.cmp(al, bl).unwrap();

        // set pc if vx != kk (update cx if needed)
        self.x86.cmovne(cx, dx).unwrap();

        self.x86.mov(word_ptr(pc_addr), cx).unwrap();

        false
    }
}

impl ArgLd<Byte> for JIT {
    fn ld(&mut self, vx: Vx, arg2: Byte) -> bool {
        let vx_addr = di + self.get_field_offset(Chip8Field::Reg(vx.0));

        self.x86.mov(al, u32::from(arg2.0)).unwrap();
        self.x86.mov(byte_ptr(vx_addr), al).unwrap();

        true
    }
}

impl ArgLd<Vy> for JIT {
    fn ld(&mut self, vx: Vx, arg2: Vy) -> bool {
        let vx_addr = di + self.get_field_offset(Chip8Field::Reg(vx.0));
        let vy_addr = di + self.get_field_offset(Chip8Field::Reg(arg2.0));

        self.x86.mov(al, byte_ptr(vy_addr)).unwrap();
        self.x86.mov(byte_ptr(vx_addr), al).unwrap();

        true
    }
}

impl ArgAdd<Byte> for JIT {
    fn add(&mut self, vx: Vx, arg2: Byte) -> bool {
        let vx_addr = di + self.get_field_offset(Chip8Field::Reg(vx.0));

        self.x86.mov(al, byte_ptr(vx_addr)).unwrap();
        self.x86.add(al, i32::from(arg2.0)).unwrap();
        self.x86.mov(byte_ptr(vx_addr), al).unwrap();

        true
    }
}

impl ArgAdd<Vy> for JIT {
    fn add(&mut self, vx: Vx, arg2: Vy) -> bool {
        let vx_addr = di + self.get_field_offset(Chip8Field::Reg(vx.0));
        let vy_addr = di + self.get_field_offset(Chip8Field::Reg(arg2.0));
        let vf_addr = di + self.get_field_offset(Chip8Field::Reg(0xf));

        // add Vx, Vy
        self.x86.mov(al, byte_ptr(vx_addr)).unwrap();
        self.x86.mov(bl, byte_ptr(vy_addr)).unwrap();
        self.x86.add(al, bl).unwrap();

        // set Vf
        self.x86.mov(cl, vf_addr).unwrap();
        self.x86.setc(byte_ptr(cl)).unwrap();

        true
    }
}
