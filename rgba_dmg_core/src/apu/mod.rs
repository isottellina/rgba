// apu.rs --- 
// 
// Filename: apu.rs
// Author: Louise <louise>
// Created: Fri Dec  8 22:08:49 2017 (+0100)
// Last-Updated: Thu Dec 28 00:56:37 2017 (+0100)
//           By: Louise <louise>
// 
mod square;
mod wave;
mod noise;

use apu::square::SquareChannel;
use apu::wave::WaveChannel;
use apu::noise::NoiseChannel;

use rgba_common::Platform;

pub struct APU {
    enabled: bool,
    
    channel1: SquareChannel,
    channel2: SquareChannel,
    channel3: WaveChannel,
    channel4: NoiseChannel,
  
    ch1_so1: bool,
    ch2_so1: bool,
    ch3_so1: bool,
    ch4_so1: bool,
    ch1_so2: bool,
    ch2_so2: bool,
    ch3_so2: bool,
    ch4_so2: bool,

    frame_cycles: u32,
    frame_sequencer: u8,

    samples: [i16; 1024],
    samples_index: usize,
    buffer_complete: bool,
    
    downsample_count: u32,
}

impl APU {
    pub fn new() -> APU {
        APU {
            enabled: true,
            
            channel1: SquareChannel::new(),
            channel2: SquareChannel::new(),
            channel3: WaveChannel::new(),
            channel4: NoiseChannel::new(),

            ch1_so1: false,
            ch2_so1: false,
            ch3_so1: false,
            ch4_so1: false,
            ch1_so2: false,
            ch2_so2: false,
            ch3_so2: false,
            ch4_so2: false,

            frame_cycles: 8192,
            frame_sequencer: 0,

            samples: [0; 1024],
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
    pub fn nr30(&self) -> u8 { self.channel3.nr0() }
    pub fn set_nr30(&mut self, nr30: u8) { self.channel3.set_nr0(nr30); }
    
    pub fn nr31(&self) -> u8 { self.channel3.nr1() }
    pub fn set_nr31(&mut self, nr31: u8) { self.channel3.set_nr1(nr31); }
    
    pub fn nr32(&self) -> u8 { self.channel3.nr2() }
    pub fn set_nr32(&mut self, nr32: u8) { self.channel3.set_nr2(nr32); }
    
    pub fn set_nr33(&mut self, nr33: u8) { self.channel3.set_nr3(nr33); }
    
    pub fn nr34(&self) -> u8 { self.channel3.nr4() }
    pub fn set_nr34(&mut self, nr34: u8) { self.channel3.set_nr4(nr34); }

    
    // Channel 4
    pub fn nr41(&self) -> u8 { self.channel4.nr1() }
    pub fn set_nr41(&mut self, nr41: u8) { self.channel4.set_nr1(nr41); }
    
    pub fn nr42(&self) -> u8 { self.channel4.nr2() }
    pub fn set_nr42(&mut self, nr42: u8) { self.channel4.set_nr2(nr42); }
    
    pub fn nr43(&self) -> u8 { self.channel4.nr3() }
    pub fn set_nr43(&mut self, nr43: u8) { self.channel4.set_nr3(nr43); }
    
    pub fn nr44(&self) -> u8 { self.channel4.nr4() }
    pub fn set_nr44(&mut self, nr44: u8) { self.channel4.set_nr4(nr44); }

    
    // Control
    pub fn nr50(&self) -> u8 { 0 }
    pub fn set_nr50(&mut self, nr50: u8) {
        if (nr50 & 0x88) != 0 {
            warn!("Vin is enabled; This emulator has no Vin support.");
        }
    }
    
    pub fn nr51(&self) -> u8 {
        ((self.ch4_so2 as u8) << 7) |
        ((self.ch3_so2 as u8) << 6) |
        ((self.ch2_so2 as u8) << 5) |
        ((self.ch1_so2 as u8) << 4) |
        ((self.ch4_so1 as u8) << 3) |
        ((self.ch3_so1 as u8) << 2) |
        ((self.ch2_so1 as u8) << 1) |
        (self.ch1_so1 as u8)
    }
    
    pub fn set_nr51(&mut self, nr51: u8) {
        self.ch1_so1 = (nr51 & 0x01) != 0;
        self.ch2_so1 = (nr51 & 0x02) != 0;
        self.ch3_so1 = (nr51 & 0x04) != 0;
        self.ch4_so1 = (nr51 & 0x08) != 0;
        self.ch1_so2 = (nr51 & 0x10) != 0;
        self.ch2_so2 = (nr51 & 0x20) != 0;
        self.ch3_so2 = (nr51 & 0x40) != 0;
        self.ch4_so2 = (nr51 & 0x80) != 0;
    }
    
    pub fn nr52(&self) -> u8 {
        ((self.enabled as u8) << 7) |
        ((self.channel3.enabled() as u8) << 2) |
        ((self.channel2.enabled() as u8) << 1) |
        (self.channel1.enabled() as u8)
    }
    
    pub fn set_nr52(&mut self, nr52: u8) {
        self.enabled = (nr52 & 0x80) != 0;
    }

    pub fn nr3_wave(&self, address: usize) -> u8 { self.channel3.wave(address) }
    pub fn set_nr3_wave(&mut self, address: usize, value: u8) {
        self.channel3.set_wave(address, value);
    }

    // Actual APU
    pub fn enabled(&self) -> bool {
        self.enabled
    }
    
    pub fn render<T: Platform>(&mut self, platform: &mut T) {
        if self.buffer_complete {
            platform.queue_samples(&self.samples);

            self.buffer_complete = false;
        }
    }

    fn get_so1(&self) -> u8 {
        let mut so1 = 0;

        if self.ch1_so1 {
            so1 += self.channel1.render();
        }

        if self.ch2_so1 {
            so1 += self.channel2.render();
        }
        
        if self.ch3_so1 {
            so1 += self.channel3.render();
        }

        if self.ch4_so1 {
            so1 += self.channel4.render();
        }
        
        so1
    }

    fn get_so2(&self) -> u8 {
        let mut so2 = 0;

        if self.ch1_so2 {
            so2 += self.channel1.render();
        }

        if self.ch2_so2 {
            so2 += self.channel2.render();
        }
        
        if self.ch3_so2 {
            so2 += self.channel3.render();
        }

        if self.ch4_so2 {
            so2 += self.channel4.render();
        }
        
        so2
    }
    
    pub fn step(&mut self) {
        self.frame_cycles = self.frame_cycles - 1;

        if self.frame_cycles == 0 {
            self.frame_cycles = 8192;
            
            match self.frame_sequencer {
                0 | 4 => {
                    self.channel1.length_click();
                    self.channel2.length_click();
                    self.channel3.length_click();
                    self.channel4.length_click();
                },
                2 | 6 => {
                    self.channel1.sweep_click();
                    self.channel1.length_click();
                    self.channel2.length_click();
                    self.channel3.length_click();
                    self.channel4.length_click();
                },
                7 => {
                    self.channel1.envelope_click();
                    self.channel2.envelope_click();
                    self.channel4.envelope_click();
                },
                _ => { }
            }

            self.frame_sequencer = (self.frame_sequencer + 1) & 0x7;
        }

        self.channel1.step();
        self.channel2.step();
        self.channel3.step();
        self.channel4.step();

        self.downsample_count = (self.downsample_count + 1) % 87;

        if self.downsample_count == 0 {
            let so1 = self.get_so1() as u16;
            let so2 = self.get_so2() as u16;

            let mix = (so1 + so2) << 7;
            
            self.samples[self.samples_index] = mix as i16;
            self.samples_index = (self.samples_index + 1) & 0x3ff;

            if self.samples_index == 0 {
                self.samples_index = 0;

                self.buffer_complete = true;
            }
        }
    }
}
