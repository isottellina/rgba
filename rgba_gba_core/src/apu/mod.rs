// mod.rs --- 
// 
// Filename: mod.rs
// Author: Louise <louise>
// Created: Sun Jan 21 22:26:01 2018 (+0100)
// Last-Updated: Thu Jul 12 16:59:10 2018 (+0200)
//           By: Louise <ludwigette>
// 

#[derive(Default, Debug, Clone)]
pub struct APU {
    soundbias: u16,
}

impl APU {
    pub fn new() -> APU {
        APU {
            soundbias: 0x0200,
        }
    }

    pub fn soundbias(&self) -> u16 {
        self.soundbias
    }
    
    pub fn set_soundbias(&mut self, soundbias: u16) {
        self.soundbias = soundbias;
    }

    pub fn io_read_u16(&self, address: usize) -> u16 {
        match address {
            SOUNDBIAS => self.soundbias(),
            _ => { warn!("Unmapped read_u16 from {:08x} (APU)", address); 0 },
        }
    }
    
    pub fn io_write_u16(&mut self, address: usize, value: u16) {
        match address {
            SOUNDBIAS => self.set_soundbias(value),
            // We don't log unknown APU writes yet, because we don't really care for now
            // _ => warn!("Unmapped write_u16 to {:08x} (APU, value={:04x})", address, value),
            _ => { }
        }
    }
}

const SOUNDBIAS: usize = 0x04000088;
