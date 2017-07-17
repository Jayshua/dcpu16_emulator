


/// The b value of a DCPU instruction
/// The a value of a DCPU instruction can be a literal, whereas the b value can not.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SimpleOperand {
   A,
   B,
   C,
   X,
   Y,
   Z,
   I,
   J,
   IndirectA,
   IndirectB,
   IndirectC,
   IndirectX,
   IndirectY,
   IndirectZ,
   IndirectI,
   IndirectJ,
   IndirectNextA,
   IndirectNextB,
   IndirectNextC,
   IndirectNextX,
   IndirectNextY,
   IndirectNextZ,
   IndirectNextI,
   IndirectNextJ,
   Push,
   Pop,
   IndirectSP,
   IndirectNextSP,
   SP,
   PC,
   EX,
   IndirectNext,
   Next,
}


impl SimpleOperand {
   pub fn decode(value: u16, use_push: bool) -> Option<Self> {
      match value {
         0x00 => Some(SimpleOperand::A),
         0x01 => Some(SimpleOperand::B),
         0x02 => Some(SimpleOperand::C),
         0x03 => Some(SimpleOperand::X),
         0x04 => Some(SimpleOperand::Y),
         0x05 => Some(SimpleOperand::Z),
         0x06 => Some(SimpleOperand::I),
         0x07 => Some(SimpleOperand::J),
         0x08 => Some(SimpleOperand::IndirectA),
         0x09 => Some(SimpleOperand::IndirectB),
         0x0a => Some(SimpleOperand::IndirectC),
         0x0b => Some(SimpleOperand::IndirectX),
         0x0c => Some(SimpleOperand::IndirectY),
         0x0d => Some(SimpleOperand::IndirectZ),
         0x0e => Some(SimpleOperand::IndirectI),
         0x0f => Some(SimpleOperand::IndirectJ),
         0x10 => Some(SimpleOperand::IndirectNextA),
         0x11 => Some(SimpleOperand::IndirectNextB),
         0x12 => Some(SimpleOperand::IndirectNextC),
         0x13 => Some(SimpleOperand::IndirectNextX),
         0x14 => Some(SimpleOperand::IndirectNextY),
         0x15 => Some(SimpleOperand::IndirectNextZ),
         0x16 => Some(SimpleOperand::IndirectNextI),
         0x17 => Some(SimpleOperand::IndirectNextJ),
         0x18 if  use_push => Some(SimpleOperand::Push),
         0x18 if !use_push => Some(SimpleOperand::Pop),
         0x19 => Some(SimpleOperand::IndirectSP),
         0x1a => Some(SimpleOperand::IndirectNextSP),
         0x1b => Some(SimpleOperand::SP),
         0x1c => Some(SimpleOperand::PC),
         0x1d => Some(SimpleOperand::EX),
         0x1e => Some(SimpleOperand::IndirectNext),
         0x1f => Some(SimpleOperand::Next),
         _ => None,
      }
   }
}


