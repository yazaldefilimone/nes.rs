use nes::cpu::CPU;
#[test]
fn test_0xa9_lda_immediate_load_data() {
  let mut cpu = CPU::new();
  let program = vec![0xa9, 0x05, 0x00];
  cpu.run(&program);
  assert_eq!(cpu.register_a, 0x05);
  assert!(cpu.status & 0b0000_0010 == 0b00);
  assert!(cpu.status & 0b1000_0000 == 0);
}

#[test]
fn test_0xa9_lda_zero_flag() {
  let mut cpu = CPU::new();
  let program = vec![0xa9, 0x00, 0x00];
  cpu.run(&program);
  assert!(cpu.status & 0b0000_0010 == 0b10);
}

#[test]
fn test_0xaa_tax_move_a_to_x() {
  let mut cpu = CPU::new();
  cpu.register_a = 10;
  let program = vec![0xaa, 0x00];
  cpu.run(&program);
  assert_eq!(cpu.register_x, 10)
}

#[test]
fn test_5_ops_working_together() {
  let mut cpu = CPU::new();
  cpu.run(&vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

  assert_eq!(cpu.register_x, 0xc1)
}

#[test]
fn test_inx_overflow() {
  let mut cpu = CPU::new();
  cpu.register_x = 0xff;
  cpu.run(&vec![0xe8, 0xe8, 0x00]);

  assert_eq!(cpu.register_x, 1)
}
