#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SpecialOperation {
   Jsr,
   Int,
   Iag,
   Ias,
   Rfi,
   Iaq,
   Hwn,
   Hwq,
   Hwi,
}


impl SpecialOperation {
   pub fn decode(value: u16) -> Option<Self> {
      match value {
         0x01 => Some(SpecialOperation::Jsr),
         0x08 => Some(SpecialOperation::Int),
         0x09 => Some(SpecialOperation::Iag),
         0x0a => Some(SpecialOperation::Ias),
         0x0b => Some(SpecialOperation::Rfi),
         0x0c => Some(SpecialOperation::Iaq),
         0x10 => Some(SpecialOperation::Hwn),
         0x11 => Some(SpecialOperation::Hwq),
         0x12 => Some(SpecialOperation::Hwi),
         _ => None
      }
   }
}
