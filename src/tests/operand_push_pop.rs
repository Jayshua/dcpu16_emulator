/*
Tests for operand 0x18 (Push and Pop)
*/

#[cfg(test)]
use super::super::Dcpu;

#[test]
fn push() {
   let mut dcpu = Dcpu::new();
   let instruction: u16 = (0 << 10) | (0x18 << 5) | 0x1; // set push, a
   dcpu.memory[0] = instruction;
   dcpu.registers[0] = 5;
   dcpu.step();
   assert_eq!(dcpu.memory[0], instruction);
   assert_eq!(dcpu.memory[0xffff], 5);
}

#[test]
fn pop() {
   let mut dcpu = Dcpu::new();
   let instruction: u16 = (0x18 << 10) | (0 << 5) | 0x1; // set a, pop
   dcpu.memory[0] = instruction;
   dcpu.memory[0xffff] = 15;
   dcpu.stack_pointer = 0xffff;
   dcpu.step();
   assert_eq!(dcpu.memory[0], instruction);
   assert_eq!(dcpu.registers[0], 15);
}

#[test]
fn push_pop() {
   let mut dcpu = Dcpu::new();
   let instruction: u16 = (0x18 << 10) | (0x18 << 5) | 0x1; // set push, pop
   dcpu.memory[0] = instruction;
   dcpu.memory[0xffff] = 0xbaad;
   dcpu.memory[1] = 0xf00d;
   dcpu.step();
   assert_eq!(dcpu.memory[0], instruction);
   assert_eq!(dcpu.memory[1], 0xf00d);
   assert_eq!(dcpu.memory[0xffff], 0xbaad);
   assert_eq!(dcpu.stack_pointer, 0);
}