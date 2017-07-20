/*
Tests for operands 0x0-0x7 (Register direct, a), 0x1b, 0x1c, 0x1d (sp, pc, ex respectively)
*/

#[cfg(test)]
use super::super::Dcpu;

#[test]
fn register_direct_a() {
   let mut dcpu = Dcpu::new();
   dcpu.registers[0] = 50;
   dcpu.memory[0] = (0 << 10) | (0 << 5) | 0x2;
   dcpu.step();
   assert_eq!(dcpu.registers[0], 100);
}

#[test]
fn register_direct_b() {
   let mut dcpu = Dcpu::new();
   dcpu.registers[1] = 50;
   dcpu.memory[0] = (1 << 10) | (1 << 5) | 0x2;
   dcpu.step();
   assert_eq!(dcpu.registers[1], 100);
}

#[test]
fn register_direct_c() {
   let mut dcpu = Dcpu::new();
   dcpu.registers[2] = 50;
   dcpu.memory[0] = (2 << 10) | (2 << 5) | 0x2;
   dcpu.step();
   assert_eq!(dcpu.registers[2], 100);
}

#[test]
fn register_direct_x() {
   let mut dcpu = Dcpu::new();
   dcpu.registers[3] = 50;
   dcpu.memory[0] = (3 << 10) | (3 << 5) | 0x2;
   dcpu.step();
   assert_eq!(dcpu.registers[3], 100);
}

#[test]
fn register_direct_y() {
   let mut dcpu = Dcpu::new();
   dcpu.registers[4] = 50;
   dcpu.memory[0] = (4 << 10) | (4 << 5) | 0x2;
   dcpu.step();
   assert_eq!(dcpu.registers[4], 100);
}

#[test]
fn register_direct_z() {
   let mut dcpu = Dcpu::new();
   dcpu.registers[5] = 50;
   dcpu.memory[0] = (5 << 10) | (5 << 5) | 0x2;
   dcpu.step();
   assert_eq!(dcpu.registers[5], 100);
}

#[test]
fn register_direct_i() {
   let mut dcpu = Dcpu::new();
   dcpu.registers[6] = 50;
   dcpu.memory[0] = (6 << 10) | (6 << 5) | 0x2;
   dcpu.step();
   assert_eq!(dcpu.registers[6], 100);
}

#[test]
fn register_direct_j() {
   let mut dcpu = Dcpu::new();
   dcpu.registers[7] = 50;
   dcpu.memory[0] = (7 << 10) | (7 << 5) | 0x2;
   dcpu.step();
   assert_eq!(dcpu.registers[7], 100);
}

#[test]
fn register_direct_sp() {
   let mut dcpu = Dcpu::new();
   dcpu.stack_pointer = 50;
   dcpu.memory[0] = (0x1b << 10) | (0x1b << 5) | 0x2;
   dcpu.step();
   assert_eq!(dcpu.stack_pointer, 100);
}

#[test]
fn register_direct_pc() {
   let mut dcpu = Dcpu::new();
   dcpu.program_counter = 50;
   dcpu.memory[50] = (0x1c << 10) | (0x1c << 5) | 0x2;
   dcpu.step();
   assert_eq!(dcpu.program_counter, 101); // The program counter increments at the end of the step operation
}

#[test]
fn register_direct_ex() {
   let mut dcpu = Dcpu::new();
   dcpu.excess = 50;
   dcpu.memory[0] = (0x1d << 10) | (0 << 5) | 0x2; // add a, ex
   dcpu.registers[0] = 50;
   dcpu.step();
   assert_eq!(dcpu.registers[0], 100);
}

#[test]
// Since the add instruction sets ex to 1 in case of overflow, or 0 otherwise as a side effect,
// adding to ex should cause the result to be overwritten
fn register_direct_ex_second() {
   let mut dcpu = Dcpu::new();
   dcpu.excess = 50;
   dcpu.memory[0] = (0x1d << 10) | (0x1d << 5) | 0x2; // add ex, ex
   dcpu.registers[0] = 50;
   dcpu.step();
   assert_eq!(dcpu.excess, 0);
}
