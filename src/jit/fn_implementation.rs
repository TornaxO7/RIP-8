use crate::chip8::{Chip8Field, INSTRUCTION_SIZE_BYTES, Chip8State};

use super::{
    fn_traits::{ArgAdd, ArgLd, ArgSe, ArgSne},
    Addr, Byte, InstructionResult, Vx, Vy, JIT, fn_extern,
};

use iced_x86::code_asm::*;

impl JIT {
    pub fn cls(&mut self) -> InstructionResult {
        let cls_addr = fn_extern::cls as unsafe extern "C" fn(state: * mut Chip8State) -> ();
        self.x86.call(cls_addr as u64)?;
        Ok(true)
    }

    pub fn ret(&mut self) -> InstructionResult {
        let sp_addr = rdi + self.get_field_offset(Chip8Field::SP);
        let stack_addr = rdi + self.get_field_offset(Chip8Field::Stack);
        let pc_addr = rdi + self.get_field_offset(Chip8Field::PC);

        // set pc to top value of stack
        self.x86.mov(r8, ptr(sp_addr))?;
        self.x86.mov(
            r9,
            ptr(stack_addr) + r8 * usize::from(INSTRUCTION_SIZE_BYTES),
        )?;
        self.x86.mov(ptr(pc_addr), r9)?;

        // decrement sp
        self.x86.dec(ptr(sp_addr))?;
        Ok(true)
    }

    pub fn sys(&mut self, _: Addr) -> InstructionResult {
        // our interpreter is a (modern) chad, so we ignore this one
        Ok(false)
    }

    pub fn jp(&mut self, addr: Addr) -> InstructionResult {
        let pc_addr = rdi + self.get_field_offset(Chip8Field::PC);
        self.x86.mov(ptr(pc_addr), u32::from(addr.0))?;
        Ok(true)
    }

    pub fn call(&mut self, addr: Addr) -> InstructionResult {
        let sp_addr = rdi + self.get_field_offset(Chip8Field::SP);
        let sp_state = usize::from(self.chip_state.borrow().sp);
        let pc_addr = rdi + self.get_field_offset(Chip8Field::PC);
        let stack_addr = rdi + self.get_field_offset(Chip8Field::Stack);

        // increment stack pointer
        self.x86.inc(ptr(sp_addr))?;

        // put current pc on top of stack
        self.x86.mov(r8, ptr(pc_addr))?;
        self.x86.mov(
            ptr(stack_addr) + sp_state * usize::from(INSTRUCTION_SIZE_BYTES),
            r8,
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
        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));
        let vy_addr = rdi + self.get_field_offset(Chip8Field::Reg(vy.0));

        // do bitwise or
        self.x86.mov(r8, ptr(vx_addr))?;
        self.x86.or(r8, ptr(vy_addr))?;
        self.x86.mov(ptr(vx_addr), r8)?;

        Ok(true)
    }

    pub fn and(&mut self, vx: Vx, vy: Vy) -> InstructionResult {
        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));
        let vy_addr = rdi + self.get_field_offset(Chip8Field::Reg(vy.0));

        // do bitwise and
        self.x86.mov(r8, ptr(vx_addr))?;
        self.x86.and(r8, ptr(vy_addr))?;
        self.x86.mov(ptr(vx_addr), r8)?;
        Ok(true)
    }

    pub fn xor(&mut self, vx: Vx, vy: Vy) -> InstructionResult {
        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));
        let vy_addr = rdi + self.get_field_offset(Chip8Field::Reg(vy.0));

        // do bitwise and
        self.x86.mov(r8, ptr(vx_addr))?;
        self.x86.xor(r8, ptr(vy_addr))?;
        self.x86.mov(ptr(vx_addr), r8)?;
        Ok(true)
    }

    pub fn sub(&mut self, vx: Vx, vy: Vy) -> InstructionResult {
        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));
        let vy_addr = rdi + self.get_field_offset(Chip8Field::Reg(vy.0));
        let vf_addr = rdi + self.get_field_offset(Chip8Field::Reg(0xf));

        // sub
        self.x86.mov(r8, ptr(vx_addr))?;
        self.x86.mov(r9, ptr(vy_addr))?;
        self.x86.sub(r8, r9)?;

        // set Vf
        self.x86.setnc(ptr(vf_addr))?;

        Ok(true)
    }

    pub fn shr(&mut self, vx: Vx, _: Vy) -> InstructionResult {
        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));
        let vf_addr = rdi + self.get_field_offset(Chip8Field::Reg(0xf));

        // set vf
        self.x86.mov(r8, ptr(vx_addr))?;
        self.x86.shr(r8, 1u32)?;
        self.x86.setb(ptr(vf_addr))?;

        // save shr
        self.x86.mov(ptr(vx_addr), r8)?;

        Ok(true)
    }

    pub fn subn(&mut self, vx: Vx, vy: Vy) -> InstructionResult {
        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));
        let vy_addr = rdi + self.get_field_offset(Chip8Field::Reg(vy.0));
        let vf_addr = rdi + self.get_field_offset(Chip8Field::Reg(0xf));

        // sub Vy, Vx
        self.x86.mov(r8, ptr(vx_addr))?;
        self.x86.mov(r9, ptr(vy_addr))?;
        self.x86.sub(r9, r8)?;

        // set vf
        self.x86.setnc(ptr(vf_addr))?;

        self.x86.mov(ptr(vx_addr), r9)?;

        Ok(true)
    }

    pub fn shl(&mut self, vx: Vx, _: Vy) -> InstructionResult {
        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));
        let vf_addr = rdi + self.get_field_offset(Chip8Field::Reg(0xf));

        // set vf
        self.x86.mov(r8, ptr(vx_addr))?;
        self.x86.shl(r8, 1u32)?;
        self.x86.setb(ptr(vf_addr))?;

        // save shl
        self.x86.mov(ptr(vx_addr), r8)?;

        Ok(true)
    }

    pub fn ld_i(&mut self, _addr: Addr) -> InstructionResult {
        let ld_i_addr = fn_extern::ld_i as unsafe extern "C" fn(state: * mut Chip8State, addr: Addr) -> ();
        self.x86.call(ld_i_addr as u64)?;
        Ok(true)
    }

    pub fn ld_v0(&mut self, addr: Addr) -> InstructionResult {
        let i_addr = rdi + self.get_field_offset(Chip8Field::I);

        self.x86.mov(r8, u64::from(addr.0))?;
        self.x86.mov(ptr(i_addr), r8)?;

        Ok(true)
    }

    pub fn rnd(&mut self, vx: Vx, kk: Byte) -> InstructionResult {
        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));

        self.x86.rdrand(r8)?;
        self.x86.and(r8, i32::from(kk.0))?;
        self.x86.mov(ptr(vx_addr), r8)?;

        Ok(true)
    }

    pub fn drw(&mut self, vx: Vx, vy: Vy, nibble: u8) -> InstructionResult {
        let drw_addr = fn_extern::drw as unsafe extern "C" fn(state: * mut Chip8State, vx: Vx, vy: Vy, nibble: u8) -> ();
        self.x86.call(drw_addr as u64)?;
        Ok(true)
    }

    pub fn skp(&mut self, vx: Vx) -> InstructionResult {
        let skp_addr = fn_extern::skp as unsafe extern "C" fn(state: * mut Chip8State, vx: Vx) -> bool;
        self.x86.call(skp_addr as u64)?;

        todo!()
    }

    pub fn sknp(&mut self, vx: Vx) -> InstructionResult {
        let sknp_addr = fn_extern::sknp as unsafe extern "C" fn(state: * mut Chip8State, vx: Vx, vy: Vy, nibble: u8) -> bool;
        self.x86.call(sknp_addr as u64)?;

        todo!()
    }

    pub fn ld_x_dt(&mut self, vx: Vx) -> InstructionResult {
        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));
        let delay_timer_addr = rdi + self.get_field_offset(Chip8Field::Delay);

        self.x86.mov(r8, ptr(delay_timer_addr))?;
        self.x86.mov(ptr(vx_addr), r8)?;

        Ok(true)
    }

    pub fn ld_k(&mut self, vx: Vx) -> InstructionResult {
        let ld_k_addr = fn_extern::ld_k as unsafe extern "C" fn(state: * mut Chip8State, vx: Vx) -> ();
        self.x86.call(ld_k_addr as u64)?;

        Ok(true)
    }

    pub fn ld_dt_x(&mut self, vx: Vx) -> InstructionResult {
        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));
        let delay_timer_addr = rdi + self.get_field_offset(Chip8Field::Delay);

        self.x86.mov(r8, ptr(vx_addr))?;
        self.x86.mov(ptr(delay_timer_addr), r8)?;

        Ok(true)
    }

    pub fn ld_st(&mut self, vx: Vx) -> InstructionResult {
        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));
        let sound_addr = rdi + self.get_field_offset(Chip8Field::Sound);

        self.x86.mov(r8, ptr(vx_addr))?;
        self.x86.mov(ptr(sound_addr), r8)?;

        Ok(true)
    }

    pub fn add_i(&mut self, vx: Vx) -> InstructionResult {
        let i_addr = rdi + self.get_field_offset(Chip8Field::I);
        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));

        self.x86.mov(r8, ptr(vx_addr))?;
        self.x86.mov(r9, ptr(i_addr))?;
        self.x86.add(r9, r8)?;
        self.x86.mov(ptr(i_addr), r9)?;

        Ok(true)
    }

    pub fn ld_f(&mut self, vx: Vx) -> InstructionResult {
        let ld_f_addr = fn_extern::ld_f as unsafe extern "C" fn(state: * mut Chip8State, vx: Vx) -> ();
        self.x86.call(ld_f_addr as u64)?;

        Ok(true)
    }

    pub fn ld_b(&mut self, vx: Vx) -> InstructionResult {
        let ld_b_addr = fn_extern::ld_b as unsafe extern "C" fn(state: * mut Chip8State, vx: Vx) -> ();
        self.x86.call(ld_b_addr as u64)?;

        Ok(true)
    }

    pub fn ld_i_x(&mut self, vx: Vx) -> InstructionResult {
        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));
        let v0_addr = rdi + self.get_field_offset(Chip8Field::Reg(0));
        let i_addr = rdi + self.get_field_offset(Chip8Field::I);

        self.x86.mov(r8, v0_addr)?; // source ptr
        self.x86.mov(r9, i_addr)?; // destination ptr
        self.x86.mov(r10, vx_addr)?; // end-addr

        // -- while loop --
        self.x86.anonymous_label()?;
        // mov <reg>, <I>
        self.x86.mov(r11, ptr(r8))?;
        self.x86.mov(ptr(r9), r11)?;

        // increment ptr
        self.x86.inc(r8)?;
        self.x86.inc(r9)?;

        // while <source ptr> <= end-addr
        self.x86.cmp(r8, r10)?;
        let jump_addr = self.x86.bwd()?;
        self.x86.jle(jump_addr)?;

        Ok(true)
    }

    pub fn ld_x_i(&mut self, vx: Vx) -> InstructionResult {
        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));
        let v0_addr = rdi + self.get_field_offset(Chip8Field::Reg(0));
        let i_addr = rdi + self.get_field_offset(Chip8Field::I);

        self.x86.mov(r8, i_addr)?; // source ptr
        self.x86.mov(r9, v0_addr)?; // destination ptr
        self.x86.mov(r10, vx_addr)?; // end-addr

        // -- while loop --
        self.x86.anonymous_label()?;
        // mov <reg>, <I>
        self.x86.mov(r11, ptr(r8))?;
        self.x86.mov(ptr(r9), r11)?;

        // increment ptr
        self.x86.inc(r8)?;
        self.x86.inc(r9)?;

        // while <dest-ptr> <= <end-addr>
        self.x86.cmp(r9, r10)?;
        let jump_addr = self.x86.bwd()?;
        self.x86.jle(jump_addr)?;

        Ok(true)
    }
}
