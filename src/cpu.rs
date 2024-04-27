use crate::beytecode;

pub struct CPU {
  pub program_counter: u16,
  pub register_a: u8,
  pub status: u8,
}

impl CPU {
  pub fn new() -> CPU {
    CPU { program_counter: 0, register_a: 0, status: 0 }
  }

  pub fn interpret(&mut self, program: &Vec<u8>) {
    self.program_counter = 0;
    loop {
      let opcode = program[self.program_counter as usize];
      self.program_counter += 1;
      match opcode {
        beytecode::LDA_IMM => {
          let value = program[self.program_counter as usize];
          self.program_counter += 1;
          self.register_a = value;

          if self.register_a == 0 {
            self.status |= 0b0000_0010; // Set zero flag if A is zero
          } else {
            self.status &= 0b1111_1101; // Clear zero flag if A is not zero
          }
          // Check if bit 7 is set
          if self.register_a & 0b1000_0000 != 0 {
            self.status |= 0b1000_0000; // Set negative flag if bit 7 is set
          } else {
            self.status &= 0b0111_1111; // Clear negative flag if bit 7 is not set
          }
        }
        _ => {
          println!("Unrecognized opcode: {}", opcode);
          break;
        }
      }
    }
  }
}
