// irq.rs --- 
// 
// Filename: irq.rs
// Author: Louise <louise>
// Created: Thu Jan 25 13:07:21 2018 (+0100)
// Last-Updated: Thu Jan 25 22:14:41 2018 (+0100)
//           By: Louise <louise>
// 
use crate::cpu::ARM7TDMI;

#[derive(Debug, Default)]
pub struct IrqManager {
    pub i_e: u16,
    pub i_f: u16,

    pub halt: bool,
    pub ime: bool,
    pub pending: bool,
}

impl IrqManager {
    pub fn new() -> IrqManager {
        Default::default()
    }

    pub fn raise_irq(&mut self, irq: u16) {
        self.i_f |= irq;

        if self.i_f & self.i_e != 0 {
            self.halt = false;
            
            if self.ime {
                self.pending = true;
            }
        }
    }

    pub fn handle(&mut self, cpu: &mut ARM7TDMI) {
        if self.pending {
            cpu.raise_irq();

            self.pending = false;
        }
    }

    #[inline]
    pub fn write_if(&mut self, i_f: u16) {
        self.i_f &= !i_f;
    }
    
    #[inline]
    pub fn write_ime(&mut self, value: u16) {
        self.ime = (value & 1) != 0;

        if self.ime && (self.i_f & self.i_e != 0) {
            self.pending = true;
        }
    }
}

pub const IRQ_VBLANK: u16 = 0x0001;
pub const IRQ_HBLANK: u16 = 0x0002;
pub const IRQ_VCOUNT: u16 = 0x0004;
