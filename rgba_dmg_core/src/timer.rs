// timer.rs --- 
// 
// Filename: timer.rs
// Author: Louise <louise>
// Created: Fri Dec  8 01:49:50 2017 (+0100)
// Last-Updated: Sun Dec 31 15:12:17 2017 (+0100)
//           By: Louise <louise>
// 

pub struct Timer {
    div: u16,

    tima: u8,
    tma: u8,

    tima_running: bool,
    speed: u8,

    it_timer: bool,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            div: 0,
            tima: 0,
            tma: 0,
            tima_running: false,
            speed: 10,

            it_timer: false
        }
    }

    pub fn reset(&mut self) {
        self.div = 0;
        self.tima = 0;
        self.tma = 0;
        self.tima_running = false;

        self.it_timer = false;
    }

    pub fn handle(&mut self) {
        self.div = self.div.wrapping_add(1);
        
        if self.tima_running {
            let mask = (1 << self.speed) - 1;
            
            if (self.div & mask) == 0 {
                self.tima = self.tima.wrapping_add(1);

                if self.tima == 0 {
                    self.tima = self.tma;
                    self.it_timer = true;
                }
            }
        }
    }

    pub fn get_internal(&self) -> u16 { self.div }
    
    pub fn div(&self) -> u8 { (self.div >> 8) as u8 }
    pub fn set_div(&mut self) { self.div = 0; }

    pub fn set_tima(&mut self, tima: u8) { self.tima = tima; }
    pub fn tima(&self) -> u8 { self.tima }
    
    pub fn tma(&self) -> u8 { self.tma }
    pub fn set_tma(&mut self, tma: u8) { self.tma = tma; }
    
    pub fn tac(&self) -> u8 {
        let speed = match self.speed {
            4 => 1,
            6 => 2,
            8 => 3,
            10 => 0,
            _ => unreachable!(),
        };
        
        ((self.tima_running as u8) << 2) | speed
    }

    pub fn set_tac(&mut self, value: u8) {
        self.speed = match value & 0x3 {
            0 => 10,
            1 => 4,
            2 => 6,
            3 => 8,
            _ => unreachable!(),
        };

        self.tima_running = (value & 0x04) != 0;
    }

    pub fn it_timer(&self) -> bool { self.it_timer }
    pub fn ack_it_timer(&mut self) { self.it_timer = false }
    pub fn set_it_timer(&mut self, v: bool) { self.it_timer = v }
}
