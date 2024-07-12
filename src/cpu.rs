use crate::memory::Memory;
use crate::opcode;

pub struct CPU {
  pub program_counter: u16,
  pub register_a: u8,
  pub register_x: u8,
  pub status: u8,
  pub mem: Memory,
}

pub enum AddressingMode {
  Immediate,
  ZeroPage,
  ZeroPageX,
  ZeroPageY,
  Absolute,
  AbsoluteX,
  AbsoluteY,
  IndirectX,
  IndirectY,
  NoneAddressing,
}

impl Default for CPU {
  fn default() -> Self {
    CPU { program_counter: 0, register_a: 0, register_x: 0, status: 0, mem: Memory::new() }
  }
}

impl CPU {
  pub fn new(mem: Memory) -> Self {
    CPU { program_counter: 0, register_a: 0, register_x: 0, status: 0, mem }
  }
  pub fn boot(&mut self, program: Vec<u8>) {
    self.mem.load_program(program);
    self.reset();
    self.run()
  }

  pub fn reset(&mut self) {
    // self.register_a = 0;
    // self.register_x = 0;
    // self.status = 0;
    self.program_counter = self.mem.read_u16(0xFFFC);
    println!("PC: 0x{:04X}", self.program_counter);
  }

  pub fn get_operand_address(&mut self, addressing_mode: AddressingMode) -> u16 {
    match addressing_mode {
      AddressingMode::Immediate => self.program_counter,
      AddressingMode::ZeroPage => self.mem.read(self.program_counter) as u16,
      AddressingMode::Absolute => self.mem.read_u16(self.program_counter),

      AddressingMode::ZeroPageX => self.zero_page_address(self.register_x),
      // AddressingMode::ZeroPageY => self.zero_page_address().wrapping_add(self.register_y as u16),
      AddressingMode::AbsoluteX => self.absolute_address(self.register_x),
      AddressingMode::AbsoluteY => self
        .mem
        .read_u16(self.program_counter)
        .wrapping_add(self.register_y as u16),
      AddressingMode::IndirectX => {
        let zero_page_address = self.mem.read(self.program_counter).wrapping_add(self.register_x) as u16;
        self.mem.read_u16(zero_page_address)
      }
      AddressingMode::IndirectY => {
        let zero_page_address = self.mem.read(self.program_counter) as u16;
        self
          .mem
          .read_u16(zero_page_address)
          .wrapping_add(self.register_y as u16)
      }
      AddressingMode::NoneAddressing => 0,
    }
  }

  pub fn zero_page_address(&mut self, adresse: u8) -> u16 {
    let base = self.mem.read(self.program_counter) as u16;
    let adress = base.wrapping_add(adresse as u16) as u16;
    adress
  }

  pub fn absolute_address(&mut self, adresse: u8) -> u16 {
    let base = self.mem.read_u16(self.program_counter);
    let adress = base.wrapping_add(adresse as u16) as u16;
    adress
  }

  pub fn indirect_address(&mut self) -> u16 {
    let pos = self.mem.read_u16(self.program_counter);
    let opr
  }

  pub fn run(&mut self) {
    loop {
      let opcode = self.mem.read(self.program_counter);
      self.program_counter += 1;
      println!("Opcode: 0x{:02X}", opcode);
      match opcode {
        opcode::OPCODE_LDA => self.load_accumulator(),
        opcode::OPCODE_BRK => return,
        opcode::OPCODE_TAX => self.transfer_accumulator(),
        opcode::OPCODE_INX => self.increment_register(),
        _ => {
          println!("Unrecognized opcode: {}", opcode);
          break;
        }
      }
    }
  }

  fn load_accumulator(&mut self) {
    let value = self.mem.read(self.program_counter);
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
      self.status |= opcode::HEX_2; // Set zero flag if A is zero
    } else {
      self.status &= opcode::HEX_253; // Clear zero flag if A is not zero
    }
    // Check if bit 7 is set
    if register & opcode::HEX_128 != 0 {
      self.status |= opcode::HEX_128; // Set negative flag if bit 7 is set
    } else {
      self.status &= opcode::HEX_127; // Clear negative flag if bit 7 is not set
    }
  }
}
