/*
Tests for operands 0x1e (next word indirect), 0x1f (next word literal), and 0x20-0x3f (embedded literal)
*/

#[cfg(test)]
use super::super::Dcpu;

/** Next Word Indirect **/
#[test]
fn next_word_indirect() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0x1e << 10) | (0x1e << 5) | 0x2; // add [11], [12]
   dcpu.memory[1] = 12;
   dcpu.memory[2] = 11;
   dcpu.memory[11] = 20;
   dcpu.memory[12] = 10;
   dcpu.step();
   assert_eq!(dcpu.memory[11], 30);
}


/** Next word literal **/
#[test]
fn next_word_literal() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0x1f << 10) | (0 << 5) | 0x2; // add a, 50
   dcpu.memory[1] = 50;
   dcpu.registers[0] = 20;
   dcpu.step();
   assert_eq!(dcpu.registers[0], 70);
}


/** Embedded Literal **/
#[test]
fn literal() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0x20 << 10) | (0 << 5) | 0x2; // add a, -1
   dcpu.registers[0] = 20;
   dcpu.step();
   assert_eq!(dcpu.registers[0], 19);
}
