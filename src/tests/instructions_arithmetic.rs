#[cfg(test)]
use super::super::Dcpu;

#[test]
fn instruction_set() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0 << 10) | (1 << 5) | 0x01; // set b, a
   dcpu.registers[0] = 10;
   dcpu.registers[1] = 5;
   dcpu.step();
   assert_eq!(dcpu.registers[1], 10);
}

#[test]
fn instruction_add() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0 << 10) | (1 << 5) | 0x02; // add b, a
   dcpu.registers[0] = 10;
   dcpu.registers[1] = 5;
   dcpu.step();
   assert_eq!(dcpu.registers[1], 15);
}

#[test]
fn instruction_sub() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0 << 10) | (1 << 5) | 0x03; // sub b, a
   dcpu.registers[0] = 10;
   dcpu.registers[1] = 5;
   dcpu.step();
   assert_eq!(dcpu.registers[1], -5i16 as u16);
}

#[test]
fn instruction_mul() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0 << 10) | (1 << 5) | 0x04; // mul b, a
   dcpu.registers[0] = 10;
   dcpu.registers[1] = 5;
   dcpu.step();
   assert_eq!(dcpu.registers[1], 50);
}

#[test]
fn instruction_mli() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0 << 10) | (1 << 5) | 0x05; // mli b, a
   dcpu.registers[0] = 10;
   dcpu.registers[1] = 5;
   dcpu.step();
   assert_eq!(dcpu.registers[1], 50);
}

#[test]
fn instruction_div() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0 << 10) | (1 << 5) | 0x06; // div b, a
   dcpu.registers[0] = 5;
   dcpu.registers[1] = 10;
   dcpu.step();
   assert_eq!(dcpu.registers[1], 2);
}

#[test]
fn instruction_dvi() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0 << 10) | (1 << 5) | 0x07; // dvi b, a
   dcpu.registers[0] = -5i16 as u16;
   dcpu.registers[1] = 10;
   dcpu.step();
   assert_eq!(dcpu.registers[1], 0xfffe);
}

#[test]
fn instruction_mod() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0 << 10) | (1 << 5) | 0x08; // mod b, a
   dcpu.registers[0] = 6;
   dcpu.registers[1] = 9;
   dcpu.step();
   assert_eq!(dcpu.registers[1], 3);
}

#[test]
fn instruction_mdi() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0 << 10) | (1 << 5) | 0x09; // mdi b, a
   dcpu.registers[0] = 5;
   dcpu.registers[1] = -7i16 as u16;
   dcpu.step();
   assert_eq!(dcpu.registers[1], 0xfffe);
}

#[test]
fn instruction_and() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0 << 10) | (1 << 5) | 0x0a; // and b, a
   dcpu.registers[0] = 10;
   dcpu.registers[1] = 5;
   dcpu.step();
   assert_eq!(dcpu.registers[1], 0);
}

#[test]
fn instruction_bor() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0 << 10) | (1 << 5) | 0x0b; // bor b, a
   dcpu.registers[0] = 10;
   dcpu.registers[1] = 5;
   dcpu.step();
   assert_eq!(dcpu.registers[1], 15);
}

#[test]
fn instruction_xor() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0 << 10) | (1 << 5) | 0x0c; // xor b, a
   dcpu.registers[0] = 10;
   dcpu.registers[1] = 5;
   dcpu.step();
   assert_eq!(dcpu.registers[1], 15);
}

#[test]
fn instruction_shr() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0 << 10) | (1 << 5) | 0x0d; // shr b, a
   dcpu.registers[0] = 10;
   dcpu.registers[1] = 5;
   dcpu.step();
   assert_eq!(dcpu.registers[1], 0);
}

#[test]
fn instruction_asr() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0 << 10) | (1 << 5) | 0x0e; // asr b, a
   dcpu.registers[0] = 10;
   dcpu.registers[1] = 5;
   dcpu.step();
   assert_eq!(dcpu.registers[1], 0);
}

#[test]
fn instruction_shl() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0 << 10) | (1 << 5) | 0x0f; // shl b, a
   dcpu.registers[0] = 10;
   dcpu.registers[1] = 5;
   dcpu.step();
   assert_eq!(dcpu.registers[1], 0x1400);
}

#[test]
fn instruction_adx() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0 << 10) | (1 << 5) | 0x1a; // adx b, a
   dcpu.registers[0] = 10;
   dcpu.registers[1] = 5;
   dcpu.excess = 13;
   dcpu.step();
   assert_eq!(dcpu.registers[1], 10 + 5 + 13);
}

#[test]
fn instruction_sbx() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0 << 10) | (1 << 5) | 0x1b; // sbx b, a
   dcpu.registers[0] = 10;
   dcpu.registers[1] = 5;
   dcpu.excess = 13;
   dcpu.step();
   assert_eq!(dcpu.registers[1], (5i16 - 10i16 + 13i16) as u16);
}

#[test]
fn instruction_sti() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0 << 10) | (1 << 5) | 0x1e; // sti b, a
   dcpu.registers[0] = 10;
   dcpu.registers[1] = 5;
   dcpu.step();
   assert_eq!(dcpu.registers[1], 10);
   assert_eq!(dcpu.registers[6], 1);
   assert_eq!(dcpu.registers[7], 1);
}

#[test]
fn instruction_std() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0 << 10) | (1 << 5) | 0x1f; // std b, a
   dcpu.registers[0] = 10;
   dcpu.registers[1] = 5;
   dcpu.step();
   assert_eq!(dcpu.registers[1], 10);
   assert_eq!(dcpu.registers[6], 0xffff);
   assert_eq!(dcpu.registers[7], 0xffff);
}
