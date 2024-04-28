use crate::beytecode;
pub struct Memory {
  pub ram: [u8; 0xFFFF],
  pub start_address: usize,
}

impl Memory {
  pub fn new() -> Memory {
    Memory { ram: [0; 0xFFFF], start_address: 0x8000 }
  }
  pub fn read(&self, address: u16) -> u8 {
    self.ram[address as usize]
  }
  pub fn write(&mut self, address: u16, value: u8) {
    self.ram[address as usize] = value;
  }

  pub fn load_program(&mut self, program: Vec<u8>) -> u16 {
    // let start_address: usize = 0x8000;
    // Load program into memory starting at 0x8000
    self.ram[self.start_address..(self.start_address + program.len())].copy_from_slice(&program[..]);
    // Set the program counter to the start address of the program
    self.write_u16(0xFFFC, self.start_address as u16);
    // Return the start address of the program
    return self.start_address as u16;
  }

  /*

  In NES CPU uses little-endian memory addressing;
  This means that the least significant byte of a multi-byte value is stored at the lowest memory address.
  For example:
  Real Memory Address: 0x8000 (big-endian)
  big-endian: 80 00
  little-endian: 00 80
  This is the opposite of big-endian memory addressing, where the most significant byte is stored at the lowest memory address.
  ------------
  little-endian (aka big-endian): https://en.wikipedia.org/wiki/Endianness


  */
  fn read_u16(&mut self, pos: u16) -> u16 {
    let lo = self.read(pos) as u16;
    let hi = self.read(pos + 1) as u16;
    (hi << 8) | (lo as u16)
  }
  fn write_u16(&mut self, pos: u16, data: u16) {
    let hi = (data >> 8) as u8;
    let lo = (data & 0xff) as u8;
    self.write(pos, lo);
    self.write(pos + 1, hi);
  }
}
pub struct CPU {
  pub program_counter: u16,
  pub register_a: u8,
  pub register_x: u8,
  pub status: u8,
  pub memory: Memory,
}

impl Default for CPU {
  fn default() -> Self {
    CPU { program_counter: 0, register_a: 0, register_x: 0, status: 0, memory: Memory::new() }
  }
}
impl CPU {
  pub fn new(memory: Memory) -> Self {
    CPU { program_counter: 0, register_a: 0, register_x: 0, status: 0, memory }
  }
  pub fn boot(&mut self, program: Vec<u8>) {
    self.memory.load_program(program);
    self.reset();
    self.run()
  }

  pub fn reset(&mut self) {
    // self.register_a = 0;
    // self.register_x = 0;
    // self.status = 0;
    self.program_counter = self.memory.read_u16(0xFFFC);
    println!("PC: 0x{:04X}", self.program_counter);
  }

  pub fn run(&mut self) {
    loop {
      let opcode = self.memory.read(self.program_counter);
      self.program_counter += 1;
      println!("Opcode: 0x{:02X}", opcode);
      match opcode {
        beytecode::OPCODE_LDA => self.load_accumulator(),
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

  fn load_accumulator(&mut self) {
    let value = self.memory.read(self.program_counter);
    self.program_counter = self.program_counter.wrapping_add(1);
    self.register_a = value;
    self.update_zero_and_negative_flags(self.register_a);
  }

  // transfer accumulator to x
  fn transfer_accumulator(&mut self) {
    println!("Transfer accumulator to x, x: 0x{:02X}", self.register_x);
    self.register_x = self.register_a;
    println!("Register X: 0x{:02X}", self.register_x);
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
