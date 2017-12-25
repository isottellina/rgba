// apu.rs --- 
// 
// Filename: apu.rs
// Author: Louise <louise>
// Created: Fri Dec  8 22:08:49 2017 (+0100)
// Last-Updated: Mon Dec 25 01:30:41 2017 (+0100)
//           By: Louise <louise>
// 
mod square;

use gb::apu::square::SquareChannel;
use common::Platform;

pub struct APU {
    channel1: SquareChannel,
    channel2: SquareChannel,
    
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

    frame_cycles: u32,
    frame_sequencer: u8,

    samples: [i16; 4096],
    samples_index: usize,
    buffer_complete: bool,
    
    downsample_count: u32,
}

impl APU {
    pub fn new() -> APU {
        APU {
            channel1: Default::default(),
            channel2: Default::default(),
            
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

            frame_cycles: 8192,
            frame_sequencer: 0,

            samples: [0; 4096],
            samples_index: 0,
            buffer_complete: false,

            downsample_count: 95,
        }
    }

    // Channel 1
    pub fn nr10(&self) -> u8 { self.channel1.nr0() }
    pub fn set_nr10(&mut self, nr10: u8) { self.channel1.set_nr0(nr10); }
    
    pub fn nr11(&self) -> u8 { self.channel1.nr1() }
    pub fn set_nr11(&mut self, nr11: u8) { self.channel1.set_nr1(nr11); }
    
    pub fn nr12(&self) -> u8 { self.channel1.nr2() }
    pub fn set_nr12(&mut self, nr12: u8) { self.channel1.set_nr2(nr12); }
    
    pub fn set_nr13(&mut self, nr13: u8) { self.channel1.set_nr3(nr13); }
    
    pub fn nr14(&self) -> u8 { self.channel1.nr4() }
    pub fn set_nr14(&mut self, nr14: u8) { self.channel1.set_nr4(nr14); }

    
    // Channel 2
    pub fn nr21(&self) -> u8 { self.channel2.nr1() }
    pub fn set_nr21(&mut self, nr21: u8) { self.channel2.set_nr1(nr21); }
    
    pub fn nr22(&self) -> u8 { self.channel2.nr2() }
    pub fn set_nr22(&mut self, nr22: u8) { self.channel2.set_nr2(nr22); }
    
    pub fn set_nr23(&mut self, nr23: u8) { self.channel2.set_nr3(nr23); }
    
    pub fn nr24(&self) -> u8 { self.channel2.nr4() }
    pub fn set_nr24(&mut self, nr24: u8) { self.channel2.set_nr4(nr24); }

    
    // Channel 3
    pub fn nr30(&self) -> u8 { self.nr30 }
    pub fn set_nr30(&mut self, nr30: u8) { self.nr30 = nr30; }
    
    pub fn nr31(&self) -> u8 { self.nr31 }
    pub fn set_nr31(&mut self, nr31: u8) { self.nr31 = nr31; }
    
    pub fn nr32(&self) -> u8 { self.nr32 }
    pub fn set_nr32(&mut self, nr32: u8) { self.nr32 = nr32; }
    
    pub fn set_nr33(&mut self, nr33: u8) { self.nr33 = nr33; }
    
    pub fn nr34(&self) -> u8 { self.nr34 }
    pub fn set_nr34(&mut self, nr34: u8) { self.nr34 = nr34; }

    
    // Channel 4
    pub fn nr41(&self) -> u8 { self.nr41 }
    pub fn set_nr41(&mut self, nr41: u8) { self.nr41 = nr41; }
    
    pub fn nr42(&self) -> u8 { self.nr42 }
    pub fn set_nr42(&mut self, nr42: u8) { self.nr42 = nr42; }
    
    pub fn nr43(&self) -> u8 { self.nr43 }
    pub fn set_nr43(&mut self, nr43: u8) { self.nr43 = nr43; }
    
    pub fn nr44(&self) -> u8 { self.nr44 }
    pub fn set_nr44(&mut self, nr44: u8) { self.nr44 = nr44; }

    
    // Control
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

    // Actual APU
    pub fn render<T: Platform>(&mut self, platform: &mut T) {
        if self.buffer_complete {
            platform.queue_samples(&self.samples);

            self.buffer_complete = false;
        }
    }
    
    pub fn step(&mut self) {
        self.frame_cycles -= 1;

        if self.frame_cycles == 0 {
            self.frame_cycles = 8192;
            
            match self.frame_sequencer {
                0 | 4 => {
                    self.channel1.length_click();
                    self.channel2.length_click();
                },
                2 | 6 => {
                    self.channel1.sweep_click();
                    self.channel1.length_click();
                    self.channel2.length_click();
                },
                7 => {
                    self.channel1.envelope_click();
                    self.channel2.envelope_click();
                },
                _ => { }
            }

            self.frame_sequencer = (self.frame_sequencer + 1) & 0x7;
        }

        self.channel1.step();
        self.channel2.step();

        self.downsample_count -= 1;

        if self.downsample_count == 0 {
            self.downsample_count = 95;

            let sample = (self.channel1.render() as u16) << 8;
            
            self.samples[self.samples_index] = sample as i16;
            self.samples_index = (self.samples_index + 1) & 0xfff;

            if self.samples_index == 0 {
                self.samples_index = 0;

                self.buffer_complete = true;
            }
        }
    }
}
