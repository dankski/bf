#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Instruction {
    pub inst: InstructionType,
    pub argument: usize,
}

