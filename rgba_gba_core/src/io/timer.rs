// timer.rs --- 
// 
// Filename: timer.rs
// Author: Louise <ludwigette>
// Created: Sun Aug 18 17:59:54 2019 (+0200)
// Last-Updated: Tue Aug 20 13:26:33 2019 (+0200)
//           By: Louise <ludwigette>
// 
use crate::irq::IrqManager;
use std::{
    rc::Rc,
    cell::RefCell
};

#[derive(Clone, Default)]
pub struct Timer {
    internal: u32,
    counter: u16,
    reload: u16,

    scale: u32,
    countup: bool,
    irq: bool,
    irq_n: u16,
    start: bool,

    next: Option<Rc<RefCell<Timer>>>
}

impl Timer {
    pub fn set_id(&mut self, id: u16) { self.irq_n = id; }
    pub fn set_next(&mut self, next: Rc<RefCell<Timer>>) { self.next = Some(next); }
    pub fn write_cnt_h(&mut self, value: u16) {
        self.reload = value;
    }

    pub fn read_cnt_l(&self) -> u16 { return self.counter; }
    pub fn read_cnt_h(&self) -> u16 {
        return
            match self.scale {
                0 => 0, 63 => 1,
                255 => 2, 1023 => 3,
                _ => unreachable!(),
            }
        | (self.countup as u16) << 3 | (self.irq as u16) << 6 | (self.start as u16) << 7;
    }
    
    pub fn write_cnt_l(&mut self, value: u16) {
        // Scale
        self.scale = match value & 3 {
            0 => 0,
            1 => 63,
            2 => 255,
            3 => 1023,
            _ => unreachable!(),
        };
        
        // Count-up
        self.countup = (value & 0x04) != 0;
        
        // IRQ
        self.irq = (value & 0x40) != 0;
        
        // Enable
        if (value & 0x80 != 0) && !self.start {
            self.counter = self.reload;
        }
        self.start = (value & 0x80) != 0;
    }

    pub fn countup_sig(&mut self) {
        if self.countup {
            self.counter = self.counter.wrapping_add(1);
        }
    }
    
    pub fn spend_cycles(&mut self, mut cycles: u32, irq: &mut IrqManager) {
        if !self.start || self.countup { return; }
        
        while cycles != 0 {
            cycles -= 1;
            self.internal = self.internal.wrapping_add(1);

            if (self.internal & self.scale) == 0 {
                self.counter = self.counter.wrapping_add(1);

                if self.counter == 0 {
                    if let Some(next) = &self.next {
                        next.borrow_mut().countup_sig();
                    }
                    
                    if self.irq {
                        irq.raise_irq(self.irq_n);
                    }
                }
            }
        }
    }
}

pub const TM0CNT_H: usize = 0x04000100;
pub const TM1CNT_H: usize = 0x04000104;
pub const TM2CNT_H: usize = 0x04000108;
pub const TM3CNT_H: usize = 0x0400010C;

pub const TM0CNT_L: usize = 0x04000102;
pub const TM1CNT_L: usize = 0x04000106;
pub const TM2CNT_L: usize = 0x0400010A;
pub const TM3CNT_L: usize = 0x0400010E;
