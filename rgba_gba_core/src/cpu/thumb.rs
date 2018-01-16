// thumb.rs --- 
// 
// Filename: thumb.rs
// Author: Louise <louise>
// Created: Tue Jan 16 20:00:15 2018 (+0100)
// Last-Updated: Tue Jan 16 20:04:20 2018 (+0100)
//           By: Louise <louise>
// 
use cpu::ARM7TDMI;
use io::Interconnect;

impl ARM7TDMI {
    pub fn next_instruction_thumb(&mut self, io: &mut Interconnect, instr: u16) {
        let instr_code = (instr & 0xFF00) >> 8;
        
        let function = THUMB_INSTRUCTIONS[instr_code as usize];
        function(self, io, instr);
    }
}

include!(concat!(env!("OUT_DIR"), "/thumb_generated.rs"));
