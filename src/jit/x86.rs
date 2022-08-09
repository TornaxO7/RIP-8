#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum x86Instruction {
    RET,
    MOVE_64,
}

impl x86Instruction {
    pub fn encode(&self) -> Vec<u8> {
        match self {
            _ => todo!(),
        }
    }
}
