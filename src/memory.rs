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
    for (i, &byte) in program.iter().enumerate() {
      self.ram[self.start_address + i] = byte;
    }
    // self.ram[self.start_address..(self.start_address + program.len ())].copy_from_slice(&program[..]);
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
  */
  pub fn read_u16(&mut self, pos: u16) -> u16 {
    let lo = self.read(pos) as u16;
    let hi = self.read(pos + 1) as u16;
    (hi << 8) | (lo as u16)
  }
  pub fn write_u16(&mut self, pos: u16, data: u16) {
    let hi = (data >> 8) as u8;
    let lo = (data & 0xff) as u8;
    self.write(pos, lo);
    self.write(pos + 1, hi);
  }
}
