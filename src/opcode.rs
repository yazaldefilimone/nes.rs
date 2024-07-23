#![allow(dead_code)]
// https://www.nesdev.org/obelisk-6502-guide/reference.html

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Opcode {
  /// Load Accumulator
  LDA = 0xA9,
  /// Force Interrupt
  BRK = 0x00,
  /// Transfer Accumulator to X
  TAX = 0xAA,
  /// Increment X Register
  INX = 0xE8,
}

impl Opcode {
  pub fn from_u8(value: u8) -> Option<Self> {
    match value {
      0xA9 => Some(Opcode::LDA),
      0x00 => Some(Opcode::BRK),
      0xAA => Some(Opcode::TAX),
      0xE8 => Some(Opcode::INX),
      _ => None,
    }
  }
}
