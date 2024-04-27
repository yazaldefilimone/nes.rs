use nes::cpu;
#[test]
fn test_0xa9_lda_immediate_load_data() {
  let mut cpu = cpu::CPU::new();
  let program = vec![0xa9, 0x05, 0x00];
  cpu.interpret(&program);
  assert_eq!(cpu.register_a, 0x05);
  assert!(cpu.status & 0b0000_0010 == 0b00);
  assert!(cpu.status & 0b1000_0000 == 0);
}

#[test]
fn test_0xa9_lda_zero_flag() {
  let mut cpu = cpu::CPU::new();
  let program = vec![0xa9, 0x00, 0x00];
  cpu.interpret(&program);
  assert!(cpu.status & 0b0000_0010 == 0b10);
}
