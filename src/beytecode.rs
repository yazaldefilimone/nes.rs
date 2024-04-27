#![allow(dead_code)]
pub const LDA_IMM: u8 = 0xA9; // Load A with immediate value
pub const LDA_ZP: u8 = 0xA5; // Load A with zero-page value
pub const LDA_ZPX: u8 = 0xB5; // Load A with zero-page value, X indexed
pub const LDA_ABS: u8 = 0xAD; // Load A with absolute value
pub const LDA_ABX: u8 = 0xBD; // Load A with absolute value, X indexed
pub const LDA_ABY: u8 = 0xB9; // Load A with absolute value, Y indexed
