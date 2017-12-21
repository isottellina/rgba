// apu.rs --- 
// 
// Filename: apu.rs
// Author: Louise <louise>
// Created: Fri Dec  8 22:08:49 2017 (+0100)
// Last-Updated: Sat Dec 16 22:20:59 2017 (+0100)
//           By: Louise <louise>
// 

pub struct APU {
    nr10: u8,
    nr11: u8,
    nr12: u8,
    nr13: u8,
    nr14: u8,
    nr21: u8,
    nr22: u8,
    nr23: u8,
    nr24: u8,
    nr30: u8,
    nr31: u8,
    nr32: u8,
    nr33: u8,
    nr34: u8,
    nr41: u8,
    nr42: u8,
    nr43: u8,
    nr44: u8,
    nr50: u8,
    nr51: u8,
    nr52: u8,

    nr3_wave: [u8; 0x10],
}

impl APU {
    pub fn new() -> APU {
        APU {
            nr10: 0,
            nr11: 0,
            nr12: 0,
            nr13: 0,
            nr14: 0,
            nr21: 0,
            nr22: 0,
            nr23: 0,
            nr24: 0,
            nr30: 0,
            nr31: 0,
            nr32: 0,
            nr33: 0,
            nr34: 0,
            nr41: 0,
            nr42: 0,
            nr43: 0,
            nr44: 0,
            nr50: 0,
            nr51: 0,
            nr52: 0,
            
            nr3_wave: [0; 0x10],
        }
    }

    pub fn nr10(&self) -> u8 { self.nr10 }
    pub fn set_nr10(&mut self, nr10: u8) { self.nr10 = nr10; }
    
    pub fn nr11(&self) -> u8 { self.nr11 }
    pub fn set_nr11(&mut self, nr11: u8) { self.nr11 = nr11; }
    
    pub fn nr12(&self) -> u8 { self.nr12 }
    pub fn set_nr12(&mut self, nr12: u8) { self.nr12 = nr12; }

    pub fn nr13(&self) -> u8 { self.nr13 }
    pub fn set_nr13(&mut self, nr13: u8) { self.nr13 = nr13; }
    
    pub fn nr14(&self) -> u8 { self.nr14 }
    pub fn set_nr14(&mut self, nr14: u8) { self.nr14 = nr14; }
    
    pub fn nr21(&self) -> u8 { self.nr21 }
    pub fn set_nr21(&mut self, nr21: u8) { self.nr21 = nr21; }
    
    pub fn nr22(&self) -> u8 { self.nr22 }
    pub fn set_nr22(&mut self, nr22: u8) { self.nr22 = nr22; }
    
    pub fn nr23(&self) -> u8 { self.nr23 }
    pub fn set_nr23(&mut self, nr23: u8) { self.nr23 = nr23; }
    
    pub fn nr24(&self) -> u8 { self.nr24 }
    pub fn set_nr24(&mut self, nr24: u8) { self.nr24 = nr24; }
    
    pub fn nr30(&self) -> u8 { self.nr30 }
    pub fn set_nr30(&mut self, nr30: u8) { self.nr30 = nr30; }
    
    pub fn nr31(&self) -> u8 { self.nr31 }
    pub fn set_nr31(&mut self, nr31: u8) { self.nr31 = nr31; }
    
    pub fn nr32(&self) -> u8 { self.nr32 }
    pub fn set_nr32(&mut self, nr32: u8) { self.nr32 = nr32; }
    
    pub fn nr33(&self) -> u8 { self.nr33 }
    pub fn set_nr33(&mut self, nr33: u8) { self.nr33 = nr33; }
    
    pub fn nr34(&self) -> u8 { self.nr34 }
    pub fn set_nr34(&mut self, nr34: u8) { self.nr34 = nr34; }
    
    pub fn nr41(&self) -> u8 { self.nr41 }
    pub fn set_nr41(&mut self, nr41: u8) { self.nr41 = nr41; }
    
    pub fn nr42(&self) -> u8 { self.nr42 }
    pub fn set_nr42(&mut self, nr42: u8) { self.nr42 = nr42; }
    
    pub fn nr43(&self) -> u8 { self.nr43 }
    pub fn set_nr43(&mut self, nr43: u8) { self.nr43 = nr43; }
    
    pub fn nr44(&self) -> u8 { self.nr44 }
    pub fn set_nr44(&mut self, nr44: u8) { self.nr44 = nr44; }

    pub fn nr50(&self) -> u8 { self.nr50 }
    pub fn set_nr50(&mut self, nr50: u8) { self.nr50 = nr50; }
    
    pub fn nr51(&self) -> u8 { self.nr51 }
    pub fn set_nr51(&mut self, nr51: u8) { self.nr51 = nr51; }
    
    pub fn nr52(&self) -> u8 { self.nr52 }
    pub fn set_nr52(&mut self, nr52: u8) { self.nr52 = nr52; }

    pub fn nr3_wave(&self, address: usize) -> u8 { self.nr3_wave[address & 0xF] }
    pub fn set_nr3_wave(&mut self, address: usize, value: u8) {
        self.nr3_wave[address & 0xF] = value;
    }
}
