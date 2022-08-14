use super::{
    fn_traits::{ArgAdd, ArgLd, ArgSe, ArgSne},
    Byte, InstructionResult, Vx, Vy, JIT,
};

impl ArgSe<Byte> for JIT<'_> {
    fn se(&mut self, vx: Vx, arg2: Byte) -> InstructionResult {
        todo!()
    }
}

impl ArgSe<Vy> for JIT<'_> {
    fn se(&mut self, vx: Vx, arg2: Vy) -> InstructionResult {
        todo!()
    }
}

impl ArgSne<Byte> for JIT<'_> {
    fn sne(&mut self, vx: Vx, arg2: Byte) -> InstructionResult {
        todo!()
    }
}

impl ArgSne<Vy> for JIT<'_> {
    fn sne(&mut self, vx: Vx, arg2: Vy) -> InstructionResult {
        todo!()
    }
}

impl ArgLd<Byte> for JIT<'_> {
    fn ld(&mut self, vx: Vx, arg2: Byte) -> InstructionResult {
        todo!()
    }
}

impl ArgLd<Vy> for JIT<'_> {
    fn ld(&mut self, vx: Vx, arg2: Vy) -> InstructionResult {
        todo!()
    }
}

impl ArgAdd<Byte> for JIT<'_> {
    fn add(&mut self, vx: Vx, arg2: Byte) -> InstructionResult {
        todo!()
    }
}

impl ArgAdd<Vy> for JIT<'_> {
    fn add(&mut self, vx: Vx, arg2: Vy) -> InstructionResult {
        todo!()
    }
}
