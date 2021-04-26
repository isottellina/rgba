// mod.rs --- 
// 
// Filename: mod.rs
// Author: Louise <louise>
// Created: Wed Dec  6 14:33:34 2017 (+0100)
// Last-Updated: Tue Jul  9 14:17:14 2019 (+0200)
//           By: Louise <ludwigette>
//
#[macro_use] extern crate log;
extern crate rgba_common;

mod cpu;
mod gpu;
mod apu;
mod timer;
mod joypad;
mod io;
mod cart;
mod debug;

use rgba_common::{Core, Platform, Event, Console};
use rgba_common::fnv_hash;
use crate::cpu::LR35902;
use crate::io::Interconnect;
use crate::debug::Debugger;

use std::thread;
use std::fs::File;
use std::time::{Instant, Duration};
use std::io::{Seek, SeekFrom, Read};

pub struct Gameboy {
    cpu: LR35902,
    io: Interconnect,

    state: bool,
    fast_mode: bool,
    last_frame: Instant,
}

impl Gameboy {
    pub fn new() -> Gameboy {
        Gameboy {
            cpu: LR35902::new(),
            io: Interconnect::new(),

            state: false,
            fast_mode: false,
            last_frame: Instant::now(),
        }
    }

    fn reset(&mut self) {
        self.cpu.reset();
        self.io.reset();
    }
    
    fn on_frame<T: Platform>(&mut self,
                             debugger: &mut Debugger,
                             platform: &mut T) {
        let elapsed = self.last_frame.elapsed();
                
        if !self.fast_mode && elapsed < Duration::new(0, 16_600_000) {
            let to_wait = Duration::new(0, 16_600_000) - elapsed;

            if to_wait > Duration::new(0, 600_000) {
                thread::sleep(to_wait);
            }
        }
        
        let new_elapsed = self.last_frame.elapsed();
        let elapsed_nanos = new_elapsed.as_secs() * 1_000_000_000 +
            u64::from(new_elapsed.subsec_nanos());
        
        let s = format!(
            "rGBA [{}/60]",
            ((1.0 / (elapsed_nanos as f64)) * 1000000000.0).round() as u32
        );
        
        platform.set_title(s);
        
        self.last_frame = Instant::now();
        
        while let Some(e) = platform.poll_event() {
            match e {
                Event::Quit => self.state = false,
                Event::FastMode => {
                    self.fast_mode = !self.fast_mode;
                    self.io.set_sound_enabled(!self.fast_mode);
                },
                Event::Debug => debugger.trigger(),
                Event::Reset => self.reset(),
                _ => self.io.handle_event(e),
            }
        }
        
        platform.present();
    }
}

impl Core for Gameboy {
    fn run<T: Platform>(&mut self, platform: &mut T, debug: bool) {
        let mut debugger = Debugger::new(debug);
        self.state = true;
        
        while self.state {
            debugger.handle(self, platform);
            
            self.cpu.step(&mut self.io);
            self.io.spend_cycles();
            self.io.render(platform);

            if self.io.is_frame_done() {
                self.on_frame(&mut debugger, platform);
                self.io.ack_frame();
            }
        }

        self.io.write_savefile();
    }
    
    fn is_file(filename: &str) -> bool {
        match File::open(filename) {
            Ok(mut file) => {
                let mut logo: [u8; 0x30] = [0; 0x30];
                
                if let Err(e) = file.seek(SeekFrom::Start(0x104)) {
                    warn!("Couldn't seek in ROM file : {}", e);

                    false
                } else if let Err(e) = file.read_exact(&mut logo) {
                    warn!("Couldn't read ROM file : {}", e);
                    
                    false
                } else {
                    fnv_hash(&logo) == 0x8fcb_d5b7
                }
            },

            Err(e) => {
                warn!("Couldn't open ROM file : {}", e);
                false
            }
        }
    }
    
    fn load_bios<T: ToString>(&mut self, filename: Option<T>) -> Result<(), &'static str> {
        match filename {
            Some(f) => self.io.load_bios(&f.to_string()),
            None => panic!("A bootrom is mandatory for GB games.")
        }
    }

    fn load_rom(&mut self, filename: &str) -> bool {
        self.io.load_rom(filename)
    }

    fn get_platform_parameters() -> (u32, u32) {
        (160, 144)
    }

    fn get_console_type() -> Console {
        Console::Gameboy
    }
}

impl Default for Gameboy {
    fn default() -> Gameboy { Gameboy::new() }
}
