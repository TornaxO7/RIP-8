use crate::chip8::{Chip8Field, INSTRUCTION_SIZE_BYTES};

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
        let sp_addr = self.get_field_addr(Chip8Field::SP);
        let stack_addr = self.get_field_addr(Chip8Field::Stack);
        let pc_addr = self.get_field_addr(Chip8Field::PC);

        // set pc to top value of stack
        self.x86.mov(rax, ptr(sp_addr))?;
        self.x86.mov(
            rdi,
            ptr(stack_addr) + rax * usize::from(INSTRUCTION_SIZE_BYTES),
        )?;
        self.x86.mov(ptr(pc_addr), rdi)?;

        // decrement sp
        self.x86.dec(ptr(sp_addr))?;
        Ok(true)
    }

    pub fn sys(&mut self, _: Addr) -> InstructionResult {
        // our interpreter is a (modern) chad, so we ignore this one
        Ok(false)
    }

    pub fn jp(&mut self, addr: Addr) -> InstructionResult {
        let pc_addr = self.get_field_addr(Chip8Field::PC);
        self.x86.mov(ptr(pc_addr), u32::from(addr.0))?;
        Ok(true)
    }

    pub fn call(&mut self, addr: Addr) -> InstructionResult {
        let sp_addr = self.get_field_addr(Chip8Field::SP);
        let sp_state = usize::from(self.chip_state.borrow().sp);
        let pc_addr = self.get_field_addr(Chip8Field::PC);
        let stack_addr = self.get_field_addr(Chip8Field::Stack);

        // increment stack pointer
        self.x86.inc(ptr(sp_addr))?;

        // put current pc on top of stack
        self.x86.mov(rdi, ptr(pc_addr))?;
        self.x86.mov(
            ptr(stack_addr) + sp_state * usize::from(INSTRUCTION_SIZE_BYTES),
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
        let vx_addr = self.get_field_addr(Chip8Field::Reg(vx.0));
        let vy_addr = self.get_field_addr(Chip8Field::Reg(vy.0));

        // do bitwise or
        self.x86.mov(rax, ptr(vx_addr))?;
        self.x86.or(rax, ptr(vy_addr))?;
        self.x86.mov(ptr(vx_addr), rax)?;

        Ok(true)
    }

    pub fn and(&mut self, vx: Vx, vy: Vy) -> InstructionResult {
        let vx_addr = self.get_field_addr(Chip8Field::Reg(vx.0));
        let vy_addr = self.get_field_addr(Chip8Field::Reg(vy.0));

        // do bitwise and
        self.x86.mov(rax, ptr(vx_addr))?;
        self.x86.and(rax, ptr(vy_addr))?;
        self.x86.mov(ptr(vx_addr), rax)?;
        Ok(true)
    }

    pub fn xor(&mut self, vx: Vx, vy: Vy) -> InstructionResult {
        let vx_addr = self.get_field_addr(Chip8Field::Reg(vx.0));
        let vy_addr = self.get_field_addr(Chip8Field::Reg(vy.0));

        // do bitwise and
        self.x86.mov(rax, ptr(vx_addr))?;
        self.x86.xor(rax, ptr(vy_addr))?;
        self.x86.mov(ptr(vx_addr), rax)?;
        Ok(true)
    }

    pub fn sub(&mut self, vx: Vx, vy: Vy) -> InstructionResult {
        let vx_addr = self.get_field_addr(Chip8Field::Reg(vx.0));
        let vy_addr = self.get_field_addr(Chip8Field::Reg(vy.0));
        let vf_addr = self.get_field_addr(Chip8Field::Reg(0xf));

        // sub
        self.x86.mov(rax, ptr(vx_addr))?;
        self.x86.mov(rdi, ptr(vy_addr))?;
        self.x86.sub(rax, rdi)?;

        // set Vf
        self.x86.setnc(ptr(vf_addr))?;

        Ok(true)
    }

    pub fn shr(&mut self, vx: Vx, _: Vy) -> InstructionResult {
        let vx_addr = self.get_field_addr(Chip8Field::Reg(vx.0));
        let vf_addr = self.get_field_addr(Chip8Field::Reg(0xf));

        // set vf
        self.x86.mov(rax, ptr(vx_addr))?;
        self.x86.shr(rax, 1u32)?;
        self.x86.setb(ptr(vf_addr))?;

        // save shr
        self.x86.mov(ptr(vx_addr), rax)?;

        Ok(true)
    }

    pub fn subn(&mut self, vx: Vx, vy: Vy) -> InstructionResult {
        let vx_addr = self.get_field_addr(Chip8Field::Reg(vx.0));
        let vy_addr = self.get_field_addr(Chip8Field::Reg(vy.0));
        let vf_addr = self.get_field_addr(Chip8Field::Reg(0xf));

        // sub Vy, Vx
        self.x86.mov(rax, ptr(vx_addr))?;
        self.x86.mov(rdi, ptr(vy_addr))?;
        self.x86.sub(rdi, rax)?;

        // set vf
        self.x86.setnc(ptr(vf_addr))?;

        self.x86.mov(ptr(vx_addr), rdi)?;

        Ok(true)
    }

    pub fn shl(&mut self, vx: Vx, _: Vy) -> InstructionResult {
        let vx_addr = self.get_field_addr(Chip8Field::Reg(vx.0));
        let vf_addr = self.get_field_addr(Chip8Field::Reg(0xf));

        // set vf
        self.x86.mov(rax, ptr(vx_addr))?;
        self.x86.shl(rax, 1u32)?;
        self.x86.setb(ptr(vf_addr))?;

        // save shl
        self.x86.mov(ptr(vx_addr), rax)?;

        Ok(true)
    }

    pub fn ld_i(&mut self, addr: Addr) -> InstructionResult {
        todo!()
    }

    pub fn ld_v0(&mut self, addr: Addr) -> InstructionResult {
        let i_addr = self.get_field_addr(Chip8Field::I);

        self.x86.mov(rax, u64::from(addr.0))?;
        self.x86.mov(ptr(i_addr), rax)?;

        Ok(true)
    }

    pub fn rnd(&mut self, vx: Vx, kk: Byte) -> InstructionResult {
        let vx_addr = self.get_field_addr(Chip8Field::Reg(vx.0));

        self.x86.rdrand(rax)?;
        self.x86.and(rax, i32::from(kk.0))?;
        self.x86.mov(ptr(vx_addr), rax)?;

        Ok(true)
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
        let vx_addr = self.get_field_addr(Chip8Field::Reg(vx.0));
        let delay_timer_addr = self.get_field_addr(Chip8Field::Delay);

        self.x86.mov(rax, ptr(delay_timer_addr))?;
        self.x86.mov(ptr(vx_addr), rax)?;

        Ok(true)
    }

    pub fn ld_k(&mut self, vx: Vx) -> InstructionResult {
        todo!()
    }

    pub fn ld_dt_x(&mut self, vx: Vx) -> InstructionResult {
        let vx_addr = self.get_field_addr(Chip8Field::Reg(vx.0));
        let delay_timer_addr = self.get_field_addr(Chip8Field::Delay);

        self.x86.mov(rax, ptr(vx_addr))?;
        self.x86.mov(ptr(delay_timer_addr), rax)?;

        Ok(true)
    }

    pub fn ld_st(&mut self, vx: Vx) -> InstructionResult {
        let vx_addr = self.get_field_addr(Chip8Field::Reg(vx.0));
        let sound_addr = self.get_field_addr(Chip8Field::Sound);

        self.x86.mov(rax, ptr(vx_addr))?;
        self.x86.mov(ptr(sound_addr), rax)?;

        Ok(true)
    }

    pub fn add_i(&mut self, vx: Vx) -> InstructionResult {
        let i_addr = self.get_field_addr(Chip8Field::I);
        let vx_addr = self.get_field_addr(Chip8Field::Reg(vx.0));

        self.x86.mov(rax, ptr(vx_addr))?;
        self.x86.mov(rdi, ptr(i_addr))?;
        self.x86.add(rdi, rax)?;
        self.x86.mov(ptr(i_addr), rdi)?;

        Ok(true)
    }

    pub fn ld_f(&mut self, vx: Vx) -> InstructionResult {
        todo!()
    }

    pub fn ld_b(&mut self, vx: Vx) -> InstructionResult {
        todo!()
    }

    pub fn ld_i_x(&mut self, vx: Vx) -> InstructionResult {
        let vx_addr = self.get_field_addr(Chip8Field::Reg(vx.0));
        let v0_addr = self.get_field_addr(Chip8Field::Reg(0));
        let i_addr = self.get_field_addr(Chip8Field::I);

        self.x86.mov(rdi, 0_u64)?;
        self.x86.mov(r8, u64::try_from(vx_addr).unwrap())?;
        self.x86.mov(r10, r8 - v0_addr)?;

        // -- while loop --
        // put reg-value to i
        self.x86.anonymous_label()?;
        self.x86.mov(rax, ptr(v0_addr + rdi))?;
        let jump_addr = self.x86.bwd()?;

        self.x86.mov(ptr(i_addr + rdi), rax)?;

        // increment offset
        self.x86.inc(rdi)?;

        // while(rdi <= r10)
        self.x86.cmp(rdi, r10)?;
        self.x86.jle(jump_addr)?;

        Ok(true)
    }

    pub fn ld_x_i(&mut self, vx: Vx) -> InstructionResult {
        let vx_addr = self.get_field_addr(Chip8Field::Reg(vx.0));
        let v0_addr = self.get_field_addr(Chip8Field::Reg(0));
        let i_addr = self.get_field_addr(Chip8Field::I);

        self.x86.mov(rdi, 0_u64)?;
        self.x86.mov(r8, u64::try_from(vx_addr).unwrap())?;
        self.x86.mov(r10, r8 - v0_addr)?;

        // -- while loop --
        // movw al, [i]
        self.x86.anonymous_label()?;
        self.x86.mov(al, ptr(i_addr + rdi))?;
        let jump_addr = self.x86.bwd()?;

        self.x86.mov(ptr(v0_addr + rdi), al)?;

        // increment offset
        self.x86.inc(rdi)?;

        // while(rdi <= r10)
        self.x86.cmp(rdi, r10)?;
        self.x86.jle(jump_addr)?;

        Ok(true)
    }
}
