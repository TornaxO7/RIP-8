use super::{
    fn_traits::{ArgAdd, ArgLd, ArgSe, ArgSne},
    Addr, Byte, InstructionResult, Vx, Vy, JIT,
};

use iced_x86::code_asm::*;

impl JIT<'_> {
    pub fn cls(&mut self) -> InstructionResult {
        todo!()
    }

    pub fn ret(&mut self) -> InstructionResult {
        self.x86.pop(ptr(self.get_pc_state_address()))?;

        self.x86.sub(rsp, 1)?;
        Ok(true)
    }

    pub fn sys(&mut self, addr: Addr) -> InstructionResult {
        self.x86.mov(ptr(self.get_pc_state_address()), u32::from(addr.0))?;
        Ok(true)
    }

    pub fn jp(&mut self, addr: Addr) -> InstructionResult {
        self.x86.mov(ptr(self.get_pc_state_address()), u32::from(addr.0))?;
        Ok(true)
    }

    pub fn call(&mut self, addr: Addr) -> InstructionResult {
        todo!()
    }

    pub fn se<T>(&mut self, vx: Vx, arg2: T) -> InstructionResult
    where
        Self: ArgSe<T>,
    {
        todo!()
    }

    pub fn sne<T>(&mut self, vx: Vx, arg2: T) -> InstructionResult
    where
        Self: ArgSne<T>,
    {
        todo!()
    }

    pub fn ld<T>(&mut self, vx: Vx, arg2: T) -> InstructionResult
    where
        Self: ArgLd<T>,
    {
        todo!()
    }

    pub fn add<T>(&mut self, vx: Vx, arg2: T) -> InstructionResult
    where
        Self: ArgAdd<T>,
    {
        todo!()
    }

    pub fn or(&mut self, vx: Vx, vy: Vy) -> InstructionResult {
        todo!()
    }

    pub fn and(&mut self, vx: Vx, vy: Vy) -> InstructionResult {
        todo!()
    }

    pub fn xor(&mut self, vx: Vx, vy: Vy) -> InstructionResult {
        todo!()
    }

    pub fn sub(&mut self, vx: Vx, vy: Vy) -> InstructionResult {
        todo!()
    }

    pub fn shr(&mut self, vx: Vx, vy: Vy) -> InstructionResult {
        todo!()
    }

    pub fn subn(&mut self, vx: Vx, vy: Vy) -> InstructionResult {
        todo!()
    }

    pub fn shl(&mut self, vx: Vx, vy: Vy) -> InstructionResult {
        todo!()
    }

    pub fn ld_i(&mut self, addr: Addr) -> InstructionResult {
        todo!()
    }

    pub fn ld_v0(&mut self, addr: Addr) -> InstructionResult {
        todo!()
    }

    pub fn rnd(&mut self, vx: Vx, kk: Byte) -> InstructionResult {
        todo!()
    }

    pub fn drw(&mut self, vx: Vx, vy: Vy, nibble: u8) -> InstructionResult {
        todo!()
    }

    pub fn skp(&mut self, vx: Vx) -> InstructionResult {
        todo!()
    }

    pub fn sknp(&mut self, vx: Vx) -> InstructionResult {
        todo!()
    }

    pub fn ld_x_dt(&mut self, vx: Vx) -> InstructionResult {
        todo!()
    }

    pub fn ld_k(&mut self, vx: Vx) -> InstructionResult {
        todo!()
    }

    pub fn ld_dt_x(&mut self, vx: Vx) -> InstructionResult {
        todo!()
    }

    pub fn ld_st(&mut self, vx: Vx) -> InstructionResult {
        todo!()
    }

    pub fn add_i(&mut self, vx: Vx) -> InstructionResult {
        todo!()
    }

    pub fn ld_f(&mut self, vx: Vx) -> InstructionResult {
        todo!()
    }

    pub fn ld_b(&mut self, vx: Vx) -> InstructionResult {
        todo!()
    }

    pub fn ld_i_x(&mut self, vx: Vx) -> InstructionResult {
        todo!()
    }

    pub fn ld_x_i(&mut self, vx: Vx) -> InstructionResult {
        todo!()
    }
}
