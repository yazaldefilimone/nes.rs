// CPU 6502 - 8-bit microprocessor
// The 6502 is a microprocessor that was designed by MOS Technology in the 1970s.
//

use crate::opcode::Opcode;

pub struct CPU {
  pub program_counter: u16, // 16 bits
  pub register_a: u8,       // 8 bits
  pub register_x: u8,       // 8 bits
  pub status: u8,           // 8 bits
}

impl Default for CPU {
  fn default() -> Self {
    CPU { program_counter: 0, register_a: 0, register_x: 0, status: 0 }
  }
}

impl CPU {
  pub fn new() -> Self {
    CPU { program_counter: 0, register_a: 0, register_x: 0, status: 0 }
  }

  pub fn boot(&mut self, program: Vec<u8>) {
    // self.mem.load_program(program);
    self.reset();
    self.execute(&program);
  }

  pub fn reset(&mut self) {
    self.program_counter = 0;
  }

  pub fn execute(&mut self, program: &Vec<u8>) {
    loop {
      let opscode = program[self.program_counter as usize];
      self.program_counter += 1;
      let opcode = Opcode::from_u8(opscode).expect("Unrecognized opcode");
      match opcode {
        Opcode::LDA => self.load_accumulator(program),
        // force interrupt
        Opcode::BRK => return,
        Opcode::TAX => self.transfer_accumulator_to_x(),
        Opcode::INX => self.increment_register(),
        _ => {
          panic!("Unrecognized opcode: {:?}", opcode);
        }
      }
    }
  }

  pub fn load_accumulator(&mut self, program: &Vec<u8>) {
    let value = program[self.program_counter as usize];
    self.program_counter += 1;
    self.register_a = value;
    self.update_zero_and_negative_flags(self.register_a);
  }

  pub fn transfer_accumulator_to_x(&mut self) {
    self.register_x = self.register_a;
    self.update_zero_and_negative_flags(self.register_x);
  }

  pub fn increment_register(&mut self) {
    self.register_x = self.register_x.wrapping_add(1);
  }

  pub fn update_zero_and_negative_flags(&mut self, register: u8) {
    if register == 0 {
      self.status |= 0b0000_0010; // Set zero flag if A is zero
    } else {
      self.status &= 0b1111_1101; // Clear zero flag if A is not zero
    }
    // Check if bit 7 is set
    if register & 0b1000_0000 != 0 {
      self.status |= 0b1000_0000; // Set negative flag if bit 7 is set
    } else {
      self.status &= 0b0111_1111; // Clear negative flag if bit 7 is not set
    }
  }
}
