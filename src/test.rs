mod Instructions {
   // Basic
   const SET: u16 = 0x01;
   const ADD: u16 = 0x02;
   const SUB: u16 = 0x03;
   const MUL: u16 = 0x04;
   const MLI: u16 = 0x05;
   const DIV: u16 = 0x06;
   const DVI: u16 = 0x07;
   const MOD: u16 = 0x08;
   const MDI: u16 = 0x09;
   const AND: u16 = 0x0a;
   const BOR: u16 = 0x0b;
   const XOR: u16 = 0x0c;
   const SHR: u16 = 0x0d;
   const ASR: u16 = 0x0e;
   const SHL: u16 = 0x0f;
   const IFB: u16 = 0x10;
   const IFC: u16 = 0x11;
   const IFE: u16 = 0x12;
   const IFN: u16 = 0x13;
   const IFG: u16 = 0x14;
   const IFA: u16 = 0x15;
   const IFL: u16 = 0x16;
   const IFU: u16 = 0x17;
   const ADX: u16 = 0x1a;
   const SBX: u16 = 0x1b;
   const STI: u16 = 0x1e;
   const STD: u16 = 0x1f;

   // Special
   const JSR: u16 = 0x01;
   const INT: u16 = 0x08;
   const IAG: u16 = 0x09;
   const IAS: u16 = 0x0a;
   const RFI: u16 = 0x0b;
   const IAQ: u16 = 0x0c;
   const HWN: u16 = 0x10;
   const HWQ: u16 = 0x11;
   const HWI: u16 = 0x12;

   // Operands
   const A: u16 = 0x0;
   const B: u16 = 0x1;
   const C: u16 = 0x2;
   const X: u16 = 0x3;
   const Y: u16 = 0x4;
   const Z: u16 = 0x5;
   const I: u16 = 0x6;
   const J: u16 = 0x7;
   const IN_A: u16 = 0x8;
   const IN_B: u16 = 0x9;
   const IN_C: u16 = 0xa;
   const IN_X: u16 = 0xb;
   const IN_Y: u16 = 0xc;
   const IN_Z: u16 = 0xd;
   const IN_I: u16 = 0xe;
   const IN_J: u16 = 0xf;
   const IN_NW_A: u16 = 0x10;
   const IN_NW_B: u16 = 0x11;
   const IN_NW_C: u16 = 0x12;
   const IN_NW_X: u16 = 0x13;
   const IN_NW_Y: u16 = 0x14;
   const IN_NW_Z: u16 = 0x15;
   const IN_NW_I: u16 = 0x16;
   const IN_NW_J: u16 = 0x17;
   const PUSH: u16 = 0x18;
   const POP: u16 = 0x18;
   const IN_SP: u16 = 0x19;
   const IN_NW_SP: u16 = 0x1a;
   const SP: u16 = 0x1b;
   const PC: u16 = 0x1c;
   const EX: u16 = 0x1d;
   const IN_NW: u16 = 0x1e;
   const NW: u16 = 0x1f;


   pub fn get_length(instruction: u16) {
      let (_, destination, value) = break_instruction(instruction);
      get_operand_length(operand_a) + get_operand_length(operand_b)
   }

   pub fn get_cost(instruction: u16) {
      let (operation, operand_b, operand_a)
   }

   fn break_instruction(instruction: u16) -> (u16, u16, u16) {
      let operand_a   = (op_code & 0b1111_1100_0000_0000) >> 10;
      let operand_b   = (op_code & 0b0000_0011_1110_0000) >> 05;
      let operation = (op_code & 0b0000_0000_0001_1111) >> 00;
      (operation, operand_b, operand_a)
   }

   fn get_operand_length(operand: u16) {
      match operand {
         0x10...0x17 | 0x1a | 0x1e => 1,
         _ => 0
      }
   }
}


pub struct Dcpu {
   pub registers: [u16; 8],
   pub stack_pointer: u16,
   pub program_counter: u16,
   pub excess: u16,

   pub cycle_count: u64,
   pub cycle_accumulator: u16,

   pub memory: [u16; 0x10000],
}



impl Dcpu {
   pub fn new() -> Dcpu {
      Dcpu {
         registers: [0; 8],
         stack_pointer: 0,
         program_counter: 0,
         excess: 0,
         cycle_count: 0,
         cycle_accumulator: 0,
         memory: [0; 0x10000],
      }
   }
