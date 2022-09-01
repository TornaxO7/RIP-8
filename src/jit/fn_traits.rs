use super::Vx;

pub trait ArgSe<T> {
    fn se(&mut self, vx: Vx, arg2: T) -> bool;
}

pub trait ArgSne<T> {
    fn sne(&mut self, vx: Vx, arg2: T) -> bool;
}

pub trait ArgLd<T> {
    fn ld(&mut self, vx: Vx, arg2: T) -> bool;
}
