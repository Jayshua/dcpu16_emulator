/*
Tests for operands 0x8-0xf (Register Indirect, [a]), 0x19 ([sp])
*/

#[cfg(test)]
use super::super::Dcpu;

#[test]
fn register_indirect_a() {
   let mut dcpu = Dcpu::new();
   dcpu.registers[0] = 50;
   dcpu.memory[0] = (8 << 10) | (8 << 5) | 0x2;
   dcpu.memory[50] = 23;
   dcpu.step();
   assert_eq!(dcpu.memory[50], 46);
}

#[test]
fn register_indirect_b() {
   let mut dcpu = Dcpu::new();
   dcpu.registers[1] = 50;
   dcpu.memory[0] = (9 << 10) | (9 << 5) | 0x2;
   dcpu.memory[50] = 23;
   dcpu.step();
   assert_eq!(dcpu.memory[50], 46);
}

#[test]
fn register_indirect_c() {
   let mut dcpu = Dcpu::new();
   dcpu.registers[2] = 50;
   dcpu.memory[0] = (10 << 10) | (10 << 5) | 0x2;
   dcpu.memory[50] = 23;
   dcpu.step();
   assert_eq!(dcpu.memory[50], 46);
}

#[test]
fn register_indirect_x() {
   let mut dcpu = Dcpu::new();
   dcpu.registers[3] = 50;
   dcpu.memory[0] = (11 << 10) | (11 << 5) | 0x2;
   dcpu.memory[50] = 23;
   dcpu.step();
   assert_eq!(dcpu.memory[50], 46);
}

#[test]
fn register_indirect_y() {
   let mut dcpu = Dcpu::new();
   dcpu.registers[4] = 50;
   dcpu.memory[0] = (12 << 10) | (12 << 5) | 0x2;
   dcpu.memory[50] = 23;
   dcpu.step();
   assert_eq!(dcpu.memory[50], 46);
}

#[test]
fn register_indirect_z() {
   let mut dcpu = Dcpu::new();
   dcpu.registers[5] = 50;
   dcpu.memory[0] = (13 << 10) | (13 << 5) | 0x2;
   dcpu.memory[50] = 23;
   dcpu.step();
   assert_eq!(dcpu.memory[50], 46);
}

#[test]
fn register_indirect_i() {
   let mut dcpu = Dcpu::new();
   dcpu.registers[6] = 50;
   dcpu.memory[0] = (14 << 10) | (14 << 5) | 0x2;
   dcpu.memory[50] = 23;
   dcpu.step();
   assert_eq!(dcpu.memory[50], 46);
}

#[test]
fn register_indirect_j() {
   let mut dcpu = Dcpu::new();
   dcpu.registers[7] = 50;
   dcpu.memory[0] = (15 << 10) | (15 << 5) | 0x2;
   dcpu.memory[50] = 23;
   dcpu.step();
   assert_eq!(dcpu.memory[50], 46);
}

#[test]
fn register_indirect_sp() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0x19 << 10) | (0x19 << 5) | 0x2; // add [sp], [sp]
   dcpu.stack_pointer = 50;
   dcpu.memory[50] = 20;
   dcpu.step();
   assert_eq!(dcpu.memory[50], 40);
}