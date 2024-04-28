use crate::beytecode;

pub struct CPU {
  pub program_counter: u16,
  pub register_a: u8,
  pub register_x: u8,
  pub status: u8,
}

impl CPU {
  pub fn new() -> CPU {
    CPU { program_counter: 0, register_a: 0, status: 0, register_x: 0 }
  }

  pub fn run(&mut self, program: &Vec<u8>) {
    self.program_counter = 0;
    loop {
      let opcode = program[self.program_counter as usize];
      self.program_counter += 1;
      match opcode {
        beytecode::OPCODE_LDA => self.load_accumulator(program),
        beytecode::OPCODE_BRK => return,
        beytecode::OPCODE_TAX => self.transfer_accumulator(),
        beytecode::OPCODE_INX => self.increment_register(),
        _ => {
          println!("Unrecognized opcode: {}", opcode);
          break;
        }
      }
    }
  }

  fn load_accumulator(&mut self, program: &Vec<u8>) {
    let value = program[self.program_counter as usize];
    self.program_counter += 1;
    self.register_a = value;
    self.update_zero_and_negative_flags(value);
  }

  // transfer accumulator to x
  fn transfer_accumulator(&mut self) {
    self.register_x = self.register_a;
    self.update_zero_and_negative_flags(self.register_x);
  }
  // increment x registier
  fn increment_register(&mut self) {
    self.register_x = self.register_x.wrapping_add(1);
    self.update_zero_and_negative_flags(self.register_x);
  }
  // helper functions...
  fn update_zero_and_negative_flags(&mut self, register: u8) {
    if register == 0 {
      self.status |= beytecode::HEX_2; // Set zero flag if A is zero
    } else {
      self.status &= beytecode::HEX_253; // Clear zero flag if A is not zero
    }
    // Check if bit 7 is set
    if register & beytecode::HEX_128 != 0 {
      self.status |= beytecode::HEX_128; // Set negative flag if bit 7 is set
    } else {
      self.status &= beytecode::HEX_127; // Clear negative flag if bit 7 is not set
    }
  }
}
