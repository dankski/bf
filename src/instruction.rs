#[derive(Debug, Clone, PartialEq)]
pub enum InstructionType {
    Inc,
    Dec,
    Left,
    Right,
    Read,
    Write,
    JumpIfZero,
    JumpIfNotZero,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub inst: InstructionType,
    pub argument: usize,
}

