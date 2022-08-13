use crate::ChipAddr;

pub type Vx = u8;
pub type Vy = Vx;
pub type byte = u8;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Instruction {
    CLS,
    RET,
    SYS(ChipAddr),
    JP(ChipAddr),
    CALL(ChipAddr),
    SE_Vx_byte(Vx, byte),
    SNE_Vx_Vy(Vx, Vy),
    SE_Vx_Vy(Vx, Vy),
    LD(Vx, byte),
    ADD
}
