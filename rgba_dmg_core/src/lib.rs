// mod.rs --- 
// 
// Filename: mod.rs
// Author: Louise <louise>
// Created: Wed Dec  6 14:33:34 2017 (+0100)
// Last-Updated: Wed Dec 27 23:57:28 2017 (+0100)
//           By: Louise <louise>
//
#[macro_use] extern crate log;
extern crate readline;
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
use cpu::LR35902;
use io::Interconnect;
use debug::Debugger;

use std::thread;
use std::fs::File;
use std::time::{Instant, Duration};
use std::io::{Seek, SeekFrom, Read};

pub struct Gameboy {
    cpu: LR35902,
    io: Interconnect,

    state: bool,
    last_frame: Instant,
}

impl Gameboy {
    pub fn new() -> Gameboy {
        Gameboy {
            cpu: LR35902::new(),
            io: Interconnect::new(),

            state: false,
            last_frame: Instant::now(),
        }
    }

    fn reset(&mut self) {
        self.cpu.reset();
        self.io.reset();
    }
    
    fn on_frame<T: Platform>(&mut self, debugger: &mut Debugger,
                             platform: &mut T) {
        let elapsed = self.last_frame.elapsed();
                
        if elapsed < Duration::new(0, 16600000) {
            let to_wait = Duration::new(0, 16600000) - elapsed;
            
            thread::sleep(to_wait);
        }
        
        let new_elapsed = self.last_frame.elapsed();
        let elapsed_nanos = new_elapsed.as_secs() * 1000000000 +
            new_elapsed.subsec_nanos() as u64;
        
        let s = format!(
            "rGBA ({:.4} FPS)",
            (1.0 / (elapsed_nanos as f64)) * 1000000000.0
        );
        
        platform.set_title(s);
        
        self.last_frame = Instant::now();
        
        while let Some(e) = platform.poll_event() {
            match e {
                Event::Quit => self.state = false,
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
            debugger.handle(self);
            
            self.cpu.step(&mut self.io);
            self.io.render(platform);

            if self.io.is_frame_done() {
                self.on_frame(&mut debugger, platform);
                self.io.ack_frame();
            }
        }
    }
    
    fn is_file(filename: &str) -> bool {
        match File::open(filename) {
            Ok(mut file) => {
                let mut logo: [u8; 0x30] = [0; 0x30];
                let mut hash: u32 = 0x811c9dc5;
                
                if let Err(e) = file.seek(SeekFrom::Start(0x104)) {
                    warn!("Couldn't seek in ROM file : {}", e);

                    false
                } else {
                    if let Err(e) = file.read_exact(&mut logo) {
                        warn!("Couldn't read ROM file : {}", e);

                        false
                    } else {
                        for byte in logo.iter() {
                            hash = hash.wrapping_mul(16777619);
                            hash ^= (*byte) as u32;
                        }
                    
                        hash == 0x8fcbd5b7
                    }
                }
            },

            Err(e) => {
                warn!("Couldn't open ROM file : {}", e);
                false
            }
        }
    }
    
    fn load_bios(&mut self, filename: &str) -> Result<(), &'static str> {
        self.io.load_bios(filename)
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
