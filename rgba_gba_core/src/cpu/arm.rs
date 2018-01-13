// arm.rs --- 
// 
// Filename: arm.rs
// Author: Louise <louise>
// Created: Sat Jan 13 11:44:36 2018 (+0100)
// Last-Updated: Sat Jan 13 22:43:30 2018 (+0100)
//           By: Louise <louise>
// 
use cpu::ARM7TDMI;
use io::Interconnect;

impl ARM7TDMI {
    pub fn next_instruction_arm(&mut self, io: &mut Interconnect, instr: u32) {
        let instr_high = (instr & 0x0FF00000) >> 16;
        let instr_low = (instr & 0x000000F0) >> 4;
        
        let function = ARM_INSTRUCTIONS[(instr_high | instr_low) as usize];

        function(self, io, instr);
    }
}

include!(concat!(env!("OUT_DIR"), "/arm_generated.rs"));
