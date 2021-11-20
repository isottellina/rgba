// mod.rs --- 
// 
// Filename: mod.rs
// Author: Louise <louise>
// Created: Wed Jan  3 16:20:45 2018 (+0100)
// Last-Updated: Thu Nov  5 21:25:16 2020 (+0100)
//           By: Louise <louise>
// 
use std::fmt;
use crate::io::Interconnect;

mod arm;
mod thumb;

#[derive(Debug, Default)]
pub struct ARM7TDMI {
    // Registers
    pub registers: [u32; 31],
    pub spsr: [u32; 5],
    pub pc: u32,

    // CPSR
    pub sign: bool,
    pub zero: bool,
    pub carry: bool,
    pub overflow: bool,

    pub irq: bool,
    pub fiq: bool,

    pub state: CpuState,
    mode: CpuMode,

    // Lines
    irq_line: bool,
}

impl ARM7TDMI {
    pub fn new() -> ARM7TDMI {
        Default::default()
    }

    pub fn reset(&mut self, io: &mut Interconnect) {
        self.mode = CpuMode::SVC;
        self.state = CpuState::ARM;
        self.registers[15] = 0;
        self.irq = false;
        self.fiq = false;
	
        self.branch(io);
    }

    pub fn raise_irq(&mut self) {
        self.irq_line = true;

        if !self.irq {
            let old_cpsr = self.cpsr();
            let old_pc = self.pc + 4;
            
            self.state = CpuState::ARM;
            self.mode = CpuMode::IRQ;
            self.irq = true;
            self.irq_line = false;
            self.set_register(14, old_pc);
            self.set_spsr(old_cpsr);

            self.pc = 0x18;
        }
    }

    pub fn raise_swi(&mut self) {
        let old_cpsr = self.cpsr();
        let old_pc = self.pc;

        self.state = CpuState::ARM;
        self.mode = CpuMode::SVC;
        self.irq = true;
        self.set_register(14, old_pc);
        self.set_spsr(old_cpsr);

        self.pc = 0x08;
    }
    
    pub fn read_u32(&self, io: &mut Interconnect, address: usize) -> u32 {
        io.declare_access(address, 2);
        io.read_u32(address)
    }

    pub fn read_u16(&self, io: &mut Interconnect, address: usize) -> u16 {
        io.declare_access(address, 1);
        io.read_u16(address)
    }

    pub fn read_u8(&self, io: &mut Interconnect, address: usize) -> u8 {
        io.declare_access(address, 0);
        io.read_u8(address)
    }

    pub fn write_u32(&self, io: &mut Interconnect, address: usize, value: u32) {
        io.declare_access(address, 2);
        io.write_u32(address, value)
    }

    pub fn write_u16(&self, io: &mut Interconnect, address: usize, value: u16) {
        io.declare_access(address, 1);
        io.write_u16(address, value)
    }

    pub fn write_u8(&self, io: &mut Interconnect, address: usize, value: u8) {
        io.declare_access(address, 0);
        io.write_u8(address, value)
    }
    
    pub fn get_register(&self, n: usize) -> u32 {
        match self.mode {
            CpuMode::User | CpuMode::System => self.registers[n],
            CpuMode::IRQ if (n == 13) || (n == 14) => self.registers[n + 3],
            CpuMode::SVC if (n == 13) || (n == 14) => self.registers[n + 5],
            CpuMode::UND if (n == 13) || (n == 14) => self.registers[n + 7],
            CpuMode::ABT if (n == 13) || (n == 14) => self.registers[n + 9],
            CpuMode::FIQ if (n >= 8) && (n <= 14)  => self.registers[n + 11],
            _ => self.registers[n]
        }
    }

    pub fn set_register(&mut self, n: usize, value: u32) {
        match self.mode {
            CpuMode::User | CpuMode::System => self.registers[n] = value,
            CpuMode::IRQ if (n == 13) || (n == 14) => self.registers[n + 3] = value,
            CpuMode::SVC if (n == 13) || (n == 14) => self.registers[n + 5] = value,
            CpuMode::UND if (n == 13) || (n == 14) => self.registers[n + 7] = value,
            CpuMode::ABT if (n == 13) || (n == 14) => self.registers[n + 9] = value,
            CpuMode::FIQ if (n >= 8) && (n <= 14)  => self.registers[n + 11] = value,
            _ => self.registers[n] = value
        }
    }

    pub fn cpsr(&self) -> u32 {
        ((self.sign as u32) << 31) | ((self.zero as u32) << 30) |
        ((self.carry as u32) << 29) | ((self.overflow as u32) << 28) |
        ((self.irq as u32) << 7) | ((self.fiq as u32) << 6) |
        ((self.state as u32) << 5) | (self.mode as u32)
    }

    pub fn set_cpsr(&mut self, cpsr: u32) {
        self.sign = (cpsr & 0x80000000) != 0;
        self.zero = (cpsr & 0x40000000) != 0;
        self.carry = (cpsr & 0x20000000) != 0;
        self.overflow = (cpsr & 0x10000000) != 0;

        self.irq = (cpsr & 0x00000080) != 0;
        self.fiq = (cpsr & 0x00000040) != 0;
        self.state = if cpsr & 0x20 != 0 { CpuState::Thumb } else { CpuState::ARM };
        self.mode = CpuMode::from_u32(cpsr & 0x1f);
    }

    pub fn set_cpsr_flg(&mut self, cpsr: u32) {
        self.sign = (cpsr & 0x80000000) != 0;
        self.zero = (cpsr & 0x40000000) != 0;
        self.carry = (cpsr & 0x20000000) != 0;
        self.overflow = (cpsr & 0x10000000) != 0;
    }
    
    pub fn spsr(&self) -> u32 {
        match self.mode {
            CpuMode::IRQ => self.spsr[0],
            CpuMode::SVC => self.spsr[1],
            CpuMode::UND => self.spsr[2],
            CpuMode::ABT => self.spsr[3],
            CpuMode::FIQ => self.spsr[4],
            _ => panic!("Mode {:?} doesn't have a SPSR", self.mode),
        }
    }

    pub fn set_spsr(&mut self, spsr: u32) {
        match self.mode {
            CpuMode::IRQ => self.spsr[0] = spsr,
            CpuMode::SVC => self.spsr[1] = spsr,
            CpuMode::UND => self.spsr[2] = spsr,
            CpuMode::ABT => self.spsr[3] = spsr,
            CpuMode::FIQ => self.spsr[4] = spsr,
            _ => panic!("Mode {:?} doesn't have a SPSR", self.mode),
        }
    }

    pub fn set_spsr_flg(&mut self, spsr: u32) {
        match self.mode {
            CpuMode::IRQ => {
                self.spsr[0] = self.spsr[0] & 0x0FFFFFFF;
                self.spsr[0] |= spsr & 0xF0000000;
            },
            CpuMode::SVC => {
                self.spsr[1] = self.spsr[1] & 0x0FFFFFFF;
                self.spsr[1] |= spsr & 0xF0000000;
            },
            CpuMode::UND => {
                self.spsr[2] = self.spsr[2] & 0x0FFFFFFF;
                self.spsr[2] |= spsr & 0xF0000000;
            },
            CpuMode::ABT => {
                self.spsr[3] = self.spsr[3] & 0x0FFFFFFF;
                self.spsr[3] |= spsr & 0xF0000000;
            },
            CpuMode::FIQ => {
                self.spsr[4] = self.spsr[4] & 0x0FFFFFFF;
                self.spsr[4] |= spsr & 0xF0000000;
            },
            _ => panic!("Mode {:?} doesn't have a SPSR", self.mode),
        }
    }
    
    pub fn advance_pipeline(&mut self, io: &mut Interconnect) {
        match self.state {
            CpuState::ARM => {
                io.declare_access(self.pc as usize, 2);
            }
            CpuState::Thumb => {
                io.declare_access(self.pc as usize, 1);
            }
        }
    }
    
    pub fn fill_pipeline(&mut self, io: &mut Interconnect) {
        match self.state {
            CpuState::ARM => {
                io.declare_access(self.pc as usize, 2);
                io.declare_access(self.pc as usize + 4, 2);
            }
            CpuState::Thumb => {
                io.declare_access(self.pc as usize, 1);
                io.declare_access(self.pc as usize + 2, 1);
            }
        }
    }

    pub fn branch(&mut self, io: &mut Interconnect) {
        self.pc = self.registers[15];
        self.fill_pipeline(io);
    }
    
    pub fn next_instruction(&mut self, io: &mut Interconnect) {
        self.advance_pipeline(io);
        
        match self.state {
            CpuState::ARM => {
                let instr = io.read_u32(self.pc as usize);
                self.pc += 4;
                
                self.next_instruction_arm(io, instr);
            },
            CpuState::Thumb => {
                let instr = io.read_u16(self.pc as usize);
                self.pc += 2;
                
                self.next_instruction_thumb(io, instr);
            }
        }
    }
}

impl fmt::Display for ARM7TDMI {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "======================ARM7TDMI=====================\n\
                R00={:08x} R01={:08x} R02={:08x} R03={:08x}\n\
                R04={:08x} R05={:08x} R06={:08x} R07={:08x}\n\
                R08={:08x} R09={:08x} R10={:08x} R11={:08x}\n\
                R12={:08x} R13={:08x} R14={:08x} R15={:08x}\n\
                CPSR: {:08x}, State: {:?}, Mode: {:?}, [{}{}{}{}{}{}]",
               self.get_register(0), self.get_register(1),
               self.get_register(2), self.get_register(3),
               self.get_register(4), self.get_register(5),
               self.get_register(6), self.get_register(7),
               self.get_register(8), self.get_register(9),
               self.get_register(10), self.get_register(11),
               self.get_register(12), self.get_register(13),
               self.get_register(14), self.get_register(15),
               self.cpsr(), self.state, self.mode,
               if self.sign { 'N' } else { '-' },
               if self.zero { 'Z' } else { '-' },
               if self.carry { 'C' } else { '-' },
               if self.overflow { 'V' } else { '-' },
               if self.irq { 'I' } else { '-' },
               if self.fiq { 'F' } else { '-' },
        )
    }
}

#[derive(Copy, Clone, Debug)]
pub enum CpuState {
    ARM = 0,
    Thumb = 1
}

impl Default for CpuState {
    fn default() -> CpuState { CpuState::ARM }
}

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
enum CpuMode {
    User = 0x10,
    System = 0x1F,
    FIQ = 0x11,
    IRQ = 0x12,
    SVC = 0x13,
    ABT = 0x17,
    UND = 0x1B
}

impl Default for CpuMode {
    fn default() -> CpuMode { CpuMode::SVC }
}

impl CpuMode {
    pub fn from_u32(value: u32) -> CpuMode {
        match value {
            0x10 => CpuMode::User,
            0x1f => CpuMode::System,
            0x11 => CpuMode::FIQ,
            0x12 => CpuMode::IRQ,
            0x13 => CpuMode::SVC,
            0x17 => CpuMode::ABT,
            0x1B => CpuMode::UND,
            _ => panic!("Writing bad bits to mode bits"),
        }
    }
}
