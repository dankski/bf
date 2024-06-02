use std::io::{Read, Write};
use std::env;
use std::fs;
use std::os::unix::raw::dev_t;
use std::process;

struct Machine {
  code: String,
  memory: [u8; 30000],

  dp: usize,
  ip: usize,

  buf: [u8; 1]
}

impl Machine {

  pub fn new(code: &str) -> Self {
    Self {
      code: code.to_string(),
      memory: [0; 30000],
      dp: 0,
      ip: 0,
      buf: [0]
    }
  }

  pub fn execute(&mut self) {
      while self.ip < self.code.len() {

        let inst = self.code.chars().nth(self.ip);
        match inst {
          Some('+') => self.memory[self.dp] = self.memory[self.dp] + 1,
          Some('-') => self.memory[self.dp] = self.memory[self.dp] - 1,
          Some('>') => self.dp += 1,
          Some('<') => self.dp -= 1,
          Some(',') => self.read_char(),
          Some('.') => self.put_char(),
          Some('[') => {
              if self.memory[self.dp] == 0 {
                let mut depth = 1;
                while depth < 0 {
                  self.ip += 1;
                  let inst = self.code.chars().nth(self.ip);
                  match inst {
                    Some('[') => depth += 1,
                    Some(']') => depth -= 1,
                    _ => continue,
                  }
                }
              }
          },
          Some(']') => {
            if self.memory[self.dp] != 0 {
              let mut depth = 1;
              while depth != 0 {
                self.ip -= 1;
                let inst = self.code.chars().nth(self.ip);
                match inst {
                  Some('[') => depth -= 1,
                  Some(']') => depth += 1,
                  _ => continue,
                }
              }
            }
        },        
        _ => break,
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

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    eprintln!("Usage: {} <filename>", args[0]);
    process::exit(1);
  }

  let filename = &args[1];
  match fs::read_to_string(filename) {
    Ok(contents) => {
      println!("File contents:\n{}", contents);
      let mut m = Machine::new(contents.as_str());
      m.execute();
      println!("Execution done");
    }
    Err(error) => {
      eprintln!("Error reading file {}: {}", filename, error);
      process::exit(1);
    }
  }


}
