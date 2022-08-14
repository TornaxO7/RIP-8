use crate::chip8::{Chip8Field, INSTRUCTION_SIZE};

use super::{
    fn_traits::{ArgAdd, ArgLd, ArgSe, ArgSne},
    Addr, Byte, InstructionResult, Vx, Vy, JIT,
};

use iced_x86::code_asm::*;

impl JIT {
    pub fn cls(&mut self) -> InstructionResult {
        todo!()
    }

    pub fn ret(&mut self) -> InstructionResult {
        let sp_addr = self.get_state_field(Chip8Field::SP);
        let sp_state = usize::from(self.chip_state.borrow().sp);
        let stack_addr = self.get_state_field(Chip8Field::Stack);
        let pc_addr = self.get_state_field(Chip8Field::PC);

        // set pc to top value of stack
        self.x86.mov(
            rdi,
            ptr(stack_addr) + sp_state * usize::from(INSTRUCTION_SIZE),
        )?;
        self.x86.mov(ptr(pc_addr), rdi)?;

        // decrement sp
        self.x86.dec(ptr(sp_addr))?;
        Ok(true)
    }

    pub fn sys(&mut self, addr: Addr) -> InstructionResult {
        // our interpreter is a chad (modern), so we ignore this one
        Ok(false)
    }

    pub fn jp(&mut self, addr: Addr) -> InstructionResult {
        let pc_addr = self.get_state_field(Chip8Field::PC);
        self.x86.mov(ptr(pc_addr), u32::from(addr.0))?;
        Ok(true)
    }

    pub fn call(&mut self, addr: Addr) -> InstructionResult {
        let sp_addr = self.get_state_field(Chip8Field::SP);
        let sp_state = usize::from(self.chip_state.borrow().sp);
        let pc_addr = self.get_state_field(Chip8Field::PC);
        let stack_addr = self.get_state_field(Chip8Field::Stack);

        // increment stack pointer
        self.x86.inc(ptr(sp_addr))?;

        // put current pc on top of stack
        self.x86.mov(rdi, ptr(pc_addr))?;
        self.x86.mov(
            ptr(stack_addr) + sp_state * usize::from(INSTRUCTION_SIZE),
            rdi,
        )?;

        // set pc to `addr`
        self.x86.mov(ptr(pc_addr), u32::from(addr.0))?;
        Ok(true)
    }

    pub fn se<T>(&mut self, vx: Vx, arg2: T) -> InstructionResult
    where
        Self: ArgSe<T>,
    {
        <Self as ArgSe<T>>::se(self, vx, arg2)?;

        Ok(false)
    }

    pub fn sne<T>(&mut self, vx: Vx, arg2: T) -> InstructionResult
    where
        Self: ArgSne<T>,
    {
        <Self as ArgSne<T>>::sne(self, vx, arg2)?;
        Ok(false)
    }

    pub fn ld<T>(&mut self, vx: Vx, arg2: T) -> InstructionResult
    where
        Self: ArgLd<T>,
    {
        <Self as ArgLd<T>>::ld(self, vx, arg2)?;
        Ok(true)
    }

    pub fn add<T>(&mut self, vx: Vx, arg2: T) -> InstructionResult
    where
        Self: ArgAdd<T>,
    {
        <Self as ArgAdd<T>>::add(self, vx, arg2)?;
        Ok(true)
    }

    pub fn or(&mut self, vx: Vx, vy: Vy) -> InstructionResult {
        let vx_addr = self.get_state_field(Chip8Field::Reg(vx.0));
        let vy_addr = self.get_state_field(Chip8Field::Reg(vy.0));

        // do bitwise or
        self.x86.mov(rax, ptr(vx_addr))?;
        self.x86.or(rax, ptr(vy_addr))?;
        self.x86.mov(ptr(vx_addr), rax)?;

        Ok(true)
    }

    pub fn and(&mut self, vx: Vx, vy: Vy) -> InstructionResult {
        let vx_addr = self.get_state_field(Chip8Field::Reg(vx.0));
        let vy_addr = self.get_state_field(Chip8Field::Reg(vy.0));

        // do bitwise and
        self.x86.mov(rax, ptr(vx_addr))?;
        self.x86.and(rax, ptr(vy_addr))?;
        self.x86.mov(ptr(vx_addr), rax)?;
        Ok(true)
    }

    pub fn xor(&mut self, vx: Vx, vy: Vy) -> InstructionResult {
        let vx_addr = self.get_state_field(Chip8Field::Reg(vx.0));
        let vy_addr = self.get_state_field(Chip8Field::Reg(vy.0));

        // do bitwise and
        self.x86.mov(rax, ptr(vx_addr))?;
        self.x86.xor(rax, ptr(vy_addr))?;
        self.x86.mov(ptr(vx_addr), rax)?;
        Ok(true)
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
