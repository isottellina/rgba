// timer.rs --- 
// 
// Filename: timer.rs
// Author: Louise <louise>
// Created: Fri Dec  8 01:49:50 2017 (+0100)
// Last-Updated: Wed Jun 13 12:45:57 2018 (+0200)
//           By: Louise <louise>
// 

pub struct Timer {
    div: u16,

    tima: u8,
    tma: u8,

    tima_running: bool,
    speed: u16,

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
            if (self.div & self.speed) == 0 {
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
            15 => 1,
            63 => 2,
            255 => 3,
            1023 => 0,
            _ => unreachable!(),
        };
        
        ((self.tima_running as u8) << 2) | speed
    }

    pub fn set_tac(&mut self, value: u8) {
        self.speed = match value & 0x3 {
            0 => 1023,
            1 => 15,
            2 => 63,
            3 => 255,
            _ => unreachable!(),
        };

        self.tima_running = (value & 0x04) != 0;
    }

    pub fn it_timer(&self) -> bool { self.it_timer }
    pub fn ack_it_timer(&mut self) { self.it_timer = false }
    pub fn set_it_timer(&mut self, v: bool) { self.it_timer = v }
}
