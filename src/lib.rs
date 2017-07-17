use std::fmt::{Debug, Formatter, Error};

pub mod types;
use types::{SimpleOperand, ComplexOperand, BasicOperation, SpecialOperation, Instruction};



pub struct Dcpu {
   pub registers: [u16; 8],
   pub stack_pointer: u16,
   pub program_counter: u16,
   pub excess: u16,

   pub cycle_count: u64,
   pub cycle_accumulator: u16,

   pub memory: [u16; 0x10000],
}



impl Debug for Dcpu {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      writeln!(f, r"
         DCPU {{
            registers: {:?}
            pc: {:x} [{:x}]
            sp: {:x} [{:x}]
            excess: {:x}
            cycle_count: {}
            cycle_accumulator: {}
         }}",
         self.registers,
         self.program_counter,       self.memory[self.program_counter as usize],
         self.stack_pointer,         self.memory[self.stack_pointer as usize],
         self.excess,
         self.cycle_count,
         self.cycle_accumulator
      )
   }
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


   // Step the DCPU forward one tick
   pub fn step(&mut self) {
      if self.cycle_accumulator == 0 {
         match Instruction::decode(self.memory[self.program_counter as usize]) {
            Some(Instruction::Basic(operation, operand_b, operand_a)) => self.basic_operation(operation, operand_b, operand_a),
            Some(Instruction::Special(operation, operand)) => self.special_operation(operation, operand),
            None => panic!("Unknown Instruction: {:?}", self),
         }

         self.program_counter = self.program_counter.wrapping_add(1);
      }

      if self.cycle_accumulator > 0 {
         self.cycle_accumulator -= 1;
         self.cycle_count += 1;
      } else {
         println!("Cycle accumulator was 0. Has the DCPU Halted?")
      }
   }


   fn handle_mutation(&mut self, operand: SimpleOperand) {
      use SimpleOperand::*;
      match operand {
         Push => self.stack_pointer = self.stack_pointer.wrapping_sub(1),

         Pop  => {
            self.memory[self.stack_pointer as usize] = 0x0000;
            self.stack_pointer = self.stack_pointer.wrapping_add(1);
         },

         IndirectNextA
         | IndirectNextB
         | IndirectNextC
         | IndirectNextX
         | IndirectNextY
         | IndirectNextZ
         | IndirectNextI
         | IndirectNextJ
         | IndirectNextSP
         | IndirectNext
         | Next
         => self.program_counter += 1,

         _ => (),
      }
   }


   // Execute a basic operation
   fn basic_operation(&mut self, operation: BasicOperation, operand_b: SimpleOperand, operand_a: ComplexOperand) {
      let value_a = match operand_a {
         ComplexOperand::Literal(val) => val,
         ComplexOperand::Simple(operand) => {
            let value = self.get_value(operand);
            self.handle_mutation(operand);
            value
         }
      };
      let value_b = self.get_value(operand_b);
      let value_a_signed = value_a as i16;
      let value_b_signed = value_b as i16;


      use BasicOperation::*;
      match operation {
         Set => {
            self.cycle_accumulator += 1;
            *self.get_pointer(operand_b) = value_a;
         },

         Add => {
            self.cycle_accumulator += 2;
            *self.get_pointer(operand_b) = value_b.wrapping_add(value_a);
            self.excess = if (value_b as u32 + value_a as u32) > 0xffff {1} else {0};
         },

         Sub => {
            self.cycle_accumulator += 2;
            *self.get_pointer(operand_b) = value_b.wrapping_sub(value_a);
            self.excess = if (value_b as i32 - value_a as i32) < 0 {0xffff} else {0};
         },

         Mul => {
            self.cycle_accumulator += 2;
            *self.get_pointer(operand_b) = value_b.wrapping_mul(value_a);
            self.excess = ((value_b as u32 * value_a as u32) >> 16) as u16;
         },

         Mli => {
            self.cycle_accumulator += 2;
            *self.get_pointer(operand_b) = value_b_signed.wrapping_mul(value_a_signed) as u16;
            self.excess = ((value_b as i32 * value_a as i32) >> 16) as u16;
         },

         Div => {
            self.cycle_accumulator += 3;

            if value_a == 0 {
               *self.get_pointer(operand_b) = 0;
               self.excess = 0;
            } else {
               *self.get_pointer(operand_b) = value_b.wrapping_div(value_a);
               self.excess = (((value_b as i32) << 16i32) / (value_a as i32)) as u16;
            }
         },

         Dvi => {
            self.cycle_accumulator += 3;

            if value_a_signed == 0 {
               *self.get_pointer(operand_b) = 0;
               self.excess = 0;
            } else {
               *self.get_pointer(operand_b) = value_b_signed.wrapping_div(value_a_signed) as u16;
               self.excess = (((value_b_signed as i32) << 16) / (value_a_signed as i32)) as u16;
            }
         },

         Mod => {
            self.cycle_accumulator += 3;

            *self.get_pointer(operand_b) = if value_a == 0 {
               0
            } else {
               value_b.wrapping_rem(value_a)
            }
         },

         Mdi => {
            self.cycle_accumulator += 3;

            if value_a == 0 {
               *self.get_pointer(operand_b) = 0;
            } else {
               *self.get_pointer(operand_b) = (value_b_signed % value_a_signed) as u16;
            }
         },

         And => {
            self.cycle_accumulator += 1;
            *self.get_pointer(operand_b) = value_b & value_a;
         },

         Bor => {
            self.cycle_accumulator += 1;
            *self.get_pointer(operand_b) = value_b | value_a;
         },

         Xor => {
            self.cycle_accumulator += 1;
            *self.get_pointer(operand_b) = value_b ^ value_a;
         },

         Shr => {
            self.cycle_accumulator += 1;
            *self.get_pointer(operand_b) = value_b >> value_a;
            self.excess = (((value_b as u32) << 16) >> value_a) as u16;
         },

         Asr => {
            self.cycle_accumulator += 1;
            *self.get_pointer(operand_b) = ((value_b as i16) >> value_a) as u16;
            self.excess = (((value_b as i32) << 16) >> value_a) as u16;
         },

         Shl => {
            self.cycle_accumulator += 1;
            *self.get_pointer(operand_b) = value_b << value_a;
         },

         Ifb => {
            self.cycle_accumulator += 2;

            if value_b & value_a != 0 {} else {
               self.consume_ifs();
            }
         },

         Ifc => {
            self.cycle_accumulator += 2;

            if value_b & value_a == 0 {} else {
               self.consume_ifs();
            }
         },

         Ife => {
            self.cycle_accumulator += 2;

            if value_b == value_a {} else {
               self.consume_ifs();
            }
         },

         Ifn => {
            self.cycle_accumulator += 2;

            if value_b != value_a {} else {
               self.consume_ifs();
            }
         },

         Ifg => {
            self.cycle_accumulator += 2;

            if value_b > value_a {} else {
               self.consume_ifs();
            }
         },

         Ifa => {
            self.cycle_accumulator += 2;

            if value_b_signed > value_b_signed {} else {
               self.consume_ifs();
            }
         },

         Ifl => {
            self.cycle_accumulator += 2;

            if value_b < value_a {} else {
               self.consume_ifs();
            }
         },

         Ifu => {
            self.cycle_accumulator += 2;

            if value_b_signed < value_b_signed {} else {
               self.consume_ifs();
            }
         },

         Adx => {
            self.cycle_accumulator += 3;
            let result = value_b as u32 + value_a as u32 + self.excess as u32;
            *self.get_pointer(operand_b) = result as u16;
            self.excess = if result > 0xffff {1} else {0};
         },

         Sbx => {
            self.cycle_accumulator += 3;
            let result = value_b as i32 - value_a as i32 + self.excess as i32;
            *self.get_pointer(operand_b) = result as u16;
            self.excess = if result < 0 {0xffff} else {0};
         },

         Sti => {
            *self.get_pointer(operand_b) = value_a;
            self.registers[6] += 1;
            self.registers[7] += 1;
         },

         Std => {
            *self.get_pointer(operand_b) = value_a;
            self.registers[6] -= 1;
            self.registers[7] -= 1;
         },
      }

      self.handle_mutation(operand_b);
   }


   // Execute a special operation
   fn special_operation(&mut self, operation: SpecialOperation, operand: ComplexOperand) {
      let value = match operand {
         ComplexOperand::Literal(val) => val as u16,
         ComplexOperand::Simple(operand) => self.get_value(operand),
      };


      use SpecialOperation::*;
      match operation {
         Jsr => {
            self.cycle_accumulator += 3;
            self.stack_pointer = self.stack_pointer.wrapping_sub(1);
            self.memory[self.stack_pointer as usize] = self.program_counter + 1;
            self.program_counter = value.wrapping_sub(1);
         },

         _ => unimplemented!(),
      }
   }



   // Increment the program counter and cycle accumulator until a non-if instruction is found
   fn consume_ifs(&mut self) {
      use Instruction::Basic;
      use BasicOperation::*;

      loop {
         match Instruction::decode(self.memory[self.program_counter as usize]) {
              Some(Basic(Ifb, _, _))
            | Some(Basic(Ifc, _, _))
            | Some(Basic(Ife, _, _))
            | Some(Basic(Ifn, _, _))
            | Some(Basic(Ifg, _, _))
            | Some(Basic(Ifa, _, _))
            | Some(Basic(Ifl, _, _))
            | Some(Basic(Ifu, _, _)) => {
               self.program_counter += 1;
               self.cycle_accumulator += 1;
            },

            _ => break,
         }
      }
   }



   // Get a pointer to the provided operand (Does not mutate state)
   fn get_pointer<'a>(&'a mut self, operand: SimpleOperand) -> &'a mut u16 {
      let next_word = self.memory[(self.program_counter + 1) as usize];

      use SimpleOperand::*;
      match operand {
         A              => &mut self.registers[0],
         B              => &mut self.registers[1],
         C              => &mut self.registers[2],
         X              => &mut self.registers[3],
         Y              => &mut self.registers[4],
         Z              => &mut self.registers[5],
         I              => &mut self.registers[6],
         J              => &mut self.registers[7],
         IndirectA      => &mut self.memory[self.registers[0] as usize],
         IndirectB      => &mut self.memory[self.registers[1] as usize],
         IndirectC      => &mut self.memory[self.registers[2] as usize],
         IndirectX      => &mut self.memory[self.registers[3] as usize],
         IndirectY      => &mut self.memory[self.registers[4] as usize],
         IndirectZ      => &mut self.memory[self.registers[5] as usize],
         IndirectI      => &mut self.memory[self.registers[6] as usize],
         IndirectJ      => &mut self.memory[self.registers[7] as usize],
         IndirectNextA  => &mut self.memory[(self.registers[0] + next_word) as usize],
         IndirectNextB  => &mut self.memory[(self.registers[1] + next_word) as usize],
         IndirectNextC  => &mut self.memory[(self.registers[2] + next_word) as usize],
         IndirectNextX  => &mut self.memory[(self.registers[3] + next_word) as usize],
         IndirectNextY  => &mut self.memory[(self.registers[4] + next_word) as usize],
         IndirectNextZ  => &mut self.memory[(self.registers[5] + next_word) as usize],
         IndirectNextI  => &mut self.memory[(self.registers[6] + next_word) as usize],
         IndirectNextJ  => &mut self.memory[(self.registers[7] + next_word) as usize],
         Push           => &mut self.memory[self.stack_pointer.wrapping_sub(1) as usize],
         Pop            => &mut self.memory[self.stack_pointer as usize],
         IndirectSP     => &mut self.memory[self.stack_pointer as usize],
         IndirectNextSP => &mut self.memory[(self.stack_pointer + next_word) as usize],
         SP             => &mut self.stack_pointer,
         PC             => &mut self.program_counter,
         EX             => &mut self.excess,
         IndirectNext   => &mut self.memory[next_word as usize],
         Next           => &mut self.memory[(self.program_counter + 1) as usize],
      }
   }


   // Get the value of the provided operand (Mutates the cycle counter and program counter as necessary)
   fn get_value(&mut self, operand: SimpleOperand) -> u16 {
      *self.get_pointer(operand)
   }
}





#[test]
fn test_set() {
   // Set a, 0x9
   let mut dcpu = Dcpu::new();
   dcpu.registers[0] = 5;
   dcpu.memory[0] = 0xa801;
   dcpu.step();
   assert_eq!(dcpu.registers[0], 9);
}

#[test]
fn test_add() {
   // Add a, 0x9
   let mut dcpu = Dcpu::new();
   dcpu.registers[0] = 5;
   dcpu.memory[0] = 0xa802;
   dcpu.step();
   assert_eq!(dcpu.registers[0], 14);
}

#[test]
fn test_sub() {
   // Sub a, 0x9
   let mut dcpu = Dcpu::new();
   dcpu.registers[0] = 5;
   dcpu.memory[0] = 0xa803;
   dcpu.step();
   assert_eq!(dcpu.registers[0], 0xfffc);
}

#[test]
fn test_mul() {
   // Mul a, 0x9
   let mut dcpu = Dcpu::new();
   dcpu.registers[0] = 5;
   dcpu.memory[0] = 0xa804;
   dcpu.step();
   assert_eq!(dcpu.registers[0], 45);
}

#[test]
fn test_mli() {
   // Mli a, 0x9
   let mut dcpu = Dcpu::new();
   dcpu.registers[0] = 5;
   dcpu.memory[0] = 0xa805;
   dcpu.step();
   assert_eq!(dcpu.registers[0], 45);
}

#[test]
fn test_div() {
   // Div a, 0x9
   let mut dcpu = Dcpu::new();
   dcpu.registers[0] = 5;
   dcpu.memory[0] = 0xa806;
   dcpu.step();
   assert_eq!(dcpu.registers[0], 0);
}

#[test]
fn test_dvi() {
   // Dvi a, 0x9
   let mut dcpu = Dcpu::new();
   dcpu.registers[0] = 5;
   dcpu.memory[0] = 0xa807;
   dcpu.step();
   assert_eq!(dcpu.registers[0], 0);
}

#[test]
fn test_mod() {
   // Mod a, 0x9
   let mut dcpu = Dcpu::new();
   dcpu.registers[0] = 5;
   dcpu.memory[0] = 0xa808;
   dcpu.step();
   assert_eq!(dcpu.registers[0], 5);
}

#[test]
fn test_mdi() {
   // Mdi a, 0x9
   let mut dcpu = Dcpu::new();
   dcpu.registers[0] = 5;
   dcpu.memory[0] = 0xa809;
   dcpu.step();
   assert_eq!(dcpu.registers[0], 5);
}

#[test]
fn test_and() {
   // And a, 0x9
   let mut dcpu = Dcpu::new();
   dcpu.registers[0] = 5;
   dcpu.memory[0] = 0xa80a;
   dcpu.step();
   assert_eq!(dcpu.registers[0], 1);
}

#[test]
fn test_bor() {
   // Bor a, 0x9
   let mut dcpu = Dcpu::new();
   dcpu.registers[0] = 5;
   dcpu.memory[0] = 0xa80b;
   dcpu.step();
   assert_eq!(dcpu.registers[0], 0xd);
}

#[test]
fn test_xor() {
   // Xor a, 0x9
   let mut dcpu = Dcpu::new();
   dcpu.registers[0] = 5;
   dcpu.memory[0] = 0xa80c;
   dcpu.step();
   assert_eq!(dcpu.registers[0], 0xc);
}

#[test]
fn test_shr() {
   // Shr a, 0x9
   let mut dcpu = Dcpu::new();
   dcpu.registers[0] = 5;
   dcpu.memory[0] = 0xa80d;
   dcpu.step();
   assert_eq!(dcpu.registers[0], 0);
}

#[test]
fn test_asr() {
   // Asr a, 0x9
   let mut dcpu = Dcpu::new();
   dcpu.registers[0] = 5;
   dcpu.memory[0] = 0xa80e;
   dcpu.step();
   assert_eq!(dcpu.registers[0], 0);
}
   
#[test]
fn test_shl() {
   // Shl a, 0x9
   let mut dcpu = Dcpu::new();
   dcpu.registers[0] = 5;
   dcpu.memory[0] = 0xa80f;
   dcpu.step();
   assert_eq!(dcpu.registers[0], 0x0a00);
}
