use crate::ChipAddr;

pub type Vx = u8;
pub type Vy = Vx;
#[allow(non_camel_case_types)]
pub type byte = u8;

#[allow(non_camel_case_types)]
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
