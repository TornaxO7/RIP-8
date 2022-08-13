#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Register {
    AX,
    BX,
    CX,
    DX,
    DI,
    SI,
    BP,
    SP
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    RET,
    PUSH,
}

impl Instruction {
    pub fn into_bin() -> &'static [u8] {
        todo!()
    }
}
