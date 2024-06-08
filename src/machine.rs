use crate::instruction::Instruction;
use crate::instruction::InstructionType;
use std::io::{Read, Write};


pub struct Machine {
  code: Vec<Instruction>,
  memory: [u8; 30000],

  dp: usize,
  ip: usize,

  buf: [u8; 1],
}

impl Machine {
  pub fn new(code: Vec<Instruction>) -> Self {
      Self {
          code: code.clone(),
          memory: [0; 30000],
          dp: 0,
          ip: 0,
          buf: [0],
      }
  }

  pub fn execute(&mut self) {
      for instruction in self.code.clone() {
          match instruction.inst {
              InstructionType::Left => {
                let mut counter = 0;
                while counter < instruction.argument {
                  self.dp -= 1;
                  counter += 1;
                }
              },
              InstructionType::Right => {
                let mut counter = 0;
                while counter < instruction.argument {
                  self.dp += 1;
                  counter += 1;
                }
              },
              InstructionType::Inc => {
                let mut counter = 0;
                while counter < instruction.argument {
                  self.memory[self.dp] += 1;
                  counter += 1;
                }
              },
              InstructionType::Dec => {
                let mut counter = 0;
                while counter < instruction.argument {
                  self.memory[self.dp] -= 1;
                  counter += 1;
                }
              },
              InstructionType::Read => self.read_char(),
              InstructionType::Write => self.put_char(),
              _ => break
          }
          self.ip += 1;
      }
  }

  pub fn read_char(&mut self) {
      let mut buf: [u8; 1] = [0];
      std::io::stdin().read_exact(&mut buf).unwrap();
      self.memory[self.dp] = buf[0];
  }

  pub fn put_char(&mut self) {
      std::io::stdout().write(&[self.memory[self.dp]]).unwrap();
  }
}
