#[cfg(test)]
use super::super::Dcpu;

#[test]
fn program_crc() {
   let mut dcpu = Dcpu::new();
   // This program calculates the CRC of a string.
   // It was found at https://pastebin.com/8wWk7h76 and assembled using Jayshua's dcpu16 assembler.
   // At the end of execution Z should be 0x749b
   dcpu.memory[0]   = 0x7c61;
   dcpu.memory[1]   = 0x8408;
   dcpu.memory[2]   = 0x7c81;
   dcpu.memory[3]   = 0x003b;
   dcpu.memory[4]   = 0x7cc1;
   dcpu.memory[5]   = 0x004f;
   dcpu.memory[6]   = 0x7c20;
   dcpu.memory[7]   = 0x0009;
   dcpu.memory[8]   = 0x0000;
   dcpu.memory[9]   = 0x1b01;
   dcpu.memory[10]  = 0x1f01;
   dcpu.memory[11]  = 0x0f01;
   dcpu.memory[12]  = 0x1301;
   dcpu.memory[13]  = 0x0301;
   dcpu.memory[14]  = 0x0701;
   dcpu.memory[15]  = 0x0b01;
   dcpu.memory[16]  = 0x8821;
   dcpu.memory[17]  = 0x30a1;
   dcpu.memory[18]  = 0x10e1;
   dcpu.memory[19]  = 0x18e2;
   dcpu.memory[20]  = 0x3ca1;
   dcpu.memory[21]  = 0x7c20;
   dcpu.memory[22]  = 0x002e;
   dcpu.memory[23]  = 0x88ad;
   dcpu.memory[24]  = 0x3c41;
   dcpu.memory[25]  = 0x044a;
   dcpu.memory[26]  = 0x8454;
   dcpu.memory[27]  = 0x7c20;
   dcpu.memory[28]  = 0x0029;
   dcpu.memory[29]  = 0x08ac;
   dcpu.memory[30]  = 0x882f;
   dcpu.memory[31]  = 0x8433;
   dcpu.memory[32]  = 0x7f81;
   dcpu.memory[33]  = 0x0015;
   dcpu.memory[34]  = 0x8821;
   dcpu.memory[35]  = 0x88e3;
   dcpu.memory[36]  = 0x10f6;
   dcpu.memory[37]  = 0x7f81;
   dcpu.memory[38]  = 0x0033;
   dcpu.memory[39]  = 0x7f81;
   dcpu.memory[40]  = 0x0015;
   dcpu.memory[41]  = 0x8852;
   dcpu.memory[42]  = 0x6381;
   dcpu.memory[43]  = 0x884d;
   dcpu.memory[44]  = 0x7f81;
   dcpu.memory[45]  = 0x0029;
   dcpu.memory[46]  = 0x1401;
   dcpu.memory[47]  = 0x880a;
   dcpu.memory[48]  = 0x8414;
   dcpu.memory[49]  = 0x0cac;
   dcpu.memory[50]  = 0x6381;
   dcpu.memory[51]  = 0x6041;
   dcpu.memory[52]  = 0x6021;
   dcpu.memory[53]  = 0x6001;
   dcpu.memory[54]  = 0x6081;
   dcpu.memory[55]  = 0x6061;
   dcpu.memory[56]  = 0x60e1;
   dcpu.memory[57]  = 0x60c1;
   dcpu.memory[58]  = 0x6381;
   dcpu.memory[59]  = 0x0054;
   dcpu.memory[60]  = 0x0065;
   dcpu.memory[61]  = 0x0073;
   dcpu.memory[62]  = 0x0074;
   dcpu.memory[63]  = 0x0020;
   dcpu.memory[64]  = 0x0074;
   dcpu.memory[65]  = 0x0073;
   dcpu.memory[66]  = 0x0065;
   dcpu.memory[67]  = 0x0054;
   dcpu.memory[68]  = 0x0020;
   dcpu.memory[69]  = 0x0054;
   dcpu.memory[70]  = 0x0065;
   dcpu.memory[71]  = 0x0073;
   dcpu.memory[72]  = 0x0074;
   dcpu.memory[73]  = 0x0020;
   dcpu.memory[74]  = 0x0074;
   dcpu.memory[75]  = 0x0073;
   dcpu.memory[76]  = 0x0065;
   dcpu.memory[77]  = 0x0054;
   dcpu.memory[78]  = 0x0020;
   dcpu.memory[79]  = 0x0054;
   dcpu.memory[80]  = 0x0065;
   dcpu.memory[81]  = 0x0073;
   dcpu.memory[82]  = 0x0074;
   dcpu.memory[83]  = 0x0020;
   dcpu.memory[84]  = 0x0074;
   dcpu.memory[85]  = 0x0073;
   dcpu.memory[86]  = 0x0065;
   dcpu.memory[87]  = 0x0054;
   dcpu.memory[88]  = 0x0020;
   dcpu.memory[89]  = 0x0074;
   dcpu.memory[90]  = 0x0073;
   dcpu.memory[91]  = 0x0065;
   dcpu.memory[92]  = 0x0054;
   dcpu.memory[93]  = 0x0020;
   dcpu.memory[94]  = 0x0074;
   dcpu.memory[95]  = 0x0073;
   dcpu.memory[96]  = 0x0065;
   dcpu.memory[97]  = 0x0054;
   dcpu.memory[98]  = 0x0020;
   dcpu.memory[99]  = 0x0054;
   dcpu.memory[100] = 0x0065;
   dcpu.memory[101] = 0x0073;
   dcpu.memory[102] = 0x0074;
   dcpu.memory[103] = 0x0020;
   dcpu.memory[104] = 0x0074;
   dcpu.memory[105] = 0x0073;
   dcpu.memory[106] = 0x0065;
   dcpu.memory[107] = 0x0054;
   dcpu.memory[108] = 0x0020;
   dcpu.memory[109] = 0x0054;
   dcpu.memory[110] = 0x0065;
   dcpu.memory[111] = 0x0073;
   dcpu.memory[112] = 0x0074;
   dcpu.memory[113] = 0x0020;
   dcpu.memory[114] = 0x0074;
   dcpu.memory[115] = 0x0073;
   dcpu.memory[116] = 0x0065;
   dcpu.memory[117] = 0x0054;
   dcpu.memory[118] = 0x0020;
   dcpu.memory[119] = 0x0054;
   dcpu.memory[120] = 0x0065;
   dcpu.memory[121] = 0x0073;
   dcpu.memory[122] = 0x0074;
   dcpu.memory[123] = 0x0020;
   dcpu.memory[124] = 0x0074;
   dcpu.memory[125] = 0x0073;
   dcpu.memory[126] = 0x0065;
   dcpu.memory[127] = 0x0054;
   dcpu.memory[128] = 0x0020;
   dcpu.memory[129] = 0x0054;
   dcpu.memory[130] = 0x0065;
   dcpu.memory[131] = 0x0073;
   dcpu.memory[132] = 0x0074;
   dcpu.memory[133] = 0x0020;
   dcpu.memory[134] = 0x0074;
   dcpu.memory[135] = 0x0073;
   dcpu.memory[136] = 0x0065;
   dcpu.memory[137] = 0x0054;

   while dcpu.memory[dcpu.program_counter as usize] != 0x0000 {
      dcpu.step();
   }

   println!("Steps: {}", dcpu.cycle_count);
   assert_eq!(dcpu.registers[5], 0x749b);
}