use crate::instruction::Instruction;
use crate::instruction::InstructionType;



#[derive(Clone)]
pub struct Compiler {
    code: String, // Brainfuck code
    code_length: usize,
    position: usize,

    instructions: Vec<Instruction>, // New instructions set
}


impl Compiler {
  pub fn new(code: &str) -> Self {
      Self {
          code: code.to_string(),
          code_length: code.len(),
          position: 0,
          instructions: Vec::new(),
      }
  }

  pub fn compile(&mut self) -> Vec<Instruction> {
      let mut stack: Vec<usize> = Vec::new();

      while self.position < self.code_length {
          let current = self.code.chars().nth(self.position);
          match current {
              Some('+') => self.compile_foldable_instruction('+', InstructionType::Inc),
              Some('-') => self.compile_foldable_instruction('-', InstructionType::Dec),
              Some('>') => self.compile_foldable_instruction('>', InstructionType::Right),
              Some('<') => self.compile_foldable_instruction('<', InstructionType::Left),
              Some(',') => self.compile_foldable_instruction(',', InstructionType::Read),
              Some('.') => self.compile_foldable_instruction('.', InstructionType::Write),
              Some('[') => {
                  let ins_pos = self.emit_with_arg(InstructionType::JumpIfZero, 0);
                  stack.push(ins_pos);
              }
              Some(']') => {
                  let open_instruction = stack.pop().unwrap();
                  let close_instruction =
                      self.emit_with_arg(InstructionType::JumpIfNotZero, open_instruction);
                  self.instructions[open_instruction].argument = close_instruction;
              }
              _ => break,
          }
          self.position += 1;
      }

      return self.instructions.clone();
  }

  pub fn compile_foldable_instruction(&mut self, c: char, inst_type: InstructionType) {
      let mut count = 1;
      while (self.position < self.code_length - 1)
          && (self.code.chars().nth(self.position + 1).unwrap() == c) {
          count += 1;
          self.position += 1;
      }
      self.emit_with_arg(inst_type, count);
  }

  pub fn emit_with_arg(&mut self, inst_type: InstructionType, arg: usize) -> usize {
      self.instructions.push(Instruction {
          inst: inst_type,
          argument: arg,
      });
      return self.instructions.len() - 1;
  }
}
