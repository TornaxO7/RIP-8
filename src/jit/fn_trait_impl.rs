use crate::chip8::{Chip8Field, INSTRUCTION_SIZE_BYTES};

use super::{
    fn_traits::{ArgAdd, ArgLd, ArgSe, ArgSne},
    Byte, InstructionResult, Vx, Vy, JIT,
};

use iced_x86::code_asm::*;

impl ArgSe<Byte> for JIT {
    fn se(&mut self, vx: Vx, arg2: Byte) -> InstructionResult {
        let vx_addr = self.get_field_addr(Chip8Field::Reg(vx.0));
        let pc_addr = self.get_field_addr(Chip8Field::PC);

        self.x86.mov(r9, ptr(pc_addr))?;
        // prepare `pc + 2`
        self.x86.mov(r8, ptr(pc_addr))?;
        self.x86.add(r8, i32::from(INSTRUCTION_SIZE_BYTES))?;

        // cmp vx, kk
        self.x86.mov(rax, ptr(vx_addr))?;
        self.x86.cmp(rax, i32::from(arg2.0))?;

        // set pc if vx == kk (update r9 if needed)
        self.x86.cmove(r9, r8)?;

        self.x86.mov(ptr(pc_addr), r9)?;

        Ok(false)
    }
}

impl ArgSe<Vy> for JIT {
    fn se(&mut self, vx: Vx, arg2: Vy) -> InstructionResult {
        let vx_addr = self.get_field_addr(Chip8Field::Reg(vx.0));
        let vy_addr = self.get_field_addr(Chip8Field::Reg(arg2.0));
        let pc_addr = self.get_field_addr(Chip8Field::PC);

        // store vx and vy in registers
        self.x86.mov(r10, ptr(vx_addr))?;
        self.x86.mov(r11, ptr(vy_addr))?;

        self.x86.mov(r9, ptr(pc_addr))?;
        // prepare `pc + 2`
        self.x86.mov(r8, ptr(pc_addr))?;
        self.x86.add(r8, i32::from(INSTRUCTION_SIZE_BYTES))?;

        // cmp vx, vy
        self.x86.cmp(r10, r11)?;

        // set pc if vx == kk (update r9 if needed)
        self.x86.cmove(r9, r8)?;

        self.x86.mov(ptr(pc_addr), r9)?;

        Ok(false)
    }
}

impl ArgSne<Byte> for JIT {
    fn sne(&mut self, vx: Vx, arg2: Byte) -> InstructionResult {
        let vx_addr = self.get_field_addr(Chip8Field::Reg(vx.0));
        let pc_addr = self.get_field_addr(Chip8Field::PC);

        self.x86.mov(r9, ptr(pc_addr))?;
        // prepare `pc + 2`
        self.x86.mov(r8, ptr(pc_addr))?;
        self.x86.add(r8, i32::from(INSTRUCTION_SIZE_BYTES))?;

        // cmp vx, kk
        self.x86.mov(rax, ptr(vx_addr))?;
        self.x86.cmp(rax, i32::from(arg2.0))?;

        // set pc if vx != kk (update r9 if needed)
        self.x86.cmovne(r9, r8)?;

        self.x86.mov(ptr(pc_addr), r9)?;

        Ok(false)
    }
}

impl ArgSne<Vy> for JIT {
    // IDEA: almost the same as `se` maybe putting the same lines together?
    fn sne(&mut self, vx: Vx, arg2: Vy) -> InstructionResult {
        let vx_addr = self.get_field_addr(Chip8Field::Reg(vx.0));
        let vy_addr = self.get_field_addr(Chip8Field::Reg(arg2.0));
        let pc_addr = self.get_field_addr(Chip8Field::PC);

        // store vx and vy in registers
        self.x86.mov(r10, ptr(vx_addr))?;
        self.x86.mov(r11, ptr(vy_addr))?;

        self.x86.mov(r9, ptr(pc_addr))?;
        // prepare `pc + 2`
        self.x86.mov(r8, ptr(pc_addr))?;
        self.x86.add(r8, i32::from(INSTRUCTION_SIZE_BYTES))?;

        // cmp vx, vy
        self.x86.cmp(r10, r11)?;

        // set pc if vx != kk (update r9 if needed)
        self.x86.cmovne(r9, r8)?;

        self.x86.mov(ptr(pc_addr), r9)?;

        Ok(false)
    }
}

impl ArgLd<Byte> for JIT {
    fn ld(&mut self, vx: Vx, arg2: Byte) -> InstructionResult {
        let vx_addr = self.get_field_addr(Chip8Field::Reg(vx.0));

        self.x86.mov(rax, u64::from(arg2.0))?;
        self.x86.mov(ptr(vx_addr), rax)?;

        Ok(true)
    }
}

impl ArgLd<Vy> for JIT {
    fn ld(&mut self, vx: Vx, arg2: Vy) -> InstructionResult {
        let vx_addr = self.get_field_addr(Chip8Field::Reg(vx.0));
        let vy_addr = self.get_field_addr(Chip8Field::Reg(arg2.0));

        self.x86.mov(rax, ptr(vy_addr))?;
        self.x86.mov(ptr(vx_addr), rax)?;

        Ok(true)
    }
}

impl ArgAdd<Byte> for JIT {
    fn add(&mut self, vx: Vx, arg2: Byte) -> InstructionResult {
        let vx_addr = self.get_field_addr(Chip8Field::Reg(vx.0));

        self.x86.mov(rax, ptr(vx_addr))?;
        self.x86.add(rax, i32::from(arg2.0))?;
        self.x86.mov(ptr(vx_addr), rax)?;

        Ok(true)
    }
}

impl ArgAdd<Vy> for JIT {
    fn add(&mut self, vx: Vx, arg2: Vy) -> InstructionResult {
        let vx_addr = self.get_field_addr(Chip8Field::Reg(vx.0));
        let vy_addr = self.get_field_addr(Chip8Field::Reg(arg2.0));
        let vf_addr = self.get_field_addr(Chip8Field::Reg(0xf));

        // add Vx, Vy
        self.x86.mov(r8, ptr(vx_addr))?;
        self.x86.mov(r9, ptr(vy_addr))?;
        self.x86.add(r8, r9)?;

        // set Vf
        self.x86.setc(ptr(vf_addr))?;

        Ok(true)
    }
}
