// square.rs --- 
// 
// Filename: square.rs
// Author: Louise <louise>
// Created: Sat Dec 23 01:16:18 2017 (+0100)
// Last-Updated: Mon Dec 25 22:47:02 2017 (+0100)
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

    duty: u8,
    duty_state: u8,
    volume: u8,

    // Sweep
    sweep_time: u8,
    sweep_negate: bool,
    sweep_shift: u8,
    sweep_shadow: u16,

    // Length
    length_counter: u8,
    length_enable: bool,

    // Envelope
    initial_volume: u8,
    last_volume: u8,
    envelope_running: bool,
    envelope_direction: bool,
    envelope_sweep: u8,
    envelope_counter: u8,

    // Other
    
}

impl SquareChannel {
    pub fn nr0(&self) -> u8 {
        (self.sweep_time << 4) |
        ((self.sweep_negate as u8) << 3) |
        self.sweep_shift
    }
    
    pub fn set_nr0(&mut self, value: u8) {
        self.sweep_time = (value & 0xf0) >> 4;
        self.sweep_negate = (value & 0x08) != 0;
        self.sweep_shift = value & 0x07;
    }

    pub fn nr1(&self) -> u8 {
        self.duty << 6
    }

    pub fn set_nr1(&mut self, value: u8) {
        self.duty = (value & 0xc0) >> 6;
        self.length_counter = 64 - (value & 0x3F);
    }

    pub fn nr2(&self) -> u8 {
        (self.initial_volume << 4) |
        ((self.envelope_direction as u8) << 3) |
        self.envelope_sweep
    }

    pub fn set_nr2(&mut self, value: u8) {
        self.initial_volume = (value & 0xf0) >> 4;
        self.last_volume = self.initial_volume;
        
        self.envelope_direction = (value & 0x08) != 0;
        self.envelope_sweep = value & 0x07;
    }

    pub fn set_nr3(&mut self, value: u8) {
        self.frequency &= 0xff00;
        self.frequency |= value as u32;
    }

    pub fn nr4(&self) -> u8 {
        0xBF | ((self.length_enable as u8) << 6)
    }

    pub fn set_nr4(&mut self, value: u8) {
        self.length_enable = (value & 0x40) != 0;
        
        self.frequency &= 0x00ff;
        self.frequency |= ((value & 0x7) as u32) << 8;

        if (value & 0x80) != 0 {
            warn!("Behavior unimplemented");
            self.enabled = true;

            if self.length_counter == 0 {
                self.length_counter = 64;
            }

            self.timer = (2048 - self.frequency as i32) << 2;
            self.initial_volume = self.last_volume;

            self.envelope_running = true;
            self.envelope_counter = self.envelope_sweep;
        }
    }

    pub fn sweep_click(&mut self) {
        
    }

    pub fn envelope_click(&mut self) {
        if self.envelope_running && self.envelope_counter != 0 {
            self.envelope_counter -= 1;
            
            if self.envelope_counter == 0 {
                if self.envelope_direction {
                    self.initial_volume += 1;
                } else {
                    self.initial_volume -= 1;
                }

                if (self.initial_volume == 0) || (self.initial_volume == 15) {
                    self.envelope_running = false;
                }
            }
        }
    }
    
    pub fn length_click(&mut self) {
        if (self.length_counter > 0) && self.length_enable {
            self.length_counter -= 1;

            if self.length_counter == 0 {
                println!("Disabling!");
                self.enabled = false;
            }
        }
    }

    pub fn render(&mut self) -> u8 {
        self.volume
    }
    
    pub fn step(&mut self) {
        self.timer = self.timer - 1;

        if self.timer <= 0 {
            self.timer = (2048 - self.frequency as i32) << 2;
            self.duty_state = (self.duty_state + 1) & 0x7;
            
            self.volume = if self.enabled {
                if DUTY_TABLE[self.duty as usize][self.duty_state as usize] {
                    self.initial_volume
                } else {
                    0
                }
            } else {
                0
            }
        }
    }
}
