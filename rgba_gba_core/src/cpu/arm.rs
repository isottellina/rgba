// arm.rs --- 
// 
// Filename: arm.rs
// Author: Louise <louise>
// Created: Sat Jan 13 11:44:36 2018 (+0100)
// Last-Updated: Thu Jan 25 23:38:52 2018 (+0100)
//           By: Louise <louise>
// 
use cpu::ARM7TDMI;
use cpu::CpuMode;
use cpu::CpuState;
use io::Interconnect;

impl ARM7TDMI {
    fn match_condition(&self, instr: u32) -> bool {
        match instr & 0xF0000000 {
            0x00000000 => self.zero,
            0x10000000 => !self.zero,
            0x20000000 => self.carry,
            0x30000000 => !self.carry,
            0x40000000 => self.sign,
            0x50000000 => !self.sign,
            0x60000000 => self.overflow,
            0x70000000 => !self.overflow,
            0x80000000 => self.carry && !self.zero,
            0x90000000 => !self.carry || self.zero,
            0xA0000000 => self.sign == self.overflow,
            0xB0000000 => self.sign != self.overflow,
            0xC0000000 => !self.zero && (self.sign == self.overflow),
            0xD0000000 => self.zero || (self.sign != self.overflow),
            _ => unreachable!(),
        }
    }
    
    pub fn next_instruction_arm(&mut self, io: &mut Interconnect, instr: u32) {
        let instr_high = (instr & 0x0FF00000) >> 16;
        let instr_low = (instr & 0x000000F0) >> 4;
        
        let function = ARM_INSTRUCTIONS[(instr_high | instr_low) as usize];

        self.registers[15] = self.pc + 4;

        if instr > 0xE0000000 || self.match_condition(instr) {
            function(self, io, instr);
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/arm_generated.rs"));
