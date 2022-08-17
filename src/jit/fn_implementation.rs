use crate::chip8::{Chip8Field, Chip8State, INSTRUCTION_SIZE_BYTES};

use super::{
    fn_extern,
    fn_traits::{ArgLd, ArgSe, ArgSne},
    Nnn, Byte, Vx, Vy, JIT, Helper,
};

use iced_x86::code_asm::*;
use log::debug;

impl JIT {
    fn function_call_prolog(&mut self) {
        self.x86.push(rbp).unwrap();
        self.x86.mov(rbp, rsp).unwrap();
    }

    fn function_call_epilog(&mut self) {
        self.x86.mov(rsp, rbp).unwrap();
        self.x86.pop(rbp).unwrap();
    }

    pub fn cls(&mut self) -> bool {
        debug!("-> CLS");

        debug!("-> CLS");
        self.function_call_prolog();

        let cls_addr = fn_extern::cls as unsafe extern "C" fn(state: *mut Chip8State) -> ();
        self.x86.mov(rax, cls_addr as u64).unwrap();
        self.x86.call(rax).unwrap();

        self.function_call_epilog();
        true
    }

    pub fn ret(&mut self) -> bool {
        debug!("-> RET");

        let sp_addr = rdi + self.get_field_offset(Chip8Field::SP);
        let stack_addr = rdi + self.get_field_offset(Chip8Field::Stack);
        let pc_addr = rdi + self.get_field_offset(Chip8Field::PC);

        self.x86.mov(r8, stack_addr).unwrap();
        self.x86.mov(r9, pc_addr).unwrap();

        // calculate sp * INSTRUCTION_SIZE_BYTES
        self.x86.mov(r10, INSTRUCTION_SIZE_BYTES).unwrap();
        self.x86.mov(rax, qword_ptr(sp_addr)).unwrap();
        self.x86.mul(r10).unwrap();
        self.x86.add(r8, rax).unwrap();

        // mov pc, ptr(stack_addr) + sp * INSTRUCTION_SIZE_BYTES
        self.x86.mov(r10, qword_ptr(r8)).unwrap();
        self.x86.mov(qword_ptr(r9), r10).unwrap();

        // decrement sp
        self.x86.dec(qword_ptr(sp_addr)).unwrap();
        true
    }

    pub fn sys(&mut self, _: Nnn) -> bool {
        debug!("-> SYS");
        // our jit is a modern jit, so we're ignoring this one
        let pc_addr = rdi + self.get_field_offset(Chip8Field::PC);

        self.x86.mov(r8, qword_ptr(pc_addr)).unwrap();
        self.x86.mov(r9, INSTRUCTION_SIZE_BYTES).unwrap();
        self.x86.add(r8, r9).unwrap();
        self.x86.mov(qword_ptr(pc_addr), r8).unwrap();

        false
    }

    pub fn jp(&mut self, addr: Nnn) -> bool {
        debug!("-> JP");

        let pc_addr = rdi + self.get_field_offset(Chip8Field::PC);
        self.x86.mov(r8, addr.0 as u64).unwrap();
        self.x86.mov(qword_ptr(pc_addr), r8).unwrap();
        true
    }

    pub fn call(&mut self, addr: Nnn) -> bool {
        debug!("-> CALL");

        let sp_addr = rdi + self.get_field_offset(Chip8Field::SP);
        let pc_addr = rdi + self.get_field_offset(Chip8Field::PC);
        let stack_addr = rdi + self.get_field_offset(Chip8Field::Stack);

        // increment stack pointer
        self.x86.mov(rax, qword_ptr(sp_addr)).unwrap();
        self.x86.inc(rax).unwrap();
        self.x86.mov(qword_ptr(sp_addr), rax).unwrap();

        // put current pc on top of stack
        self.x86.mov(r9, qword_ptr(pc_addr)).unwrap();

        // calculate address of the stack, where to store pc
        self.x86.mov(r10, stack_addr).unwrap();

        // sp * INSTRUCTION_SIZE_BYTES
        self.x86.mov(r11, INSTRUCTION_SIZE_BYTES).unwrap();
        self.x86.mul(r11).unwrap();

        // stack_addr + (sp * INSTRUCTION_SIZE_BYTES)
        self.x86.add(r10, rax).unwrap();

        // move pc value to stack
        self.x86.mov(qword_ptr(r10), r9).unwrap();

        // set pc to `addr`
        self.x86.mov(r8, u64::from(addr.0)).unwrap();
        self.x86.mov(qword_ptr(pc_addr), r8).unwrap();
        true
    }

    pub fn se<T>(&mut self, vx: Vx, arg2: T) -> bool
    where
        Self: ArgSe<T>,
    {
        debug!("--> SE");
        <Self as ArgSe<T>>::se(self, vx, arg2);
        false
    }

    pub fn sne<T>(&mut self, vx: Vx, arg2: T) -> bool
    where
        Self: ArgSne<T>,
    {
        debug!("--> SNE");
        <Self as ArgSne<T>>::sne(self, vx, arg2);
        false
    }

    pub fn ld<T>(&mut self, vx: Vx, arg2: T) -> bool
    where
        Self: ArgLd<T>,
    {
        debug!("--> LD");
        <Self as ArgLd<T>>::ld(self, vx, arg2);
        true
    }

    pub fn add_kk(&mut self, vx: Vx, kk: Helper) -> bool {
        debug!("--> ADD_KK");

        let vx_addr = self.get_field_offset(Chip8Field::Reg(vx.0));
        let helper_addr = self.get_field_offset(Chip8Field::Helper(kk.0));

        self.x86.mov(r8, vx_addr).unwrap();
        self.x86.mov(r9, qword_ptr(helper_addr)).unwrap();
        self.x86.add(qword_ptr(r8), r9).unwrap();

        true
    }

    pub fn add_y(&mut self, vx: Vx, vy: Vy) -> bool {
        debug!("--> ADD_Y");

        let vx_addr = self.get_field_offset(Chip8Field::Reg(vx.0));
        let vy_addr = self.get_field_offset(Chip8Field::Reg(vy.0));
        let vf_addr = self.get_field_offset(Chip8Field::Reg(0xf));

        self.x86.mov(r8, qword_ptr(vx_addr)).unwrap();
        self.x86.mov(r9, qword_ptr(vy_addr)).unwrap();
        self.x86.mov(r10, vf_addr).unwrap();

        // add Vx, Vy
        self.x86.add(r8, r9).unwrap();
        self.x86.mov(qword_ptr(vx_addr), r8).unwrap();

        // set Vf
        self.x86.setc(qword_ptr(r10)).unwrap();

        true
    }

    pub fn or(&mut self, vx: Vx, vy: Vy) -> bool {
        debug!("-> OR");

        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));
        let vy_addr = rdi + self.get_field_offset(Chip8Field::Reg(vy.0));

        // do bitwise or
        self.x86.mov(r8, qword_ptr(vx_addr)).unwrap();
        self.x86.or(r8, qword_ptr(vy_addr)).unwrap();
        self.x86.mov(qword_ptr(vx_addr), r8).unwrap();

        true
    }

    pub fn and(&mut self, vx: Vx, vy: Vy) -> bool {
        debug!("-> AND");

        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));
        let vy_addr = rdi + self.get_field_offset(Chip8Field::Reg(vy.0));

        // do bitwise and
        self.x86.mov(r8, qword_ptr(vx_addr)).unwrap();
        self.x86.and(r8, qword_ptr(vy_addr)).unwrap();
        self.x86.mov(qword_ptr(vx_addr), r8).unwrap();
        true
    }

    pub fn xor(&mut self, vx: Vx, vy: Vy) -> bool {
        debug!("-> XOR");

        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));
        let vy_addr = rdi + self.get_field_offset(Chip8Field::Reg(vy.0));

        // do bitwise and
        self.x86.mov(r8, qword_ptr(vx_addr)).unwrap();
        self.x86.xor(r8, qword_ptr(vy_addr)).unwrap();
        self.x86.mov(qword_ptr(vx_addr), r8).unwrap();
        true
    }

    pub fn sub(&mut self, vx: Vx, vy: Vy) -> bool {
        debug!("-> SUB");

        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));
        let vy_addr = rdi + self.get_field_offset(Chip8Field::Reg(vy.0));
        let vf_addr = rdi + self.get_field_offset(Chip8Field::Reg(0xf));

        // sub
        self.x86.mov(r8, qword_ptr(vx_addr)).unwrap();
        self.x86.mov(r9, qword_ptr(vy_addr)).unwrap();
        self.x86.sub(r8, r9).unwrap();

        // set Vf
        self.x86.mov(r8, vf_addr).unwrap();
        self.x86.setnc(qword_ptr(r8)).unwrap();

        true
    }

    pub fn shr(&mut self, vx: Vx, _: Vy) -> bool {
        debug!("-> SHR");

        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));
        let vf_addr = rdi + self.get_field_offset(Chip8Field::Reg(0xf));

        // set Vf
        self.x86.mov(r8, qword_ptr(vx_addr)).unwrap();
        self.x86.shr(r8, 1u32).unwrap();
        self.x86.mov(r8, vf_addr).unwrap();
        self.x86.setb(qword_ptr(r8)).unwrap();

        // save shr
        self.x86.mov(qword_ptr(vx_addr), r8).unwrap();

        true
    }

    pub fn subn(&mut self, vx: Vx, vy: Vy) -> bool {
        debug!("-> SUBN");

        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));
        let vy_addr = rdi + self.get_field_offset(Chip8Field::Reg(vy.0));
        let vf_addr = rdi + self.get_field_offset(Chip8Field::Reg(0xf));

        // sub Vy, Vx
        self.x86.mov(r8, qword_ptr(vx_addr)).unwrap();
        self.x86.mov(r9, qword_ptr(vy_addr)).unwrap();
        self.x86.sub(r9, r8).unwrap();

        // set Vf
        self.x86.mov(r10, vf_addr).unwrap();
        self.x86.setnc(qword_ptr(r10)).unwrap();

        self.x86.mov(qword_ptr(vx_addr), r9).unwrap();

        true
    }

    pub fn shl(&mut self, vx: Vx, _: Vy) -> bool {
        debug!("-> SHL");

        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));
        let vf_addr = rdi + self.get_field_offset(Chip8Field::Reg(0xf));

        // set Vf
        self.x86.mov(r8, qword_ptr(vx_addr)).unwrap();
        self.x86.shl(r8, 1u32).unwrap();
        self.x86.mov(r9, vf_addr).unwrap();
        self.x86.setb(qword_ptr(r9)).unwrap();

        // save shl
        self.x86.mov(qword_ptr(vx_addr), r8).unwrap();

        true
    }

    pub fn ld_i(&mut self, addr: Nnn) -> bool {
        debug!("-> LD_I");

        let i_addr = rdi + self.get_field_offset(Chip8Field::I);

        self.x86.mov(r8, u64::from(addr.0)).unwrap();
        self.x86.mov(qword_ptr(i_addr), r8).unwrap();

        true
    }

    pub fn ld_v0(&mut self, addr: Nnn) -> bool {
        debug!("-> LD_V0");

        let i_addr = rdi + self.get_field_offset(Chip8Field::I);

        self.x86.mov(r8, u64::from(addr.0)).unwrap();
        self.x86.mov(qword_ptr(i_addr), r8).unwrap();

        true
    }

    pub fn rnd(&mut self, vx: Vx, kk: Byte) -> bool {
        debug!("-> RND");

        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));

        self.x86.mov(r8, u64::from(kk.0)).unwrap();
        self.x86.rdrand(r9).unwrap();
        self.x86.and(r9, r8).unwrap();
        self.x86.mov(qword_ptr(vx_addr), r9).unwrap();

        true
    }

    pub fn drw(&mut self, vx: Vx, vy: Vy, nibble: u64) -> bool {
        debug!("-> DRW");

        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));
        let vy_addr = rdi + self.get_field_offset(Chip8Field::Reg(vy.0));
        let _nibble_addr = &mut nibble.clone() as * const u64 as u64;

        self.function_call_prolog();

        self.x86.mov(rsi, qword_ptr(vx_addr)).unwrap();
        self.x86.mov(rdx, qword_ptr(vy_addr)).unwrap();
        // TODO: self.x86.mov(rcx, qword_ptr(nibble_addr)).unwrap();
        self.x86.mov(rcx, 10u64).unwrap();

        let drw_addr = fn_extern::drw
            as unsafe extern "C" fn(state: *mut Chip8State, vx: u64, vy: u64, nibble: u64) -> ();
        self.x86.call(drw_addr as u64).unwrap();

        self.function_call_epilog();
        true
    }

    pub fn skp(&mut self, vx: Vx) -> bool {
        debug!("-> SKP");

        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));

        self.function_call_prolog();

        self.x86.mov(rsi, qword_ptr(vx_addr)).unwrap();

        let skp_addr =
            fn_extern::skp as unsafe extern "C" fn(state: *mut Chip8State, vx: u64) -> ();
        self.x86.call(skp_addr as u64).unwrap();

        self.function_call_epilog();
        false
    }

    pub fn sknp(&mut self, vx: Vx) -> bool {
        debug!("-> SKNP");
        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));

        self.function_call_prolog();

        self.x86.mov(rsi, qword_ptr(vx_addr)).unwrap();

        let sknp_addr =
            fn_extern::sknp as unsafe extern "C" fn(state: *mut Chip8State, vx: u64) -> ();
        self.x86.call(sknp_addr as u64).unwrap();

        self.function_call_epilog();

        false
    }

    pub fn ld_x_dt(&mut self, vx: Vx) -> bool {
        debug!("-> LD_X_DT");

        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));
        let delay_timer_addr = rdi + self.get_field_offset(Chip8Field::Delay);

        self.x86.mov(r8, qword_ptr(delay_timer_addr)).unwrap();
        self.x86.mov(qword_ptr(vx_addr), r8).unwrap();

        true
    }

    pub fn ld_k(&mut self, vx: Vx) -> bool {
        debug!("-> LD_K");
        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));

        self.function_call_prolog();

        self.x86.mov(rsi, qword_ptr(vx_addr)).unwrap();

        let ld_k_addr =
            fn_extern::ld_k as unsafe extern "C" fn(state: *mut Chip8State, vx: u64) -> ();
        self.x86.call(ld_k_addr as u64).unwrap();

        self.function_call_epilog();
        true
    }

    pub fn ld_dt_x(&mut self, vx: Vx) -> bool {
        debug!("-> LD_DT_X");

        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));
        let delay_timer_addr = rdi + self.get_field_offset(Chip8Field::Delay);

        self.x86.mov(r8, qword_ptr(vx_addr)).unwrap();
        self.x86.mov(qword_ptr(delay_timer_addr), r8).unwrap();

        true
    }

    pub fn ld_st(&mut self, vx: Vx) -> bool {
        debug!("-> LD_ST");

        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));
        let sound_addr = rdi + self.get_field_offset(Chip8Field::Sound);

        self.x86.mov(r8, qword_ptr(vx_addr)).unwrap();
        self.x86.mov(qword_ptr(sound_addr), r8).unwrap();

        true
    }

    pub fn add_i(&mut self, vx: Vx) -> bool {
        debug!("-> ADD_I");

        let i_addr = rdi + self.get_field_offset(Chip8Field::I);
        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));

        self.x86.mov(r8, qword_ptr(vx_addr)).unwrap();
        self.x86.mov(r9, qword_ptr(i_addr)).unwrap();
        self.x86.add(r9, r8).unwrap();
        self.x86.mov(qword_ptr(i_addr), r9).unwrap();

        true
    }

    pub fn ld_f(&mut self, vx: Vx) -> bool {
        debug!("-> LD_F");

        self.function_call_prolog();

        self.x86.mov(rsi, u64::from(vx.0)).unwrap();

        let ld_f_addr =
            fn_extern::ld_f as unsafe extern "C" fn(state: *mut Chip8State, vx: u64) -> ();
        self.x86.call(ld_f_addr as u64).unwrap();

        self.function_call_epilog();
        true
    }

    pub fn ld_b(&mut self, vx: Vx) -> bool {
        debug!("-> LD_B");

        self.function_call_prolog();

        self.x86.mov(rsi, u64::from(vx.0)).unwrap();

        let ld_b_addr =
            fn_extern::ld_b as unsafe extern "C" fn(state: *mut Chip8State, vx: u64) -> ();
        self.x86.call(ld_b_addr as u64).unwrap();

        self.function_call_epilog();
        true
    }

    pub fn ld_i_x(&mut self, vx: Vx) -> bool {
        debug!("-> LD_I_X");

        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));
        let v0_addr = rdi + self.get_field_offset(Chip8Field::Reg(0));
        let i_addr = rdi + self.get_field_offset(Chip8Field::I);

        self.x86.mov(r8, v0_addr).unwrap(); // source ptr
        self.x86.mov(r9, i_addr).unwrap(); // destination ptr
        self.x86.mov(r10, vx_addr).unwrap(); // end-addr

        // -- while loop --
        self.x86.anonymous_label().unwrap();
        // mov <reg>, <I>
        self.x86.mov(r11, qword_ptr(r8)).unwrap();
        self.x86.mov(qword_ptr(r9), r11).unwrap();

        // increment ptr
        self.x86.inc(r8).unwrap();
        self.x86.inc(r9).unwrap();

        // while <source ptr> <= end-addr
        self.x86.cmp(r8, r10).unwrap();
        let jump_addr = self.x86.bwd().unwrap();
        self.x86.jle(jump_addr).unwrap();

        true
    }

    pub fn ld_x_i(&mut self, vx: Vx) -> bool {
        debug!("-> LD_X_I");

        let vx_addr = rdi + self.get_field_offset(Chip8Field::Reg(vx.0));
        let v0_addr = rdi + self.get_field_offset(Chip8Field::Reg(0));
        let i_addr = rdi + self.get_field_offset(Chip8Field::I);

        self.x86.mov(r8, i_addr).unwrap(); // source ptr
        self.x86.mov(r9, v0_addr).unwrap(); // destination ptr
        self.x86.mov(r10, vx_addr).unwrap(); // end-addr

        // -- while loop --
        self.x86.anonymous_label().unwrap();
        // mov <reg>, <I>
        self.x86.mov(r11, qword_ptr(r8)).unwrap();
        self.x86.mov(qword_ptr(r9), r11).unwrap();

        // increment ptr
        self.x86.inc(r8).unwrap();
        self.x86.inc(r9).unwrap();

        // while <dest-ptr> <= <end-addr>
        self.x86.cmp(r9, r10).unwrap();
        let jump_addr = self.x86.bwd().unwrap();
        self.x86.jle(jump_addr).unwrap();

        true
    }
}
