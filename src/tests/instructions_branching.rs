#[cfg(test)]
use super::super::Dcpu;

#[test]
fn instruction_ifb_true() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0 << 10) | (1 << 5) | 0x10; // ifb b, a
   dcpu.registers[0] = 0b0101;
   dcpu.registers[1] = 0b1011;
   dcpu.step();
   assert_eq!(dcpu.program_counter, 1);
}

#[test]
fn instruction_ifb_false() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0 << 10) | (1 << 5) | 0x10; // ifb b, a
   dcpu.registers[0] = 0b0101;
   dcpu.registers[1] = 0b1010;
   dcpu.step();
   assert_eq!(dcpu.program_counter, 2);
}

#[test]
fn instruction_ifc_true() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0 << 10) | (1 << 5) | 0x11; // ifc b, a
   dcpu.registers[0] = 0b0101;
   dcpu.registers[1] = 0b1010;
   dcpu.step();
   assert_eq!(dcpu.program_counter, 1);
}

#[test]
fn instruction_ifc_false() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0 << 10) | (1 << 5) | 0x11; // ifc b, a
   dcpu.registers[0] = 0b0101;
   dcpu.registers[1] = 0b1011;
   dcpu.step();
   assert_eq!(dcpu.program_counter, 2);
}

#[test]
fn instruction_ife_true() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0 << 10) | (1 << 5) | 0x12; // ife b, a
   dcpu.registers[0] = 5;
   dcpu.registers[1] = 5;
   dcpu.step();
   assert_eq!(dcpu.program_counter, 1);
}

#[test]
fn instruction_ife_false() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0 << 10) | (1 << 5) | 0x12; // ife b, a
   dcpu.registers[0] = 5;
   dcpu.registers[1] = 4;
   dcpu.step();
   assert_eq!(dcpu.program_counter, 2);
}

#[test]
fn instruction_ifn_true() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0 << 10) | (1 << 5) | 0x13; // ifn b, a
   dcpu.registers[0] = 5;
   dcpu.registers[1] = 4;
   dcpu.step();
   assert_eq!(dcpu.program_counter, 1);
}

#[test]
fn instruction_ifn_false() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0 << 10) | (1 << 5) | 0x13; // ifn b, a
   dcpu.registers[0] = 5;
   dcpu.registers[1] = 5;
   dcpu.step();
   assert_eq!(dcpu.program_counter, 2);
}

#[test]
fn instruction_ifg_true() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0 << 10) | (1 << 5) | 0x14; // ifg b, a
   dcpu.registers[0] = 4;
   dcpu.registers[1] = 5;
   dcpu.step();
   assert_eq!(dcpu.program_counter, 1);
}

#[test]
fn instruction_ifg_false() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0 << 10) | (1 << 5) | 0x14; // ifg b, a
   dcpu.registers[0] = 5;
   dcpu.registers[1] = 4;
   dcpu.step();
   assert_eq!(dcpu.program_counter, 2);
}

#[test]
fn instruction_ifl_true() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0 << 10) | (1 << 5) | 0x16; // ifl b, a
   dcpu.registers[0] = 5;
   dcpu.registers[1] = 4;
   dcpu.step();
   assert_eq!(dcpu.program_counter, 1);
}

#[test]
fn instruction_ifl_false() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0 << 10) | (1 << 5) | 0x16; // ifl b, a
   dcpu.registers[0] = 4;
   dcpu.registers[1] = 5;
   dcpu.step();
   assert_eq!(dcpu.program_counter, 2);
}

#[test]
fn instruction_ifa_true() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0 << 10) | (1 << 5) | 0x15; // ifa b, a
   dcpu.registers[0] = -2i16 as u16; // 0xfffe
   dcpu.registers[1] = 5;
   dcpu.step();
   assert_eq!(dcpu.program_counter, 1);
}

#[test]
fn instruction_ifa_false() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0 << 10) | (1 << 5) | 0x15; // ifa b, a
   dcpu.registers[0] = 5;
   dcpu.registers[1] = -2i16 as u16; // 0xfffe
   dcpu.step();
   assert_eq!(dcpu.program_counter, 2);
}

#[test]
fn instruction_ifu_true() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0 << 10) | (1 << 5) | 0x17; // ifu b, a
   dcpu.registers[0] = 5;
   dcpu.registers[1] = -2i16 as u16;
   dcpu.step();
   assert_eq!(dcpu.program_counter, 1);
}

#[test]
fn instruction_ifu_false() {
   let mut dcpu = Dcpu::new();
   dcpu.memory[0] = (0 << 10) | (1 << 5) | 0x17; // ifu b, a
   dcpu.registers[0] = -2i16 as u16;
   dcpu.registers[1] = 5;
   dcpu.step();
   assert_eq!(dcpu.program_counter, 2);
}