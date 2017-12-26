// square.rs --- 
// 
// Filename: square.rs
// Author: Louise <louise>
// Created: Sat Dec 23 01:16:18 2017 (+0100)
// Last-Updated: Tue Dec 26 20:00:44 2017 (+0100)
//           By: Louise <louise>
// 

const DUTY_TABLE: [[bool; 8]; 4] = [
    [false, false, false, false, false, false, false, true],
    [true, false, false, false, false, false, false, true],
    [true, false, false, false, false, true, true, true],
    [false, true, true, true, true, true, true, false]
];

#[derive(Debug, Default)]
pub struct SquareChannel {
    timer: i32,
    frequency: u32,

    enabled: bool,
    dac_enabled: bool,

    duty: u8,
    duty_state: u8,
    out_volume: u8,

    // Sweep
    sweep_enable: bool,
    sweep_period: u8,
    sweep_period_load: u8,
    sweep_negate: bool,
    sweep_shift: u8,
    sweep_shadow: u16,

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

impl SquareChannel {
    pub fn new() -> SquareChannel {
        SquareChannel {
            timer: 0,
            frequency: 0,
            
            enabled: false,
            dac_enabled: true,
            
            duty: 2,
            duty_state: 5,
            out_volume: 0,

            sweep_enable: false,
            sweep_period: 0,
            sweep_period_load: 0,
            sweep_negate: false,
            sweep_shift: 0,
            sweep_shadow: 0,

            length_load: 0,
            length_counter: 0,
            length_enable: false,

            volume: 0,
            volume_load: 0,
            envelope_running: true,
            envelope_direction: false,
            envelope_period: 0,
            envelope_period_load: 0,

            last_trigger: false
        }
    }
    
    pub fn nr0(&self) -> u8 {
        (self.sweep_period << 4) |
        ((self.sweep_negate as u8) << 3) |
        self.sweep_shift
    }
    
    pub fn set_nr0(&mut self, value: u8) {
        self.sweep_period = (value & 0x70) >> 4;
        self.sweep_negate = (value & 0x08) != 0;
        self.sweep_shift = value & 0x07;
    }

    pub fn nr1(&self) -> u8 {
        self.duty << 6
    }

    pub fn set_nr1(&mut self, value: u8) {
        self.duty = (value & 0xc0) >> 6;
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

    pub fn set_nr3(&mut self, value: u8) {
        self.frequency &= 0x0700;
        self.frequency |= value as u32;
    }

    pub fn nr4(&self) -> u8 {
        0xBF | ((self.length_enable as u8) << 6)
    }

    pub fn set_nr4(&mut self, value: u8) {
        self.length_enable = (value & 0x40) != 0;
        
        self.frequency &= 0x00ff;
        self.frequency |= ((value & 0x7) as u32) << 8;

        self.last_trigger = (value & 0x80) != 0;
        
        if (value & 0x80) != 0 {
            self.enabled = true;

            if self.length_counter == 0 {
                self.length_counter = 64;
            }

            self.timer = (2048 - self.frequency as i32) << 2;
            self.volume = self.volume_load;

            self.envelope_running = true;
            self.envelope_period = self.envelope_period_load;

            self.sweep_shadow = self.frequency as u16;
            self.sweep_period = self.sweep_period_load;

            if self.sweep_period == 0 {
                self.sweep_period = 8;
            }

            self.sweep_enable = (self.sweep_period > 0)||(self.sweep_shift > 0);
        }
    }

    pub fn sweep_calc(&mut self) -> u16 {
        let freq = if self.sweep_negate {
            self.sweep_shadow - (self.sweep_shadow >> self.sweep_shift)
        } else {
            self.sweep_shadow + (self.sweep_shadow >> self.sweep_shift)
        };

        if freq > 2047 {
            self.enabled = false;
        }

        freq
    }
    
    pub fn sweep_click(&mut self) {
        self.sweep_period = self.sweep_period.wrapping_sub(1);

        if self.sweep_period == 0 {
            self.sweep_period = self.sweep_period_load;

            if self.sweep_period == 0 {
                self.sweep_period = 8;
            }

            if self.sweep_enable && (self.sweep_period_load > 0) {
                let freq = self.sweep_calc();

                if (freq <= 2047) && (self.sweep_shift > 0) {
                    self.sweep_shadow = freq;
                    self.frequency = freq as u32;
                    
                }
            }
        }
    }

    pub fn envelope_click(&mut self) {
        if self.envelope_running && self.envelope_period != 0 {
            self.envelope_period -= 1;
            
            if self.envelope_period == 0 {
                self.envelope_period = self.envelope_period_load;
                
                if self.envelope_direction {
                    if self.volume < 15 { self.volume += 1; }
                } else {
                    if self.volume > 0 { self.volume -= 1; }
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

    pub fn render(&mut self) -> u8 {
        self.out_volume
    }
    
    pub fn step(&mut self) {
        self.timer = self.timer - 1;

        if self.timer <= 0 {
            self.timer = (2048 - self.frequency as i32) << 2;
            self.duty_state = (self.duty_state + 1) & 0x7;
            
            self.out_volume = if self.enabled && self.dac_enabled {
                if DUTY_TABLE[self.duty as usize][self.duty_state as usize] {
                    self.volume
                } else {
                    0
                }
            } else {
                0
            }
        }
    }
}
