use crate::chip8::{Chip8Field, INSTRUCTION_SIZE_BYTES};

use super::{
    fn_traits::{ArgAdd, ArgLd, ArgSe, ArgSne},
    Byte, Vx, Vy, JIT,
};

use iced_x86::code_asm::*;

impl ArgSe<Byte> for JIT {
    fn se(&mut self, vx: Vx, arg2: Byte) -> bool {
        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));
        let pc_addr = rdi + self.get_field_offset(Chip8Field::PC);

        self.x86.mov(r9, ptr(pc_addr)).unwrap();
        // prepare `pc + 2`
        self.x86.mov(r8, ptr(pc_addr)).unwrap();
        self.x86.add(r8, i32::from(INSTRUCTION_SIZE_BYTES)).unwrap();

        // cmp vx, kk
        self.x86.mov(r10, ptr(vx_addr)).unwrap();
        self.x86.cmp(r10, i32::from(arg2.0)).unwrap();

        // set pc if vx == kk (update r9 if needed)
        self.x86.cmove(r9, r8).unwrap();

        self.x86.mov(ptr(pc_addr), r9).unwrap();

        false
    }
}

impl ArgSe<Vy> for JIT {
    fn se(&mut self, vx: Vx, arg2: Vy) -> bool {
        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));
        let vy_addr = rdi + self.get_field_offset(Chip8Field::Reg(arg2.0));
        let pc_addr = rdi + self.get_field_offset(Chip8Field::PC);

        // store vx and vy in registers
        self.x86.mov(r10, ptr(vx_addr)).unwrap();
        self.x86.mov(r11, ptr(vy_addr)).unwrap();

        self.x86.mov(r9, ptr(pc_addr)).unwrap();
        // prepare `pc + 2`
        self.x86.mov(r8, ptr(pc_addr)).unwrap();
        self.x86.add(r8, i32::from(INSTRUCTION_SIZE_BYTES)).unwrap();

        // cmp vx, vy
        self.x86.cmp(r10, r11).unwrap();

        // set pc if vx == kk (update r9 if needed)
        self.x86.cmove(r9, r8).unwrap();

        self.x86.mov(ptr(pc_addr), r9).unwrap();

        false
    }
}

impl ArgSne<Byte> for JIT {
    fn sne(&mut self, vx: Vx, arg2: Byte) -> bool {
        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));
        let pc_addr = rdi + self.get_field_offset(Chip8Field::PC);

        self.x86.mov(r9, ptr(pc_addr)).unwrap();
        // prepare `pc + 2`
        self.x86.mov(r8, ptr(pc_addr)).unwrap();
        self.x86.add(r8, i32::from(INSTRUCTION_SIZE_BYTES)).unwrap();

        // cmp vx, kk
        self.x86.mov(r10, ptr(vx_addr)).unwrap();
        self.x86.cmp(r10, i32::from(arg2.0)).unwrap();

        // set pc if vx != kk (update r9 if needed)
        self.x86.cmovne(r9, r8).unwrap();

        self.x86.mov(ptr(pc_addr), r9).unwrap();

        false
    }
}

impl ArgSne<Vy> for JIT {
    // IDEA: almost the same as `se` maybe putting the same lines together.unwrap()
    fn sne(&mut self, vx: Vx, arg2: Vy) -> bool {
        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));
        let vy_addr = rdi + self.get_field_offset(Chip8Field::Reg(arg2.0));
        let pc_addr = rdi + self.get_field_offset(Chip8Field::PC);

        // store vx and vy in registers
        self.x86.mov(r10, ptr(vx_addr)).unwrap();
        self.x86.mov(r11, ptr(vy_addr)).unwrap();

        self.x86.mov(r9, ptr(pc_addr)).unwrap();
        // prepare `pc + 2`
        self.x86.mov(r8, ptr(pc_addr)).unwrap();
        self.x86.add(r8, i32::from(INSTRUCTION_SIZE_BYTES)).unwrap();

        // cmp vx, vy
        self.x86.cmp(r10, r11).unwrap();

        // set pc if vx != kk (update r9 if needed)
        self.x86.cmovne(r9, r8).unwrap();

        self.x86.mov(ptr(pc_addr), r9).unwrap();

        false
    }
}

impl ArgLd<Byte> for JIT {
    fn ld(&mut self, vx: Vx, arg2: Byte) -> bool {
        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));

        self.x86.mov(r8, u64::from(arg2.0)).unwrap();
        self.x86.mov(ptr(vx_addr), r8).unwrap();

        true
    }
}

impl ArgLd<Vy> for JIT {
    fn ld(&mut self, vx: Vx, arg2: Vy) -> bool {
        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));
        let vy_addr = rdi + self.get_field_offset(Chip8Field::Reg(arg2.0));

        self.x86.mov(r8, ptr(vy_addr)).unwrap();
        self.x86.mov(ptr(vx_addr), r8).unwrap();

        true
    }
}

impl ArgAdd<Byte> for JIT {
    fn add(&mut self, vx: Vx, arg2: Byte) -> bool {
        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));

        self.x86.mov(r8, ptr(vx_addr)).unwrap();
        self.x86.add(r8, i32::from(arg2.0)).unwrap();
        self.x86.mov(ptr(vx_addr), r8).unwrap();

        true
    }
}

impl ArgAdd<Vy> for JIT {
    fn add(&mut self, vx: Vx, arg2: Vy) -> bool {
        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));
        let vy_addr = rdi + self.get_field_offset(Chip8Field::Reg(arg2.0));
        let vf_addr = rdi + self.get_field_offset(Chip8Field::Reg(0xf));

        // add Vx, Vy
        self.x86.mov(r8, ptr(vx_addr)).unwrap();
        self.x86.mov(r9, ptr(vy_addr)).unwrap();
        self.x86.add(r8, r9).unwrap();

        // set Vf
        self.x86.setc(ptr(vf_addr)).unwrap();

        true
    }
}
