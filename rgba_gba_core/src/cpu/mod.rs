// mod.rs --- 
// 
// Filename: mod.rs
// Author: Louise <louise>
// Created: Wed Jan  3 16:20:45 2018 (+0100)
// Last-Updated: Thu Jan  4 20:16:50 2018 (+0100)
//           By: Louise <louise>
// 
use std::fmt;

#[derive(Debug, Default)]
pub struct ARM7TDMI {
    // Registers
    registers: [u32; 31],
    spsr: [u32; 5],

    // CPSR
    sign: bool,
    zero: bool,
    carry: bool,
    overflow: bool,

    irq: bool,
    fiq: bool,

    state: CpuState,
    mode: CpuMode,
    
    // Pipeline
    instr_fetched: u32,
    instr_decoded: u32,
}

impl ARM7TDMI {
    pub fn new() -> ARM7TDMI {
        Default::default()
    }

    pub fn get_register(&self, n: usize) -> u32 {
        match self.mode {
            CpuMode::User | CpuMode::System => self.registers[n],
            CpuMode::IRQ if (n == 13) || (n == 14) => self.registers[n + 3],
            CpuMode::SVC if (n == 13) || (n == 14) => self.registers[n + 5],
            CpuMode::UND if (n == 13) || (n == 14) => self.registers[n + 7],
            CpuMode::ABT if (n == 13) || (n == 14) => self.registers[n + 9],
            CpuMode::FIQ if (n >= 8) || (n <= 14)  => self.registers[n + 11],
            _ => self.registers[n]
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
                State: {:?}, Mode: {:?}, IRQ = {}, FIQ = {}, [{}{}{}{}]",
               self.get_register(0), self.get_register(1),
               self.get_register(2), self.get_register(3),
               self.get_register(4), self.get_register(5),
               self.get_register(6), self.get_register(7),
               self.get_register(8), self.get_register(9),
               self.get_register(10), self.get_register(11),
               self.get_register(12), self.get_register(13),
               self.get_register(14), self.get_register(15),
               self.state, self.mode, self.irq, self.fiq,
               if self.sign { "N" } else { "-" },
               if self.zero { "Z" } else { "-" },
               if self.carry { "C" } else { "-" },
               if self.overflow { "V" } else { "-" }
        )
    }
}

#[derive(Debug)]
enum CpuState {
    ARM = 0,
    Thumb = 1
}

impl Default for CpuState {
    fn default() -> CpuState { CpuState::ARM }
}

#[derive(Debug)]
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
