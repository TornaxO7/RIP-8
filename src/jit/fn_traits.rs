use super::{InstructionResult, Vx};

pub trait ArgSe<T> {
    fn se(&mut self, vx: Vx, arg2: T) -> InstructionResult;
}

pub trait ArgSne<T> {
    fn sne(&mut self, vx: Vx, arg2: T) -> InstructionResult;
}

pub trait ArgLd<T> {
    fn ld(&mut self, vx: Vx, arg2: T) -> InstructionResult;
}

pub trait ArgAdd<T> {
    fn add(&mut self, vx: Vx, arg2: T) -> InstructionResult;
}
