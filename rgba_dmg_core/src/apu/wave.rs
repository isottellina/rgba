// wave.rs --- 
// 
// Filename: wave.rs
// Author: Louise <louise>
// Created: Wed Dec 27 15:27:30 2017 (+0100)
// Last-Updated: Tue Jul 10 00:55:19 2018 (+0200)
//           By: Louise <ludwigette>
// 

#[derive(Debug, Default)]
pub struct WaveChannel {
    pub enabled: bool,
    
    timer: u16,
    timer_load: u16,
    frequency: u16,
    out_volume: u8,
    volume: u8,

    // Length counter
    length_counter: u8,
    length_load: u8,
    length_enable: bool,

    // Misc
    last_trigger: bool,

    wave_data: [u8; 0x20],
    wave_state: usize,
}

impl WaveChannel {
    pub fn new() -> WaveChannel {
        Default::default()
    }

    pub fn enabled(&self) -> bool { self.enabled }
    
    pub fn nr0(&self) -> u8 { (self.enabled as u8) << 7 }
    pub fn set_nr0(&mut self, nr0: u8) { self.enabled = (nr0 & 0x80) != 0; }

    pub fn nr1(&self) -> u8 { self.length_load }
    pub fn set_nr1(&mut self, nr1: u8) { self.length_load = nr1; }

    pub fn nr2(&self) -> u8 { self.volume << 5 }
    pub fn set_nr2(&mut self, nr2: u8) { self.volume = (nr2 & 0x60) >> 5; }

    pub fn set_nr3(&mut self, nr3: u8) {
        self.frequency = (self.frequency & 0x700) | (nr3 as u16);
    }

    pub fn nr4(&self) -> u8 { 0xBF | ((self.length_enable as u8) << 6) }
    pub fn set_nr4(&mut self, nr4: u8) {
        self.length_enable = (nr4 & 0x40) != 0;
        
        self.frequency = (self.frequency & 0xff) |(((nr4 & 0x7) as u16) << 8);

        self.last_trigger = (nr4 & 0x80) != 0;
        
        if (nr4 & 0x80) != 0 {
            self.enabled = true;

            if self.length_counter == 0 {
                self.length_counter = 255;
            }

            self.timer_load = (2048 - self.frequency) << 2;
            self.timer = 0;
        }
    }

    pub fn wave(&self, address: usize) -> u8 {
        (self.wave_data[((address & 0xF) << 1)] << 4) |
        self.wave_data[((address & 0xF) << 1) + 1]
    }
    
    pub fn set_wave(&mut self, address: usize, value: u8) {
        self.wave_data[((address & 0xF) << 1)] = (value & 0xF0) >> 4;
        self.wave_data[((address & 0xF) << 1) + 1] = value & 0xF;
    }
    
    pub fn length_click(&mut self) {
        if (self.length_counter > 0) && self.length_enable {
            self.length_counter -= 1;
            
            if self.length_counter == 0 {
                self.enabled = false;
            }
        }
    }

    #[inline]
    pub fn render(&self) -> u8 { self.out_volume }
    
    pub fn spend_cycles(&mut self, cycles: u16) {
        self.timer = self.timer.wrapping_add(cycles);

        if self.timer >= self.timer_load {
            self.timer -= self.timer_load;
            self.wave_state = (self.wave_state + 1) & 0x1f;
            
            self.out_volume = if self.enabled  {
                if self.volume != 0 {
                    self.wave_data[self.wave_state] >> (self.volume - 1)
                } else {
                    0
                }
            } else {
                0
            }
        }
    }
}
