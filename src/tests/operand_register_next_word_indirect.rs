/*
Tests for operands 0x10-0x17 (Register + Next Word Indirect, [a + 0x5]), and 0x19 ([sp + 0x5])
*/

#[cfg(test)]
use super::super::Dcpu;

#[test]
fn register_next_word_indirect_a() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (16 << 10) | (16 << 5) | 0x2;
   dcpu.memory[1] = 6;
   dcpu.memory[2] = 5;
   dcpu.memory[15] = 2;
   dcpu.memory[16] = 3;
   dcpu.registers[0] = 10;
   dcpu.step();
   assert_eq!(dcpu.memory[15], 5);
}

#[test]
fn register_next_word_indirect_b() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (17 << 10) | (17 << 5) | 0x2;
   dcpu.memory[1] = 6;
   dcpu.memory[2] = 5;
   dcpu.memory[15] = 2;
   dcpu.memory[16] = 3;
   dcpu.registers[1] = 10;
   dcpu.step();
   assert_eq!(dcpu.memory[15], 5);
}

#[test]
fn register_next_word_indirect_c() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (18 << 10) | (18 << 5) | 0x2;
   dcpu.memory[1] = 6;
   dcpu.memory[2] = 5;
   dcpu.memory[15] = 2;
   dcpu.memory[16] = 3;
   dcpu.registers[2] = 10;
   dcpu.step();
   assert_eq!(dcpu.memory[15], 5);
}

#[test]
fn register_next_word_indirect_x() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (19 << 10) | (19 << 5) | 0x2;
   dcpu.memory[1] = 6;
   dcpu.memory[2] = 5;
   dcpu.memory[15] = 2;
   dcpu.memory[16] = 3;
   dcpu.registers[3] = 10;
   dcpu.step();
   assert_eq!(dcpu.memory[15], 5);
}

#[test]
fn register_next_word_indirect_y() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (20 << 10) | (20 << 5) | 0x2;
   dcpu.memory[1] = 6;
   dcpu.memory[2] = 5;
   dcpu.memory[15] = 2;
   dcpu.memory[16] = 3;
   dcpu.registers[4] = 10;
   dcpu.step();
   assert_eq!(dcpu.memory[15], 5);
}

#[test]
fn register_next_word_indirect_z() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (21 << 10) | (21 << 5) | 0x2;
   dcpu.memory[1] = 6;
   dcpu.memory[2] = 5;
   dcpu.memory[15] = 2;
   dcpu.memory[16] = 3;
   dcpu.registers[5] = 10;
   dcpu.step();
   assert_eq!(dcpu.memory[15], 5);
}

#[test]
fn register_next_word_indirect_i() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (22 << 10) | (22 << 5) | 0x2;
   dcpu.memory[1] = 6;
   dcpu.memory[2] = 5;
   dcpu.memory[15] = 2;
   dcpu.memory[16] = 3;
   dcpu.registers[6] = 10;
   dcpu.step();
   assert_eq!(dcpu.memory[15], 5);
}

#[test]
fn register_next_word_indirect_j() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (23 << 10) | (23 << 5) | 0x2;
   dcpu.memory[1] = 6;
   dcpu.memory[2] = 5;
   dcpu.memory[15] = 2;
   dcpu.memory[16] = 3;
   dcpu.registers[7] = 10;
   dcpu.step();
   assert_eq!(dcpu.memory[15], 5);
}

#[test]
fn register_next_word_indirect_sp() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0x1a << 10) | (0x1a << 5) | 0x2; // add [sp + 3], [sp + 2]
   dcpu.memory[1] = 2;
   dcpu.memory[2] = 3;
   dcpu.stack_pointer = 50;
   dcpu.memory[52] = 20;
   dcpu.memory[53] = 10;
   dcpu.step();
   assert_eq!(dcpu.memory[53], 30);
}