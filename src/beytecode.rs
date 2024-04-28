#![allow(dead_code)]
// https://www.nesdev.org/obelisk-6502-guide/reference.html

pub const OPCODE_LDA: u8 = 0xA9; // Load Accumulator
pub const OPCODE_BRK: u8 = 0x00; // Force Interrupt
pub const OPCODE_TAX: u8 = 0xAA; // Transfer Accumulator to X
pub const OPCODE_INX: u8 = 0xe8; // Increment X Register
pub const HEX_2: u8 = 0b0000_0010;
pub const HEX_7: u8 = 0b1000_0000;
pub const HEX_128: u8 = 0b1000_0000;
pub const HEX_127: u8 = 0b0111_1111;
pub const HEX_253: u8 = 0b1111_1101;
