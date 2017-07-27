#[cfg(test)]
use super::super::Dcpu;

#[test]
fn instruction_jsr() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0x26 << 10) | (0x01 << 5) | 0x0; // jsr 5
   dcpu.step();
   assert_eq!(dcpu.program_counter, 5);
   assert_eq!(dcpu.stack_pointer, 0xffff);
   assert_eq!(dcpu.memory[0xffff], 1);
}

#[test]
fn instruction_iag() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0 << 10) | (0x09 << 5) | 0x0; // iag a
   dcpu.interrupt_address = 52;
   dcpu.step();
   assert_eq!(dcpu.registers[0], 52);
}

#[test]
fn instruction_ias() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0 << 10) | (0x0a << 5) | 0x0; // ias a
   dcpu.registers[0] = 39;
   dcpu.step();
   assert_eq!(dcpu.interrupt_address, 39);
}