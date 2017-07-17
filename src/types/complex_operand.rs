use super::SimpleOperand;

/// The a value of a DCPU instruction
/// Can be a literal value where the a value of an instruction can not.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ComplexOperand {
   Literal(u16),
   Simple(SimpleOperand),
}

impl ComplexOperand {
   pub fn decode(value: u16) -> Option<Self> {
      match value {
         0x40...0xffff => None,
         0x20...0x3f   => Some(ComplexOperand::Literal((value as i16).wrapping_sub(0x21i16) as u16)),
         _ => {
            let operand = SimpleOperand::decode(value, false);

            match operand {
               Some(v) => Some(ComplexOperand::Simple(v)),
               None => None,
            }
         },
      }
   }
}



#[test]
fn decode_complex_operand() {
   assert_eq!(ComplexOperand::decode(0x20), Some(ComplexOperand::Literal(0xffff)));
   assert_eq!(ComplexOperand::decode(0x21), Some(ComplexOperand::Literal(0)));
   assert_eq!(ComplexOperand::decode(0x22), Some(ComplexOperand::Literal(1)));
   assert_eq!(ComplexOperand::decode(0x23), Some(ComplexOperand::Literal(2)));
   assert_eq!(ComplexOperand::decode(0x24), Some(ComplexOperand::Literal(3)));
   assert_eq!(ComplexOperand::decode(0x25), Some(ComplexOperand::Literal(4)));
   assert_eq!(ComplexOperand::decode(0x26), Some(ComplexOperand::Literal(5)));
   assert_eq!(ComplexOperand::decode(0x27), Some(ComplexOperand::Literal(6)));
   assert_eq!(ComplexOperand::decode(0x28), Some(ComplexOperand::Literal(7)));
   assert_eq!(ComplexOperand::decode(0x29), Some(ComplexOperand::Literal(8)));
   assert_eq!(ComplexOperand::decode(0x2a), Some(ComplexOperand::Literal(9)));
   assert_eq!(ComplexOperand::decode(0x2b), Some(ComplexOperand::Literal(10)));
   assert_eq!(ComplexOperand::decode(0x2c), Some(ComplexOperand::Literal(11)));
   assert_eq!(ComplexOperand::decode(0x2d), Some(ComplexOperand::Literal(12)));
   assert_eq!(ComplexOperand::decode(0x2e), Some(ComplexOperand::Literal(13)));
   assert_eq!(ComplexOperand::decode(0x2f), Some(ComplexOperand::Literal(14)));
   assert_eq!(ComplexOperand::decode(0x30), Some(ComplexOperand::Literal(15)));
   assert_eq!(ComplexOperand::decode(0x31), Some(ComplexOperand::Literal(16)));
   assert_eq!(ComplexOperand::decode(0x32), Some(ComplexOperand::Literal(17)));
   assert_eq!(ComplexOperand::decode(0x33), Some(ComplexOperand::Literal(18)));
   assert_eq!(ComplexOperand::decode(0x34), Some(ComplexOperand::Literal(19)));
   assert_eq!(ComplexOperand::decode(0x35), Some(ComplexOperand::Literal(20)));
   assert_eq!(ComplexOperand::decode(0x36), Some(ComplexOperand::Literal(21)));
   assert_eq!(ComplexOperand::decode(0x37), Some(ComplexOperand::Literal(22)));
   assert_eq!(ComplexOperand::decode(0x38), Some(ComplexOperand::Literal(23)));
   assert_eq!(ComplexOperand::decode(0x39), Some(ComplexOperand::Literal(24)));
   assert_eq!(ComplexOperand::decode(0x3a), Some(ComplexOperand::Literal(25)));
   assert_eq!(ComplexOperand::decode(0x3b), Some(ComplexOperand::Literal(26)));
   assert_eq!(ComplexOperand::decode(0x3c), Some(ComplexOperand::Literal(27)));
   assert_eq!(ComplexOperand::decode(0x3d), Some(ComplexOperand::Literal(28)));
   assert_eq!(ComplexOperand::decode(0x3e), Some(ComplexOperand::Literal(29)));
   assert_eq!(ComplexOperand::decode(0x3f), Some(ComplexOperand::Literal(30)));
   assert_eq!(ComplexOperand::decode(0x40), None);
   assert_eq!(ComplexOperand::decode(0x1f), Some(ComplexOperand::Simple(SimpleOperand::Next)));
}