// noise.rs --- 
// 
// Filename: noise.rs
// Author: Louise <louise>
// Created: Thu Dec 28 00:06:44 2017 (+0100)
// Last-Updated: Tue Jan  9 13:03:05 2018 (+0100)
//           By: Louise <louise>
// 

const DIVISORS: [u16; 8] = [8, 16, 32, 48, 64, 80, 96, 112];

#[derive(Debug, Default)]
pub struct NoiseChannel {
    timer: u16,
    divisor: u8,
    out_volume: u8,
    
    enabled: bool,
    dac_enabled: bool,
    
    lfsr: u16,

    // Noise things
    width_mode: bool,
    clock_shift: u8,
    
    // Length
    length_load: u8,
    length_counter: u8,
    length_enable: bool,
    
    // Envelope
    volume: u8,
    volume_load: u8,
    envelope_running: bool,
    envelope_direction: bool,
    envelope_period: u8,
    envelope_period_load: u8,

    // Other
    last_trigger: bool,
}

impl NoiseChannel {
    pub fn new() -> NoiseChannel {
        Default::default()
    }
    
    pub fn nr1(&self) -> u8 {
        self.length_load
    }

    pub fn set_nr1(&mut self, value: u8) {
        self.length_load = value & 0x3F;
        self.length_counter = 64 - self.length_load;
    }

    pub fn nr2(&self) -> u8 {
        (self.volume_load << 4) |
        ((self.envelope_direction as u8) << 3) |
        self.envelope_period
    }

    pub fn set_nr2(&mut self, value: u8) {
        self.dac_enabled = (value & 0xf8) != 0;
        
        self.volume = (value & 0xf0) >> 4;
        self.volume_load = self.volume;
        
        self.envelope_direction = (value & 0x08) != 0;
        self.envelope_period = value & 0x07;
        self.envelope_period_load = self.envelope_period;
    }

    pub fn nr3(&self) -> u8 {
        (self.clock_shift << 4) |
        ((self.width_mode as u8) << 3) |
        self.divisor
    }
    
    pub fn set_nr3(&mut self, value: u8) {
        self.clock_shift = (value & 0xf0) >> 4;
        self.width_mode = (value & 0x08) != 0;
        self.divisor = value & 0x07;
    }

    pub fn nr4(&self) -> u8 {
        0xBF | ((self.length_enable as u8) << 6)
    }

    pub fn set_nr4(&mut self, value: u8) {
        self.length_enable = (value & 0x40) != 0;
        
        self.last_trigger = (value & 0x80) != 0;
        
        if self.last_trigger {
            self.enabled = true;
            self.lfsr = 0x7FFF;
            
            if self.length_counter == 0 {
                self.length_counter = 64;
            }

            self.timer = DIVISORS[self.divisor as usize] << self.clock_shift;
            self.volume = self.volume_load;

            self.envelope_running = true;
            self.envelope_period = self.envelope_period_load;
        }
    }
    
    pub fn envelope_click(&mut self) {
        if self.envelope_running && self.envelope_period != 0 {
            self.envelope_period -= 1;
            
            if self.envelope_period == 0 {
                self.envelope_period = self.envelope_period_load;
                
                if self.envelope_direction && self.volume < 15 {
                    self.volume += 1;
                } else if self.volume > 0 {
                    self.volume -= 1;
                }

                if (self.volume == 0) || (self.volume == 15) {
                    self.envelope_running = false;
                }
            }
        }
    }
    
    pub fn length_click(&mut self) {
        if (self.length_counter > 0) && self.length_enable {
            self.length_counter -= 1;

            if self.length_counter == 0 {
                self.enabled = false;
            }
        }
    }

    pub fn render(&self) -> u8 {
        self.out_volume
    }

    pub fn step(&mut self) {
        self.timer = self.timer.wrapping_sub(1);

        if self.timer == 0 {
            self.timer = DIVISORS[self.divisor as usize] << self.clock_shift;

            let res = (self.lfsr & 1) ^ ((self.lfsr >> 1) & 1);
            self.lfsr = (self.lfsr >> 1) | (res << 14);

            if self.width_mode {
                self.lfsr = (self.lfsr & 0x7FBF) | (res << 6);
            }

            self.out_volume = if self.lfsr & 1 == 0 {
                self.volume
            } else {
                0
            }
        }
    }
}
